use encoding::all::UTF_16LE;
use encoding::{Encoding, DecoderTrap};
use byteorder::{ReadBytesExt, LittleEndian};
use rwinstructs::timestamp::{WinTimestamp};
use rshellitems::shellitem::{ShellItem};
use lnkpkg::flags::{DataFlags,FileFlags};
use lnkpkg::errors::{LnkError};
use lnkpkg::utils;
use std::io::SeekFrom;
use std::io::Read;
use std::io::Seek;

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

#[derive(Debug)]
pub struct TargetIdList {
    pub list_size: u16,
    pub shell_items: Vec<ShellItem>
}

impl TargetIdList{
    pub fn new<R: Read>(mut reader: R) -> Result<TargetIdList, LnkError> {
        let list_size = reader.read_u16::<LittleEndian>()?;
        let mut shell_items: Vec<ShellItem> = Vec::new();

        let mut total_read: u16 = 0;
        while total_read < list_size {
            let mut shell_item = ShellItem::new(&mut reader)?;
            let size = shell_item.get_size();
            total_read += size;

            if size == 0 {
                // Null shell item is terminator
                break
            }

            shell_items.push(shell_item);
        }

        Ok(
            TargetIdList {
                list_size: list_size,
                shell_items: shell_items
            }
        )
    }

    pub fn get_size(&self) -> u16 {
        self.list_size
    }
}

#[derive(Debug)]
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

    pub fn get_volume_info_offset(&self) -> u32 {
        self.offset_vol_info
    }
}

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
            let utf16_buffer = utils::get_u8_vec(&mut reader)?;
            let utf16_string = match UTF_16LE.decode(&utf16_buffer,DecoderTrap::Ignore) {
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

#[derive(Debug)]
pub struct Lnk {
    pub header: ShellLinkHeader,
    pub target_list: TargetIdList,
    pub location_info: LocationInfo,
    pub volume_info: VolumeInfo
}

impl Lnk {
    pub fn new<Rs: Read+Seek>(mut reader: Rs) -> Result<Lnk,LnkError> {
        let header = ShellLinkHeader::new(&mut reader)?;
        let target_list = TargetIdList::new(&mut reader)?;

        let location_offset = reader.seek(
            SeekFrom::Current(0)
        )?;

        let location_info = LocationInfo::new(&mut reader)?;

        // Seek to volume info
        reader.seek(
            SeekFrom::Start(
                location_offset + location_info.get_volume_info_offset() as u64
            )
        )?;
        let volume_info = VolumeInfo::new(&mut reader)?;

        Ok(
            Lnk {
                header: header,
                target_list: target_list,
                location_info: location_info,
                volume_info: volume_info
            }
        )
    }
}
