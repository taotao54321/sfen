use std::fmt::Write as _;

use crate::bytes::Bytes;
use crate::hand::*;
use crate::parse::*;
use crate::square::*;

/// 指し手。
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Move {
    /// 盤上の駒を動かす指し手。
    Walk(MoveWalk),

    /// 駒打ちの指し手。
    Drop(MoveDrop),
}

impl Move {
    /// 盤上の駒を動かす指し手を作る。
    pub const fn walk(src: Square, dst: Square, promo: bool) -> Self {
        Self::Walk(MoveWalk::new(src, dst, promo))
    }

    /// 駒打ちの指し手を作る。
    pub const fn drop(hpk: HandPieceKind, dst: Square) -> Self {
        Self::Drop(MoveDrop::new(hpk, dst))
    }

    /// 移動先を返す。
    pub const fn dst(self) -> Square {
        match self {
            Self::Walk(w) => w.dst(),
            Self::Drop(d) => d.dst(),
        }
    }

    pub(crate) fn parse(bytes: Bytes) -> SfenParseResult<(Bytes, Self)> {
        // MoveWalk | MoveDrop

        if let Ok((remain, walk)) = MoveWalk::parse(bytes) {
            Ok((remain, Self::Walk(walk)))
        } else if let Ok((remain, drop)) = MoveDrop::parse(bytes) {
            Ok((remain, Self::Drop(drop)))
        } else {
            Err(SfenParseError::invalid_input(bytes, "`Move` expected"))
        }
    }
}

impl std::str::FromStr for Move {
    type Err = SfenParseError;

    /// SFEN 指し手文字列をパースする。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser_complete(Self::parse)(Bytes::from(s))
    }
}

impl std::fmt::Display for Move {
    /// SFEN 指し手文字列を出力する。
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Walk(walk) => walk.fmt(f),
            Self::Drop(drop) => drop.fmt(f),
        }
    }
}

/// 盤上の駒を動かす指し手。
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MoveWalk {
    src: Square,
    dst: Square,
    promo: bool,
}

impl MoveWalk {
    /// 盤上の駒を動かす指し手を作る。
    pub const fn new(src: Square, dst: Square, promo: bool) -> Self {
        Self { src, dst, promo }
    }

    /// 移動元を返す。
    pub const fn src(self) -> Square {
        self.src
    }

    /// 移動先を返す。
    pub const fn dst(self) -> Square {
        self.dst
    }

    /// 成りかどうかを返す。
    pub const fn is_promotion(self) -> bool {
        self.promo
    }

    pub(crate) fn parse(bytes: Bytes) -> SfenParseResult<(Bytes, Self)> {
        // Square Square '+'?

        let (bytes, src) = Square::parse(bytes)?;
        let (bytes, dst) = Square::parse(bytes)?;
        let (remain, promo) = if bytes.get(0) == Some(&b'+') {
            (bytes.range_from(1..), true)
        } else {
            (bytes, false)
        };

        let walk = Self::new(src, dst, promo);

        Ok((remain, walk))
    }
}

impl std::str::FromStr for MoveWalk {
    type Err = SfenParseError;

    /// SFEN 盤上の駒を動かす指し手文字列をパースする。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser_complete(Self::parse)(Bytes::from(s))
    }
}

impl std::fmt::Display for MoveWalk {
    /// SFEN 盤上の駒を動かす指し手文字列を出力する。
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.src.fmt(f)?;
        self.dst.fmt(f)?;
        if self.promo {
            f.write_char('+')?;
        }

        Ok(())
    }
}

/// 駒打ちの指し手。
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MoveDrop {
    hpk: HandPieceKind,
    dst: Square,
}

impl MoveDrop {
    /// 駒打ちの指し手を作る。
    pub const fn new(hpk: HandPieceKind, dst: Square) -> Self {
        Self { hpk, dst }
    }

    /// 駒種を返す。
    pub const fn piece_kind(self) -> HandPieceKind {
        self.hpk
    }

    /// 移動先を返す。
    pub const fn dst(self) -> Square {
        self.dst
    }

    pub(crate) fn parse(bytes: Bytes) -> SfenParseResult<(Bytes, Self)> {
        // HandPieceKind '*' Square

        let (bytes, hpk) = HandPieceKind::parse(bytes)?;
        if bytes.get(0) != Some(&b'*') {
            return Err(SfenParseError::invalid_input(bytes, "'*' expected"));
        }
        let bytes = bytes.range_from(1..);
        let (remain, dst) = Square::parse(bytes)?;

        let drop = Self::new(hpk, dst);

        Ok((remain, drop))
    }
}

impl std::str::FromStr for MoveDrop {
    type Err = SfenParseError;

    /// SFEN 駒打ちの指し手文字列をパースする。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser_complete(Self::parse)(Bytes::from(s))
    }
}

impl std::fmt::Display for MoveDrop {
    /// SFEN 駒打ちの指し手文字列を出力する。
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.hpk.fmt(f)?;
        f.write_char('*')?;
        self.dst.fmt(f)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    #[test]
    fn test_move_parse() {
        assert_eq!(
            Move::from_str("7g7f").unwrap(),
            Move::walk(SQ_77, SQ_76, false)
        );
        assert_eq!(
            Move::from_str("5a9e+").unwrap(),
            Move::walk(SQ_51, SQ_95, true)
        );
        assert_eq!(
            Move::from_str("G*4i").unwrap(),
            Move::drop(HAND_GOLD, SQ_49)
        );

        assert!(Move::from_str("0a1a").is_err()); // 移動元の筋が不正
        assert!(Move::from_str("1j1a").is_err()); // 移動元の段が不正
        assert!(Move::from_str("1a0a").is_err()); // 移動先の筋が不正
        assert!(Move::from_str("1a1j").is_err()); // 移動先の段が不正
        assert!(Move::from_str("K*1a").is_err()); // 打つ駒種が不正
        assert!(Move::from_str("G+1a").is_err()); // 駒打ちの 2 バイト目が不正
    }

    #[test]
    fn test_move_fmt() {
        assert_eq!(Move::walk(SQ_77, SQ_76, false).to_string(), "7g7f");
        assert_eq!(Move::walk(SQ_51, SQ_95, true).to_string(), "5a9e+");
        assert_eq!(Move::drop(HAND_GOLD, SQ_49).to_string(), "G*4i");
    }
}
