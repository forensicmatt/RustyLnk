use byteorder::{ReadBytesExt, LittleEndian};
use rwinstructs::timestamp::{WinTimestamp};
use rshellitems::shellitem::{ShellItem};
use lnkpkg::locationinfo::{LocationInfo};
use lnkpkg::flags::{DataFlags,FileFlags};
use lnkpkg::flags;
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

    pub fn get_data_flags(&self) -> DataFlags {
        self.data_flags
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
pub struct DataStrings {
    pub description: Option<String>,
    pub relative_path: Option<String>,
    pub working_directory: Option<String>,
    pub command_line_args: Option<String>,
    pub icon_location: Option<String>
}
impl DataStrings {
    pub fn new<R: Read>(mut reader: R, data_flags: DataFlags) -> Result<DataStrings,LnkError> {
        let unicode_flag = data_flags.contains(flags::IS_UNICODE);

        let mut description = None;
        if data_flags.contains(flags::HAS_NAME) {
            if unicode_flag {
                description = Some(utils::read_string_utf16(&mut reader)?);
            } else {
                description = Some(utils::read_string_utf8(&mut reader)?);
            }
        }

        let mut relative_path = None;
        if data_flags.contains(flags::HAS_RELATIVE_PATH) {
            if unicode_flag {
                relative_path = Some(utils::read_string_utf16(&mut reader)?);
            } else {
                relative_path = Some(utils::read_string_utf8(&mut reader)?);
            }
        }

        let mut working_directory = None;
        if data_flags.contains(flags::HAS_WORKING_DIR) {
            if unicode_flag {
                working_directory = Some(utils::read_string_utf16(&mut reader)?);
            } else {
                working_directory = Some(utils::read_string_utf8(&mut reader)?);
            }
        }

        let mut command_line_args = None;
        if data_flags.contains(flags::HAS_ARGUMENTS) {
            if unicode_flag {
                command_line_args = Some(utils::read_string_utf16(&mut reader)?);
            } else {
                command_line_args = Some(utils::read_string_utf8(&mut reader)?);
            }
        }

        let mut icon_location = None;
        if data_flags.contains(flags::HAS_ICON_LOCATION) {
            if unicode_flag {
                icon_location = Some(utils::read_string_utf16(&mut reader)?);
            } else {
                icon_location = Some(utils::read_string_utf8(&mut reader)?);
            }
        }

        Ok(
            DataStrings {
                description: description,
                relative_path: relative_path,
                working_directory: working_directory,
                command_line_args: command_line_args,
                icon_location: icon_location
            }
        )
    }
}

#[derive(Debug)]
pub struct Lnk {
    pub header: ShellLinkHeader,
    pub target_list: Option<TargetIdList>,
    pub location_info: Option<LocationInfo>,
    pub data_strings: DataStrings
}

impl Lnk {
    pub fn new<Rs: Read+Seek>(mut reader: Rs) -> Result<Lnk,LnkError> {
        let header = ShellLinkHeader::new(&mut reader)?;
        let header_flags = header.get_data_flags();

        let mut target_list = None;
        if header_flags.contains(flags::HAS_TARGET_ID_LIST) {
            target_list = Some(TargetIdList::new(&mut reader)?);
        }

        let mut location_offset = reader.seek(
            SeekFrom::Current(0)
        )?;

        let mut location_info = None;
        if header_flags.contains(flags::HAS_LINK_INFO) {
            location_info = Some(LocationInfo::new(&mut reader)?);
        }

        println!("offset: {}",reader.seek(
            SeekFrom::Current(0)
        )?);
        let data_strings = DataStrings::new(
            &mut reader,
            header_flags
        )?;

        Ok(
            Lnk {
                header: header,
                target_list: target_list,
                location_info: location_info,
                data_strings: data_strings
            }
        )
    }
}
