use std::fmt::Write as _;
use std::num::NonZeroU8;

use crate::bytes::Bytes;
use crate::parse::*;
use crate::piece::*;
use crate::side::*;

/// 手駒の駒種。
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HandPieceKind {
    Pawn = 0,
    Lance,
    Knight,
    Silver,
    Bishop,
    Rook,
    Gold,
}

pub const HAND_PAWN: HandPieceKind = HandPieceKind::Pawn;
pub const HAND_LANCE: HandPieceKind = HandPieceKind::Lance;
pub const HAND_KNIGHT: HandPieceKind = HandPieceKind::Knight;
pub const HAND_SILVER: HandPieceKind = HandPieceKind::Silver;
pub const HAND_BISHOP: HandPieceKind = HandPieceKind::Bishop;
pub const HAND_ROOK: HandPieceKind = HandPieceKind::Rook;
pub const HAND_GOLD: HandPieceKind = HandPieceKind::Gold;

impl HandPieceKind {
    const NUM: usize = 7;

    const fn to_index(self) -> usize {
        self as usize
    }

    /// 全ての手駒の駒種を返す。順序は未規定。
    pub const fn all() -> [Self; Self::NUM] {
        [
            HAND_PAWN,
            HAND_LANCE,
            HAND_KNIGHT,
            HAND_SILVER,
            HAND_BISHOP,
            HAND_ROOK,
            HAND_GOLD,
        ]
    }

    pub(crate) fn parse(bytes: Bytes) -> SfenParseResult<(Bytes, Self)> {
        let (bytes, remain) = bytes
            .try_split_at(1)
            .ok_or_else(|| SfenParseError::invalid_input(bytes, "hand piece kind expected"))?;

        let hpk = match bytes[0] {
            b'P' => HAND_PAWN,
            b'L' => HAND_LANCE,
            b'N' => HAND_KNIGHT,
            b'S' => HAND_SILVER,
            b'B' => HAND_BISHOP,
            b'R' => HAND_ROOK,
            b'G' => HAND_GOLD,
            _ => {
                return Err(SfenParseError::invalid_input(
                    bytes,
                    "hand piece kind expected",
                ))?
            }
        };

        Ok((remain, hpk))
    }
}

impl From<HandPieceKind> for PieceKind {
    fn from(hpk: HandPieceKind) -> Self {
        match hpk {
            HAND_PAWN => PAWN,
            HAND_LANCE => LANCE,
            HAND_KNIGHT => KNIGHT,
            HAND_SILVER => SILVER,
            HAND_BISHOP => BISHOP,
            HAND_ROOK => ROOK,
            HAND_GOLD => GOLD,
        }
    }
}

impl TryFrom<PieceKind> for HandPieceKind {
    type Error = NotHandPieceKindError;

    fn try_from(pk: PieceKind) -> Result<Self, Self::Error> {
        match pk {
            PAWN => Ok(HAND_PAWN),
            LANCE => Ok(HAND_LANCE),
            KNIGHT => Ok(HAND_KNIGHT),
            SILVER => Ok(HAND_SILVER),
            BISHOP => Ok(HAND_BISHOP),
            ROOK => Ok(HAND_ROOK),
            GOLD => Ok(HAND_GOLD),
            _ => Err(NotHandPieceKindError(pk)),
        }
    }
}

/// 駒種が手駒となりえないことを表すエラー。
#[derive(Clone, Debug)]
pub struct NotHandPieceKindError(PieceKind);

impl std::fmt::Display for NotHandPieceKindError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "not hand piece kind: {:?}", self.0)
    }
}

impl std::error::Error for NotHandPieceKindError {}

impl std::str::FromStr for HandPieceKind {
    type Err = SfenParseError;

    /// SFEN 手駒の駒種文字列をパースする。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser_complete(Self::parse)(Bytes::from(s))
    }
}

impl std::fmt::Display for HandPieceKind {
    /// SFEN 手駒の駒種文字列を出力する。
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match *self {
            HAND_PAWN => 'P',
            HAND_LANCE => 'L',
            HAND_KNIGHT => 'N',
            HAND_SILVER => 'S',
            HAND_BISHOP => 'B',
            HAND_ROOK => 'R',
            HAND_GOLD => 'G',
        };
        f.write_char(c)
    }
}

/// `HandPieceKind` でインデックスアクセスできる配列。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ArrayHandPieceKind<T>([T; HandPieceKind::NUM]);

impl<T> ArrayHandPieceKind<T> {
    #[allow(dead_code)]
    pub(crate) const fn new(inner: [T; HandPieceKind::NUM]) -> Self {
        Self(inner)
    }

    pub(crate) const fn index_const(&self, hpk: HandPieceKind) -> &T {
        &self.0[hpk.to_index()]
    }
}

impl<T: Copy> ArrayHandPieceKind<T> {
    pub(crate) const fn from_elem(elem: T) -> Self {
        Self([elem; HandPieceKind::NUM])
    }
}

impl<T: Copy + Default> Default for ArrayHandPieceKind<T> {
    fn default() -> Self {
        Self([T::default(); HandPieceKind::NUM])
    }
}

impl<T> std::ops::Index<HandPieceKind> for ArrayHandPieceKind<T> {
    type Output = T;

    fn index(&self, hpk: HandPieceKind) -> &Self::Output {
        self.index_const(hpk)
    }
}

impl<T> std::ops::IndexMut<HandPieceKind> for ArrayHandPieceKind<T> {
    fn index_mut(&mut self, hpk: HandPieceKind) -> &mut Self::Output {
        &mut self.0[hpk.to_index()]
    }
}

/// 一方の陣営の手駒。
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Hand(ArrayHandPieceKind<u8>);

impl Hand {
    /// 空の手駒を返す。
    pub const fn empty() -> Self {
        Self(ArrayHandPieceKind::from_elem(0))
    }
}

impl std::ops::Index<HandPieceKind> for Hand {
    type Output = u8;

    fn index(&self, hpk: HandPieceKind) -> &Self::Output {
        &self.0[hpk]
    }
}

impl std::ops::IndexMut<HandPieceKind> for Hand {
    fn index_mut(&mut self, hpk: HandPieceKind) -> &mut Self::Output {
        &mut self.0[hpk]
    }
}

/// 両陣営の手駒。
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Hands(ArraySide<Hand>);

impl Hands {
    /// 両陣営とも空の手駒を返す。
    pub const fn empty() -> Self {
        Self(ArraySide::new([Hand::empty(), Hand::empty()]))
    }

    fn is_empty(&self) -> bool {
        *self == Self::empty()
    }

    /// 入力の終端まで読む。
    pub(crate) fn parse(mut bytes: Bytes) -> SfenParseResult<(Bytes, Self)> {
        // ('-' | HandsElem+) EOF

        if bytes.is_empty() {
            return Err(SfenParseError::invalid_input(bytes, "`Hands` expected"));
        }

        if bytes.as_slice() == b"-" {
            return Ok((bytes.range_from(bytes.len()..), Self::empty()));
        }

        let mut hands = Self::empty();

        while !bytes.is_empty() {
            let elem;
            (bytes, elem) = HandsElem::parse(bytes)?;
            let HandsElem { side, hpk, count } = elem;

            hands[side][hpk] = hands[side][hpk].saturating_add(count.get());
        }

        Ok((bytes, hands))
    }

    fn fmt_hand_piece(
        side: Side,
        hpk: HandPieceKind,
        sink: &mut impl std::fmt::Write,
    ) -> std::fmt::Result {
        let c = match (side, hpk) {
            (SENTE, HAND_PAWN) => 'P',
            (SENTE, HAND_LANCE) => 'L',
            (SENTE, HAND_KNIGHT) => 'N',
            (SENTE, HAND_SILVER) => 'S',
            (SENTE, HAND_BISHOP) => 'B',
            (SENTE, HAND_ROOK) => 'R',
            (SENTE, HAND_GOLD) => 'G',
            (GOTE, HAND_PAWN) => 'p',
            (GOTE, HAND_LANCE) => 'l',
            (GOTE, HAND_KNIGHT) => 'n',
            (GOTE, HAND_SILVER) => 's',
            (GOTE, HAND_BISHOP) => 'b',
            (GOTE, HAND_ROOK) => 'r',
            (GOTE, HAND_GOLD) => 'g',
        };
        sink.write_char(c)
    }
}

impl std::ops::Index<Side> for Hands {
    type Output = Hand;

    fn index(&self, side: Side) -> &Self::Output {
        &self.0[side]
    }
}

impl std::ops::IndexMut<Side> for Hands {
    fn index_mut(&mut self, side: Side) -> &mut Self::Output {
        &mut self.0[side]
    }
}

impl std::str::FromStr for Hands {
    type Err = SfenParseError;

    /// SFEN 両陣営の手駒文字列をパースする。
    ///
    /// 厳密な仕様に従っていない入力も許す:
    ///
    /// * 駒種の順序は任意。
    /// * 同じ駒種が複数回現れたら単に合算する。
    /// * 個数として 1 も許す。
    /// * 個数は制限しない(ただし 1 要素あたり 2 桁まで)。オーバーフローする場合、saturate する。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser_complete(Self::parse)(Bytes::from(s))
    }
}

impl std::fmt::Display for Hands {
    /// SFEN 両陣営の手駒文字列を出力する。
    ///
    /// 駒種の順序は SFEN の仕様に従う。
    ///
    /// ref: <https://yaneuraou.yaneu.com/2016/07/15/sfen%E6%96%87%E5%AD%97%E5%88%97%E3%81%AF%E6%9C%AC%E6%9D%A5%E3%81%AF%E4%B8%80%E6%84%8F%E3%81%AB%E5%AE%9A%E3%81%BE%E3%82%8B%E4%BB%B6/>
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const HPKS: [HandPieceKind; 7] = [
            HAND_ROOK,
            HAND_BISHOP,
            HAND_GOLD,
            HAND_SILVER,
            HAND_KNIGHT,
            HAND_LANCE,
            HAND_PAWN,
        ];

        if self.is_empty() {
            return f.write_char('-');
        }

        for side in [SENTE, GOTE] {
            for hpk in HPKS {
                let n = self[side][hpk];
                if n == 0 {
                    continue;
                }

                if n > 1 {
                    n.fmt(f)?;
                }
                Self::fmt_hand_piece(side, hpk, f)?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct HandsElem {
    side: Side,
    hpk: HandPieceKind,
    count: NonZeroU8,
}

impl HandsElem {
    fn parse(bytes: Bytes) -> SfenParseResult<(Bytes, Self)> {
        // Count? HandPiece

        let (bytes, count) =
            Self::parse_count(bytes).unwrap_or((bytes, unsafe { NonZeroU8::new_unchecked(1) }));
        let (remain, (side, hpk)) = Self::parse_hand_piece(bytes)?;

        let elem = HandsElem { side, hpk, count };

        Ok((remain, elem))
    }

    fn parse_count(bytes: Bytes) -> SfenParseResult<(Bytes, NonZeroU8)> {
        // [1-9] [0-9]?

        let (first, bytes) = bytes
            .try_split_at(1)
            .ok_or_else(|| SfenParseError::invalid_input(bytes, "hand piece count expected"))?;
        let count = match first[0] {
            b @ b'1'..=b'9' => b - b'0',
            _ => {
                return Err(SfenParseError::invalid_input(
                    first,
                    "hand piece count expected",
                ))
            }
        };

        let (remain, count) = match bytes.get(0) {
            Some(b @ b'0'..=b'9') => {
                let count = 10 * count + b - b'0';
                (bytes.range_from(1..), count)
            }
            _ => (bytes, count),
        };
        let count = NonZeroU8::new(count).unwrap();

        Ok((remain, count))
    }

    fn parse_hand_piece(bytes: Bytes) -> SfenParseResult<(Bytes, (Side, HandPieceKind))> {
        let (bytes, remain) = bytes
            .try_split_at(1)
            .ok_or_else(|| SfenParseError::invalid_input(bytes, "hand piece expected"))?;

        let (side, hpk) = match bytes[0] {
            b'P' => (SENTE, HAND_PAWN),
            b'L' => (SENTE, HAND_LANCE),
            b'N' => (SENTE, HAND_KNIGHT),
            b'S' => (SENTE, HAND_SILVER),
            b'B' => (SENTE, HAND_BISHOP),
            b'R' => (SENTE, HAND_ROOK),
            b'G' => (SENTE, HAND_GOLD),
            b'p' => (GOTE, HAND_PAWN),
            b'l' => (GOTE, HAND_LANCE),
            b'n' => (GOTE, HAND_KNIGHT),
            b's' => (GOTE, HAND_SILVER),
            b'b' => (GOTE, HAND_BISHOP),
            b'r' => (GOTE, HAND_ROOK),
            b'g' => (GOTE, HAND_GOLD),
            _ => return Err(SfenParseError::invalid_input(bytes, "hand piece expected")),
        };

        Ok((remain, (side, hpk)))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    fn make_hands(elems: impl IntoIterator<Item = (Side, HandPieceKind, u8)>) -> Hands {
        let mut hands = Hands::empty();

        for (side, hpk, n) in elems.into_iter() {
            hands[side][hpk] = n;
        }

        hands
    }

    #[test]
    fn test_hand_piece_kind_parse() {
        assert_eq!(HandPieceKind::from_str("P").unwrap(), HAND_PAWN);
        assert_eq!(HandPieceKind::from_str("L").unwrap(), HAND_LANCE);
        assert_eq!(HandPieceKind::from_str("N").unwrap(), HAND_KNIGHT);
        assert_eq!(HandPieceKind::from_str("S").unwrap(), HAND_SILVER);
        assert_eq!(HandPieceKind::from_str("B").unwrap(), HAND_BISHOP);
        assert_eq!(HandPieceKind::from_str("R").unwrap(), HAND_ROOK);
        assert_eq!(HandPieceKind::from_str("G").unwrap(), HAND_GOLD);
    }

    #[test]
    fn test_hand_piece_kind_fmt() {
        assert_eq!(HAND_PAWN.to_string(), "P");
        assert_eq!(HAND_LANCE.to_string(), "L");
        assert_eq!(HAND_KNIGHT.to_string(), "N");
        assert_eq!(HAND_SILVER.to_string(), "S");
        assert_eq!(HAND_BISHOP.to_string(), "B");
        assert_eq!(HAND_ROOK.to_string(), "R");
        assert_eq!(HAND_GOLD.to_string(), "G");
    }

    #[test]
    fn test_hands_parse() {
        assert_eq!(Hands::from_str("-").unwrap(), Hands::empty());
        assert_eq!(
            Hands::from_str("Pp").unwrap(),
            make_hands([(SENTE, HAND_PAWN, 1), (GOTE, HAND_PAWN, 1)])
        );
        assert_eq!(
            Hands::from_str("S2Pb3p").unwrap(),
            make_hands([
                (SENTE, HAND_SILVER, 1),
                (SENTE, HAND_PAWN, 2),
                (GOTE, HAND_BISHOP, 1),
                (GOTE, HAND_PAWN, 3),
            ])
        );
        assert_eq!(
            Hands::from_str("RB2G2S2N2L15Prb2g2s2n2l3p").unwrap(),
            make_hands([
                (SENTE, HAND_ROOK, 1),
                (SENTE, HAND_BISHOP, 1),
                (SENTE, HAND_GOLD, 2),
                (SENTE, HAND_SILVER, 2),
                (SENTE, HAND_KNIGHT, 2),
                (SENTE, HAND_LANCE, 2),
                (SENTE, HAND_PAWN, 15),
                (GOTE, HAND_ROOK, 1),
                (GOTE, HAND_BISHOP, 1),
                (GOTE, HAND_GOLD, 2),
                (GOTE, HAND_SILVER, 2),
                (GOTE, HAND_KNIGHT, 2),
                (GOTE, HAND_LANCE, 2),
                (GOTE, HAND_PAWN, 3),
            ])
        );

        assert!(Hands::from_str("").is_err());
        assert!(matches!(
            Hands::from_str("RB2G?"),
            Err(SfenParseError::InvalidInput { offset: 4, .. })
        ));
    }

    #[test]
    fn test_hands_fmt() {
        assert_eq!(Hands::empty().to_string(), "-");
        assert_eq!(
            make_hands([(SENTE, HAND_PAWN, 1), (GOTE, HAND_PAWN, 1)]).to_string(),
            "Pp"
        );
        assert_eq!(
            make_hands([
                (SENTE, HAND_SILVER, 1),
                (SENTE, HAND_PAWN, 2),
                (GOTE, HAND_BISHOP, 1),
                (GOTE, HAND_PAWN, 3),
            ])
            .to_string(),
            "S2Pb3p"
        );
        assert_eq!(
            make_hands([
                (SENTE, HAND_ROOK, 1),
                (SENTE, HAND_BISHOP, 1),
                (SENTE, HAND_GOLD, 2),
                (SENTE, HAND_SILVER, 2),
                (SENTE, HAND_KNIGHT, 2),
                (SENTE, HAND_LANCE, 2),
                (SENTE, HAND_PAWN, 15),
                (GOTE, HAND_ROOK, 1),
                (GOTE, HAND_BISHOP, 1),
                (GOTE, HAND_GOLD, 2),
                (GOTE, HAND_SILVER, 2),
                (GOTE, HAND_KNIGHT, 2),
                (GOTE, HAND_LANCE, 2),
                (GOTE, HAND_PAWN, 3),
            ])
            .to_string(),
            "RB2G2S2N2L15Prb2g2s2n2l3p"
        );
    }
}
