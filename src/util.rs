use std::io::{Read, Seek, SeekFrom};
use std::str;

use error::Result;

pub static APE_PREAMBLE: &'static [u8] = b"APETAGEX";
static ID3V1_HEADER: &'static [u8] = b"TAG";
static LYRICS3V2_HEADER: &'static [u8] = b"LYRICS200";

/// Position of ID3v1 tag
pub const ID3V1_OFFSET: i64 = -128;

/// Number of bytes, which are text digits
/// that give the total number of bytes
/// in the Lyrics3 v2.00 tag field.
const LYRICS3V2_SIZE: i64 = 6;

/// Checks whether ape tag exists
pub fn probe_ape<R: Read + Seek>(reader: &mut R, pos: SeekFrom) -> Result<bool> {
    let capacity = APE_PREAMBLE.len();
    let mut preamble = Vec::<u8>::with_capacity(capacity);
    try!(reader.seek(pos));
    try!(reader.take(capacity as u64).read_to_end(&mut preamble));
    Ok(preamble == APE_PREAMBLE)
}

/// Whether ID3v1 tag exists
pub fn probe_id3v1<R: Read + Seek>(reader: &mut R) -> Result<bool> {
    let capacity = ID3V1_HEADER.len();
    let mut header = Vec::<u8>::with_capacity(capacity);
    try!(reader.seek(SeekFrom::End(ID3V1_OFFSET)));
    try!(reader.take(capacity as u64).read_to_end(&mut header));
    Ok(header == ID3V1_HEADER)
}

/// Returns the size of the Lyrics3 v2.00 tag or -1 if the tag does not exists.
/// See http://id3.org/Lyrics3v2 for more details.
pub fn probe_lyrics3v2<R: Read + Seek>(reader: &mut R) -> Result<i64> {
    let capacity = LYRICS3V2_HEADER.len();
    let mut header = Vec::<u8>::with_capacity(capacity);
    try!(reader.seek(SeekFrom::End(ID3V1_OFFSET - capacity as i64)));
    try!(reader.take(capacity as u64).read_to_end(&mut header));
    try!(reader.seek(SeekFrom::Current(0 - capacity as i64)));
    if header == LYRICS3V2_HEADER {
        let mut buf = Vec::<u8>::with_capacity(LYRICS3V2_SIZE as usize);
        try!(reader.seek(SeekFrom::Current(-LYRICS3V2_SIZE)));
        try!(reader.take(LYRICS3V2_SIZE as u64).read_to_end(&mut buf));
        let raw_size = try!(str::from_utf8(&buf));
        let int_size = try!(raw_size.parse::<i64>());
        Ok(int_size + LYRICS3V2_SIZE + capacity as i64)
    } else {
        Ok(-1)
    }
}
