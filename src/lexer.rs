use crate::error::{ErrorId, FeoError};
use std::path::Path;
use token::{TokType, Token};

mod token;

/// States whether or not the lexer is in a comment block, determining certain behaviour
///
static mut COMMENT_BLOCK: bool = false;

/// Returns a list of `Token` collected into a list of the input file's individual lines
///
/// # Arguments
///
/// * `file` - the contents of the file
/// * `path` - a reference to the location of the file in memory
///
pub fn lex(file: &String, path: &'static Path) -> Option<Vec<Vec<Token>>> {
    let mut tokens: Vec<Vec<Token>> = Vec::new();
    let mut line_num: usize = 1;

    // iterate over the lines of a source `String` as individual string slices
    // check that each string slice is not empty
    for l in file.lines() {
        let chars: Vec<char> = l.chars().collect();

        // check for valid input parameters
        let line: Vec<Token> = match tokenize_line(&chars, line_num, l.len(), path) {
            Ok(l) => l,
            Err(_) => {
                let err = FeoError::new(
                    ErrorId::InvalidData,
                    None,
                    path,
                    line_num,
                    1,
                    "Unable to tokenize line",
                );

                panic!("{}", err);
            }
        };

        tokens.push(line);
        line_num += 1;
    }

    Some(tokens)
}

/// Returns a list of the input line's tokens
///
/// # Arguments
///
/// * `chars` - a reference to a line in the input `&str`, collected into a list of `char`
/// * `line_num` - the line's number (index + 1)
/// * `line_len` - the length of the line
/// * `path` - the location of the source file in memory
///
fn tokenize_line(
    chars: &Vec<char>,
    line_num: usize,
    line_len: usize,
    path: &'static Path,
) -> Result<Vec<Token>, FeoError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut i: usize = 0;
    let mut tok_i: usize = 0;
    let mut is_negative_number: bool = false;

    while i < line_len {
        match chars[i] {
            _ if chars[i].is_ascii_whitespace() || chars[i].is_ascii_control() => {
                i += 1;
                continue;
            }

            _ if chars[i] == '*' && chars[i + 1] == '/' => {
                if unsafe { COMMENT_BLOCK == false } {
                    let err: FeoError = FeoError::new(
                        ErrorId::InvalidChar,
                        Some('*'),
                        path,
                        line_num,
                        i + 1,
                        "Encountered multi-line comment terminator in an uncommented block",
                    );

                    panic!("{}", err);
                }

                i += 2;
                make_comment_false();
                continue;
            }

            _ if chars[i] == '/' && chars[i + 1] == '*' => {
                i += 2;
                make_comment_true();
                continue;
            }

            _ if unsafe { COMMENT_BLOCK == true } => {
                i += 1;
                continue;
            }

            _ if chars[i] == '/' && chars[i + 1] == '/' => break,

            _ if chars[i].is_ascii_alphabetic() || chars[i] == '_' => {
                let start_index: usize = i;
                let alpha = get_alpha(chars, line_num, line_len, i, path).unwrap();
                let tok_type: TokType = match alpha.as_str() {
                    "const" => TokType::TConst,
                    "let" => TokType::TLet,
                    "var" => TokType::TVar,
                    "bool" => TokType::TBool,
                    "char" => TokType::TChar,
                    "int" => TokType::TInt,
                    "float" => TokType::TFloat,
                    "String" => TokType::TString,
                    "false" => TokType::TBoolLit(false),
                    "true" => TokType::TBoolLit(true),
                    "alias" => TokType::TKeyword("alias"),
                    "as" => TokType::TKeyword("as"),
                    "break" => TokType::TKeyword("break"),
                    "class" => TokType::TKeyword("class"),
                    "continue" => TokType::TKeyword("continue"),
                    "else" => TokType::TKeyword("else"),
                    "enum" => TokType::TKeyword("enum"),
                    "for" => TokType::TKeyword("for"),
                    "func" => TokType::TKeyword("func"),
                    "if" => TokType::TKeyword("if"),
                    "import" => TokType::TKeyword("import"),
                    "in" => TokType::TKeyword("in"),
                    "interface" => TokType::TKeyword("interface"),
                    "loop" => TokType::TKeyword("loop"),
                    "match" => TokType::TKeyword("match"),
                    "new" => TokType::TKeyword("new"),
                    "None" => TokType::TKeyword("None"),
                    "public" => TokType::TKeyword("public"),
                    "self" => TokType::TKeyword("self"),
                    "static" => TokType::TKeyword("static"),
                    "struct" => TokType::TKeyword("struct"),
                    "return" => TokType::TKeyword("return"),
                    "while" => TokType::TKeyword("while"),
                    "_" => {
                        let err = FeoError::new(
                            ErrorId::InvalidChar,
                            Some('_'),
                            path,
                            line_num,
                            i + 1,
                            "Invalid keyword or identifier",
                        );

                        panic!("{}", err);
                    }

                    _ => TokType::TIden(alpha.clone()),
                };

                i += alpha.len();

                let tok = Token::new(tok_type, line_num, start_index + 1);
                tokens.push(tok);
                tok_i += 1;
                continue;
            }

            _ if i < line_len - 1 && chars[i] == '-' && chars[i + 1].is_ascii_digit() => {
                is_negative_number = true;
                i += 1;
                continue;
            }

            _ if i < line_len - 1 && chars[i] == '0' && chars[i + 1] == 'x' => {
                let start_index: usize;

                if is_negative_number {
                    start_index = &i - 1;
                } else {
                    start_index = i;
                }

                let hex = get_hex(chars, line_num, line_len, i, is_negative_number, path).unwrap();
                let tok_type: TokType = match hex {
                    _ if hex.contains('.') => {
                        let err = FeoError::new(
                            ErrorId::InvalidChar,
                            Some('.'),
                            path,
                            line_num,
                            i + 1,
                            "Hexadecimal float types are unsupported",
                        );

                        panic!("{}", err);
                    }

                    _ => TokType::TIntLit(i32::from_str_radix(hex.as_str(), 16).unwrap()),
                };

                if is_negative_number {
                    i += &hex.len() + 1;
                } else {
                    i += &hex.len() + 2;
                }

                let tok = Token::new(tok_type, line_num, start_index + 1);
                tokens.push(tok);
                tok_i += 1;
                continue;
            }

            _ if chars[i].is_ascii_digit() => {
                let start_index: usize;

                if is_negative_number {
                    start_index = &i - 1;
                } else {
                    start_index = i;
                }

                let num = get_num(chars, line_num, line_len, i, is_negative_number, path).unwrap();
                let tok_type: TokType = match num {
                    _ if num.contains('.') => TokType::TFltLit(num.parse::<f64>().unwrap()),
                    _ => TokType::TIntLit(num.parse::<i32>().unwrap()),
                };

                if is_negative_number {
                    i += &num.len() - 1;
                } else {
                    i += &num.len();
                }

                let tok = Token::new(tok_type, line_num, start_index + 1);
                tokens.push(tok);
                tok_i += 1;
                continue;
            }

            _ if chars[i] == '\'' || chars[i] == '"' => {
                let start_index: usize = i;

                if i < line_len - 1 {
                    i += 1;
                } else {
                    break;
                }

                let lit = get_text_literal(chars, line_num, line_len, i, path).unwrap();
                let tok_type: TokType = match chars[start_index] {
                    '\'' => TokType::TCharLit(lit.parse::<char>().unwrap()),
                    '"' => TokType::TStrLit(lit.to_owned()),
                    _ => TokType::TInvalid(chars[start_index]),
                };

                i += &lit.len() + 1; // +1 to skip closing quote

                let tok = Token::new(tok_type, line_num, start_index + 1);
                tokens.push(tok);
                tok_i += 1;
                continue;
            }

            '=' => {
                if i < line_len - 1
                    && (tokens[tok_i - 1].tok_type().is_iden()
                        || tokens[tok_i - 1].tok_type().is_literal())
                {
                    {
                        let start_index: usize = i;

                        let tok_type: TokType = match chars[i] {
                            _ if chars[i + 1] == '=' => {
                                i += 1;
                                TokType::TLogicalOp("==")
                            }
                            _ if chars[i + 1] == '>' => {
                                i += 1;
                                TokType::TFatArrow
                            }

                            _ => TokType::TAssign,
                        };

                        i += 1;

                        let tok = Token::new(tok_type, line_num, start_index + 1);
                        tokens.push(tok);
                        tok_i += 1;
                        continue;
                    }
                }
            }

            _ if chars[i].is_ascii_punctuation()
                && (tokens[tok_i - 1].tok_type().is_iden()
                    || tokens[tok_i - 1].tok_type().is_literal()) =>
            {
                let start_index: usize = i;

                let tok_type: TokType = match chars[i] {
                    '+' if i < line_len - 1 => {
                        if chars[i + 1] == '=' {
                            i += 1;
                            TokType::TArithmeticOp("+=")
                        } else {
                            TokType::TArithmeticOp("+")
                        }
                    }

                    '-' if i < line_len - 1 => {
                        if chars[i + 1] == '=' {
                            i += 1;
                            TokType::TArithmeticOp("-=")
                        } else if chars[i + 1] == '>' {
                            i += 1;
                            TokType::TThinArrow
                        } else {
                            TokType::TArithmeticOp("-")
                        }
                    }

                    '*' if i < line_len - 1 => {
                        if chars[i + 1] == '=' {
                            i += 1;
                            TokType::TArithmeticOp("*=")
                        } else {
                            TokType::TArithmeticOp("*")
                        }
                    }

                    '/' if i < line_len - 1 => {
                        if chars[i + 1] == '=' {
                            i += 1;
                            TokType::TArithmeticOp("/=")
                        } else {
                            TokType::TArithmeticOp("/")
                        }
                    }

                    '%' if i < line_len - 1 => {
                        if chars[i + 1] == '=' {
                            i += 1;
                            TokType::TArithmeticOp("%=")
                        } else {
                            TokType::TArithmeticOp("%")
                        }
                    }

                    '<' if i < line_len - 1 => {
                        if chars[i + 1] == '=' {
                            i += 1;
                            TokType::TLogicalOp("<=")
                        } else {
                            TokType::TLogicalOp("<")
                        }
                    }

                    '>' if i < line_len - 1 => {
                        if chars[i + 1] == '=' {
                            i += 1;
                            TokType::TLogicalOp(">=")
                        } else {
                            TokType::TLogicalOp(">")
                        }
                    }

                    '!' if i < line_len - 1 => {
                        if chars[i + 1] == '=' {
                            i += 1;
                            TokType::TLogicalOp("!=")
                        } else {
                            TokType::TLogicalOp("!")
                        }
                    }

                    '&' if i < line_len - 1 => {
                        if chars[i + 1] == '&' {
                            i += 1;
                            TokType::TLogicalOp("&&")
                        } else {
                            TokType::TLogicalOp("&")
                        }
                    }

                    '|' if i < line_len - 1 => {
                        if chars[i + 1] == '|' {
                            i += 1;
                            TokType::TLogicalOp("||")
                        } else {
                            TokType::TLogicalOp("|")
                        }
                    }

                    '?' => TokType::TLogicalOp("?"),

                    ';' => TokType::TSemicolon,

                    ':' => TokType::TColon,

                    ',' => TokType::TComma,

                    '.' => TokType::TDot,

                    '(' => TokType::TOpenParen,

                    ')' => TokType::TCloseParen,

                    '[' => TokType::TOpenBracket,

                    ']' => TokType::TCloseBracket,

                    '{' => TokType::TOpenBrace,

                    '}' => TokType::TCloseBrace,

                    '\\' => {
                        let err = FeoError::new(
                            ErrorId::InvalidChar,
                            Some('\\'),
                            path,
                            line_num,
                            i + 1,
                            "Escape character encountered out of context",
                        );

                        panic!("{}", err);
                    }

                    _ => TokType::TInvalid(chars[i]),
                };

                let tok = Token::new(tok_type, line_num, start_index + 1);
                tokens.push(tok);
                tok_i += 1;
            }

            _ => {
                tokens.push(Token::new(TokType::TInvalid(chars[i]), line_num, i + 1));
                tok_i += 1;
            }
        }

        i += 1;
    }

    Ok(tokens)
}

/// Returns a `String` containing either a keyword or identifier
///
/// # Arguments
///
/// * `chars` - a reference to a list of `char` (a line in the file)
/// * `line_len` - the length of the line
/// * `i` - the index of a given `char` in the line
/// * `path` - the location of the source file in memory
///
fn get_alpha(
    chars: &Vec<char>,
    line_num: usize,
    line_len: usize,
    mut i: usize,
    path: &'static Path,
) -> Option<String> {
    let mut buf = String::new();

    buf.push(chars[i]);
    i += 1;

    while i < line_len {
        if !(chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
            break;
        }

        buf.push(chars[i]);
        i += 1;
    }

    if buf.is_empty() {
        let err = FeoError::new(
            ErrorId::InvalidChar,
            None,
            path,
            line_num,
            i + 1,
            "Missing alphanumeric or `_` value",
        );

        panic!("{}", err);
    } else {
        Some(buf)
    }
}

/// Returns a `String` containing a number literal
///
/// # Arguments
///
/// * `chars` - a reference to a list of `char` (a line in the file)
/// * `line_num` - the line's number (index + 1)
/// * `line_len` - the length of the line
/// * `i` - the index of a given `char` in the line
/// * `is_negative` - a `bool` to indicate the number's sign
/// * `path` - the location of the source file in memory
///
fn get_num(
    chars: &Vec<char>,
    line_num: usize,
    line_len: usize,
    mut i: usize,
    is_negative: bool,
    path: &'static Path,
) -> Option<String> {
    let mut buf = String::new();

    if is_negative {
        buf.push('-')
    }

    while i < line_len && (chars[i].is_ascii_digit() || chars[i] == '_' || chars[i] == '.') {
        if chars[i] == '_' {
            if i < line_len - 1 {
                i += 1;
            } else {
                break;
            }
        }

        if chars[i] == '.' && buf.contains('.') {
            let err = FeoError::new(
                ErrorId::InvalidChar,
                Some('.'),
                path,
                line_num,
                i + 1,
                "Float types can only have one point",
            );

            panic!("{}", err);
        }

        buf.push(chars[i]);
        i += 1;
    }

    if buf.is_empty() {
        let err = FeoError::new(
            ErrorId::InvalidChar,
            None,
            path,
            line_num,
            i + 1,
            "Missing numeric type",
        );

        panic!("{}", err);
    } else {
        Some(buf)
    }
}

/// Returns a `String` containing a hexadecimal number literal
///
/// # Arguments
///
/// * `chars` - a reference to a list of `char` (a line in the file)
/// * `line_num` - the line's number (index + 1)
/// * `line_len` - the length of the line
/// * `i` - the index of a given `char` in the line
/// * `is_negative` - a `bool` to indicate the number's sign
/// * `path` - the location of the source file in memory
///
fn get_hex(
    chars: &Vec<char>,
    line_num: usize,
    line_len: usize,
    mut i: usize,
    is_negative: bool,
    path: &'static Path,
) -> Option<String> {
    let mut buf = String::new();

    if is_negative {
        buf.push('-')
    }

    i += 2;

    while i < line_len && (chars[i].is_ascii_hexdigit() || chars[i] == '_' || chars[i] == '.') {
        if chars[i] == '_' {
            if i < line_len - 1 {
                i += 1;
            } else {
                break;
            }
        }

        if chars[i] == '.' && buf.contains('.') {
            let err = FeoError::new(
                ErrorId::InvalidChar,
                Some('.'),
                path,
                line_num,
                i + 1,
                "Float types can only have one point",
            );

            panic!("{}", err);
        }

        buf.push(chars[i]);
        i += 1;
    }

    if buf.is_empty() {
        let err = FeoError::new(
            ErrorId::InvalidChar,
            None,
            path,
            line_num,
            i + 1,
            "Missing numeric type",
        );

        panic!("{}", err);
    } else {
        Some(buf)
    }
}

/// Returns a `String` containing either a `char` or `String` literal
///
/// # Arguments
///
/// * `chars` - a reference to a list of `char` (a line in the file)
/// * `line_num` - the line's number (index + 1)
/// * `line_len` - the length of the line
/// * `i` - the index of a given `char` in the line
/// * `path` - the location of the source file in memory
///
fn get_text_literal(
    chars: &Vec<char>,
    line_num: usize,
    line_len: usize,
    mut i: usize,
    path: &'static Path,
) -> Option<String> {
    let start_index: usize = &i - 1;
    let quote_type: char = chars[start_index];
    let mut buf = String::new();

    while i < line_len - 1 {
        if chars[i] == quote_type {
            break;
        }

        if chars[i] == '\\' {
            i += 1;
        }

        buf.push(chars[i]);
        i += 1;
    }

    if chars[i] != quote_type {
        let err = FeoError::new(
            ErrorId::InvalidChar,
            Some(quote_type),
            path,
            line_num,
            start_index + 1,
            "Missing quote character",
        );

        panic!("{}", err);
    };

    if quote_type == '\'' {
        if buf.is_empty() {
            let err = FeoError::new(
                ErrorId::InvalidChar,
                None,
                path,
                line_num,
                start_index + 1,
                "Missing character value. Character literals cannot be empty",
            );

            panic!("{}", err);
        }

        if buf.len() > 1 {
            let err = FeoError::new(
                ErrorId::InvalidChar,
                None,
                path,
                line_num,
                start_index + 1,
                "Character literals must consist of a single value",
            );

            panic!("{}", err);
        }
    }

    Some(buf)
}

/// Change the value of `COMMENT_BLOCK` to true
///
fn make_comment_true() {
    unsafe { COMMENT_BLOCK = true }
}

/// Change the value of `COMMENT_BLOCK` to true
///
fn make_comment_false() {
    unsafe { COMMENT_BLOCK = false }
}

