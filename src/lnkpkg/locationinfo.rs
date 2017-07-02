use encoding::all::UTF_16LE;
use encoding::{Encoding, DecoderTrap};
use byteorder::{ReadBytesExt, LittleEndian};
use lnkpkg::errors::{LnkError};
use lnkpkg::utils;
use lnkpkg::volumeinfo::{VolumeInfo};
use lnkpkg::netshareinfo::{NetworkShareInfo};
use std::io::SeekFrom;
use std::io::Read;
use std::io::Seek;

#[derive(Debug)]
pub struct LocationInfo {
    pub info_size: u32,
    pub header_size: u32,
    pub flags: u32,
    pub offset_vol_info: u32,
    pub offset_loc_path: u32,
    pub offset_net_share: u32,
    pub offset_common_path: u32,
    pub volume_info: VolumeInfo,
    pub netshare_info: NetworkShareInfo
}
impl LocationInfo {
    pub fn new<Rs: Read + Seek>(mut reader: Rs) -> Result<LocationInfo,LnkError> {
        let _offset = reader.seek(
            SeekFrom::Current(0)
        )?;

        let info_size = reader.read_u32::<LittleEndian>()?;
        let header_size = reader.read_u32::<LittleEndian>()?;
        let flags = reader.read_u32::<LittleEndian>()?;
        let offset_vol_info = reader.read_u32::<LittleEndian>()?;
        let offset_loc_path = reader.read_u32::<LittleEndian>()?;
        let offset_net_share = reader.read_u32::<LittleEndian>()?;
        let offset_common_path = reader.read_u32::<LittleEndian>()?;

        // read volume_info
        reader.seek(
            SeekFrom::Start(_offset + offset_vol_info as u64)
        )?;
        let volume_info = VolumeInfo::new(&mut reader)?;

        // read netshare info
        reader.seek(
            SeekFrom::Start(_offset + offset_net_share as u64)
        )?;
        let netshare_info = NetworkShareInfo::new(&mut reader)?;

        // seek to end of location info
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
                netshare_info: netshare_info
            }
        )
    }
}
