#![allow(unused_parens)]
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

pub fn  get_line(readline: &mut String) -> String
{
    readline = match rl.readline("Rustshell $> ")
    {
        Ok(result) => {
            if (rl.add_history_entry(result.as_str()).is_err()){
                println!("Rshell: Unable to add last command to history");
            }
            result
        }
        Err(ReadlineError::Interrupted) => {
            println!("Cntrl C");
            break
        }
        Err(ReadlineError::Eof) =>{
            println!("Cntrl D");
            break
        }
        Err(err) => {
            println!("Error {:?}", err);
            break
        }
    };
}
