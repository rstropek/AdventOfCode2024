use std::{fs::File, io::Read};
use anyhow::Result;

pub fn read_input_file(day: &str,input_type: &str) -> Result<String> {
    let mut file = File::open(format!("{day}/data/{input_type}.txt"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
