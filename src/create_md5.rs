#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::create_md5;
    fn create_md5_test() -> String {

        let opt = create_md5::create_md5::Opt{
            input: PathBuf::from("src/main.rs"),
            output: "l".to_string(),
            t: "1".to_string(),
        };
        let res = create_md5::create_md5::create_md5(opt).unwrap();
        return res
    }

    #[test]
    fn it_works() {
        assert_eq!(create_md5_test(), "639fbc4ef05b315af92b4d836c31b023".to_string());
    }
}



pub mod create_md5 {
    use structopt::StructOpt;
    use std::path::PathBuf;
    use failure::ResultExt;
    use exitfailure::ExitFailure;

    #[derive(StructOpt, Debug)]
    pub struct Opt {
        /// Input string or file path
        #[structopt(short, long, parse(from_os_str))]
        pub input: PathBuf,

        /// Input-type string=1, file_path=2
        #[structopt(short, long, default_value = "1")]
        pub t: String,

        /// Output uppercase or lowercase, u or l
        #[structopt(short, long, default_value = "l")]
        pub output: String,
    }

    pub fn create_md5(opt: Opt) -> Result<String, ExitFailure> {
        let mut val:String;
        if opt.t == "1" {
            let input = opt.input.to_str().unwrap();
            let digest = md5::compute(input);
            val = format!("{:?}", digest);
        } else {
            let result = std::fs::read_to_string(opt.input).with_context(|err| format!("could not read file `{:?}`", err))?;
            let digest = md5::compute(result);
            val = format!("{:?}", digest);
        }
        if &opt.output == "u" {
            val = val.to_uppercase()
        }
        Ok(val)
    }
}


