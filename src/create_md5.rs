#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::create_md5;
    fn create_md5_test() -> String {
        let opt = create_md5::create_md5::Opt{
            input: PathBuf::from("src/main.rs"),
            output: "l".to_string(),
            t: "file".to_string(),
        };
        let res = create_md5::create_md5::create_md5(opt).unwrap();
        return res
    }

    #[test]
    fn it_works() {
        assert_eq!(create_md5_test(), "b29827ecc0ced7dc011014868259a780".to_string());
    }
}

pub mod create_md5 {
    use structopt::StructOpt;
    use exitfailure::ExitFailure;
    use std::path::{Path, PathBuf};
    use std::io::{BufReader, Read};
    use std::fs::File;
    use failure::{Error, ResultExt, ensure};

    #[derive(StructOpt, Debug)]
    pub struct Opt {
        /// Input string or file path
        #[structopt(short, long, parse(from_os_str))]
        pub input: PathBuf,

        /// Input-type t = (str or file)
        #[structopt(short, long, default_value = "str")]
        pub t: String,

        /// Output uppercase or lowercase,  o = (u or l)
        #[structopt(short, long, default_value = "l")]
        pub output: String,
    }

    pub fn create_md5(opt: Opt) -> Result<String, ExitFailure> {
        let mut val:String;
        if &opt.t == "str" {
            let input = opt.input.to_str().unwrap();
            let digest = md5::compute(input);
            val = format!("{:?}", digest);
        } else if &opt.t == "file" {
            let result = read_file(&opt.input)
                .with_context(|_| format!("could not read file `{:?}`", &opt.input))?;
            let digest = md5::compute(result);
            val = format!("{:?}", digest);
        } else {
            panic!("请输入正确的 -t 参数")
        };
        if &opt.output == "u" {
            val = val.to_uppercase()
        }
        Ok(val)
    }

    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
        let path = path.as_ref();
        ensure!(
        path.exists() && path.is_file(),
        "Path {:?} is not a file!",
        path
    );
        let file = File::open(path).with_context(|_| format!("Could not open file {:?}", path))?;
        let mut file = BufReader::new(file);
        let mut result = Vec::new();
        file.read_to_end(&mut result)
            .with_context(|_| format!("Could not read file {:?}", path))?;

        Ok(result)
    }
}


