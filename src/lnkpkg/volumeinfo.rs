use byteorder::{ReadBytesExt, LittleEndian, ByteOrder};
use serde::{ser};
use lnkpkg::errors::{LnkError};
use lnkpkg::utils;
use std::mem::transmute;
use std::fmt;
use std::fmt::{Display,Debug};
use std::io::SeekFrom;
use std::io::Read;
use std::io::Seek;

pub struct VolumeSerialNumber(pub u32);
impl VolumeSerialNumber{
    pub fn to_string(&self)->String{
        let buffer: [u8; 4] = unsafe {
            transmute(self.0.to_le())
        };

        format!(
            "{:04X}-{:04X}",
            LittleEndian::read_u16(&buffer[2..4]),
            LittleEndian::read_u16(&buffer[0..2])
        )
    }
}
impl Display for VolumeSerialNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.to_string())
    }
}
impl Debug for VolumeSerialNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.to_string())
    }
}
impl ser::Serialize for VolumeSerialNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_str(
            &self.to_string()
        )
    }
}

pub struct DriveType(pub u32);
impl DriveType{
    pub fn to_string(&self)->String{
        match self.0 {
            0 => "DRIVE_UNKNOWN".to_string(),
            1 => "DRIVE_NO_ROOT_DIR".to_string(),
            2 => "DRIVE_REMOVABLE".to_string(),
            3 => "DRIVE_FIXED".to_string(),
            4 => "DRIVE_REMOTE".to_string(),
            5 => "DRIVE_CDROM".to_string(),
            6 => "DRIVE_RAMDISK".to_string(),
            _ => format!("UNKOWN_VALUE_0x{:08X}",self.0)
        }
    }
}
impl Display for DriveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.to_string())
    }
}
impl Debug for DriveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.to_string())
    }
}
impl ser::Serialize for DriveType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_str(
            &self.to_string()
        )
    }
}

#[derive(Serialize,Debug)]
pub struct VolumeInfo {
    #[serde(skip_serializing)]
    pub vi_size: u32,
    pub drive_type: DriveType,
    pub serial_number: VolumeSerialNumber,
    #[serde(skip_serializing)]
    pub offset_vol_label: u32,
    #[serde(skip_serializing)]
    pub offset_vol_label_unicode: Option<u32>,
    pub volume_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_label_unicode: Option<String>
}

impl VolumeInfo {
    pub fn new<Rs: Read+Seek>(mut reader: Rs) -> Result<VolumeInfo,LnkError> {
        // Get Current Pos
        let _offset = reader.seek(
            SeekFrom::Current(0)
        )?;

        let vi_size = reader.read_u32::<LittleEndian>()?;
        let drive_type = DriveType(reader.read_u32::<LittleEndian>()?);
        let serial_number = VolumeSerialNumber(reader.read_u32::<LittleEndian>()?);
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
