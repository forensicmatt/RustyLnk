use byteorder::{ReadBytesExt, LittleEndian};
use rwinstructs::timestamp::{WinTimestamp};
use rwinstructs::guid::{Guid};
use rshellitems::shelllist::{ShellList};
use lnkpkg::locationinfo::{LocationInfo};
use lnkpkg::datablocks::{ExtraDataBlocks};
use lnkpkg::flags::{DataFlags, FileFlags};
use lnkpkg::errors::{LnkError};
use lnkpkg::utils;
use std::io::Read;
use std::io::{Seek,SeekFrom};

#[derive(Serialize,Debug)]
// 76 bytes long,
pub struct LnkHeader {
    #[serde(skip_serializing)]
    _offset: u64,
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
    #[serde(skip_serializing)]
    pub unknown1: u16,
    #[serde(skip_serializing)]
    pub unknown2: u32,
    #[serde(skip_serializing)]
    pub unknown3: u32
}
impl LnkHeader {
    pub fn new<Rs: Read+Seek>(mut reader: Rs) -> Result<LnkHeader,LnkError> {
        let _offset = reader.seek(SeekFrom::Current(0))?;
        let header_size = reader.read_u32::<LittleEndian>()?;
        if header_size != 76 {
            return Err(
                LnkError::validation_error(
                    format!("Header size unhandled. Got {}, expected 76.",header_size)
                )
            );
        }

        let mut guid = Guid([0; 16]);
        reader.read_exact(&mut guid.0)?;

        // Do a GUID check
        if guid.to_string() != "00021401-0000-0000-C000-000000000046" {
            return Err(
                LnkError::validation_error(
                    format!(
                        "GUID {} is not the known LNK GUID of 00021401-0000-0000-C000-000000000046",
                        guid.to_string()
                    )
                )
            );
        }

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
            LnkHeader {
                _offset: _offset,
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

#[derive(Serialize,Debug)]
pub struct TargetIdList {
    #[serde(skip_serializing)]
    _offset: u64,
    pub list_size: u16,
    pub shell_items: ShellList
}

impl TargetIdList{
    pub fn new<Rs: Read+Seek>(mut reader: Rs) -> Result<TargetIdList, LnkError> {
        let _offset = reader.seek(SeekFrom::Current(0))?;
        let list_size = reader.read_u16::<LittleEndian>()?;

        let shell_items = ShellList::new(
            &mut reader
        )?;

        Ok(
            TargetIdList {
                _offset: _offset,
                list_size: list_size,
                shell_items: shell_items
            }
        )
    }

    pub fn get_size(&self) -> u16 {
        self.list_size
    }
}

#[derive(Serialize,Debug)]
pub struct DataStrings {
    pub description: Option<String>,
    pub relative_path: Option<String>,
    pub working_directory: Option<String>,
    pub command_line_args: Option<String>,
    pub icon_location: Option<String>
}
impl DataStrings {
    pub fn new<R: Read>(mut reader: R, data_flags: DataFlags) -> Result<DataStrings,LnkError> {
        let unicode_flag = data_flags.contains(DataFlags::IS_UNICODE);

        let mut description = None;
        if data_flags.contains(DataFlags::HAS_NAME) {
            if unicode_flag {
                description = Some(utils::read_string_u16_w_size(&mut reader)?);
            } else {
                description = Some(utils::read_string_u8_w_size(&mut reader)?);
            }
        }

        let mut relative_path = None;
        if data_flags.contains(DataFlags::HAS_RELATIVE_PATH) {
            if unicode_flag {
                relative_path = Some(utils::read_string_u16_w_size(&mut reader)?);
            } else {
                relative_path = Some(utils::read_string_u8_w_size(&mut reader)?);
            }
        }

        let mut working_directory = None;
        if data_flags.contains(DataFlags::HAS_WORKING_DIR) {
            if unicode_flag {
                working_directory = Some(utils::read_string_u16_w_size(&mut reader)?);
            } else {
                working_directory = Some(utils::read_string_u8_w_size(&mut reader)?);
            }
        }

        let mut command_line_args = None;
        if data_flags.contains(DataFlags::HAS_ARGUMENTS) {
            if unicode_flag {
                command_line_args = Some(utils::read_string_u16_w_size(&mut reader)?);
            } else {
                command_line_args = Some(utils::read_string_u8_w_size(&mut reader)?);
            }
        }

        let mut icon_location = None;
        if data_flags.contains(DataFlags::HAS_ICON_LOCATION) {
            if unicode_flag {
                icon_location = Some(utils::read_string_u16_w_size(&mut reader)?);
            } else {
                icon_location = Some(utils::read_string_u8_w_size(&mut reader)?);
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

#[derive(Serialize,Debug)]
pub struct Lnk {
    pub header: LnkHeader,
    pub target_list: Option<TargetIdList>,
    pub location_info: Option<LocationInfo>,
    pub data_strings: DataStrings,
    pub extra_data: ExtraDataBlocks
}

impl Lnk {
    pub fn new<Rs: Read+Seek>(mut reader: Rs) -> Result<Lnk,LnkError> {
        // Check reader size
        let reader_size = utils::get_reader_size(
            &mut reader
        )?;
        if reader_size < 76 {
            return Err(
                LnkError::validation_error(
                    format!("File size of {} is less than 76.",reader_size)
                )
            );
        }

        let header = LnkHeader::new(
            &mut reader
        )?;
        let header_flags = header.get_data_flags();

        let mut target_list = None;
        if header_flags.contains(DataFlags::HAS_TARGET_ID_LIST) {
            target_list = Some(
                TargetIdList::new(&mut reader)?
            );
        }

        let mut location_info = None;
        if header_flags.contains(DataFlags::HAS_LINK_INFO) {
            location_info = Some(
                LocationInfo::new(&mut reader)?
            );
        }

        let data_strings = DataStrings::new(
            &mut reader,
            header_flags
        )?;

        let extra_data = ExtraDataBlocks::new(
            &mut reader
        )?;

        Ok(
            Lnk {
                header: header,
                target_list: target_list,
                location_info: location_info,
                data_strings: data_strings,
                extra_data: extra_data
            }
        )
    }
}
