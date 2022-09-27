use std::fmt::Write as _;

use crate::bytes::Bytes;
use crate::move_::*;
use crate::parse::*;
use crate::position::*;

/// 棋譜。
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Kifu {
    pos: Position,
    mvs: Vec<Move>,
}

impl Kifu {
    /// 開始局面とそこからの手順を指定して棋譜を作る。
    pub fn new(pos: Position, mvs: impl IntoIterator<Item = Move>) -> Self {
        Self {
            pos,
            mvs: mvs.into_iter().collect(),
        }
    }

    /// 平手初期局面に対応する棋譜を返す。
    pub const fn startpos() -> Self {
        Self {
            pos: Position::startpos(),
            mvs: vec![],
        }
    }

    /// 開始局面を返す。
    pub const fn position(&self) -> &Position {
        &self.pos
    }

    /// 開始局面からの手順を返す。
    pub fn moves(&self) -> &[Move] {
        &self.mvs
    }

    /// 入力の終端まで読む。
    pub(crate) fn parse(bytes: Bytes) -> SfenParseResult<(Bytes, Self)> {
        // Position (EOF | "moves" Move*)

        let (bytes, pos) = Position::parse(bytes)?;

        let mut tokens = bytes.tokens();

        match tokens.next() {
            Some(magic) => {
                if magic.as_slice() != b"moves" {
                    return Err(SfenParseError::invalid_input(magic, r#""moves" expected"#));
                }
            }
            None => return Ok((tokens.remain(), Kifu::new(pos, []))),
        }

        let mut mvs = vec![];
        for token in tokens {
            let (_, mv) = Move::parse(token)?;
            mvs.push(mv);
        }

        let kifu = Kifu::new(pos, mvs);

        Ok((bytes.range_from(bytes.len()..), kifu))
    }
}

impl std::str::FromStr for Kifu {
    type Err = SfenParseError;

    /// SFEN 棋譜文字列をパースする。
    ///
    /// 先頭/末尾の ASCII spaces は無視する。
    /// "position", "sfen" の有無は任意。
    /// 手順が空の場合、"moves" の有無は任意。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser_complete(Self::parse)(Bytes::from(s))
    }
}

impl std::fmt::Display for Kifu {
    /// SFEN 棋譜文字列を出力する。
    ///
    /// 開始局面が平手初期局面なら、局面文字列として "startpos" を出力する。
    ///
    /// 先頭に "position" を付ける。
    /// 開始局面が平手初期局面でなければ "sfen" を付ける。
    /// 手順が空の場合、"moves" は付けない。
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("position ")?;
        self.pos.fmt(f)?;

        if !self.mvs.is_empty() {
            f.write_str(" moves")?;
            for &mv in &self.mvs {
                f.write_char(' ')?;
                mv.fmt(f)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use crate::hand::*;
    use crate::square::*;

    use super::*;

    #[test]
    fn test_kifu_parse() {
        assert_eq!(Kifu::from_str("startpos").unwrap(), Kifu::startpos());
        assert_eq!(
            Kifu::from_str("position startpos").unwrap(),
            Kifu::startpos()
        );
        assert_eq!(
            Kifu::from_str("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1")
                .unwrap(),
            Kifu::startpos()
        );
        assert_eq!(
            Kifu::from_str("sfen lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1")
                .unwrap(),
            Kifu::startpos()
        );
        assert_eq!(
            Kifu::from_str(
                "position sfen lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1"
            )
            .unwrap(),
            Kifu::startpos()
        );
        assert_eq!(
            Kifu::from_str(
                "position sfen lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1 moves"
            )
            .unwrap(),
            Kifu::startpos()
        );
        assert_eq!(
            Kifu::from_str("position startpos moves 7g7f 3c3d 8h2b+ 8b2b B*6e").unwrap(),
            Kifu::new(
                Position::startpos(),
                [
                    Move::walk(SQ_77, SQ_76, false),
                    Move::walk(SQ_33, SQ_34, false),
                    Move::walk(SQ_88, SQ_22, true),
                    Move::walk(SQ_82, SQ_22, false),
                    Move::drop(HAND_BISHOP, SQ_65)
                ]
            )
        );
    }

    #[test]
    fn test_kifu_fmt() {
        assert_eq!(Kifu::startpos().to_string(), "position startpos");
        assert_eq!(
            Kifu::new(
                Position::startpos(),
                [
                    Move::walk(SQ_77, SQ_76, false),
                    Move::walk(SQ_33, SQ_34, false),
                    Move::walk(SQ_88, SQ_22, true),
                    Move::walk(SQ_82, SQ_22, false),
                    Move::drop(HAND_BISHOP, SQ_65)
                ]
            )
            .to_string(),
            "position startpos moves 7g7f 3c3d 8h2b+ 8b2b B*6e"
        );
    }
}
