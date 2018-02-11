kick
====

kick the file by appropriate method

* detect filetype by magic and signatures
    * exe / dll
    * doc / docx / xls / xlsx / ppt / pptx
    * jar

* execute target
    * exe   => `cmd /c start <target>.exe`
    * dll   => cmd /c `start rundll32 <DLL>.dll,#1`
        * you can change entrypoint with commandline option
    * jar   => `cmd /c java -jar <target>.jar`
    * other => `cmd /c start <target>.(ext)`

usage
-----

```bash
kick


USAGE:
    kick.exe [OPTIONS] <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --entry <entrypoint>    entrypoint (used when target is dll) [default: #1]

ARGS:
    <FILE>    execution target
```

setup
-----

```bash
# clone this repository
$ git clone git@github.com:0x75960/kick

# build
$ cd ./kick
$ cargo build --release
```
