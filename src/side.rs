use std::fmt::Write as _;

use crate::bytes::Bytes;
use crate::parse::*;

/// 陣営。
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Side {
    Sente = 0,
    Gote,
}

pub const SENTE: Side = Side::Sente;
pub const GOTE: Side = Side::Gote;

impl Side {
    const NUM: usize = 2;

    const fn to_index(self) -> usize {
        self as usize
    }

    /// 両陣営を返す。順序は未規定。
    pub const fn all() -> [Self; Self::NUM] {
        [SENTE, GOTE]
    }

    pub(crate) fn parse(bytes: Bytes) -> SfenParseResult<(Bytes, Self)> {
        // 'b' | 'w'

        let (bytes, remain) = bytes
            .try_split_at(1)
            .ok_or_else(|| SfenParseError::invalid_input(bytes, "`Side` ('b' | 'w') expected"))?;

        let side = match bytes[0] {
            b'b' => SENTE,
            b'w' => GOTE,
            _ => {
                return Err(SfenParseError::invalid_input(
                    bytes,
                    "`Side` ('b' | 'w') expected",
                ))
            }
        };

        Ok((remain, side))
    }
}

impl std::str::FromStr for Side {
    type Err = SfenParseError;

    /// SFEN 陣営文字列をパースする。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser_complete(Self::parse)(Bytes::from(s))
    }
}

impl std::fmt::Display for Side {
    /// SFEN 陣営文字列を出力する。
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match *self {
            SENTE => 'b',
            GOTE => 'w',
        };
        f.write_char(c)
    }
}

/// `Side` でインデックスアクセスできる配列。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ArraySide<T>([T; Side::NUM]);

impl<T> ArraySide<T> {
    pub(crate) const fn new(inner: [T; Side::NUM]) -> Self {
        Self(inner)
    }

    pub(crate) const fn index_const(&self, side: Side) -> &T {
        &self.0[side.to_index()]
    }
}

impl<T: Copy> ArraySide<T> {
    #[allow(dead_code)]
    pub(crate) const fn from_elem(elem: T) -> Self {
        Self([elem; Side::NUM])
    }
}

impl<T: Copy + Default> Default for ArraySide<T> {
    fn default() -> Self {
        Self([T::default(); Side::NUM])
    }
}

impl<T> std::ops::Index<Side> for ArraySide<T> {
    type Output = T;

    fn index(&self, side: Side) -> &Self::Output {
        self.index_const(side)
    }
}

impl<T> std::ops::IndexMut<Side> for ArraySide<T> {
    fn index_mut(&mut self, side: Side) -> &mut Self::Output {
        &mut self.0[side.to_index()]
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    #[test]
    fn test_side_parse() {
        assert_eq!(Side::from_str("b").unwrap(), SENTE);
        assert_eq!(Side::from_str("w").unwrap(), GOTE);

        assert!(matches! {
            Side::from_str(""),
            Err(SfenParseError::InvalidInput {
                offset: 0, ..
            })
        });
        assert!(matches! {
            Side::from_str("B"),
            Err(SfenParseError::InvalidInput {
                offset: 0, ..
            })
        });
        assert!(matches! {
            Side::from_str("bw"),
            Err(SfenParseError::Extra{
                offset: 1
            })
        });
    }

    #[test]
    fn test_side_fmt() {
        assert_eq!(SENTE.to_string(), "b");
        assert_eq!(GOTE.to_string(), "w");
    }
}
