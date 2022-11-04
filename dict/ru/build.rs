use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let dict_path = Path::new("./data").canonicalize()?;

    let mut f = File::create("src/release.rs").expect("Can't create a file");

    writeln!(f, r"pub const DICT_PATH: &str = {:?};", dict_path)?;

    Ok(())
}
