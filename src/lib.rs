pub mod error;

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn it_reads_a_file() {
        let path = Path::new("./examples/ex01.feo");
        match std::fs::read_to_string(path) {
            Ok(f) => f,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => panic!("Unable to locate file"),
                std::io::ErrorKind::InvalidInput => panic!("Path not valid ({:?})", path),
                _ => panic!("Unable to read file"),
            },
        };
    }

    #[test]
    fn it_checks_file_contents() {
        let path = Path::new("./examples/ex01.feo");
        assert!(!std::fs::read_to_string(path).unwrap().is_empty());
    }
}
