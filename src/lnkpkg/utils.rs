use byteorder::{ReadBytesExt, LittleEndian};
use std::io::Read;
use std::io::Error;
use std::slice;

pub fn get_u8_vec<R: Read>(mut reader: R) -> Result<Vec<u8>,Error> {
    let mut string_vec: Vec<u8> = Vec::new();

    loop {
        let u8_char = reader.read_u8()?;
        if u8_char == 0x00 {
            break
        } else {
            string_vec.push(u8_char);
        }
    }

    Ok(string_vec)
}

pub fn get_u8_vec_utf16<R: Read>(mut reader: R) -> Result<Vec<u8>,Error>{
    let mut u16_vec: Vec<u16> = Vec::new();

    loop {
        let u16_char = reader.read_u16::<LittleEndian>()?;
        if u16_char == 0x0000 {
            break
        } else {
            u16_vec.push(u16_char);
        }
    }

    let u8_slice: &[u8] = unsafe {
        slice::from_raw_parts(u16_vec.as_ptr() as *const u8, u16_vec.len() * 2)
    };

    Ok(u8_slice.to_vec())
}

pub fn get_u16_vec<R: Read>(mut reader: R) -> Result<Vec<u16>,Error> {
    let mut string_vec: Vec<u16> = Vec::new();

    loop {
        let u16_char = reader.read_u16::<LittleEndian>()?;
        if u16_char == 0x0000 {
            break
        } else {
            string_vec.push(u16_char);
        }
    }

    Ok(string_vec)
}
