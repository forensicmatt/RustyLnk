use byteorder::{ReadBytesExt, LittleEndian};
use lnkpkg::errors::{LnkError};
use lnkpkg::utils;
use std::io::SeekFrom;
use std::io::Read;
use std::io::Seek;

#[derive(Serialize,Debug)]
pub struct NetworkShareInfo {
    pub size: u32,
    pub flags: u32,
    #[serde(skip_serializing)]
    pub offset_share_name: u32,
    #[serde(skip_serializing)]
    pub offset_device_name: u32,
    pub provider_type: u32,
    #[serde(skip_serializing)]
    pub offset_share_name_unicode: Option<u32>,
    #[serde(skip_serializing)]
    pub offset_device_name_unicode: Option<u32>,
    pub share_name: String,
    pub device_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_name_unicode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name_unicode: Option<String>
}
impl NetworkShareInfo {
    pub fn new<Rs: Read+Seek>(mut reader: Rs) -> Result<NetworkShareInfo,LnkError> {
        // Get Current Pos
        let cur_pos = reader.seek(
            SeekFrom::Current(0)
        )?;

        let size = reader.read_u32::<LittleEndian>()?;
        let flags = reader.read_u32::<LittleEndian>()?;
        let offset_share_name = reader.read_u32::<LittleEndian>()?;
        let offset_device_name = reader.read_u32::<LittleEndian>()?;
        let provider_type = reader.read_u32::<LittleEndian>()?;

        let mut offset_share_name_unicode = None;
        let mut offset_device_name_unicode = None;
        if offset_share_name > 20 {
            offset_share_name_unicode = Some(reader.read_u32::<LittleEndian>()?);
            offset_device_name_unicode = Some(reader.read_u32::<LittleEndian>()?);
        }

        reader.seek(
            SeekFrom::Start(cur_pos + offset_share_name as u64)
        )?;
        let share_name = utils::read_string_u8_till_null(&mut reader)?;

        reader.seek(
            SeekFrom::Start(cur_pos + offset_device_name as u64)
        )?;
        // Get device name
        let device_name = utils::read_string_u8_till_null(&mut reader)?;

        let mut share_name_unicode = None;
        if offset_share_name_unicode.is_some() {
            reader.seek(
                SeekFrom::Start(cur_pos + offset_share_name_unicode.unwrap() as u64)
            )?;
            share_name_unicode = Some(utils::read_string_u16_till_null(&mut reader)?);
        }

        let mut device_name_unicode = None;
        if offset_device_name_unicode.is_some() {
            reader.seek(
                SeekFrom::Start(cur_pos + offset_device_name_unicode.unwrap() as u64)
            )?;
            device_name_unicode = Some(utils::read_string_u16_till_null(&mut reader)?);
        }

        Ok(
            NetworkShareInfo {
                size: size,
                flags: flags,
                offset_share_name: offset_share_name,
                offset_device_name: offset_device_name,
                provider_type: provider_type,
                offset_share_name_unicode: offset_share_name_unicode,
                offset_device_name_unicode: offset_device_name_unicode,
                share_name: share_name,
                device_name: device_name,
                share_name_unicode: share_name_unicode,
                device_name_unicode: device_name_unicode
            }
        )
    }
}
