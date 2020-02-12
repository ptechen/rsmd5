# Md5 Command Line Tool

[![Version info](https://img.shields.io/crates/v/rsmd5.svg)](https://crates.io/crates/rsmd5)

Command line tool to encrypt and output md5 strings or files

对字符串或文件md5加密并输出的命令行工具
Install:

    cargo install rsmd5

USAGE:

    rsmd5 [OPTIONS] --input <input>

FLAGS:

    -h, --help       Prints help information

    -V, --version    Prints version information


OPTIONS:

    -i, --input <input>      Input string or file path

    -o, --output <output>    Output uppercase or lowercase,  o = (u or l) [default: l]

    -t, --t <t>              Input-type t = (str or file) [default: str]
