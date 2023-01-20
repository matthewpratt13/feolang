#[derive(Debug, PartialEq)]
pub enum TokType {
    TConst,
    TLet,
    TVar,
    TKeyword(&'static str),
    TIden(String),

    TBool,
    TChar,
    TInt,
    TFloat,
    TString,

    TBoolLit(bool),
    TCharLit(char),
    TFltLit(f64),
    TIntLit(i32),
    TStrLit(String),

    TAssign,
    TFatArrow,
    TThinArrow,

    TArithmeticOp(&'static str),
    TLogicalOp(&'static str),

    TComma,
    TDot,
    TColon,
    TSemicolon,
    TOpenBrace,
    TCloseBrace,
    TOpenBracket,
    TCloseBracket,
    TOpenParen,
    TCloseParen,

    TInvalid(char),
}

impl From<String> for TokType {
    fn from(v: String) -> Self {
        Self::TStrLit(v)
    }
}

impl From<i32> for TokType {
    fn from(v: i32) -> Self {
        Self::TIntLit(v)
    }
}

impl From<f64> for TokType {
    fn from(v: f64) -> Self {
        Self::TFltLit(v)
    }
}

impl From<char> for TokType {
    fn from(v: char) -> Self {
        Self::TCharLit(v)
    }
}

impl From<bool> for TokType {
    fn from(v: bool) -> Self {
        Self::TBoolLit(v)
    }
}

impl TokType {
    /// Returns `true` if the tok type is [`TIden`].
    ///
    /// [`TIden`]: TokType::TIden
    #[must_use]
    pub fn is_iden(&self) -> bool {
        matches!(self, Self::TIden(..))
    }

    /// Returns `true` if the tok type is [`TIntLit`].
    ///
    /// [`TIntLit`]: TokType::TIntLit
    #[must_use]
    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            Self::TBoolLit(..)
                | Self::TCharLit(..)
                | Self::TFltLit(..)
                | Self::TIntLit(..)
                | Self::TStrLit(..)
        )
    }
}

#[derive(Debug)]
pub struct Token {
    tok_type: TokType,
    _line: usize,
    _col: usize,
}

impl Token {
    pub fn new(tok_type: TokType, _line: usize, _col: usize) -> Token {
        Token {
            tok_type,
            _line,
            _col,
        }
    }

    pub fn tok_type(&self) -> &TokType {
        &self.tok_type
    }
}
