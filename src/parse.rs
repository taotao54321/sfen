use crate::bytes::Bytes;

/// SFEN パースエラー。
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum SfenParseError {
    /// `s[offset..]` のパースに失敗した。
    InvalidInput {
        offset: usize,
        description: &'static str,
    },

    /// パース完了後、`s[offset..]` に余分な入力がある。
    Extra { offset: usize },
}

impl SfenParseError {
    pub(crate) fn invalid_input(bytes: Bytes, description: &'static str) -> Self {
        Self::InvalidInput {
            offset: bytes.base_offset(),
            description,
        }
    }

    pub(crate) fn extra(bytes: Bytes) -> Self {
        Self::Extra {
            offset: bytes.base_offset(),
        }
    }
}

impl std::fmt::Display for SfenParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidInput {
                offset,
                description,
            } => write!(f, "invalid input at s[{offset}..]: {description}"),
            Self::Extra { offset } => write!(f, "extra input at s[{offset}..]"),
        }
    }
}

impl std::error::Error for SfenParseError {}

pub type SfenParseResult<T> = Result<T, SfenParseError>;

pub(crate) fn parser_complete<T, F>(f: F) -> impl Fn(Bytes) -> SfenParseResult<T>
where
    F: Fn(Bytes) -> SfenParseResult<(Bytes, T)>,
{
    move |bytes| {
        let (remain, res) = f(bytes)?;
        remain
            .is_empty()
            .then_some(res)
            .ok_or_else(|| SfenParseError::extra(remain))
    }
}
