use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct FileReader {
    path: String
}

type ReadResult = Result<(), std::io::Error>;

impl FileReader {
    pub fn new(path: String) -> Self {
        Self { path }
    }
    
    fn read<F>(&self, mut f: F) -> Result<(), std::io::Error> where F: FnMut(&mut Vec<u8>) -> ReadResult {
        let mut buf_reader = BufReader::new(File::open(&self.path)?);
        let mut buffer = Vec::new();

        loop {
            buffer.clear();
            // 띄어쓰기 전까지 읽기
            match buf_reader.read_until(b' ', &mut buffer) {
                Ok(0) => return Ok(()),
                Ok(_) => {
                    let _ = f(&mut buffer);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    pub fn find_pattern(&self, pattern: &str) -> Result<bool, std::io::Error> {
        let mut found = false;
        self.read(|buffer| -> ReadResult {
            let buf = String::from_utf8_lossy(buffer).trim().to_string();
            println!("buf: {}, pattern: {}", buf, pattern);
            if buf == pattern {
                found = true;
            }
            Ok(())
        })?;
        Ok(found)
    }
}