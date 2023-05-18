#![allow(unused_parens)]
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::process;
mod parser;
 
pub fn main()
{
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
        readline = parser::get_line(& mut rl);
        std::process::Command::new(readline.as_str()).spawn().expect("uepaaa").wait().expect("ueepaaaa 2");
        //println!("Line is > {}", readline);
    }
}
