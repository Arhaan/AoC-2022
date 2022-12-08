use std::io;
use regex::Regex;


fn main(){
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut ans = 0;
    loop{

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Heyo an error");
        line = line.trim().to_string();
        if line.is_empty(){
            break;
        }
        let line = re.captures_iter(&line);
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;
        for l in line{
            a = l.get(1).unwrap().as_str().parse().expect("Invalid");
            b = l.get(2).unwrap().as_str().parse().expect("Invalid");
            c = l.get(3).unwrap().as_str().parse().expect("Invalid");
            d = l.get(4).unwrap().as_str().parse().expect("Invalid");
        }
        if c > b || a > d{
            ans += 0;
        } 
        else{
            ans += 1;
        }
    }
    println!("{ans}");
}
