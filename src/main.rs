#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
 let stdin = io::stdin();
 let mut input = String::new();
 loop {
    input.clear();
     print!("$ ");
     io::stdout().flush().unwrap();
    stdin.read_line(&mut input).unwrap();
     match input.trim() {
        input => {
            if input.starts_with("echo ")  {
                print!("{}\n", &input[5..])
            }else if input.starts_with("type ") {
               let builtin = ["echo","exit"];
               for i in builtin {
                   if i == &input[5..] {
                       print!("{} is a shell builtin\n",&input[5..].trim())
                   }
               }
            }
        },
        "exit 0" => break,
          & _=> {
            print!("{}: not found\n",input.trim());
        }
    }
 }
}
