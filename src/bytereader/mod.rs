use std::io::*;
use std::mem;

// ByteReader for read binary
pub struct ByteReader<T: Seek + Read> {
  buffer: T,
}

impl<T: Seek + Read> ByteReader<T> {

  pub fn from(b: T) -> ByteReader<T> {
    ByteReader{buffer: b}
  }

  pub fn seek(&mut self, seekto: SeekFrom) -> Result<()> {
    self.buffer.seek(seekto)?;
    Ok(())
  }

  // raw bytes
  pub fn take_raw_bytes(&mut self, buffer: &mut [u8]) -> Result<()> {
    self.buffer.read(buffer)?;
    Ok(())
  }

  pub fn take_raw_bytes_offset(&mut self, buffer: &mut [u8], offset: u64) -> Result<()> {
    self.seek(SeekFrom::Start(offset))?;
    self.take_raw_bytes(buffer)
  }

  // BYTE (8bit : 1byte)
  pub fn read_as_byte(&mut self) -> Result<u8> {
    let b = &mut [0; 1];
    self.buffer.read(b)?;
    Ok(b[0])
  }

  pub fn read_as_byte_offset(&mut self, offset: u64) -> Result<u8> {
    self.seek(SeekFrom::Start(offset))?;
    self.read_as_byte()
  }

  // WORD (16bit : 2byte)
  pub fn read_as_word(&mut self) -> Result<u16> {
    let b: &mut [u8; 2] = &mut [0; 2];
    self.take_raw_bytes(b)?;

    unsafe {
      let addr = mem::transmute::<[u8; 2], u16>(*b);
      return Ok(addr)
    }
  }

  pub fn read_as_word_offset(&mut self, offset: u64) -> Result<u16> {
    self.seek(SeekFrom::Start(offset))?;
    self.read_as_word()
  }

  // DWORD (32bit : 4byte)
  pub fn read_as_dword(&mut self) -> Result<u32> {
    let b: &mut [u8; 4] = &mut [0; 4];
    self.take_raw_bytes(b)?;

    unsafe {
      let addr = mem::transmute::<[u8; 4], u32>(*b);
      return Ok(addr)
    }
  }

  pub fn read_as_dword_offset(&mut self, offset: u64) -> Result<u32> {
    self.seek(SeekFrom::Start(offset))?;
    self.read_as_dword()
  }

  // QWORD (64bit : 8byte)
  pub fn read_as_qword(&mut self) -> Result<u64> {
    let b: &mut [u8; 8] = &mut [0; 8];
    self.take_raw_bytes(b)?;

    unsafe {
      let addr = mem::transmute::<[u8; 8], u64>(*b);
      return Ok(addr)
    }
  }

  pub fn read_as_qword_offset(&mut self, offset: u64) -> Result<u64> {
    self.seek(SeekFrom::Start(offset))?;
    self.read_as_qword()
  }

}
