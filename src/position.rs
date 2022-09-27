use std::fmt::Write as _;
use std::num::NonZeroU32;

use crate::board::*;
use crate::bytes::Bytes;
use crate::hand::*;
use crate::parse::*;
use crate::side::*;

/// 局面。
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    side_to_move: Side,
    board: Board,
    hands: Hands,
    ply: NonZeroU32,
}

impl Position {
    /// 手番、盤面、両陣営の手駒、手数(次の手が何手目か)を指定して局面を作る。
    pub const fn new(side_to_move: Side, board: Board, hands: Hands, ply: NonZeroU32) -> Self {
        Self {
            side_to_move,
            board,
            hands,
            ply,
        }
    }

    /// 平手初期局面を返す。
    pub const fn startpos() -> Self {
        Self {
            side_to_move: Side::Sente,
            board: Board::startpos(),
            hands: Hands::empty(),
            ply: unsafe { NonZeroU32::new_unchecked(1) },
        }
    }

    fn is_startpos(&self) -> bool {
        *self == Self::startpos()
    }

    /// 手番を返す。
    pub const fn side_to_move(&self) -> Side {
        self.side_to_move
    }

    /// 盤面への参照を返す。
    pub const fn board(&self) -> &Board {
        &self.board
    }

    /// 両陣営の手駒への参照を返す。
    pub const fn hands(&self) -> &Hands {
        &self.hands
    }

    /// 手数(次の手が何手目か)を返す。
    pub const fn ply(&self) -> NonZeroU32 {
        self.ply
    }

    pub(crate) fn parse(bytes: Bytes) -> SfenParseResult<(Bytes, Self)> {
        // "position"? ("startpos" | "sfen"? Board Side Hands Ply)

        let mut tokens = bytes.tokens();

        macro_rules! next_token {
            () => {{
                // ok_or_else() を使うと、クロージャに tokens がムーブされてコンパイルできない。
                match tokens.next() {
                    Some(token) => token,
                    None => {
                        return Err(SfenParseError::invalid_input(
                            tokens.remain(),
                            "premature end of position",
                        ));
                    }
                }
            }};
        }

        // "position" は読み飛ばす。
        let mut first = next_token!();
        if first.as_slice() == b"position" {
            first = next_token!();
        }

        if first.as_slice() == b"startpos" {
            return Ok((tokens.remain(), Self::startpos()));
        }

        // "sfen" は読み飛ばす。
        let bytes_board = if first.as_slice() == b"sfen" {
            next_token!()
        } else {
            first
        };
        let (_, board) = Board::parse(bytes_board)?;
        let (_, side_to_move) = Side::parse(next_token!())?;
        let (_, hands) = Hands::parse(next_token!())?;
        let ply = Self::parse_ply(next_token!())?;

        let pos = Position::new(side_to_move, board, hands, ply);

        Ok((tokens.remain(), pos))
    }

    fn parse_ply(bytes: Bytes) -> SfenParseResult<NonZeroU32> {
        let s = std::str::from_utf8(bytes.as_slice())
            .map_err(|_| SfenParseError::invalid_input(bytes, "ply string is not valid UTF-8"))?;

        let ply: NonZeroU32 = s
            .parse()
            .map_err(|_| SfenParseError::invalid_input(bytes, "ply must be NonZeroU32"))?;

        Ok(ply)
    }
}

impl std::str::FromStr for Position {
    type Err = SfenParseError;

    /// SFEN 局面文字列をパースする。
    ///
    /// 先頭/末尾の ASCII spaces は無視する。
    /// "position", "sfen" の有無は任意。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser_complete(Self::parse)(Bytes::from(s))
    }
}

impl std::fmt::Display for Position {
    /// SFEN 局面文字列を出力する。
    ///
    /// 平手初期局面なら "startpos" を出力する。
    ///
    /// "position" は付けない。平手初期局面以外の場合、"sfen" は付ける。
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_startpos() {
            return f.write_str("startpos");
        }

        f.write_str("sfen ")?;
        self.board.fmt(f)?;
        f.write_char(' ')?;
        self.side_to_move.fmt(f)?;
        f.write_char(' ')?;
        self.hands.fmt(f)?;
        f.write_char(' ')?;
        self.ply.fmt(f)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use crate::square::*;

    use super::*;

    const PLY_1: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(1) };

    fn empty_position() -> Position {
        Position::new(SENTE, Board::empty(), Hands::empty(), PLY_1)
    }

    fn sample_position() -> Position {
        // 香落ち。
        let mut board = Board::startpos();
        board[SQ_11] = None;
        Position::new(GOTE, board, Hands::empty(), PLY_1)
    }

    #[test]
    fn test_position_parse() {
        assert_eq!(
            Position::from_str("startpos").unwrap(),
            Position::startpos()
        );
        assert_eq!(
            Position::from_str("position startpos").unwrap(),
            Position::startpos()
        );
        assert_eq!(
            Position::from_str("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1")
                .unwrap(),
            Position::startpos()
        );
        assert_eq!(
            Position::from_str(
                "sfen lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1"
            )
            .unwrap(),
            Position::startpos()
        );
        assert_eq!(
            Position::from_str(
                "position sfen lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1"
            )
            .unwrap(),
            Position::startpos()
        );
        assert_eq!(
            Position::from_str("sfen 9/9/9/9/9/9/9/9/9 b - 1").unwrap(),
            empty_position()
        );
        assert_eq!(
            Position::from_str(
                "sfen lnsgkgsn1/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL w - 1"
            )
            .unwrap(),
            sample_position()
        );
    }

    #[test]
    fn test_position_fmt() {
        assert_eq!(Position::startpos().to_string(), "startpos");
        assert_eq!(empty_position().to_string(), "sfen 9/9/9/9/9/9/9/9/9 b - 1");
        assert_eq!(
            sample_position().to_string(),
            "sfen lnsgkgsn1/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL w - 1"
        );
    }
}
