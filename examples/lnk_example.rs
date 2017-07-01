use std::io::Cursor;
extern crate rustylnk;
use rustylnk::lnkpkg::lnk;

fn lnk_test() {
    let buffer: &[u8] = &[
        0x4C,0x00,0x00,0x00,0x01,0x14,0x02,0x00,0x00,0x00,0x00,0x00,0xC0,0x00,0x00,0x00,
        0x00,0x00,0x00,0x46,0x9B,0x00,0x00,0x00,0x20,0x00,0x00,0x00,0x14,0xD5,0xA5,0x67,
        0x78,0xFD,0xCC,0x01,0x7D,0x84,0x6B,0xDE,0xAF,0x03,0xCD,0x01,0x40,0x52,0xED,0x67,
        0x78,0xFD,0xCC,0x01,0xFA,0x0A,0x01,0x00,0x00,0x00,0x00,0x00,0x01,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x36,0x01,0x14,0x00,
        0x1F,0x48,0xBA,0x8F,0x0D,0x45,0x25,0xAD,0xD0,0x11,0x98,0xA8,0x08,0x00,0x36,0x1B,
        0x11,0x03,0x4C,0x00,0x31,0x00,0x00,0x00,0x00,0x00,0x6C,0x40,0x62,0xAB,0x10,0x00,
        0x41,0x4C,0x4C,0x4F,0x59,0x52,0x7E,0x31,0x00,0x00,0x34,0x00,0x03,0x00,0x04,0x00,
        0xEF,0xBE,0x69,0x40,0x3A,0x8D,0x70,0x40,0x6B,0xA0,0x14,0x00,0x00,0x00,0x41,0x00,
        0x6C,0x00,0x6C,0x00,0x6F,0x00,0x79,0x00,0x20,0x00,0x52,0x00,0x65,0x00,0x73,0x00,
        0x65,0x00,0x61,0x00,0x72,0x00,0x63,0x00,0x68,0x00,0x00,0x00,0x18,0x00,0x54,0x00,
        0x31,0x00,0x00,0x00,0x00,0x00,0x70,0x40,0x72,0xA0,0x10,0x00,0x44,0x45,0x54,0x41,
        0x49,0x4C,0x7E,0x31,0x00,0x00,0x3C,0x00,0x03,0x00,0x04,0x00,0xEF,0xBE,0x68,0x40,
        0x5A,0xB1,0x70,0x40,0x72,0xA0,0x14,0x00,0x00,0x00,0x44,0x00,0x65,0x00,0x74,0x00,
        0x61,0x00,0x69,0x00,0x6C,0x00,0x65,0x00,0x64,0x00,0x20,0x00,0x44,0x00,0x6F,0x00,
        0x63,0x00,0x75,0x00,0x6D,0x00,0x65,0x00,0x6E,0x00,0x74,0x00,0x73,0x00,0x00,0x00,
        0x18,0x00,0x80,0x00,0x32,0x00,0xFA,0x0A,0x01,0x00,0x68,0x40,0x6E,0xB1,0x20,0x00,
        0x43,0x4F,0x50,0x59,0x4F,0x46,0x7E,0x31,0x2E,0x58,0x4C,0x53,0x00,0x00,0x64,0x00,
        0x03,0x00,0x04,0x00,0xEF,0xBE,0x68,0x40,0x6E,0xB1,0x70,0x40,0x70,0xA0,0x14,0x00,
        0x00,0x00,0x43,0x00,0x6F,0x00,0x70,0x00,0x79,0x00,0x20,0x00,0x6F,0x00,0x66,0x00,
        0x20,0x00,0x4D,0x00,0x65,0x00,0x74,0x00,0x61,0x00,0x6C,0x00,0x20,0x00,0x41,0x00,
        0x6C,0x00,0x6C,0x00,0x6F,0x00,0x79,0x00,0x20,0x00,0x4C,0x00,0x69,0x00,0x73,0x00,
        0x74,0x00,0x20,0x00,0x52,0x00,0x65,0x00,0x73,0x00,0x65,0x00,0x61,0x00,0x72,0x00,
        0x63,0x00,0x68,0x00,0x2E,0x00,0x78,0x00,0x6C,0x00,0x73,0x00,0x78,0x00,0x00,0x00,
        0x1C,0x00,0x00,0x00,0xA6,0x00,0x00,0x00,0x1C,0x00,0x00,0x00,0x01,0x00,0x00,0x00,
        0x1C,0x00,0x00,0x00,0x2D,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xA5,0x00,0x00,0x00,
        0x11,0x00,0x00,0x00,0x03,0x00,0x00,0x00,0x22,0x4C,0x17,0x20,0x10,0x00,0x00,0x00,
        0x00,0x43,0x3A,0x5C,0x44,0x6F,0x63,0x75,0x6D,0x65,0x6E,0x74,0x73,0x20,0x61,0x6E,
        0x64,0x20,0x53,0x65,0x74,0x74,0x69,0x6E,0x67,0x73,0x5C,0x74,0x64,0x75,0x6E,0x67,
        0x61,0x6E,0x5C,0x4D,0x79,0x20,0x44,0x6F,0x63,0x75,0x6D,0x65,0x6E,0x74,0x73,0x5C,
        0x41,0x6C,0x6C,0x6F,0x79,0x20,0x52,0x65,0x73,0x65,0x61,0x72,0x63,0x68,0x5C,0x44,
        0x65,0x74,0x61,0x69,0x6C,0x65,0x64,0x20,0x44,0x6F,0x63,0x75,0x6D,0x65,0x6E,0x74,
        0x73,0x5C,0x43,0x6F,0x70,0x79,0x20,0x6F,0x66,0x20,0x4D,0x65,0x74,0x61,0x6C,0x20,
        0x41,0x6C,0x6C,0x6F,0x79,0x20,0x4C,0x69,0x73,0x74,0x20,0x52,0x65,0x73,0x65,0x61,
        0x72,0x63,0x68,0x2E,0x78,0x6C,0x73,0x78,0x00,0x00,0x58,0x00,0x2E,0x00,0x2E,0x00,
        0x5C,0x00,0x4D,0x00,0x79,0x00,0x20,0x00,0x44,0x00,0x6F,0x00,0x63,0x00,0x75,0x00,
        0x6D,0x00,0x65,0x00,0x6E,0x00,0x74,0x00,0x73,0x00,0x5C,0x00,0x41,0x00,0x6C,0x00,
        0x6C,0x00,0x6F,0x00,0x79,0x00,0x20,0x00,0x52,0x00,0x65,0x00,0x73,0x00,0x65,0x00,
        0x61,0x00,0x72,0x00,0x63,0x00,0x68,0x00,0x5C,0x00,0x44,0x00,0x65,0x00,0x74,0x00,
        0x61,0x00,0x69,0x00,0x6C,0x00,0x65,0x00,0x64,0x00,0x20,0x00,0x44,0x00,0x6F,0x00,
        0x63,0x00,0x75,0x00,0x6D,0x00,0x65,0x00,0x6E,0x00,0x74,0x00,0x73,0x00,0x5C,0x00,
        0x43,0x00,0x6F,0x00,0x70,0x00,0x79,0x00,0x20,0x00,0x6F,0x00,0x66,0x00,0x20,0x00,
        0x4D,0x00,0x65,0x00,0x74,0x00,0x61,0x00,0x6C,0x00,0x20,0x00,0x41,0x00,0x6C,0x00,
        0x6C,0x00,0x6F,0x00,0x79,0x00,0x20,0x00,0x4C,0x00,0x69,0x00,0x73,0x00,0x74,0x00,
        0x20,0x00,0x52,0x00,0x65,0x00,0x73,0x00,0x65,0x00,0x61,0x00,0x72,0x00,0x63,0x00,
        0x68,0x00,0x2E,0x00,0x78,0x00,0x6C,0x00,0x73,0x00,0x78,0x00,0x50,0x00,0x43,0x00,
        0x3A,0x00,0x5C,0x00,0x44,0x00,0x6F,0x00,0x63,0x00,0x75,0x00,0x6D,0x00,0x65,0x00,
        0x6E,0x00,0x74,0x00,0x73,0x00,0x20,0x00,0x61,0x00,0x6E,0x00,0x64,0x00,0x20,0x00,
        0x53,0x00,0x65,0x00,0x74,0x00,0x74,0x00,0x69,0x00,0x6E,0x00,0x67,0x00,0x73,0x00,
        0x5C,0x00,0x74,0x00,0x64,0x00,0x75,0x00,0x6E,0x00,0x67,0x00,0x61,0x00,0x6E,0x00,
        0x5C,0x00,0x4D,0x00,0x79,0x00,0x20,0x00,0x44,0x00,0x6F,0x00,0x63,0x00,0x75,0x00,
        0x6D,0x00,0x65,0x00,0x6E,0x00,0x74,0x00,0x73,0x00,0x5C,0x00,0x41,0x00,0x6C,0x00,
        0x6C,0x00,0x6F,0x00,0x79,0x00,0x20,0x00,0x52,0x00,0x65,0x00,0x73,0x00,0x65,0x00,
        0x61,0x00,0x72,0x00,0x63,0x00,0x68,0x00,0x5C,0x00,0x44,0x00,0x65,0x00,0x74,0x00,
        0x61,0x00,0x69,0x00,0x6C,0x00,0x65,0x00,0x64,0x00,0x20,0x00,0x44,0x00,0x6F,0x00,
        0x63,0x00,0x75,0x00,0x6D,0x00,0x65,0x00,0x6E,0x00,0x74,0x00,0x73,0x00,0x10,0x00,
        0x00,0x00,0x05,0x00,0x00,0xA0,0x05,0x00,0x00,0x00,0x14,0x00,0x00,0x00,0x60,0x00,
        0x00,0x00,0x03,0x00,0x00,0xA0,0x58,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x77,0x6B,
        0x73,0x2D,0x77,0x69,0x6E,0x78,0x70,0x33,0x32,0x62,0x69,0x74,0x00,0x00,0x4E,0xEB,
        0xCB,0x79,0x9D,0xF2,0x0C,0x4A,0xA7,0x0E,0xE5,0x64,0x7A,0x53,0x97,0x0B,0x7C,0x75,
        0x19,0xD9,0xD3,0x66,0xE1,0x11,0xA3,0xF6,0x00,0x50,0x56,0xA5,0x00,0x10,0x4E,0xEB,
        0xCB,0x79,0x9D,0xF2,0x0C,0x4A,0xA7,0x0E,0xE5,0x64,0x7A,0x53,0x97,0x0B,0x7C,0x75,
        0x19,0xD9,0xD3,0x66,0xE1,0x11,0xA3,0xF6,0x00,0x50,0x56,0xA5,0x00,0x10,0x00,0x00,
        0x00,0x00
    ];

    let lnk_file = match lnk::Lnk::new(Cursor::new(buffer)) {
        Ok(lnk) => lnk,
        Err(error) => panic!(error)
    };

    println!("{:#?}",lnk_file);
}

fn main(){
    lnk_test()
}
