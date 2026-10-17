use std::io::Write;
use std::path::PathBuf;
use tempfile::{TempDir, tempdir};

pub fn fill_temp_file(file_size: usize, file: &mut std::fs::File) {
    let mut data = vec![0u8; file_size];

    for (index, byte) in data.iter_mut().enumerate() {
        *byte = index as u8;
    }

    file.write_all(&data).unwrap();
    file.sync_all().unwrap();
}

pub fn create_filled_tempfile(file_size: usize) -> (TempDir, PathBuf) {
    let dir = tempdir().unwrap();
    let path = dir.path().join("tmp");
    let mut temp_file = std::fs::File::create(dir.path().join("tmp")).unwrap();
    fill_temp_file(file_size, &mut temp_file);
    (dir, path)
}

pub fn fill_pattern(key: u8, buffer: &mut [u8]) {
    assert_eq!(buffer.len() % 2, 0);

    for (index, chunk) in buffer.chunks_exact_mut(2).enumerate() {
        chunk[0] = key;
        chunk[1] = index as u8;
    }
}

pub fn validate_pattern(key: u8, buffer: &[u8]) -> bool {
    assert_eq!(buffer.len() % 2, 0);

    for (index, chunk) in buffer.chunks_exact(2).enumerate() {
        if chunk[0] != key || chunk[1] != index as u8 {
            return false;
        }
    }

    true
}

pub fn validate_block(data: &[u8]) -> bool {
    for (index, byte) in data.iter().enumerate() {
        if *byte != index as u8 {
            return false;
        }
    }

    true
}
