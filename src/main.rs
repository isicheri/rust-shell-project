mod mods {
    pub mod builtins;
}

#[allow(unused_imports)]
use pathsearch::find_executable_in_path;
use std::io::{self, Write};
use std::process;

const BUILTINS: [&str; 3] = ["exit", "echo", "type"];


fn parse_input(input: &str) -> Result<String,String> {
    let line = input.trim();
    let line: Vec<&str> = line.split(" ").collect(); 
    if line.len() == 0 {
     panic!("Command was not provided");
    }
    let command = line[0];
    match command {
        "exit"=> {
            if line.len() > 1 {
                if let Ok(i) = line[1].parse::<i32>() {
                    process::exit(i)
                }else {
                    process::exit(-1)
                }
            }
            process::exit(0)
        },
        "echo" => {
            let echo = &line[1..].join(" ");
            println!("{}",echo);
            Ok(echo.to_string())
        },
        "type" => {
           if line.len() < 2 {
            println!("The syntax of the command is incorrect");
            return Ok(String::from("type"));
           }
           if BUILTINS.contains(&line[1]) {
            println!("{} is a shell builtin",line[1]);
            return Ok(String::from("type"));
           }else if let Some(exe) = find_executable_in_path(line[1]) {
            println!("{} is {}",line[1],exe.display());
            return Ok(String::from("type"));
           }else {
           println!("{} not found",line[1]);
           return Ok(String::from("type"));
           }
        },
        _=> return Err(String::from(command))
       }
    }


fn main() {
 loop {
    print!("$ ");
    io::stdout().flush().unwrap();
    
 let stdin = io::stdin();
 let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    if let Err(command) = parse_input(&input.as_str()) {
        if command.is_empty() {
        println!("input cannot be empty");
        }else {
        println!("{}: command not found",command);
        }
    }

    // let command: Vec<_> = input.trim().split(' ').collect();
    //  match command[..] {
    //      ["exit", code] => process::exit(code.parse::<i32>().unwrap()),
    //      ["echo",..] => print!("{}\n",command[1..].join(" ")),
    //      ["type", ..] => match command[1] {
    //        "exit" | "echo" | "type" => println!("{} is a shell builtin",command[1]),
    //        _=> println!("{}: not found",command[1])
    //      },
    //      _=> println!("{}: command not found",input.trim())
    //  }
    //  input.clear();
    //  print!("$ ");
    //  io::stdout().flush().unwrap();
 }
}
