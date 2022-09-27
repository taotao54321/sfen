use std::fmt::Write as _;

use crate::bytes::Bytes;
use crate::parse::*;
use crate::piece::*;
use crate::square::*;

/// 盤面。
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Board(ArrayRow<BoardRow>);

impl Board {
    /// 空の盤面を返す。
    pub const fn empty() -> Self {
        Self(ArrayRow::new([
            BoardRow::empty(),
            BoardRow::empty(),
            BoardRow::empty(),
            BoardRow::empty(),
            BoardRow::empty(),
            BoardRow::empty(),
            BoardRow::empty(),
            BoardRow::empty(),
            BoardRow::empty(),
        ]))
    }

    /// 平手初期盤面を返す。
    pub const fn startpos() -> Self {
        Self(ArrayRow::new([
            BoardRow(ArrayCol::new([
                Some(G_LANCE),
                Some(G_KNIGHT),
                Some(G_SILVER),
                Some(G_GOLD),
                Some(G_KING),
                Some(G_GOLD),
                Some(G_SILVER),
                Some(G_KNIGHT),
                Some(G_LANCE),
            ])),
            BoardRow(ArrayCol::new([
                None,
                Some(G_ROOK),
                None,
                None,
                None,
                None,
                None,
                Some(G_BISHOP),
                None,
            ])),
            BoardRow(ArrayCol::new([
                Some(G_PAWN),
                Some(G_PAWN),
                Some(G_PAWN),
                Some(G_PAWN),
                Some(G_PAWN),
                Some(G_PAWN),
                Some(G_PAWN),
                Some(G_PAWN),
                Some(G_PAWN),
            ])),
            BoardRow::empty(),
            BoardRow::empty(),
            BoardRow::empty(),
            BoardRow(ArrayCol::new([
                Some(S_PAWN),
                Some(S_PAWN),
                Some(S_PAWN),
                Some(S_PAWN),
                Some(S_PAWN),
                Some(S_PAWN),
                Some(S_PAWN),
                Some(S_PAWN),
                Some(S_PAWN),
            ])),
            BoardRow(ArrayCol::new([
                None,
                Some(S_BISHOP),
                None,
                None,
                None,
                None,
                None,
                Some(S_ROOK),
                None,
            ])),
            BoardRow(ArrayCol::new([
                Some(S_LANCE),
                Some(S_KNIGHT),
                Some(S_SILVER),
                Some(S_GOLD),
                Some(S_KING),
                Some(S_GOLD),
                Some(S_SILVER),
                Some(S_KNIGHT),
                Some(S_LANCE),
            ])),
        ]))
    }

    /// 入力の終端まで読む。
    pub(crate) fn parse(bytes: Bytes) -> SfenParseResult<(Bytes, Self)> {
        // BoardRow '/' ... '/' BoardRow EOF

        let mut board = Self::empty();

        let mut fields = bytes.split(|&b| b == b'/');
        for row in Row::all_private() {
            let field = fields.next().ok_or_else(|| {
                SfenParseError::invalid_input(bytes, "board has less than 9 rows")
            })?;
            let (_, board_row) = BoardRow::parse(field)?;

            board.0[row] = board_row;
        }

        if fields.next().is_some() {
            return Err(SfenParseError::invalid_input(
                bytes,
                "board has more than 9 rows",
            ));
        }

        Ok((bytes.range_from(bytes.len()..), board))
    }
}

impl std::ops::Index<Square> for Board {
    type Output = Option<Piece>;

    fn index(&self, sq: Square) -> &Self::Output {
        &self.0[sq.row()][sq.col()]
    }
}

impl std::ops::IndexMut<Square> for Board {
    fn index_mut(&mut self, sq: Square) -> &mut Self::Output {
        &mut self.0[sq.row()][sq.col()]
    }
}

impl std::str::FromStr for Board {
    type Err = SfenParseError;

    /// SFEN 盤面文字列をパースする。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser_complete(Self::parse)(Bytes::from(s))
    }
}

impl std::fmt::Display for Board {
    /// SFEN 盤面文字列を出力する。
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in Row::all_private() {
            if row != ROW_1 {
                f.write_char('/')?;
            }
            self.0[row].fmt(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct BoardRow(ArrayCol<Option<Piece>>);

impl BoardRow {
    const fn empty() -> Self {
        Self(ArrayCol::from_elem(None))
    }

    fn parse(mut bytes: Bytes) -> SfenParseResult<(Bytes, Self)> {
        // BoardRowElem+ EOF

        if bytes.is_empty() {
            return Err(SfenParseError::invalid_input(bytes, "board row expected"));
        }

        let mut board_row = Self::empty();
        let mut i = 0;

        while !bytes.is_empty() {
            let elem;
            (bytes, elem) = BoardRowElem::parse(bytes)?;

            match elem {
                BoardRowElem::Blanks(n) => i += n,
                BoardRowElem::Piece(pc) => {
                    board_row.0[Col::all_private()[i]] = Some(pc);
                    i += 1;
                }
            }

            if i > 9 {
                return Err(SfenParseError::invalid_input(
                    bytes,
                    "board row has more than 9 columns",
                ));
            }
        }

        if i != 9 {
            return Err(SfenParseError::invalid_input(
                bytes,
                "board row has less than 9 columns",
            ));
        }

        Ok((bytes.range_from(bytes.len()..), board_row))
    }
}

impl std::ops::Index<Col> for BoardRow {
    type Output = Option<Piece>;

    fn index(&self, col: Col) -> &Self::Output {
        &self.0[col]
    }
}

impl std::ops::IndexMut<Col> for BoardRow {
    fn index_mut(&mut self, col: Col) -> &mut Self::Output {
        &mut self.0[col]
    }
}

impl std::fmt::Display for BoardRow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        struct State<'a, 'b> {
            f: &'a mut std::fmt::Formatter<'b>,
            run_blank: u32,
        }
        impl<'a, 'b> State<'a, 'b> {
            fn new(f: &'a mut std::fmt::Formatter<'b>) -> Self {
                Self { f, run_blank: 0 }
            }
            fn update(&mut self, pc: Option<Piece>) -> std::fmt::Result {
                if let Some(pc) = pc {
                    self.flush_blanks()?;
                    write!(self.f, "{pc}")?;
                } else {
                    self.run_blank += 1;
                }
                Ok(())
            }
            fn flush_blanks(&mut self) -> std::fmt::Result {
                if self.run_blank > 0 {
                    write!(self.f, "{}", self.run_blank)?;
                    self.run_blank = 0;
                }
                Ok(())
            }
        }

        let mut state = State::new(f);
        for col in Col::all_private() {
            state.update(self[col])?;
        }
        state.flush_blanks()?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum BoardRowElem {
    Blanks(usize),
    Piece(Piece),
}

impl BoardRowElem {
    fn parse(bytes: Bytes) -> SfenParseResult<(Bytes, Self)> {
        // [1-9] | Piece

        match bytes.get(0) {
            Some(b @ b'1'..=b'9') => {
                let n = usize::from(b - b'0');
                Ok((bytes.range_from(1..), Self::Blanks(n)))
            }
            _ => {
                let (remain, pc) = Piece::parse(bytes)?;
                Ok((remain, Self::Piece(pc)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    fn sample_board() -> Board {
        const TABLE: &[(Square, Piece)] = &[
            (SQ_81, S_PRO_PAWN),
            (SQ_92, S_PRO_LANCE),
            (SQ_13, S_HORSE),
            (SQ_24, S_DRAGON),
            (SQ_35, G_PRO_KNIGHT),
            (SQ_46, G_PRO_SILVER),
            (SQ_57, G_PRO_PAWN),
            (SQ_68, G_HORSE),
            (SQ_79, G_DRAGON),
        ];

        let mut board = Board::empty();
        for &(sq, pc) in TABLE {
            board[sq] = Some(pc);
        }
        board
    }

    #[test]
    fn test_board_parse() {
        assert_eq!(
            Board::from_str("9/9/9/9/9/9/9/9/9").unwrap(),
            Board::empty()
        );
        assert_eq!(
            Board::from_str("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL").unwrap(),
            Board::startpos()
        );
        assert_eq!(
            Board::from_str("1+P7/+L8/8+B/7+R1/6+n2/5+s3/4+p4/3+b5/2+r6").unwrap(),
            sample_board()
        );
    }

    #[test]
    fn test_board_fmt() {
        assert_eq!(Board::empty().to_string(), "9/9/9/9/9/9/9/9/9");
        assert_eq!(
            Board::startpos().to_string(),
            "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL"
        );
        assert_eq!(
            sample_board().to_string(),
            "1+P7/+L8/8+B/7+R1/6+n2/5+s3/4+p4/3+b5/2+r6"
        );
    }
}
