use std::ops::{Range, RangeFrom, RangeTo};

/// 大元のバイト列からのオフセットを保持するバイトスライス。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Bytes<'buf> {
    buf: &'buf [u8],
    base_offset: usize,
}

impl<'buf> Bytes<'buf> {
    pub(crate) fn new(buf: &'buf [u8]) -> Self {
        Self::_new(buf, 0)
    }

    fn _new(buf: &'buf [u8], base_offset: usize) -> Self {
        Self { buf, base_offset }
    }

    pub(crate) fn base_offset(self) -> usize {
        self.base_offset
    }

    pub(crate) fn as_slice(self) -> &'buf [u8] {
        self.buf
    }

    pub(crate) fn is_empty(self) -> bool {
        self.buf.is_empty()
    }

    pub(crate) fn len(self) -> usize {
        self.buf.len()
    }

    pub(crate) fn get(self, idx: usize) -> Option<&'buf u8> {
        self.buf.get(idx)
    }

    #[allow(dead_code)]
    pub(crate) fn range(self, range: Range<usize>) -> Self {
        let base_offset = self.base_offset + range.start;
        let buf = &self.buf[range];
        Self::_new(buf, base_offset)
    }

    pub(crate) fn range_from(self, range: RangeFrom<usize>) -> Self {
        let base_offset = self.base_offset + range.start;
        let buf = &self.buf[range];
        Self::_new(buf, base_offset)
    }

    pub(crate) fn range_to(self, range: RangeTo<usize>) -> Self {
        let buf = &self.buf[range];
        Self::_new(buf, self.base_offset)
    }

    pub(crate) fn iter(self) -> std::slice::Iter<'buf, u8> {
        self.into_iter()
    }

    pub(crate) fn split_at(self, mid: usize) -> (Self, Self) {
        let (buf_l, buf_r) = self.buf.split_at(mid);
        let l = Self::_new(buf_l, self.base_offset);
        let r = Self::_new(buf_r, self.base_offset + mid);
        (l, r)
    }

    pub(crate) fn try_split_at(self, mid: usize) -> Option<(Self, Self)> {
        (mid <= self.len()).then(|| self.split_at(mid))
    }

    pub(crate) fn split<P: FnMut(&u8) -> bool>(self, pred: P) -> Split<'buf, P> {
        Split::new(self, pred)
    }

    pub(crate) fn tokens(self) -> Tokens<'buf> {
        Tokens::new(self)
    }
}

impl<'s> From<&'s str> for Bytes<'s> {
    fn from(s: &'s str) -> Self {
        Self::new(s.as_bytes())
    }
}

impl std::ops::Index<usize> for Bytes<'_> {
    type Output = u8;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.buf[idx]
    }
}

impl<'buf> IntoIterator for Bytes<'buf> {
    type Item = &'buf u8;
    type IntoIter = std::slice::Iter<'buf, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.buf.iter()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Split<'buf, P> {
    bytes: Bytes<'buf>,
    pred: P,
    finished: bool,
}

impl<'buf, P: FnMut(&u8) -> bool> Split<'buf, P> {
    fn new(bytes: Bytes<'buf>, pred: P) -> Self {
        Self {
            bytes,
            pred,
            finished: false,
        }
    }
}

impl<'buf, P: FnMut(&u8) -> bool> Iterator for Split<'buf, P> {
    type Item = Bytes<'buf>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        if let Some(pos) = self.bytes.iter().position(|b| (self.pred)(b)) {
            let item = self.bytes.range_to(..pos);
            self.bytes = self.bytes.range_from(pos + 1..);
            Some(item)
        } else {
            self.finished = true;
            Some(self.bytes)
        }
    }
}

impl<P: FnMut(&u8) -> bool> std::iter::FusedIterator for Split<'_, P> {}

/// 非 ASCII space からなるトークンを切り出す。
#[derive(Clone, Debug)]
pub(crate) struct Tokens<'buf> {
    bytes: Bytes<'buf>,
}

impl<'buf> Tokens<'buf> {
    fn new(bytes: Bytes<'buf>) -> Self {
        Self { bytes }
    }

    /// 残りのスライスを返す。戻り値の先頭に ASCII space は付かない。
    pub(crate) fn remain(self) -> Bytes<'buf> {
        let start = self
            .bytes
            .iter()
            .position(|&b| b != b' ')
            .unwrap_or_else(|| self.bytes.len());

        self.bytes.range_from(start..)
    }
}

impl<'buf> Iterator for Tokens<'buf> {
    type Item = Bytes<'buf>;

    fn next(&mut self) -> Option<Self::Item> {
        // 最初の非 ASCII space 文字を探し、そこを開始位置とする。
        // 見つからないまま終端に達したらトークンは尽きている。
        let bytes = {
            let start = self.bytes.iter().position(|&b| b != b' ')?;
            self.bytes.range_from(start..)
        };

        // 最初の ASCII space 文字を探し、そこを終了位置とする。
        // 見つからないまま終端に達したらそこが終了位置となる。
        let end = bytes
            .iter()
            .position(|&b| b == b' ')
            .unwrap_or_else(|| bytes.len());

        let item;
        (item, self.bytes) = bytes.split_at(end);

        Some(item)
    }
}

impl std::iter::FusedIterator for Tokens<'_> {}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn test_tokens() {
        assert_eq!(Bytes::new(b"").tokens().next(), None);
        assert_eq!(Bytes::new(b"   ").tokens().next(), None);

        assert_equal(
            Bytes::new(b"foo bar hoge").tokens(),
            [
                Bytes::_new(b"foo", 0),
                Bytes::_new(b"bar", 4),
                Bytes::_new(b"hoge", 8),
            ],
        );

        assert_equal(
            Bytes::new(b"   foo  bar   hoge  ").tokens(),
            [
                Bytes::_new(b"foo", 3),
                Bytes::_new(b"bar", 8),
                Bytes::_new(b"hoge", 14),
            ],
        );

        {
            let mut tokens = Bytes::new(b"foo").tokens();
            assert_eq!(tokens.next(), Some(Bytes::_new(b"foo", 0)));
            assert_eq!(tokens.remain(), Bytes::_new(b"", 3));
        }
        {
            let mut tokens = Bytes::new(b" foo bar  remain ").tokens();
            assert_eq!(tokens.next(), Some(Bytes::_new(b"foo", 1)));
            assert_eq!(tokens.next(), Some(Bytes::_new(b"bar", 5)));
            assert_eq!(tokens.remain(), Bytes::_new(b"remain ", 10));
        }
    }

    #[test]
    fn test_split() {
        assert_equal(Bytes::new(b"").split(|&b| b == b'o'), [Bytes::_new(b"", 0)]);
        assert_equal(
            Bytes::new(b"o").split(|&b| b == b'o'),
            [Bytes::_new(b"", 0), Bytes::_new(b"", 1)],
        );
        assert_equal(
            Bytes::new(b"oo").split(|&b| b == b'o'),
            [
                Bytes::_new(b"", 0),
                Bytes::_new(b"", 1),
                Bytes::_new(b"", 2),
            ],
        );

        assert_equal(
            Bytes::new(b"foo/bar/hoge").split(|&b| b == b'/'),
            [
                Bytes::_new(b"foo", 0),
                Bytes::_new(b"bar", 4),
                Bytes::_new(b"hoge", 8),
            ],
        );
    }
}
