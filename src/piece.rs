use crate::bytes::Bytes;
use crate::parse::*;
use crate::side::*;

/// 駒種(先後の区別なし)。
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PieceKind {
    Pawn,
    Lance,
    Knight,
    Silver,
    Bishop,
    Rook,
    Gold,
    King,
    ProPawn,
    ProLance,
    ProKnight,
    ProSilver,
    Horse,
    Dragon,
}

pub const PAWN: PieceKind = PieceKind::Pawn;
pub const LANCE: PieceKind = PieceKind::Lance;
pub const KNIGHT: PieceKind = PieceKind::Knight;
pub const SILVER: PieceKind = PieceKind::Silver;
pub const BISHOP: PieceKind = PieceKind::Bishop;
pub const ROOK: PieceKind = PieceKind::Rook;
pub const GOLD: PieceKind = PieceKind::Gold;
pub const KING: PieceKind = PieceKind::King;
pub const PRO_PAWN: PieceKind = PieceKind::ProPawn;
pub const PRO_LANCE: PieceKind = PieceKind::ProLance;
pub const PRO_KNIGHT: PieceKind = PieceKind::ProKnight;
pub const PRO_SILVER: PieceKind = PieceKind::ProSilver;
pub const HORSE: PieceKind = PieceKind::Horse;
pub const DRAGON: PieceKind = PieceKind::Dragon;

/// 駒(先後の区別あり)。
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Piece {
    SentePawn,
    SenteLance,
    SenteKnight,
    SenteSilver,
    SenteBishop,
    SenteRook,
    SenteGold,
    SenteKing,
    SenteProPawn,
    SenteProLance,
    SenteProKnight,
    SenteProSilver,
    SenteHorse,
    SenteDragon,
    GotePawn,
    GoteLance,
    GoteKnight,
    GoteSilver,
    GoteBishop,
    GoteRook,
    GoteGold,
    GoteKing,
    GoteProPawn,
    GoteProLance,
    GoteProKnight,
    GoteProSilver,
    GoteHorse,
    GoteDragon,
}

pub const S_PAWN: Piece = Piece::new(SENTE, PAWN);
pub const S_LANCE: Piece = Piece::new(SENTE, LANCE);
pub const S_KNIGHT: Piece = Piece::new(SENTE, KNIGHT);
pub const S_SILVER: Piece = Piece::new(SENTE, SILVER);
pub const S_BISHOP: Piece = Piece::new(SENTE, BISHOP);
pub const S_ROOK: Piece = Piece::new(SENTE, ROOK);
pub const S_GOLD: Piece = Piece::new(SENTE, GOLD);
pub const S_KING: Piece = Piece::new(SENTE, KING);
pub const S_PRO_PAWN: Piece = Piece::new(SENTE, PRO_PAWN);
pub const S_PRO_LANCE: Piece = Piece::new(SENTE, PRO_LANCE);
pub const S_PRO_KNIGHT: Piece = Piece::new(SENTE, PRO_KNIGHT);
pub const S_PRO_SILVER: Piece = Piece::new(SENTE, PRO_SILVER);
pub const S_HORSE: Piece = Piece::new(SENTE, HORSE);
pub const S_DRAGON: Piece = Piece::new(SENTE, DRAGON);

pub const G_PAWN: Piece = Piece::new(GOTE, PAWN);
pub const G_LANCE: Piece = Piece::new(GOTE, LANCE);
pub const G_KNIGHT: Piece = Piece::new(GOTE, KNIGHT);
pub const G_SILVER: Piece = Piece::new(GOTE, SILVER);
pub const G_BISHOP: Piece = Piece::new(GOTE, BISHOP);
pub const G_ROOK: Piece = Piece::new(GOTE, ROOK);
pub const G_GOLD: Piece = Piece::new(GOTE, GOLD);
pub const G_KING: Piece = Piece::new(GOTE, KING);
pub const G_PRO_PAWN: Piece = Piece::new(GOTE, PRO_PAWN);
pub const G_PRO_LANCE: Piece = Piece::new(GOTE, PRO_LANCE);
pub const G_PRO_KNIGHT: Piece = Piece::new(GOTE, PRO_KNIGHT);
pub const G_PRO_SILVER: Piece = Piece::new(GOTE, PRO_SILVER);
pub const G_HORSE: Piece = Piece::new(GOTE, HORSE);
pub const G_DRAGON: Piece = Piece::new(GOTE, DRAGON);

impl Piece {
    /// 陣営と駒種を指定して駒を作る。
    pub const fn new(side: Side, pk: PieceKind) -> Self {
        match (side, pk) {
            (SENTE, PAWN) => Self::SentePawn,
            (SENTE, LANCE) => Self::SenteLance,
            (SENTE, KNIGHT) => Self::SenteKnight,
            (SENTE, SILVER) => Self::SenteSilver,
            (SENTE, BISHOP) => Self::SenteBishop,
            (SENTE, ROOK) => Self::SenteRook,
            (SENTE, GOLD) => Self::SenteGold,
            (SENTE, KING) => Self::SenteKing,
            (SENTE, PRO_PAWN) => Self::SenteProPawn,
            (SENTE, PRO_LANCE) => Self::SenteProLance,
            (SENTE, PRO_KNIGHT) => Self::SenteProKnight,
            (SENTE, PRO_SILVER) => Self::SenteProSilver,
            (SENTE, HORSE) => Self::SenteHorse,
            (SENTE, DRAGON) => Self::SenteDragon,

            (GOTE, PAWN) => Self::GotePawn,
            (GOTE, LANCE) => Self::GoteLance,
            (GOTE, KNIGHT) => Self::GoteKnight,
            (GOTE, SILVER) => Self::GoteSilver,
            (GOTE, BISHOP) => Self::GoteBishop,
            (GOTE, ROOK) => Self::GoteRook,
            (GOTE, GOLD) => Self::GoteGold,
            (GOTE, KING) => Self::GoteKing,
            (GOTE, PRO_PAWN) => Self::GoteProPawn,
            (GOTE, PRO_LANCE) => Self::GoteProLance,
            (GOTE, PRO_KNIGHT) => Self::GoteProKnight,
            (GOTE, PRO_SILVER) => Self::GoteProSilver,
            (GOTE, HORSE) => Self::GoteHorse,
            (GOTE, DRAGON) => Self::GoteDragon,
        }
    }

    /// 所属陣営を返す。
    pub const fn side(self) -> Side {
        match self {
            S_PAWN => SENTE,
            S_LANCE => SENTE,
            S_KNIGHT => SENTE,
            S_SILVER => SENTE,
            S_BISHOP => SENTE,
            S_ROOK => SENTE,
            S_GOLD => SENTE,
            S_KING => SENTE,
            S_PRO_PAWN => SENTE,
            S_PRO_LANCE => SENTE,
            S_PRO_KNIGHT => SENTE,
            S_PRO_SILVER => SENTE,
            S_HORSE => SENTE,
            S_DRAGON => SENTE,

            G_PAWN => GOTE,
            G_LANCE => GOTE,
            G_KNIGHT => GOTE,
            G_SILVER => GOTE,
            G_BISHOP => GOTE,
            G_ROOK => GOTE,
            G_GOLD => GOTE,
            G_KING => GOTE,
            G_PRO_PAWN => GOTE,
            G_PRO_LANCE => GOTE,
            G_PRO_KNIGHT => GOTE,
            G_PRO_SILVER => GOTE,
            G_HORSE => GOTE,
            G_DRAGON => GOTE,
        }
    }

    /// 駒種を返す。
    pub const fn kind(self) -> PieceKind {
        match self {
            S_PAWN => PAWN,
            S_LANCE => LANCE,
            S_KNIGHT => KNIGHT,
            S_SILVER => SILVER,
            S_BISHOP => BISHOP,
            S_ROOK => ROOK,
            S_GOLD => GOLD,
            S_KING => KING,
            S_PRO_PAWN => PRO_PAWN,
            S_PRO_LANCE => PRO_LANCE,
            S_PRO_KNIGHT => PRO_KNIGHT,
            S_PRO_SILVER => PRO_SILVER,
            S_HORSE => HORSE,
            S_DRAGON => DRAGON,

            G_PAWN => PAWN,
            G_LANCE => LANCE,
            G_KNIGHT => KNIGHT,
            G_SILVER => SILVER,
            G_BISHOP => BISHOP,
            G_ROOK => ROOK,
            G_GOLD => GOLD,
            G_KING => KING,
            G_PRO_PAWN => PRO_PAWN,
            G_PRO_LANCE => PRO_LANCE,
            G_PRO_KNIGHT => PRO_KNIGHT,
            G_PRO_SILVER => PRO_SILVER,
            G_HORSE => HORSE,
            G_DRAGON => DRAGON,
        }
    }

    pub(crate) fn parse(bytes: Bytes) -> SfenParseResult<(Bytes, Self)> {
        // RawPiece | '+' PromotedPiece

        if bytes.get(0) == Some(&b'+') {
            Self::parse_promoted_piece(bytes.range_from(1..))
        } else {
            Self::parse_raw_piece(bytes)
        }
    }

    fn parse_raw_piece(bytes: Bytes) -> SfenParseResult<(Bytes, Piece)> {
        let (bytes, remain) = bytes
            .try_split_at(1)
            .ok_or_else(|| SfenParseError::invalid_input(bytes, "raw piece expected"))?;

        let pc = match bytes[0] {
            b'P' => S_PAWN,
            b'L' => S_LANCE,
            b'N' => S_KNIGHT,
            b'S' => S_SILVER,
            b'B' => S_BISHOP,
            b'R' => S_ROOK,
            b'G' => S_GOLD,
            b'K' => S_KING,
            b'p' => G_PAWN,
            b'l' => G_LANCE,
            b'n' => G_KNIGHT,
            b's' => G_SILVER,
            b'b' => G_BISHOP,
            b'r' => G_ROOK,
            b'g' => G_GOLD,
            b'k' => G_KING,
            _ => return Err(SfenParseError::invalid_input(bytes, "raw piece expected")),
        };

        Ok((remain, pc))
    }

    fn parse_promoted_piece(bytes: Bytes) -> SfenParseResult<(Bytes, Piece)> {
        let (bytes, remain) = bytes
            .try_split_at(1)
            .ok_or_else(|| SfenParseError::invalid_input(bytes, "promoted piece expected"))?;

        let pc = match bytes[0] {
            b'P' => S_PRO_PAWN,
            b'L' => S_PRO_LANCE,
            b'N' => S_PRO_KNIGHT,
            b'S' => S_PRO_SILVER,
            b'B' => S_HORSE,
            b'R' => S_DRAGON,
            b'p' => G_PRO_PAWN,
            b'l' => G_PRO_LANCE,
            b'n' => G_PRO_KNIGHT,
            b's' => G_PRO_SILVER,
            b'b' => G_HORSE,
            b'r' => G_DRAGON,
            _ => {
                return Err(SfenParseError::invalid_input(
                    bytes,
                    "promoted piece expected",
                ))
            }
        };

        Ok((remain, pc))
    }
}

impl std::str::FromStr for Piece {
    type Err = SfenParseError;

    /// SFEN 駒文字列をパースする。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser_complete(Self::parse)(Bytes::from(s))
    }
}

impl std::fmt::Display for Piece {
    /// SFEN 駒文字列を出力する。
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match *self {
            S_PAWN => "P",
            S_LANCE => "L",
            S_KNIGHT => "N",
            S_SILVER => "S",
            S_BISHOP => "B",
            S_ROOK => "R",
            S_GOLD => "G",
            S_KING => "K",
            S_PRO_PAWN => "+P",
            S_PRO_LANCE => "+L",
            S_PRO_KNIGHT => "+N",
            S_PRO_SILVER => "+S",
            S_HORSE => "+B",
            S_DRAGON => "+R",
            G_PAWN => "p",
            G_LANCE => "l",
            G_KNIGHT => "n",
            G_SILVER => "s",
            G_BISHOP => "b",
            G_ROOK => "r",
            G_GOLD => "g",
            G_KING => "k",
            G_PRO_PAWN => "+p",
            G_PRO_LANCE => "+l",
            G_PRO_KNIGHT => "+n",
            G_PRO_SILVER => "+s",
            G_HORSE => "+b",
            G_DRAGON => "+r",
        };
        f.write_str(s)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    const TABLE: [(&str, Piece); 28] = [
        ("P", S_PAWN),
        ("L", S_LANCE),
        ("N", S_KNIGHT),
        ("S", S_SILVER),
        ("G", S_GOLD),
        ("B", S_BISHOP),
        ("R", S_ROOK),
        ("K", S_KING),
        ("+P", S_PRO_PAWN),
        ("+L", S_PRO_LANCE),
        ("+N", S_PRO_KNIGHT),
        ("+S", S_PRO_SILVER),
        ("+B", S_HORSE),
        ("+R", S_DRAGON),
        ("p", G_PAWN),
        ("l", G_LANCE),
        ("n", G_KNIGHT),
        ("s", G_SILVER),
        ("g", G_GOLD),
        ("b", G_BISHOP),
        ("r", G_ROOK),
        ("k", G_KING),
        ("+p", G_PRO_PAWN),
        ("+l", G_PRO_LANCE),
        ("+n", G_PRO_KNIGHT),
        ("+s", G_PRO_SILVER),
        ("+b", G_HORSE),
        ("+r", G_DRAGON),
    ];

    #[test]
    fn test_piece_parse() {
        for (s, pc) in TABLE {
            assert_eq!(Piece::from_str(s).unwrap(), pc);
        }
    }

    #[test]
    fn test_piece_fmt() {
        for (s, pc) in TABLE {
            assert_eq!(pc.to_string(), s);
        }
    }
}
