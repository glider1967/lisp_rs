use rustyline::{error::ReadlineError, DefaultEditor, Result};

use crate::printer::print;
use crate::reader::read_str;
mod printer;
mod reader;
mod types;

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                println!("Line: {}", &line);
                dbg!(print(&read_str(&line).unwrap()));
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
