pub mod error;
pub mod lexer;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn lexer_works() {
        let path = Path::new("./examples/ex01.feo");
        let file = match std::fs::read_to_string(path) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => panic!("Unable to locate file"),
                std::io::ErrorKind::InvalidInput => panic!("Path not valid ({:?})", path),
                _ => panic!("Unable to read file"),
            },
        };

        let value = lexer::lex(&file, path).unwrap();
        println!("{:?}", value);
    }

    #[test]
    fn it_checks_file_contents() {
        let path = Path::new("./examples/ex01.feo");
        assert!(
            !std::fs::read_to_string(path).unwrap().is_empty(),
            "File is empty"
        );
    }
}
