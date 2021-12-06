pub struct Config {
    pub path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 1 {
            return Err("Not enough arguments");
        }

        let path = args[1].clone();

        Ok(Config { path })
    }
}
