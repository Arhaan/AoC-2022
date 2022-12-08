use std::io;

fn main(){


    let mut score = 0;
    loop{
        let mut line = String::new();
        match io::stdin().read_line(&mut line){
            Ok(val) => val,
            Err(_) => break,
        };
        if line.is_empty(){
            break;
        }
        let line = line.trim().split(" ").collect::<Vec<&str>>();

        let their_move: u32 = line[0].chars().next().unwrap() as u32 - 'A' as u32 + 1;
        let win_or_lose: u32 = line[1].chars().next().unwrap() as u32 - 'X' as u32;

        let mut my_move: u32 = (their_move + 2 + win_or_lose)%3;
        if my_move == 0{
            my_move = 3;
        }

        println!("They played {their_move} and I played {my_move}");
        score += my_move;
        score += win_or_lose * 3;
        println!("{score}");

    }
    println!("{score}");
    
}
