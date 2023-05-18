#![allow(unused_parens)]
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
 
pub fn main() {
    let mut readline: String;
    let mut rl = match DefaultEditor::new(){
        Ok(editor) => {editor},
        Err(_) => panic!("Error! Unable to init DefaultEditor"),
    };

    if (rl.load_history("../history.txt").is_err()){
        println!("No previous history.");
    }
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
                return ;
            }
            Err(err) => {
                println!("Error {:?}", err);
                continue ;
            }
        };
        println!("Line is > {}", readline);
    }
}
