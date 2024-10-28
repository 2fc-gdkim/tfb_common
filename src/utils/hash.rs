use data_encoding::HEXLOWER;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;

/// calculates sha256 digest as lowercase hex string
pub fn sha256_digest(path: &Path) -> Result<String, io::Error> {
    let input = File::open(path)?;
    let mut reader = BufReader::new(input);

    let digest = {
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 { break }
            hasher.update(&buffer[..count]);
        }
        hasher.finalize()
    };
    Ok(HEXLOWER.encode(digest.as_ref()))
}

pub fn get_file_hash256(file_path: &str) -> String {
    //아래쪽의 속도가 약간더 빠름
    // let mut file = File::open(file_path).unwrap();
    // let mut hasher = Sha256::new();
    // std::io::copy(&mut file, &mut hasher).unwrap();
    // let digest = hasher.finalize();
    
    // // 해시 결과를 16진수 문자열로 변환합니다.
    // format!("{:X}", digest)


    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);

    // SHA256 해시를 계산합니다.
    let digest = {
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer).unwrap();
            if count == 0 { break }
            hasher.update(&buffer[..count]);
        }
        hasher.finalize()
    };
    
    // 해시 결과를 16진수 문자열로 변환합니다.
    format!("{:X}", digest)
}