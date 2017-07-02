use encoding::all::UTF_16LE;
use encoding::{Encoding, DecoderTrap};
use byteorder::{ReadBytesExt, LittleEndian};
use lnkpkg::errors::{LnkError};
use lnkpkg::utils;
use std::io::SeekFrom;
use std::io::Read;
use std::io::Seek;

#[derive(Debug)]
pub struct VolumeInfo {
    pub vi_size: u32,
    pub drive_type: u32,
    pub serial_number: u32,
    pub offset_vol_label: u32,
    pub offset_vol_label_unicode: Option<u32>,
    pub volume_label: String,
    pub volume_label_unicode: Option<String>
}

impl VolumeInfo {
    pub fn new<Rs: Read+Seek>(mut reader: Rs) -> Result<VolumeInfo,LnkError> {
        // Get Current Pos
        let cur_pos = reader.seek(
            SeekFrom::Current(0)
        )?;

        let vi_size = reader.read_u32::<LittleEndian>()?;
        let drive_type = reader.read_u32::<LittleEndian>()?;
        let serial_number = reader.read_u32::<LittleEndian>()?;
        let offset_vol_label = reader.read_u32::<LittleEndian>()?;

        let mut offset_vol_label_unicode = None;
        if offset_vol_label > 16 {
            offset_vol_label_unicode = Some(reader.read_u32::<LittleEndian>()?);
        }

        // Go back to start of VolumeInfo
        reader.seek(
            SeekFrom::Start(cur_pos + offset_vol_label as u64)
        )?;

        // Get volume label
        let volume_label = String::from_utf8(
            utils::get_u8_vec(&mut reader)?
        )?;

        let mut volume_label_unicode = None;
        if offset_vol_label_unicode.is_some() {
            let utf16_buffer = utils::get_u8_vec_utf16(&mut reader)?;
            let utf16_string = match UTF_16LE.decode(&utf16_buffer.as_slice(),DecoderTrap::Ignore) {
                Ok(utf16) => utf16,
                Err(error) => return Err(
                    LnkError::utf16_decode_error(
                        format!("Error decoding name in volume_label_unicode field. [{}]",error)
                    )
                )
            };
            volume_label_unicode = Some(utf16_string);
        }

        Ok(
            VolumeInfo {
                vi_size: vi_size,
                drive_type: drive_type,
                serial_number: serial_number,
                offset_vol_label: offset_vol_label,
                offset_vol_label_unicode: offset_vol_label_unicode,
                volume_label: volume_label,
                volume_label_unicode: volume_label_unicode
            }
        )
    }
}
