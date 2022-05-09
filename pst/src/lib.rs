use std::error::Error;
use std::fs;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::path::PathBuf;
use std::process::Command;


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
    let to_path = resolve_path(&config.path)?;
    let mut ctx = ClipboardContext::new()?;
    let from_path = ctx.get_contents()?;

    println!("cp -i -r {} {}", from_path, to_path);
    Command::new("cp")
        .arg("-i")
        .arg("-r")
        .arg(from_path)
        .arg(to_path)
        .spawn()?;

    Ok(())
}

// returns a resolved path from a reliative path
fn resolve_path(path: &str) -> Result<String, Box<dyn Error>> {
    let path_buf = PathBuf::from(path);
    let resolved = fs::canonicalize(&path_buf)?;
    Ok(resolved.to_string_lossy().replace("\"",""))
}

