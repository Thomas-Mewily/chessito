#![allow(dead_code)]
#![allow(unused_assignments)]
use std::time::Instant;

use board::*;


fn player_vs_player(mut b : BoardGame)
{
    while !b.is_end_of_the_game()
    {
        b.console_display();
        println!("=================");
        let action_id = b.console_input();
        b.execute(action_id);
    }
    b.console_display();
    

    /* 
    let mut idx = 0;
    let lines : Vec<&str> = include_str!("./contre_kt.txt").lines().collect();

    while !b.is_end_of_the_game()
    {
        b.console_display();
        println!("=================");

        let action_id = if idx >= lines.len()
        {
            b.console_input()
        }else
        {
            idx += 1;
            b.console_input_from_str(lines[idx - 1]).unwrap()
        };
        
        b.execute(action_id);

    }
    b.console_display();*/
}

fn ai_vs_ai(board : BoardGame)
{
    let mut b = board.clone();

    let nb_turn = 16;
    println!("Ai vs Ai, {} turns", nb_turn);
    println!();

    /* 
    for _ in 0..nb_turn
    {
        b.console_display();
        b.execute(b.console_ai_best_move());
    }
    b.console_display();*/

    
    b = board.clone();
    let start = Instant::now();

    for _ in 0..nb_turn
    {
        b.execute(b.console_ai_best_move());
    }

    let elapsed_s = start.elapsed().as_secs_f64();

    b.console_display();
    println!("Total wait time : {:.4}s for {} turns ({:.4} s/turn)", elapsed_s, nb_turn, elapsed_s / nb_turn as f64);


}

/* 
cargo run --package=board_console --release
*/

fn main()
{
    for _ in 0..64 { println!(); }
    println!("Hello, world!");
    println!();

    let b = BoardGame::new_default();
    //let b = BoardGame::new_checker();
    println!("size of the board without the piece : {} octets", std::mem::size_of_val::<BoardGameFixedTime>(b.current()));
    println!("size of the board with the piece : {} octets", std::mem::size_of_val::<BoardGameFixedTime>(b.current()) + (b.current().board.size().x * b.current().board.size().y) as usize * std::mem::size_of_val::<Piece>(&b.current()[at(0, 0)]));
    println!("size of a single piece : {} octets", std::mem::size_of_val(&b[at(0, 0)]));

    player_vs_player(b);
    //ai_vs_ai(b);

}