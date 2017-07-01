use byteorder::{ReadBytesExt, LittleEndian};
use rwinstructs::timestamp::{WinTimestamp};
use lnkpkg::flags::{DataFlags,FileFlags};
use lnkpkg::errors::{LnkError};
use std::io::Read;

#[derive(Clone, Debug)]
pub struct Guid(pub [u8;16]);

#[derive(Debug)]
// 76 bytes long
pub struct ShellLinkHeader {
    pub header_size: u32,
    pub guid: Guid,
    pub data_flags: DataFlags,
    pub file_flags: FileFlags,
    pub created: WinTimestamp,
    pub accessed: WinTimestamp,
    pub modified: WinTimestamp,
    pub file_size: u32,
    pub icon_offset: i32,
    pub window_flag: u32,
    pub hot_key: u16,
    pub unknown1: u16,
    pub unknown2: u32,
    pub unknown3: u32
}

impl ShellLinkHeader {
    pub fn new<R: Read>(mut reader: R) -> Result<ShellLinkHeader,LnkError> {
        let header_size = reader.read_u32::<LittleEndian>()?;
        let mut guid = Guid([0; 16]);
        reader.read_exact(&mut guid.0)?;

        let data_flags = DataFlags::from_bits_truncate(
            reader.read_u32::<LittleEndian>()?
        );

        let file_flags = FileFlags::from_bits_truncate(
            reader.read_u32::<LittleEndian>()?
        );
        let created = WinTimestamp(reader.read_u64::<LittleEndian>()?);
        let accessed = WinTimestamp(reader.read_u64::<LittleEndian>()?);
        let modified = WinTimestamp(reader.read_u64::<LittleEndian>()?);
        let file_size = reader.read_u32::<LittleEndian>()?;
        let icon_offset = reader.read_i32::<LittleEndian>()?;
        let window_flag = reader.read_u32::<LittleEndian>()?;
        let hot_key = reader.read_u16::<LittleEndian>()?;
        let unknown1 = reader.read_u16::<LittleEndian>()?;
        let unknown2 = reader.read_u32::<LittleEndian>()?;
        let unknown3 = reader.read_u32::<LittleEndian>()?;

        Ok(
            ShellLinkHeader {
                header_size: header_size,
                guid: guid,
                data_flags: data_flags,
                file_flags: file_flags,
                created: created,
                accessed: accessed,
                modified: modified,
                file_size: file_size,
                icon_offset: icon_offset,
                window_flag: window_flag,
                hot_key: hot_key,
                unknown1: unknown1,
                unknown2: unknown2,
                unknown3: unknown3
            }
        )
    }
}

pub struct TargetIdList {
    pub list_size: u16
}

impl TargetIdList{
    pub fn new<R: Read>(mut reader: R) -> Result<TargetIdList, LnkError> {
        let list_size = reader.read_u16::<LittleEndian>()?;

        Ok(
            TargetIdList {
                list_size: list_size
            }
        )
    }
}

pub struct LocationInfo {
    pub info_size: u32,
    pub header_size: u32,
    pub flags: u32,
    pub offset_vol_info: u32,
    pub offset_loc_path: u32,
    pub offset_net_share: u32,
    pub offset_common_path: u32
}
impl LocationInfo {
    pub fn new<R: Read>(mut reader: R) -> Result<LocationInfo,LnkError> {
        let info_size = reader.read_u32::<LittleEndian>()?;
        let header_size = reader.read_u32::<LittleEndian>()?;
        let flags = reader.read_u32::<LittleEndian>()?;
        let offset_vol_info = reader.read_u32::<LittleEndian>()?;
        let offset_loc_path = reader.read_u32::<LittleEndian>()?;
        let offset_net_share = reader.read_u32::<LittleEndian>()?;
        let offset_common_path = reader.read_u32::<LittleEndian>()?;

        Ok (
            LocationInfo {
                info_size: info_size,
                header_size: header_size,
                flags: flags,
                offset_vol_info: offset_vol_info,
                offset_loc_path: offset_loc_path,
                offset_net_share: offset_net_share,
                offset_common_path: offset_common_path
            }
        )
    }
}

pub struct Lnk {
    pub header: ShellLinkHeader,
    pub target_list: TargetIdList,
    pub location_info: LocationInfo
}

impl Lnk {
    pub fn new<R: Read>(mut reader: R) -> Result<Lnk,LnkError> {
        let header = ShellLinkHeader::new(reader)?;

        Ok(
            Lnk {
                header: header
            }
        )
    }
}
