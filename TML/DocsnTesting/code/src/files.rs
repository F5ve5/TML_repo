use std::{fs::{File, OpenOptions}, io::{Error, Read, Result, Seek, SeekFrom, Write}, sync::{Mutex, OnceLock}};

static BIN_FILE: OnceLock<File> = OnceLock::new();
static BIN_FILE_BOOKMARK: OnceLock<Mutex<u64>> = OnceLock::new();

pub fn bin_establish() -> Result<()> {

    let filename = chrono::Local::now()
        .format("TML_%Y-%m-%d_%H-%M-%S.bin")
        .to_string();

    let bf_set = BIN_FILE.set(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&filename)?
    );
    match bf_set{
        Ok(_) => {},
        Err(_) => return Err(Error::new(std::io::ErrorKind::AlreadyExists, "bin file already established")) 
    };

    BIN_FILE_BOOKMARK.set(Mutex::new(0)).unwrap();

    return Ok(());
}

pub fn bin_write(byte_slice: &[u8]) -> Result<()>{

    BIN_FILE
        .get()
        .unwrap()
        .write_all(byte_slice)?;

    return Ok(());
}

pub fn bin_read_unread() -> Result<Vec<u8>> {

    let mut bin_vec: Vec<u8> = Vec::new();

    let mut bf = BIN_FILE
        .get()
        .unwrap();

    bf
        .seek(SeekFrom::Start(
        *(BIN_FILE_BOOKMARK
            .get()
            .unwrap()
            .lock()
            .unwrap())))?;

    bf.read_to_end(&mut bin_vec)?;

    //update bookmark, beautiful
    *(BIN_FILE_BOOKMARK
        .get()
        .unwrap()
        .lock()
        .unwrap()) = 
    BIN_FILE
        .get()
        .unwrap()
        .stream_position()?;

    return Ok(bin_vec);
}