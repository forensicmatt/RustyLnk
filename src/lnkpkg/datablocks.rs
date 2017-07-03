use byteorder::{ReadBytesExt, LittleEndian};
use lnkpkg::errors::{LnkError};
use lnkpkg::utils;
use rwinstructs::guid::{Guid};
use std::io::SeekFrom;
use std::io::Read;
use std::io::Seek;

#[derive(Debug)]
pub struct SpecialFolder {
    special_folder_id: u32,
    first_child_segment_offset: u32
}
impl SpecialFolder {
    pub fn new<R: Read>(mut reader: R) -> Result<SpecialFolder,LnkError> {
        let special_folder_id = reader.read_u32::<LittleEndian>()?;
        let first_child_segment_offset = reader.read_u32::<LittleEndian>()?;

        Ok (
            SpecialFolder {
                special_folder_id: special_folder_id,
                first_child_segment_offset: first_child_segment_offset
            }
        )
    }
}

#[derive(Debug)]
pub struct Codepage {
    codepage: u32
}
impl Codepage {
    pub fn new<R: Read>(mut reader: R) -> Result<Codepage,LnkError> {
        let codepage = reader.read_u32::<LittleEndian>()?;

        Ok (
            Codepage {
                codepage: codepage
            }
        )
    }
}

#[derive(Debug)]
pub struct DistributedTracker {
    size: u32,
    version: u32,
    machine_id: String, // 16 bytes (ascii)
    droid_volume: Guid,
    droid_file: Guid,
    birth_droid_volume: Guid,
    birth_droid_file: Guid
}
impl DistributedTracker {
    pub fn new<R: Read>(mut reader: R) -> Result<DistributedTracker,LnkError> {
        let size = reader.read_u32::<LittleEndian>()?;
        let version = reader.read_u32::<LittleEndian>()?;

        let mut buff_machine_id = vec![0; 16];
        reader.read_exact(buff_machine_id.as_mut_slice())?;
        let machine_id = utils::read_string_u8_till_null(
            buff_machine_id.as_slice()
        )?;

        let droid_volume = Guid::new(&mut reader)?;
        let droid_file = Guid::new(&mut reader)?;
        let birth_droid_volume = Guid::new(&mut reader)?;
        let birth_droid_file = Guid::new(&mut reader)?;

        Ok (
            DistributedTracker {
                size: size,
                version: version,
                machine_id: machine_id,
                droid_volume: droid_volume,
                droid_file: droid_file,
                birth_droid_volume: birth_droid_volume,
                birth_droid_file: birth_droid_file
            }
        )
    }
}

#[derive(Debug)]
pub struct EnvironmentVars {
    location: String, //size is 260 bytes (Unicode string terminated by an end-of-string character)
    location_unicode: String //size is 520 bytes (Unicode string terminated by an end-of-string character)
}
impl EnvironmentVars {
    pub fn new<R: Read>(mut reader: R) -> Result<EnvironmentVars,LnkError> {
        let mut buff_location = vec![0; 260];
        reader.read_exact(buff_location.as_mut_slice())?;
        let location = utils::read_string_u8_till_null(
            buff_location.as_slice()
        )?;

        let mut buff_location_unicode = vec![0; 520];
        reader.read_exact(buff_location_unicode.as_mut_slice())?;
        let location_unicode = utils::read_string_u16_till_null(
            buff_location_unicode.as_slice()
        )?;

        Ok(
            EnvironmentVars {
                location: location,
                location_unicode: location_unicode
            }
        )
    }
}

#[derive(Debug)]
pub struct ConsoleProperties {
    color_flags: u16,
    popup_attribs: u16,
    screen_width_buff_size: u16,
    screen_height_buff_size: u16,
    window_width: u16,
    window_height: u16,
    window_x: u16,
    window_y: u16,
    unknown1: u32,
    unknown2: u32,
    font_size: u32,
    font_family_value: u32,
    font_weight: u32,
    face_name: String, //size is 64 bytes (Unicode string terminated by an end-of-string character)
    cursor_size: u32,
    full_screen: u32,
    insert_mode: u32,
    auto_positioning: u32,
    history_buff_size: u32,
    history_buff_count: u32,
    duplicates_allowed: u32,
    color_table: utils::ByteArray
}
impl ConsoleProperties {
    pub fn new<R: Read>(mut reader: R) -> Result<ConsoleProperties,LnkError> {
        let color_flags = reader.read_u16::<LittleEndian>()?;
        let popup_attribs = reader.read_u16::<LittleEndian>()?;
        let screen_width_buff_size = reader.read_u16::<LittleEndian>()?;
        let screen_height_buff_size = reader.read_u16::<LittleEndian>()?;
        let window_width = reader.read_u16::<LittleEndian>()?;
        let window_height = reader.read_u16::<LittleEndian>()?;
        let window_x = reader.read_u16::<LittleEndian>()?;
        let window_y = reader.read_u16::<LittleEndian>()?;
        let unknown1 = reader.read_u32::<LittleEndian>()?;
        let unknown2 = reader.read_u32::<LittleEndian>()?;
        let font_size = reader.read_u32::<LittleEndian>()?;
        let font_family_value = reader.read_u32::<LittleEndian>()?;
        let font_weight = reader.read_u32::<LittleEndian>()?;

        // Read string
        let mut face_buff = vec![0; 64];
        reader.read_exact(face_buff.as_mut_slice())?;
        let face_name = utils::read_string_u16_till_null(face_buff.as_slice())?;

        let cursor_size = reader.read_u32::<LittleEndian>()?;
        let full_screen = reader.read_u32::<LittleEndian>()?;
        let insert_mode = reader.read_u32::<LittleEndian>()?;
        let auto_positioning = reader.read_u32::<LittleEndian>()?;
        let history_buff_size = reader.read_u32::<LittleEndian>()?;
        let history_buff_count = reader.read_u32::<LittleEndian>()?;
        let duplicates_allowed = reader.read_u32::<LittleEndian>()?;

        let mut color_buff = vec![0; 64];
        reader.read_exact(color_buff.as_mut_slice())?;
        let color_table = utils::ByteArray(color_buff);

        Ok (
            ConsoleProperties {
                color_flags: color_flags,
                popup_attribs: popup_attribs,
                screen_width_buff_size: screen_width_buff_size,
                screen_height_buff_size: screen_height_buff_size,
                window_width: window_width,
                window_height: window_height,
                window_x: window_x,
                window_y: window_y,
                unknown1: unknown1,
                unknown2: unknown2,
                font_size: font_size,
                font_family_value: font_family_value,
                font_weight: font_weight,
                face_name: face_name,
                cursor_size: cursor_size,
                full_screen: full_screen,
                insert_mode: insert_mode,
                auto_positioning: auto_positioning,
                history_buff_size: history_buff_size,
                history_buff_count: history_buff_count,
                duplicates_allowed: duplicates_allowed,
                color_table: color_table
            }
        )
    }
}

#[derive(Debug)]
pub struct ExtraDataBlocks {
    pub console_properties: Option<ConsoleProperties>,
    pub environment_vars: Option<EnvironmentVars>,
    pub distributed_tracker: Option<DistributedTracker>,
    pub codepage: Option<Codepage>,
    pub special_folder: Option<SpecialFolder>
}
impl ExtraDataBlocks {
    pub fn new<Rs: Read+Seek>(mut reader: Rs) -> Result<ExtraDataBlocks,LnkError> {
        let mut console_properties = None;
        let mut environment_vars = None;
        let mut distributed_tracker = None;
        let mut codepage = None;
        let mut special_folder = None;

        let mut _offset = reader.seek(
            SeekFrom::Current(0)
        )?;

        loop {
            let size = reader.read_u32::<LittleEndian>()?;
            if size != 0 {
                let signature = reader.read_u32::<LittleEndian>()?;
                match signature {
                    0xa0000001 => {
                        environment_vars = Some(EnvironmentVars::new(&mut reader)?);
                    },
                    0xa0000002 => {
                        console_properties = Some(ConsoleProperties::new(&mut reader)?);
                    },
                    0xa0000003 => {
                        distributed_tracker = Some(DistributedTracker::new(&mut reader)?);
                    },
                    0xa0000004 => {
                        codepage = Some(Codepage::new(&mut reader)?);
                    },
                    0xa0000005 => {
                        special_folder = Some(SpecialFolder::new(&mut reader)?);
                    },
                    _ => {
                        println!(
                            "Unhandled data block with signature {} at offset {}",
                            signature,
                            _offset
                        );
                    }
                }

                _offset += size as u64;

                reader.seek(
                    SeekFrom::Start(_offset)
                )?;
            } else {
                break
            }
        }

        Ok(
            ExtraDataBlocks {
                console_properties: console_properties,
                environment_vars: environment_vars,
                distributed_tracker: distributed_tracker,
                codepage: codepage,
                special_folder: special_folder
            }
        )
    }
}
