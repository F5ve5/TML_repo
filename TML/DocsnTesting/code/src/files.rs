use std::{fs::{File, OpenOptions}, io::{Result, Write, Error}, string::String, sync::OnceLock};

pub static BIN_FILE: OnceLock<File> = OnceLock::new();

pub fn bin_establish() -> Result<()> {

    let filename = chrono::Local::now()
        .format("TML_%Y-%m-%d_%H-%M-%S.bin")
        .to_string();

    let reject = BIN_FILE.set(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&filename)?
    );

    match reject{
        Ok(()) => return Ok(()),
        Err(_) => return Err(Error::new(std::io::ErrorKind::AlreadyExists, "bin file already initialized")) 
    };
}

pub fn bin_write(byte_slice: &[u8]) -> Result<()>{

    BIN_FILE
        .get()
        .unwrap()
        .write_all(byte_slice)?;

    return Ok(());
}