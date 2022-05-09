use std::error::Error;
use std::fs;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::path::PathBuf;


pub struct Config {
    pub path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let path = args[1].clone();

        Ok(Config {
            path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let path = resolve_path(&config.path)?;
    println!("{}", path);
    let mut ctx = ClipboardContext::new()?;
    ctx.set_contents(path)?;

    Ok(())
}


// returns a resolved path from a reliative path
fn resolve_path(path: &str) -> Result<String, Box<dyn Error>> {
    let path_buf = PathBuf::from(path);
    let resolved = fs::canonicalize(&path_buf)?;
    Ok(resolved.to_string_lossy().replace("\"",""))
}

