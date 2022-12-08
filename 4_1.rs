use std::io;
use regex::Regex;


fn main(){
    loop{
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Heyo an error");
        line = line.trim().to_string();
        if line.is_empty(){
            break;
        }

    }
}
