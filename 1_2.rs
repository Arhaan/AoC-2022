use std::io;

fn main(){

    let mut max = [0,0,0];
    let mut index = 0;

    'eachElf : loop{
        index += 1;

        let mut sum = 0;
        loop{
            let mut elem = String::new();
            match io::stdin().read_line(&mut elem){
                Ok(num) => num,
                Err(_) =>  continue,           
            };
            elem = elem.trim().to_string();
            if elem.is_empty(){
                if sum == 0 {
                    break 'eachElf
                }
                else{
                    break
                }
            }
            let elem: u32 = elem.trim().parse().expect("Couldnt read as a number");
            sum += elem;

        }
        println!("For elf {index}, the amount is {sum}");
        if sum > max[0] {
            max[2] = max[1];
            max[1] = max[0];
            max[0] = sum;
        }
        else if sum > max[1]{
            max[2] = max[1];
            max[1] = sum;
        }
        else if sum > max[2]{
            max[2] = sum;
        }
    }
    let ans = max[0] + max[1] + max[2];
    println!("{ans}");
    
}
