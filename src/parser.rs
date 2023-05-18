#![allow(unused_parens)]
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::process;

pub fn  get_line(rl: &mut DefaultEditor) -> String
{
    let readline: String;

    loop
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
                continue ;
            }
            Err(ReadlineError::Eof) =>{
                if (rl.save_history("history.txt").is_err()){
                     println!("Rshell: Unable to save history");
                }
                println!("Exiting Rustshell");
                process::exit(0x0);
            }
            Err(err) => {
                println!("Error {:?}", err);
                continue ;
            }
        };
        return (readline);
    }
}
