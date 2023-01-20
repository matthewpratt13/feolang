use std::{fs, io::ErrorKind, path::Path};

mod lexer;

fn main() {
    const SRC: &str = "./examples/ex01.feo";

    let path: &Path = Path::new(SRC);
    let file: String = match fs::read_to_string(path) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => panic!("Unable to locate file"),
            ErrorKind::InvalidInput => panic!("Path not valid ({:?})", path),
            _ => panic!("Unable to read file"),
        },
    };

    let tokens = lexer::lex(&file, path).unwrap();
    println!("{:?}", tokens);
}
