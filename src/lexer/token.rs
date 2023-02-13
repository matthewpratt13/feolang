#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TokType {
    // keywords
    KW_ABSTRACT,
    KW_ALIAS,
    KW_AS,
    KW_BOOL,
    KW_BREAK,
    KW_CHAR,
    KW_CLASS,
    KW_CONST,
    KW_CONTINUE,
    KW_ELSE,
    KW_ENUM,
    KW_EXTERN,
    KW_FINAL,
    KW_FLOAT,
    KW_FOR,
    KW_FUNC,
    KW_IF,
    KW_IMPORT,
    KW_INT,
    KW_IS,
    KW_LET,
    KW_LIB,
    KW_LOOP,
    KW_MATCH,
    KW_NEW,
    KW_OVERRIDE,
    KW_PROTOCOL,
    KW_PUBLIC,
    KW_RETURN,
    KW_SELF,
    KW_STATIC,
    KW_STRING,
    KW_STRUCT,
    KW_SUPER,
    KW_TYPE,
    KW_UINT,
    KW_VAR,
    KW_VIRTUAL,
    KW_WHILE,

    // identifier
    IDEN(String),

    // literals
    LIT_BOOL(bool),
    LIT_CHAR(char),
    LIT_FLOAT(f64),
    LIT_INT(i32),
    LIT_STRING(String),

    // operators
    OP_AMPERSAND,
    OP_AND,
    OP_ASSIGN,
    OP_BANG,
    OP_DIV,
    OP_DIV_EQ,
    OP_EQ,
    OP_FAT_ARW,
    OP_GRTR,
    OP_GRTR_EQ,
    OP_LESS,
    OP_LESS_EQ,
    OP_MINUS,
    OP_MINUS_EQ,
    OP_MOD,
    OP_MOD_EQ,
    OP_MULT,
    OP_MULT_EQ,
    OP_NOT_EQ,
    OP_OR,
    OP_PIPE,
    OP_PLUS,
    OP_PLUS_EQ,
    OP_TERNARY,
    OP_THIN_ARW,

    // punctuation
    PUNC_DOT,
    PUNC_COMMA,
    PUNC_COLON,
    PUNC_SEMICOLON,
    PUNC_OPEN_CRL_BRC,
    PUNC_CLS_CRL_BRC,
    PUNC_OPEN_SQ_BKT,
    PUNC_CLS_SQ_BKT,
    PUNC_OPEN_PAREN,
    PUNC_CLS_PAREN,

    INVALID_CHAR(char),
}

impl From<String> for TokType {
    fn from(v: String) -> Self {
        Self::LIT_STRING(v)
    }
}

impl From<i32> for TokType {
    fn from(v: i32) -> Self {
        Self::LIT_INT(v)
    }
}

impl From<f64> for TokType {
    fn from(v: f64) -> Self {
        Self::LIT_FLOAT(v)
    }
}

impl From<char> for TokType {
    fn from(v: char) -> Self {
        Self::LIT_CHAR(v)
    }
}

impl From<bool> for TokType {
    fn from(v: bool) -> Self {
        Self::LIT_BOOL(v)
    }
}

impl TokType {
    /// Returns `true` if the tok type is [`TIden`].
    ///
    #[must_use]
    pub fn is_iden(&self) -> bool {
        matches!(self, Self::IDEN(..))
    }

    /// Returns `true` if the tok type is a literal
    ///
    #[must_use]
    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            Self::LIT_BOOL(..)
                | Self::LIT_CHAR(..)
                | Self::LIT_FLOAT(..)
                | Self::LIT_INT(..)
                | Self::LIT_STRING(..)
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
    pub fn new(tok_type: TokType, line: usize, col: usize) -> Self {
        Self {
            tok_type,
            _line: line,
            _col: col,
        }
    }

    pub fn tok_type(&self) -> &TokType {
        &self.tok_type
    }
}
