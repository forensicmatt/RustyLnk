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
        let _offset = reader.seek(
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

        // Get volume label
        reader.seek(
            SeekFrom::Start(_offset + offset_vol_label as u64)
        )?;
        let volume_label = utils::read_string_u8_till_null(&mut reader)?;

        let mut volume_label_unicode = None;
        if offset_vol_label_unicode.is_some() {
            reader.seek(
                SeekFrom::Start(_offset + offset_vol_label_unicode.unwrap() as u64)
            )?;
            volume_label_unicode = Some(utils::read_string_u16_till_null(&mut reader)?);
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
