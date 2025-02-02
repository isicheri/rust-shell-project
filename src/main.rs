#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();
    
 let stdin = io::stdin();
 let mut input = String::new();
 loop {
    stdin.read_line(&mut input).unwrap();
    let command: Vec<_> = input.trim().split(' ').collect();
     match command[..] {
         ["exit", code] => process::exit(code.parse::<i32>().unwrap()),
         ["echo",..] => print!("{}\n",command[1..].join(" ")),
         ["type", ..] => match command[1] {
           "exit" | "echo" | "type" => println!("{} is a shell builtin",command[1]),
           _=> println!("{}: not found",command[1])
         },
         _=> println!("{}: command not found",input.trim())
     }
     input.clear();
     print!("$ ");
     io::stdout().flush().unwrap();
 }
}
