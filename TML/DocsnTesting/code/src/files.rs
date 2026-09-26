use std::{fs::{File, OpenOptions},io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write},sync::{Mutex, OnceLock}};

static BIN: OnceLock<Mutex<Bin>> = OnceLock::new();

struct Bin {
    file: File,
    bookmark: u64,
    verified_len: u64
}

pub fn bin_establish() -> Result<()> {
    let filename = chrono::Local::now()
        .format("TML_%Y-%m-%d_%H-%M-%S.bin")
        .to_string();

    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create_new(true)
        .open(&filename)?;

    BIN.set(Mutex::new(Bin {
        file: file,
        bookmark: 0,
        verified_len: 0
    }))
        .map_err(|_| {
            Error::new(
            ErrorKind::AlreadyExists,
            "bin file already established",
            )
    })?;

    Ok(())
}

#[inline]
pub fn bin_write(byte_slice: &[u8]) -> Result<()> {
    let mut bin = BIN
        .get()
        .unwrap()
        .lock()
        .unwrap();

    bin.file.write_all(byte_slice)?;

    bin.verified_len = bin.file.seek(SeekFrom::End(0))?;
    Ok(())
}

#[inline]
pub fn bin_read_unread() -> Result<Vec<u8>> {
    let mut bin_vec: Vec<u8> = Vec::new();

    let mut bin = BIN
        .get()
        .unwrap()
        .lock()
        .unwrap();

    let bm = bin.bookmark;

    bin.file.seek(SeekFrom::Start(bm))?;

    bin.file.read_to_end(&mut bin_vec)?;

    bin.bookmark = bin.file.stream_position()?;

    Ok(bin_vec)
}

fn bin_verify(){
    let mut bin = BIN
        .get()
        .unwrap()
        .lock()
        .unwrap();

    if !bin.verified_len == bin.file.seek(SeekFrom::End(0)).unwrap(){
        todo!("Manual event bin verification logic not made yet!");
    }
}