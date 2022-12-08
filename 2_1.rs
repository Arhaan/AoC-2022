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
        let my_move: u32 = line[1].chars().next().unwrap() as u32 - 'X' as u32 + 1;
        println!("They played {their_move} and I played {my_move}");
        score += my_move;
        if my_move == their_move{
            println!("Draw");
            score += 3;
        }
        else if my_move %3 == (their_move+1)%3{
            // I won
            println!("I won");
            score += 6;
        }

    }
    println!("{score}");
    
}
