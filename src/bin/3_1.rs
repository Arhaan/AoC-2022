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
    loop{
        let mut arr: [u32; 52] = [0; 52];
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Error in input");
        line = line.trim().to_string();
        if line.trim().to_string().is_empty(){
                break;
        }
        let mut index = 0;
        for i in line.chars(){
            index+=1;
            let prior = priority(i)-1;
            if arr[prior as usize] == 1 && 2*index <=line.len() { continue;}

            if arr[prior as usize] == 0 && 2*index <=line.len() { 
                arr[prior as usize] += 1;
            }
            if arr[prior as usize] == 1 && 2*index >line.len() { 
                arr[prior as usize] += 1;
            }
            if arr[prior as usize] > 1{
                sum += prior+1;
                println!("{i}");
                break;
            }
        }

    }
    println!("{sum}")
}
