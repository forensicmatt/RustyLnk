use std::fmt;
use serde::{ser};

pub static mut FLAGS_AS_INT: bool = false;

bitflags! {
    pub struct DataFlags: u32 {
        const HAS_TARGET_ID_LIST                    = 0x00000001;
        const HAS_LINK_INFO                         = 0x00000002;
        const HAS_NAME                              = 0x00000004;
        const HAS_RELATIVE_PATH                     = 0x00000008;
        const HAS_WORKING_DIR                       = 0x00000010;
        const HAS_ARGUMENTS                         = 0x00000020;
        const HAS_ICON_LOCATION                     = 0x00000040;
        const IS_UNICODE                            = 0x00000080;
        const FORCE_NO_LINK_INFO                    = 0x00000100;
        const HAS_EXP_STRING                        = 0x00000200;
        const RUN_IN_SEPARATE_PROCESS               = 0x00000400;
        const UNKNOWN_1                             = 0x00000800;
        const HAS_DARWIN_ID                         = 0x00001000;
        const RUN_AS_USER                           = 0x00002000;
        const HAS_EXP_ICON                          = 0x00004000;
        const NO_PIDL_ALIAS                         = 0x00008000;
        const UNKNOWN_2                             = 0x00010000;
        const RUN_WITH_SHIM_LAYER                   = 0x00020000;
        const FORCE_NO_LINK_TRACK                   = 0x00040000;
        const ENABLE_TARGET_METADATA                = 0x00080000;
        const DISABLE_LINK_PATH_TRACKING            = 0x00100000;
        const DISABLE_KNOWN_FOLDER_TRACKING         = 0x00200000;
        const DISABLE_KNOWN_FOLDER_ALIAS            = 0x00400000;
        const ALLOW_LINK_TO_LINK                    = 0x00800000;
        const UNALIAS_ON_SAVE                       = 0x01000000;
        const PREFER_ENVIRONMENT_PATH               = 0x02000000;
        const KEEP_LOCAL_ID_LIST_FOR_UNC_TARGET     = 0x04000000;
    }
}

bitflags! {
    pub struct FileFlags: u32 {
        const FILE_ATTRIBUTE_READONLY               = 0x00000001;
        const FILE_ATTRIBUTE_HIDDEN                 = 0x00000002;
        const FILE_ATTRIBUTE_SYSTEM                 = 0x00000004;
        const RESERVED                              = 0x00000008;
        const FILE_ATTRIBUTE_DIRECTORY              = 0x00000010;
        const FILE_ATTRIBUTE_ARCHIVE                = 0x00000020;
        const FILE_ATTRIBUTE_DEVICE                 = 0x00000040;
        const FILE_ATTRIBUTE_NORMAL                 = 0x00000080;
        const FILE_ATTRIBUTE_TEMPORARY              = 0x00000100;
        const FILE_ATTRIBUTE_SPARSE_FILE            = 0x00000200;
        const FILE_ATTRIBUTE_REPARSE_POINT          = 0x00000400;
        const FILE_ATTRIBUTE_COMPRESSED             = 0x00000800;
        const FILE_ATTRIBUTE_OFFLINE                = 0x00001000;
        const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED    = 0x00002000;
        const FILE_ATTRIBUTE_ENCRYPTED              = 0x00004000;
        const FILE_ATTRIBUTE_UNKNOWN1               = 0x00008000;
        const FILE_ATTRIBUTE_VIRTUAL                = 0x00010000;
    }
}

impl fmt::Display for DataFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.bits())
    }
}
impl ser::Serialize for DataFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        if unsafe{FLAGS_AS_INT} {
            serializer.serialize_u32(self.bits())
        } else {
            serializer.serialize_str(&format!("{:?}", self))
        }
    }
}

impl fmt::Display for FileFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.bits())
    }
}
impl ser::Serialize for FileFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        if unsafe{FLAGS_AS_INT} {
            serializer.serialize_u32(self.bits())
        } else {
            serializer.serialize_str(&format!("{:?}", self))
        }
    }
}
