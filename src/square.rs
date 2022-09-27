use std::fmt::Write as _;

use crate::bytes::Bytes;
use crate::parse::*;

/// 盤面の筋。たとえば `Col::Col1` は 1 筋。
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Col {
    Col9 = 0,
    Col8,
    Col7,
    Col6,
    Col5,
    Col4,
    Col3,
    Col2,
    Col1,
}

pub const COL_9: Col = Col::Col9;
pub const COL_8: Col = Col::Col8;
pub const COL_7: Col = Col::Col7;
pub const COL_6: Col = Col::Col6;
pub const COL_5: Col = Col::Col5;
pub const COL_4: Col = Col::Col4;
pub const COL_3: Col = Col::Col3;
pub const COL_2: Col = Col::Col2;
pub const COL_1: Col = Col::Col1;

impl Col {
    const NUM: usize = 9;

    const fn to_index(self) -> usize {
        self as usize
    }

    /// 全ての筋を返す。順序は未規定。
    pub const fn all() -> [Self; Self::NUM] {
        Self::all_private()
    }

    /// 全ての筋を昇順で返す。
    pub(crate) const fn all_private() -> [Self; Self::NUM] {
        [
            COL_9, COL_8, COL_7, COL_6, COL_5, COL_4, COL_3, COL_2, COL_1,
        ]
    }

    pub(crate) fn parse(bytes: Bytes) -> SfenParseResult<(Bytes, Self)> {
        // [1-9]

        let (bytes, remain) = bytes
            .try_split_at(1)
            .ok_or_else(|| SfenParseError::invalid_input(bytes, "`Col` ([1-9]) expected"))?;

        let col = match bytes[0] {
            b'9' => COL_9,
            b'8' => COL_8,
            b'7' => COL_7,
            b'6' => COL_6,
            b'5' => COL_5,
            b'4' => COL_4,
            b'3' => COL_3,
            b'2' => COL_2,
            b'1' => COL_1,
            _ => {
                return Err(SfenParseError::invalid_input(
                    bytes,
                    "`Col` ([1-9]) expected",
                ))
            }
        };

        Ok((remain, col))
    }
}

impl std::str::FromStr for Col {
    type Err = SfenParseError;

    /// SFEN 筋文字列をパースする。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser_complete(Self::parse)(Bytes::from(s))
    }
}

impl std::fmt::Display for Col {
    /// SFEN 筋文字列を出力する。
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match *self {
            COL_9 => '9',
            COL_8 => '8',
            COL_7 => '7',
            COL_6 => '6',
            COL_5 => '5',
            COL_4 => '4',
            COL_3 => '3',
            COL_2 => '2',
            COL_1 => '1',
        };
        f.write_char(c)
    }
}

/// `Col` でインデックスアクセスできる配列。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ArrayCol<T>([T; Col::NUM]);

impl<T> ArrayCol<T> {
    pub(crate) const fn new(inner: [T; Col::NUM]) -> Self {
        Self(inner)
    }

    pub(crate) const fn index_const(&self, col: Col) -> &T {
        &self.0[col.to_index()]
    }
}

impl<T: Copy> ArrayCol<T> {
    pub(crate) const fn from_elem(elem: T) -> Self {
        Self([elem; Col::NUM])
    }
}

impl<T: Copy + Default> Default for ArrayCol<T> {
    fn default() -> Self {
        Self([T::default(); Col::NUM])
    }
}

impl<T> std::ops::Index<Col> for ArrayCol<T> {
    type Output = T;

    fn index(&self, col: Col) -> &Self::Output {
        self.index_const(col)
    }
}

impl<T> std::ops::IndexMut<Col> for ArrayCol<T> {
    fn index_mut(&mut self, col: Col) -> &mut Self::Output {
        &mut self.0[col.to_index()]
    }
}

/// 盤面の筋。たとえば `Row::Row1` は一段目。
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Row {
    Row1 = 0,
    Row2,
    Row3,
    Row4,
    Row5,
    Row6,
    Row7,
    Row8,
    Row9,
}

pub const ROW_1: Row = Row::Row1;
pub const ROW_2: Row = Row::Row2;
pub const ROW_3: Row = Row::Row3;
pub const ROW_4: Row = Row::Row4;
pub const ROW_5: Row = Row::Row5;
pub const ROW_6: Row = Row::Row6;
pub const ROW_7: Row = Row::Row7;
pub const ROW_8: Row = Row::Row8;
pub const ROW_9: Row = Row::Row9;

impl Row {
    const NUM: usize = 9;

    const fn to_index(self) -> usize {
        self as usize
    }

    /// 全ての段を返す。順序は未規定。
    pub const fn all() -> [Self; Self::NUM] {
        Self::all_private()
    }

    /// 全ての段を昇順で返す。
    pub(crate) const fn all_private() -> [Self; Self::NUM] {
        [
            ROW_1, ROW_2, ROW_3, ROW_4, ROW_5, ROW_6, ROW_7, ROW_8, ROW_9,
        ]
    }

    pub(crate) fn parse(bytes: Bytes) -> SfenParseResult<(Bytes, Self)> {
        // [a-i]

        let (bytes, remain) = bytes
            .try_split_at(1)
            .ok_or_else(|| SfenParseError::invalid_input(bytes, "`Row` ([a-i]) expected"))?;

        let row = match bytes[0] {
            b'a' => ROW_1,
            b'b' => ROW_2,
            b'c' => ROW_3,
            b'd' => ROW_4,
            b'e' => ROW_5,
            b'f' => ROW_6,
            b'g' => ROW_7,
            b'h' => ROW_8,
            b'i' => ROW_9,
            _ => {
                return Err(SfenParseError::invalid_input(
                    bytes,
                    "`Row` ([a-i]) expected",
                ))
            }
        };

        Ok((remain, row))
    }
}

impl std::str::FromStr for Row {
    type Err = SfenParseError;

    /// SFEN 段文字列をパースする。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser_complete(Self::parse)(Bytes::from(s))
    }
}

impl std::fmt::Display for Row {
    /// SFEN 段文字列を出力する。
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match *self {
            ROW_1 => 'a',
            ROW_2 => 'b',
            ROW_3 => 'c',
            ROW_4 => 'd',
            ROW_5 => 'e',
            ROW_6 => 'f',
            ROW_7 => 'g',
            ROW_8 => 'h',
            ROW_9 => 'i',
        };
        f.write_char(c)
    }
}

/// `Row` でインデックスアクセスできる配列。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ArrayRow<T>([T; Row::NUM]);

impl<T> ArrayRow<T> {
    pub(crate) const fn new(inner: [T; Row::NUM]) -> Self {
        Self(inner)
    }

    pub(crate) const fn index_const(&self, row: Row) -> &T {
        &self.0[row.to_index()]
    }
}

impl<T: Copy> ArrayRow<T> {
    #[allow(dead_code)]
    pub(crate) const fn from_elem(elem: T) -> Self {
        Self([elem; Row::NUM])
    }
}

impl<T: Copy + Default> Default for ArrayRow<T> {
    fn default() -> Self {
        Self([T::default(); Row::NUM])
    }
}

impl<T> std::ops::Index<Row> for ArrayRow<T> {
    type Output = T;

    fn index(&self, row: Row) -> &Self::Output {
        self.index_const(row)
    }
}

impl<T> std::ops::IndexMut<Row> for ArrayRow<T> {
    fn index_mut(&mut self, row: Row) -> &mut Self::Output {
        &mut self.0[row.to_index()]
    }
}

/// 盤面のマス。
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[rustfmt::skip]
pub enum Square {
    Sq91 = 0, Sq81, Sq71, Sq61, Sq51, Sq41, Sq31, Sq21, Sq11,
    Sq92,     Sq82, Sq72, Sq62, Sq52, Sq42, Sq32, Sq22, Sq12,
    Sq93,     Sq83, Sq73, Sq63, Sq53, Sq43, Sq33, Sq23, Sq13,
    Sq94,     Sq84, Sq74, Sq64, Sq54, Sq44, Sq34, Sq24, Sq14,
    Sq95,     Sq85, Sq75, Sq65, Sq55, Sq45, Sq35, Sq25, Sq15,
    Sq96,     Sq86, Sq76, Sq66, Sq56, Sq46, Sq36, Sq26, Sq16,
    Sq97,     Sq87, Sq77, Sq67, Sq57, Sq47, Sq37, Sq27, Sq17,
    Sq98,     Sq88, Sq78, Sq68, Sq58, Sq48, Sq38, Sq28, Sq18,
    Sq99,     Sq89, Sq79, Sq69, Sq59, Sq49, Sq39, Sq29, Sq19,
}

pub const SQ_91: Square = Square::Sq91;
pub const SQ_81: Square = Square::Sq81;
pub const SQ_71: Square = Square::Sq71;
pub const SQ_61: Square = Square::Sq61;
pub const SQ_51: Square = Square::Sq51;
pub const SQ_41: Square = Square::Sq41;
pub const SQ_31: Square = Square::Sq31;
pub const SQ_21: Square = Square::Sq21;
pub const SQ_11: Square = Square::Sq11;
pub const SQ_92: Square = Square::Sq92;
pub const SQ_82: Square = Square::Sq82;
pub const SQ_72: Square = Square::Sq72;
pub const SQ_62: Square = Square::Sq62;
pub const SQ_52: Square = Square::Sq52;
pub const SQ_42: Square = Square::Sq42;
pub const SQ_32: Square = Square::Sq32;
pub const SQ_22: Square = Square::Sq22;
pub const SQ_12: Square = Square::Sq12;
pub const SQ_93: Square = Square::Sq93;
pub const SQ_83: Square = Square::Sq83;
pub const SQ_73: Square = Square::Sq73;
pub const SQ_63: Square = Square::Sq63;
pub const SQ_53: Square = Square::Sq53;
pub const SQ_43: Square = Square::Sq43;
pub const SQ_33: Square = Square::Sq33;
pub const SQ_23: Square = Square::Sq23;
pub const SQ_13: Square = Square::Sq13;
pub const SQ_94: Square = Square::Sq94;
pub const SQ_84: Square = Square::Sq84;
pub const SQ_74: Square = Square::Sq74;
pub const SQ_64: Square = Square::Sq64;
pub const SQ_54: Square = Square::Sq54;
pub const SQ_44: Square = Square::Sq44;
pub const SQ_34: Square = Square::Sq34;
pub const SQ_24: Square = Square::Sq24;
pub const SQ_14: Square = Square::Sq14;
pub const SQ_95: Square = Square::Sq95;
pub const SQ_85: Square = Square::Sq85;
pub const SQ_75: Square = Square::Sq75;
pub const SQ_65: Square = Square::Sq65;
pub const SQ_55: Square = Square::Sq55;
pub const SQ_45: Square = Square::Sq45;
pub const SQ_35: Square = Square::Sq35;
pub const SQ_25: Square = Square::Sq25;
pub const SQ_15: Square = Square::Sq15;
pub const SQ_96: Square = Square::Sq96;
pub const SQ_86: Square = Square::Sq86;
pub const SQ_76: Square = Square::Sq76;
pub const SQ_66: Square = Square::Sq66;
pub const SQ_56: Square = Square::Sq56;
pub const SQ_46: Square = Square::Sq46;
pub const SQ_36: Square = Square::Sq36;
pub const SQ_26: Square = Square::Sq26;
pub const SQ_16: Square = Square::Sq16;
pub const SQ_97: Square = Square::Sq97;
pub const SQ_87: Square = Square::Sq87;
pub const SQ_77: Square = Square::Sq77;
pub const SQ_67: Square = Square::Sq67;
pub const SQ_57: Square = Square::Sq57;
pub const SQ_47: Square = Square::Sq47;
pub const SQ_37: Square = Square::Sq37;
pub const SQ_27: Square = Square::Sq27;
pub const SQ_17: Square = Square::Sq17;
pub const SQ_98: Square = Square::Sq98;
pub const SQ_88: Square = Square::Sq88;
pub const SQ_78: Square = Square::Sq78;
pub const SQ_68: Square = Square::Sq68;
pub const SQ_58: Square = Square::Sq58;
pub const SQ_48: Square = Square::Sq48;
pub const SQ_38: Square = Square::Sq38;
pub const SQ_28: Square = Square::Sq28;
pub const SQ_18: Square = Square::Sq18;
pub const SQ_99: Square = Square::Sq99;
pub const SQ_89: Square = Square::Sq89;
pub const SQ_79: Square = Square::Sq79;
pub const SQ_69: Square = Square::Sq69;
pub const SQ_59: Square = Square::Sq59;
pub const SQ_49: Square = Square::Sq49;
pub const SQ_39: Square = Square::Sq39;
pub const SQ_29: Square = Square::Sq29;
pub const SQ_19: Square = Square::Sq19;

impl Square {
    const NUM: usize = 81;

    const fn to_index(self) -> usize {
        self as usize
    }

    /// 筋と段を指定してマスを作る。
    pub const fn new(col: Col, row: Row) -> Self {
        match (col, row) {
            (COL_9, ROW_1) => SQ_91,
            (COL_8, ROW_1) => SQ_81,
            (COL_7, ROW_1) => SQ_71,
            (COL_6, ROW_1) => SQ_61,
            (COL_5, ROW_1) => SQ_51,
            (COL_4, ROW_1) => SQ_41,
            (COL_3, ROW_1) => SQ_31,
            (COL_2, ROW_1) => SQ_21,
            (COL_1, ROW_1) => SQ_11,
            (COL_9, ROW_2) => SQ_92,
            (COL_8, ROW_2) => SQ_82,
            (COL_7, ROW_2) => SQ_72,
            (COL_6, ROW_2) => SQ_62,
            (COL_5, ROW_2) => SQ_52,
            (COL_4, ROW_2) => SQ_42,
            (COL_3, ROW_2) => SQ_32,
            (COL_2, ROW_2) => SQ_22,
            (COL_1, ROW_2) => SQ_12,
            (COL_9, ROW_3) => SQ_93,
            (COL_8, ROW_3) => SQ_83,
            (COL_7, ROW_3) => SQ_73,
            (COL_6, ROW_3) => SQ_63,
            (COL_5, ROW_3) => SQ_53,
            (COL_4, ROW_3) => SQ_43,
            (COL_3, ROW_3) => SQ_33,
            (COL_2, ROW_3) => SQ_23,
            (COL_1, ROW_3) => SQ_13,
            (COL_9, ROW_4) => SQ_94,
            (COL_8, ROW_4) => SQ_84,
            (COL_7, ROW_4) => SQ_74,
            (COL_6, ROW_4) => SQ_64,
            (COL_5, ROW_4) => SQ_54,
            (COL_4, ROW_4) => SQ_44,
            (COL_3, ROW_4) => SQ_34,
            (COL_2, ROW_4) => SQ_24,
            (COL_1, ROW_4) => SQ_14,
            (COL_9, ROW_5) => SQ_95,
            (COL_8, ROW_5) => SQ_85,
            (COL_7, ROW_5) => SQ_75,
            (COL_6, ROW_5) => SQ_65,
            (COL_5, ROW_5) => SQ_55,
            (COL_4, ROW_5) => SQ_45,
            (COL_3, ROW_5) => SQ_35,
            (COL_2, ROW_5) => SQ_25,
            (COL_1, ROW_5) => SQ_15,
            (COL_9, ROW_6) => SQ_96,
            (COL_8, ROW_6) => SQ_86,
            (COL_7, ROW_6) => SQ_76,
            (COL_6, ROW_6) => SQ_66,
            (COL_5, ROW_6) => SQ_56,
            (COL_4, ROW_6) => SQ_46,
            (COL_3, ROW_6) => SQ_36,
            (COL_2, ROW_6) => SQ_26,
            (COL_1, ROW_6) => SQ_16,
            (COL_9, ROW_7) => SQ_97,
            (COL_8, ROW_7) => SQ_87,
            (COL_7, ROW_7) => SQ_77,
            (COL_6, ROW_7) => SQ_67,
            (COL_5, ROW_7) => SQ_57,
            (COL_4, ROW_7) => SQ_47,
            (COL_3, ROW_7) => SQ_37,
            (COL_2, ROW_7) => SQ_27,
            (COL_1, ROW_7) => SQ_17,
            (COL_9, ROW_8) => SQ_98,
            (COL_8, ROW_8) => SQ_88,
            (COL_7, ROW_8) => SQ_78,
            (COL_6, ROW_8) => SQ_68,
            (COL_5, ROW_8) => SQ_58,
            (COL_4, ROW_8) => SQ_48,
            (COL_3, ROW_8) => SQ_38,
            (COL_2, ROW_8) => SQ_28,
            (COL_1, ROW_8) => SQ_18,
            (COL_9, ROW_9) => SQ_99,
            (COL_8, ROW_9) => SQ_89,
            (COL_7, ROW_9) => SQ_79,
            (COL_6, ROW_9) => SQ_69,
            (COL_5, ROW_9) => SQ_59,
            (COL_4, ROW_9) => SQ_49,
            (COL_3, ROW_9) => SQ_39,
            (COL_2, ROW_9) => SQ_29,
            (COL_1, ROW_9) => SQ_19,
        }
    }

    /// マスが属する筋を返す。
    pub const fn col(self) -> Col {
        #[rustfmt::skip]
        const TABLE: ArraySquare<Col> = ArraySquare::new([
            COL_9, COL_8, COL_7, COL_6, COL_5, COL_4, COL_3, COL_2, COL_1,
            COL_9, COL_8, COL_7, COL_6, COL_5, COL_4, COL_3, COL_2, COL_1,
            COL_9, COL_8, COL_7, COL_6, COL_5, COL_4, COL_3, COL_2, COL_1,
            COL_9, COL_8, COL_7, COL_6, COL_5, COL_4, COL_3, COL_2, COL_1,
            COL_9, COL_8, COL_7, COL_6, COL_5, COL_4, COL_3, COL_2, COL_1,
            COL_9, COL_8, COL_7, COL_6, COL_5, COL_4, COL_3, COL_2, COL_1,
            COL_9, COL_8, COL_7, COL_6, COL_5, COL_4, COL_3, COL_2, COL_1,
            COL_9, COL_8, COL_7, COL_6, COL_5, COL_4, COL_3, COL_2, COL_1,
            COL_9, COL_8, COL_7, COL_6, COL_5, COL_4, COL_3, COL_2, COL_1,
        ]);
        *TABLE.index_const(self)
    }

    /// マスが属する段を返す。
    pub const fn row(self) -> Row {
        #[rustfmt::skip]
        const TABLE: ArraySquare<Row> = ArraySquare::new([
            ROW_1, ROW_1, ROW_1, ROW_1, ROW_1, ROW_1, ROW_1, ROW_1, ROW_1,
            ROW_2, ROW_2, ROW_2, ROW_2, ROW_2, ROW_2, ROW_2, ROW_2, ROW_2,
            ROW_3, ROW_3, ROW_3, ROW_3, ROW_3, ROW_3, ROW_3, ROW_3, ROW_3,
            ROW_4, ROW_4, ROW_4, ROW_4, ROW_4, ROW_4, ROW_4, ROW_4, ROW_4,
            ROW_5, ROW_5, ROW_5, ROW_5, ROW_5, ROW_5, ROW_5, ROW_5, ROW_5,
            ROW_6, ROW_6, ROW_6, ROW_6, ROW_6, ROW_6, ROW_6, ROW_6, ROW_6,
            ROW_7, ROW_7, ROW_7, ROW_7, ROW_7, ROW_7, ROW_7, ROW_7, ROW_7,
            ROW_8, ROW_8, ROW_8, ROW_8, ROW_8, ROW_8, ROW_8, ROW_8, ROW_8,
            ROW_9, ROW_9, ROW_9, ROW_9, ROW_9, ROW_9, ROW_9, ROW_9, ROW_9,
        ]);
        *TABLE.index_const(self)
    }

    /// 全てのマスを返す。順序は未規定。
    pub const fn all() -> [Self; Self::NUM] {
        #[rustfmt::skip]
        const ALL: [Square; 81] = [
            SQ_91, SQ_81, SQ_71, SQ_61, SQ_51, SQ_41, SQ_31, SQ_21, SQ_11,
            SQ_92, SQ_82, SQ_72, SQ_62, SQ_52, SQ_42, SQ_32, SQ_22, SQ_12,
            SQ_93, SQ_83, SQ_73, SQ_63, SQ_53, SQ_43, SQ_33, SQ_23, SQ_13,
            SQ_94, SQ_84, SQ_74, SQ_64, SQ_54, SQ_44, SQ_34, SQ_24, SQ_14,
            SQ_95, SQ_85, SQ_75, SQ_65, SQ_55, SQ_45, SQ_35, SQ_25, SQ_15,
            SQ_96, SQ_86, SQ_76, SQ_66, SQ_56, SQ_46, SQ_36, SQ_26, SQ_16,
            SQ_97, SQ_87, SQ_77, SQ_67, SQ_57, SQ_47, SQ_37, SQ_27, SQ_17,
            SQ_98, SQ_88, SQ_78, SQ_68, SQ_58, SQ_48, SQ_38, SQ_28, SQ_18,
            SQ_99, SQ_89, SQ_79, SQ_69, SQ_59, SQ_49, SQ_39, SQ_29, SQ_19,
        ];
        ALL
    }

    pub(crate) fn parse(bytes: Bytes) -> SfenParseResult<(Bytes, Self)> {
        // Col Row

        let (bytes, col) = Col::parse(bytes)?;
        let (remain, row) = Row::parse(bytes)?;

        let sq = Self::new(col, row);

        Ok((remain, sq))
    }
}

impl std::str::FromStr for Square {
    type Err = SfenParseError;

    /// SFEN マス文字列をパースする。
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser_complete(Self::parse)(Bytes::from(s))
    }
}

impl std::fmt::Display for Square {
    /// SFEN マス文字列を出力する。
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.col().fmt(f)?;
        self.row().fmt(f)?;

        Ok(())
    }
}

/// `Square` でインデックスアクセスできる配列。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ArraySquare<T>([T; Square::NUM]);

impl<T> ArraySquare<T> {
    pub(crate) const fn new(inner: [T; Square::NUM]) -> Self {
        Self(inner)
    }

    pub(crate) const fn index_const(&self, sq: Square) -> &T {
        &self.0[sq.to_index()]
    }
}

impl<T: Copy> ArraySquare<T> {
    #[allow(dead_code)]
    pub(crate) const fn from_elem(elem: T) -> Self {
        Self([elem; Square::NUM])
    }
}

impl<T: Copy + Default> Default for ArraySquare<T> {
    fn default() -> Self {
        Self([T::default(); Square::NUM])
    }
}

impl<T> std::ops::Index<Square> for ArraySquare<T> {
    type Output = T;

    fn index(&self, sq: Square) -> &Self::Output {
        self.index_const(sq)
    }
}

impl<T> std::ops::IndexMut<Square> for ArraySquare<T> {
    fn index_mut(&mut self, sq: Square) -> &mut Self::Output {
        &mut self.0[sq.to_index()]
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    fn square_to_string(sq: Square) -> String {
        let b_col = b'0' + 9 - u8::try_from(sq.col().to_index()).unwrap();
        let b_row = b'a' + u8::try_from(sq.row().to_index()).unwrap();
        String::from_iter([char::from(b_col), char::from(b_row)])
    }

    #[test]
    fn test_square_parse() {
        for sq in Square::all() {
            let s = square_to_string(sq);
            assert_eq!(Square::from_str(&s).unwrap(), sq);
        }
    }

    #[test]
    fn test_square_fmt() {
        for sq in Square::all() {
            assert_eq!(sq.to_string(), square_to_string(sq));
        }
    }
}
