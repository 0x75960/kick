extern crate zip;

use std::io::*;
use std::fs::*;

use bytereader::*;
use self::zip::ZipArchive;

pub enum FileType {
  Exe,
  Dll,
  Doc,
  Xls,
  Ppt,
  Zip,
  Docx,
  Xlsx,
  Pptx,
  Jar,
  Ohter,
}

pub fn is_pe<T: Seek + Read>(buffer: &mut ByteReader<T>) -> Result<bool> {

  let bmagic = &mut [0; 2];
  buffer.take_raw_bytes_offset(bmagic, 0)?;

  if *bmagic != [0x4d, 0x5a] {
    // not starts with signature "MZ"
    return Ok(false);
  }

  // check pe signature
  let bsig = &mut[0; 4];

  let ntheader_offset = buffer.read_as_dword_offset(0x3c)? as u64;
  buffer.take_raw_bytes_offset(bsig, ntheader_offset)?;

  // has PE signatue?
  Ok(*bsig == [0x50, 0x45, 0x00, 0x00])

}


fn is_dll<T: Seek + Read>(buffer: &mut ByteReader<T>) -> Result<bool> {

  // e_lfanew -> DWORD offset of IMAGE_NT_HEADER @ offset 0x3c from file head
  let ntheader_offset = buffer.read_as_dword_offset(0x3c)? as u64;

  // FileHeader -> head of Coff File Image header @ offset 0x04 from IMAGE_NT_HEADER
  let fileheader_offset = ntheader_offset + 0x04;

  // Characteristics -> flag value contains image type @ offset 0x12 from IMAGE_FILE_HEADER
  let chr = buffer.read_as_word_offset(fileheader_offset + 0x12)?;

  // dll file has 0x2000 flag
  Ok(chr & 0x2000 != 0)

}

fn is_zip_archive<T: Seek + Read>(buffer: &mut ByteReader<T>) -> Result<bool> {
  let bmagic = &mut[0; 2];
  buffer.take_raw_bytes_offset(bmagic, 0)?;

  Ok(*bmagic == [0x50, 0x4b])
}

fn is_jar<T: Seek + Read>(buffer: &mut ByteReader<T>) -> Result<bool> {
  let bmagic = &mut[0; 10];
  buffer.take_raw_bytes_offset(bmagic, 0)?;

  Ok(*bmagic == [0x50, 0x4B, 0x03, 0x04, 0x14, 0x00, 0x08, 0x00, 0x08, 0x00])
}

fn is_ole2<T: Seek + Read>(buffer: &mut ByteReader<T>) -> Result<bool> {
  let bmagic = &mut[0; 8];
  buffer.take_raw_bytes_offset(bmagic, 0)?;

  Ok(*bmagic == [0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1])
}

fn is_doc<T: Seek + Read>(buffer: &mut ByteReader<T>) -> Result<bool> {
  let bident = buffer.read_as_word_offset(512)?;
  Ok(bident == 0xa5ec)
}

fn is_xls<T: Seek + Read>(buffer: &mut ByteReader<T>) -> Result<bool> {

  let bytes = buffer.read_as_word_offset(30)? as u64;
  let bpos = buffer.read_as_dword_offset(48)? as u64;

  let baddr = 512 + bpos * (1 << bytes) + 128;

  let b_pattern1 = &mut [0; 16];
  buffer.take_raw_bytes_offset(b_pattern1, baddr);
  if *b_pattern1 == [0x57, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x6b, 0x00, 0x62, 0x00, 0x6f, 0x00, 0x6f, 0x00, 0x6b, 0x00] {
    return Ok(true);
  }

  let b_pattern2 = &mut [0; 8];
  buffer.take_raw_bytes_offset(b_pattern2, baddr);
  if *b_pattern2 == [0x42, 0x00, 0x6f, 0x00, 0x6f, 0x00, 0x6b, 0x00] {
    return Ok(true);
  }

  Ok(false)
}

fn is_ppt<T: Seek + Read>(buffer: &mut ByteReader<T>) -> Result<bool> {
  let bytes = buffer.read_as_word_offset(30)? as u64;
  let bpos = buffer.read_as_dword_offset(48)? as u64;

  let baddr = 512 + bpos * (1 << bytes) + 128;

  let b_pattern1 = &mut [0; 19];
  buffer.take_raw_bytes_offset(b_pattern1, baddr);
  if *b_pattern1 == [
    0x50, 0x00, 0x6f, 0x00, 0x77, 0x00, 0x65, 0x00, 0x72, 0x00,
    0x50, 0x00, 0x6f, 0x00, 0x69, 0x00, 0x6e, 0x00, 0x74] {

    buffer.take_raw_bytes_offset(b_pattern1, baddr + 19);
    if *b_pattern1 == [
      0x00, 0x20, 0x00, 0x44, 0x00, 0x6f, 0x00, 0x63, 0x00, 0x75,
      0x00, 0x6d, 0x00, 0x65, 0x00, 0x6e, 0x00, 0x74, 0x00] {
      return Ok(true);
    }

  }


  let b_pattern2 = &mut [0; 24];
  buffer.take_raw_bytes_offset(b_pattern2, baddr);
  if *b_pattern2 == [
    0x43, 0x00, 0x75, 0x00, 0x72, 0x00, 0x72, 0x00, 0x65, 0x00,
    0x6e, 0x00, 0x74, 0x00, 0x20, 0x00, 0x55, 0x00, 0x73, 0x00,
    0x65, 0x00, 0x72, 0x00] {
    return Ok(true);
  }


  let b_pattern3 = &mut [0; 16];
  buffer.take_raw_bytes_offset(b_pattern3, baddr);
  if *b_pattern3 == [
    0x50, 0x00, 0x69, 0x00, 0x63, 0x00, 0x74, 0x00, 0x75, 0x00, 0x72, 0x00,
    0x65, 0x00, 0x73, 0x00] {
    return Ok(true);
  }


  Ok(false)
}

pub fn analze_zip(t: &str) -> Result<FileType> {

  let f = File::open(t)?;
  let mut z = zip::ZipArchive::new(f)?;

  if let Ok(_) = z.by_name("word/document.xml") {
    return Ok(FileType::Docx);
  }

  if let Ok(_) = z.by_name("xl/workbook.xml") {
    return Ok(FileType::Xlsx);
  }

  if let Ok(_) = z.by_name("ppt/presentation.xml") {
    return Ok(FileType::Pptx);
  }

  return Ok(FileType::Zip);
}

pub fn detect_filetype(t: &str) -> Result<FileType> {
  let f = File::open(t)?;
  detect_type(f)
}

pub fn detect_type<T: Seek + Read>(t: T) -> Result<FileType> {

  let mut buffer = ByteReader::from(t);

  if is_pe(&mut buffer)? {

    if is_dll(&mut buffer)? {
      return Ok(FileType::Dll);
    }

    return Ok(FileType::Exe);
  }

  if is_ole2(&mut buffer)? {

    if is_doc(&mut buffer)? {
      return Ok(FileType::Doc);
    }

    if is_xls(&mut buffer)? {
      return Ok(FileType::Xls);
    }

    if is_ppt(&mut buffer)? {
      return Ok(FileType::Ppt);
    }

  }

  if is_jar(&mut buffer)? {
    return Ok(FileType::Jar);
  }

  if is_zip_archive(&mut buffer)? {
    return Ok(FileType::Zip);
  }

  Ok(FileType::Ohter)

}
