use std::{io::Read, path::Path};
use sha256::digest;
pub struct Chunk {
    pub data: Vec<u8>,
    pub hash: String
}

pub fn chunk_file<P>(path: P, chunk_size: usize, use_blake: bool) -> Vec<Chunk> where P: AsRef<Path> {
    let mut file = std::fs::File::open(path).unwrap();
    let mut output: Vec<Chunk> = Vec::new();

    loop {
        let mut chunk: Vec<u8> = Vec::with_capacity(chunk_size);
        let n = file.by_ref().take(chunk_size as u64).read_to_end(&mut chunk).unwrap();
        if n == 0 { break; } // nothing read
        let hash;
        if use_blake {
            hash = blake3::hash(&chunk).to_string();
        }
        else {
            hash = digest(chunk.clone());
        }
        output.push(Chunk { data: chunk, hash });
        if n < chunk_size { break; } // if we read less bytes than chunk size, exit loop too
    }

    output
}

pub fn hash_file<P>(path: P, chunk_size: usize, use_blake: bool) -> Vec<String> where P: AsRef<Path> {
    let mut file = std::fs::File::open(path).unwrap();
    let mut output: Vec<String> = Vec::new();

    loop {
        let mut chunk: Vec<u8> = Vec::with_capacity(chunk_size);
        let n = file.by_ref().take(chunk_size as u64).read_to_end(&mut chunk).unwrap();
        if n == 0 { break; } // nothing read
        let hash: String;
        if use_blake {
            hash = blake3::hash(&chunk).to_string();
        }
        else {
            hash = digest(chunk.clone());
        }
        output.push(hash);
        if n < chunk_size { break; } // if we read less bytes than chunk size, exit loop too
    }

    output
}