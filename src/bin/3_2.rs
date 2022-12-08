use std::io;

fn priority(c: char) -> u32{
    let c: u32 = c as u32;
    if c >= 'a' as u32 && c <= 'z' as u32{
        c-'a' as u32 + 1
    }
    else{
        c-'A' as u32 + 27
    }
}
fn main(){
    let mut sum: u32 = 0;
    'global: loop{
        let mut arr: [(u32, u32); 52] = [(0, 0); 52]; // (how many times the char has occured, the string in which the last char occurred)
        let mut n_seen = 0;
        loop{
            // Read the 3 lines, one in each iteration
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("Error in input");
            line = line.trim().to_string();
            if line.trim().to_string().is_empty(){
                    break 'global;
            }
            n_seen += 1;
            for i in line.chars(){
                let prior = priority(i)-1;
                let prior: usize = prior as usize;
                if arr[prior].1 == n_seen{
                    // The char has already appeared in the string
                    continue;
                }
                else{
                    arr[prior] = (arr[prior].0+1, n_seen);
                    if arr[prior].0 == 3{
                        // println!("{i}");
                        // println!("{prior as u32}");
                        sum += prior as u32 + 1;
                        continue 'global;
                    }
                }
            }
        };
    }
    println!("{sum}")
}
