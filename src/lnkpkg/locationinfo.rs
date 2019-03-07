use byteorder::{ReadBytesExt, LittleEndian};
use lnkpkg::errors::{LnkError};
use lnkpkg::utils;
use lnkpkg::volumeinfo::{VolumeInfo};
use lnkpkg::netshareinfo::{NetworkShareInfo};
use std::io::SeekFrom;
use std::io::Read;
use std::io::Seek;
use serde::{ser};
use std::fmt;

bitflags! {
    pub struct LocationInfoFalgs: u32 {
        const VolumeIDAndLocalBasePath                    = 0x00000001;
        const CommonNetworkRelativeLinkAndPathSuffix      = 0x00000002;
    }
}
impl fmt::Display for LocationInfoFalgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.bits())
    }
}
impl ser::Serialize for LocationInfoFalgs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_str(&format!("{:?}", self))
    }
}

#[derive(Serialize,Debug)]
pub struct LocationInfo {
    #[serde(skip_serializing)]
    info_size: u32,
    #[serde(skip_serializing)]
    header_size: u32,
    flags: LocationInfoFalgs,
    #[serde(skip_serializing)]
    offset_vol_info: u32,
    #[serde(skip_serializing)]
    offset_loc_path: u32,
    #[serde(skip_serializing)]
    offset_net_share: u32,
    #[serde(skip_serializing)]
    offset_common_path: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_info: Option<VolumeInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    netshare_info: Option<NetworkShareInfo>
}
impl LocationInfo {
    pub fn new<Rs: Read + Seek>(mut reader: Rs) -> Result<LocationInfo,LnkError> {
        let _offset = reader.seek(
            SeekFrom::Current(0)
        )?;

        let info_size = reader.read_u32::<LittleEndian>()?;
        let header_size = reader.read_u32::<LittleEndian>()?;
        let flags = LocationInfoFalgs::from_bits_truncate(
            reader.read_u32::<LittleEndian>()?
        );
        let offset_vol_info = reader.read_u32::<LittleEndian>()?;
        let offset_loc_path = reader.read_u32::<LittleEndian>()?;
        let offset_net_share = reader.read_u32::<LittleEndian>()?;
        let offset_common_path = reader.read_u32::<LittleEndian>()?;

        let mut volume_info = None;
        let mut local_path = None;
        let mut common_path = None;
        let mut netshare_info = None;

        if flags.contains(LocationInfoFalgs::VolumeIDAndLocalBasePath) {
            // read volume_info
            reader.seek(
                SeekFrom::Start(_offset + offset_vol_info as u64)
            )?;
            volume_info = Some(
                VolumeInfo::new(
                    &mut reader
                )?
            );

            reader.seek(
                SeekFrom::Start(_offset + offset_loc_path as u64)
            )?;
            local_path = Some(
                utils::read_string_u8_till_null(&mut reader)?
            );

            reader.seek(
                SeekFrom::Start(_offset + offset_common_path as u64)
            )?;
            common_path = Some(
                utils::read_string_u8_till_null(&mut reader)?
            );
        }
        if flags.contains(LocationInfoFalgs::CommonNetworkRelativeLinkAndPathSuffix) {
            // read netshare info
            reader.seek(
                SeekFrom::Start(_offset + offset_net_share as u64)
            )?;
            netshare_info = Some(
                NetworkShareInfo::new(&mut reader)?
            );
        }
        
        reader.seek(
            SeekFrom::Start(_offset + info_size as u64)
        )?;

        Ok (
            LocationInfo {
                info_size: info_size,
                header_size: header_size,
                flags: flags,
                offset_vol_info: offset_vol_info,
                offset_loc_path: offset_loc_path,
                offset_net_share: offset_net_share,
                offset_common_path: offset_common_path,
                volume_info: volume_info,
                local_path: local_path,
                netshare_info: netshare_info,
                common_path: common_path
            }
        )
    }
}
