use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub struct FileReader {
    path: String
}

type ReadResult = Result<(), MiniGrepError>;

#[derive(Debug)]
pub enum MiniGrepError {
    InvalidUtf8,
    Io(std::io::Error),
    Other(String),
}

impl From<std::io::Error> for MiniGrepError {
    fn from(e: std::io::Error) -> Self {
        MiniGrepError::Io(e)
    }
}

impl From<String> for MiniGrepError {
    fn from(e: String) -> Self {
        MiniGrepError::Other(e)
    }
}

impl FileReader {
    const BUFFER_SIZE: usize = 1024;

    // 파일 경로를 받아서 파일 리더 인스턴스 생성
    pub fn new(path: String) -> Self {
        Self { path }
    }
    
    fn read<F>(&self, mut f: F) -> Result<(), MiniGrepError> where F: FnMut(&mut [u8; Self::BUFFER_SIZE]) -> ReadResult {
        let mut buf_reader = BufReader::new(File::open(&self.path)?);
        let mut buffer: [u8; Self::BUFFER_SIZE] = [0; Self::BUFFER_SIZE];

        loop {
            // 띄어쓰기 전까지 읽기
            // match buf_reader.read_until(b' ', &mut buffer) {
            //     Ok(0) => return Ok(()),
            //     Ok(_) => {
            //         let _ = f(&mut buffer);
            //     }
            //     Err(e) => {
            //         return Err(e);
            //     }
            // }
            match buf_reader.read(&mut buffer) {
                Ok(0) => return Ok(()),
                Ok(_) => {
                    let _ = f(&mut buffer);
                }
                Err(e) => {
                    return Err(MiniGrepError::Io(e).into());
                }
            }
        }
    }

    pub fn find_pattern(&self, pattern: &str) -> Result<bool, MiniGrepError> {
        let mut found = false;
        self.read(|buffer| -> ReadResult {
            let s = std::str::from_utf8(buffer).map_err(|_| MiniGrepError::InvalidUtf8)?;
            if let Some(index) = s.find(pattern) {
                println!("Found at index: {}", index);
                found = true;
            } else {
                println!("Not found");
            }
            Ok(())
        })?;
        Ok(found)
    }
}