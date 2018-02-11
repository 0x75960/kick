#[macro_use]
extern crate structopt;

use std::fmt::*;
use std::fs::File;
use std::process::*;

use structopt::StructOpt;

mod bytereader;
mod signatures;

use bytereader::*;
use signatures::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "kick")]
struct Options {

  /// entrypoint (used when target is dll)
  #[structopt(short = "e", long = "entry", default_value = "#1")]
  entrypoint: String,

  /// execution target
  #[structopt(name = "FILE")]
  file: String,

}

fn copy_with_ext(p: &str, ext: &str) -> std::io::Result<String> {
  let dest = String::from(format!("{}.{}", p, ext));
  std::fs::copy(p, dest.as_str())?;
  Ok(dest)
}

fn kick(opt: &Options) -> std::io::Result<()> {

  match detect_filetype(opt.file.as_str())? {
    // exe file
    FileType::Exe => {
      println!("deal as exe");
      let dest = copy_with_ext(opt.file.as_str(), "exe")?;
      Command::new("cmd")
        .arg("/c")
        .arg("start")
        .arg(dest.as_str())
        .spawn()?;
    },

    // dll file
    FileType::Dll => {
      println!("deal as dll");
      let dest = copy_with_ext(opt.file.as_str(), "dll")?;
      Command::new("cmd")
        .arg("/c")
        .arg("rundll32.exe")
        .arg(format!("{},{}", dest.as_str(), opt.entrypoint.as_str()))
        .spawn()?;
    },

    // doc file
    FileType::Doc => {
      println!("deal as doc");
      let dest = copy_with_ext(opt.file.as_str(), "doc")?;
      Command::new("cmd")
        .arg("/c")
        .arg("start")
        .arg(dest.as_str())
        .spawn()?;
    }

    // xls file
    FileType::Xls => {
      println!("deal as xls");
      let dest = copy_with_ext(opt.file.as_str(), "xls")?;
      Command::new("cmd")
        .arg("/c")
        .arg("start")
        .arg(dest.as_str())
        .spawn()?;
    }

    // ppt file
    FileType::Ppt => {
      println!("deal as ppt");
      let dest = copy_with_ext(opt.file.as_str(), "ppt")?;
      Command::new("cmd")
        .arg("/c")
        .arg("start")
        .arg(dest.as_str())
        .spawn()?;
    }

    FileType::Jar => {
      println!("deal as jar");
      let dest = copy_with_ext(opt.file.as_str(), "jar")?;
      Command::new("cmd")
        .arg("/c")
        .arg("java")
        .arg("-jar")
        .arg(dest.as_str())
        .spawn()?;
    },

    FileType::Zip => {
      println!("deal as zip");
      println!("analyzing detail..");

      match analze_zip(opt.file.as_str())? {

        FileType::Docx => {
          println!("this is docx!");
          let dest = copy_with_ext(opt.file.as_str(), "docx")?;
          Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg(dest.as_str())
            .spawn()?;
        },

        FileType::Xlsx => {
          println!("this is xlsx!");
          let dest = copy_with_ext(opt.file.as_str(), "xlsx")?;
          Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg(dest.as_str())
            .spawn()?;
        },

        FileType::Pptx => {
          println!("this is pptx!");
          let dest = copy_with_ext(opt.file.as_str(), "pptx")?;
          Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg(dest.as_str())
            .spawn()?;
        },

        _ => {
          println!("other zip archive..");
        },

      }

    }

    // other
    _ => {
      println!("deal as other");
      Command::new("cmd")
        .arg("/c")
        .arg("start")
        .arg(opt.file.as_str())
        .spawn()?;
    },
  }

  Ok(())
}

fn main() {
  let opt = Options::from_args();

  match kick(&opt) {
    Ok(_) => println!("kicked!"),
    Err(e) => panic!(e),
  }

}
