use std::{iter::Enumerate, usize};

const WINNING_PATTERNS: [[usize;3];8]= [[1,4,7],[2,5,8],[3,6,9],[1,2,3],[4,5,6],[7,8,9],[1,5,9],[3,5,7]];

struct GameResult{
    status:bool, // 0 continue 1 won draw
    winner:char
}
fn check_winner(board:&[char]) ->GameResult{
  /*[1,2,3],
    [4,5,6],
    [7,8,9] */

    let mut result = GameResult{status:false,winner:' '};
        for pattern in WINNING_PATTERNS{
        result = GameResult{status:result.status ,winner:board[pattern[0]-1]};
        let mut has_won = true;
        // Iterate over current winning pattern
        for (j,pos) in pattern.iter().enumerate(){ 
            has_won &= (board[*pos - 1] == result.winner) & ( board[*pos -1] != ' ');
            result = GameResult{status:result.status ,winner:board[pattern[j]-1]};
        }
        // Return game result and don't check for any other moves
        if has_won{
            println!("Winning combo: {:?}", pattern);
            return GameResult{status:true ,winner:result.winner};
        }
        else{
            println!("Failed: {:?}", pattern);
        }
    }
    return result;
}
// turn is the current players turn x,y  Should have made an enum >:/
fn capture_move(board:&[char]) -> usize{
    let final_positon;
    print!("play position: ");
    
    loop{

    let mut value = String::new();
    let _ = std::io::stdin().read_line(&mut value);
    println!();
     match value.trim().parse::<usize>() {
        Ok(mut position)  => {
            position -= 1;
            if !(0..9).contains(&position){
                println!("Please Enter a number between 1-9");
            }

            else if board[position] != ' '{
                println!("Pick an empty slot");
            }
            else{
                final_positon = position;
                break;
            }
        },
        Err(msg) => {println!("{msg} Please Enter a valid number "); continue;}
     };
    }
    final_positon
}
fn play_move(board:&mut [char],turn:char,position:usize){
    board[position] = turn;

}
fn display_board(board:&[char]){
    for i in 0..9{
        print!("| {} ",board[i]);

        if (i+1) % 3 == 0{
            print!("|\t| {} ",i-1);
            print!("| {} ",i);
            print!("| {} ",i+1);
            print!("|\n-------------\t");
            print!("-------------\n");
        }
    }
}

fn is_draw(board:&[char]) -> bool{
    let mut available = false;
    for i in board{
        available |= i == &' ';
    }
    !available
}
fn compute_move(board:&[char]) -> usize{
    //Avalible spots 
    let mut available_slots :Vec<usize> = vec![];
    for (pos,value) in board.iter().enumerate(){
        if value == &' '{
            available_slots.push(pos);
        } 
    }
    println!("{:?}", available_slots);
    for i in WINNING_PATTERNS{
        println!("{:?}",i);
        for position in i
        {
            let choosen_postion = position -1;
            if available_slots.contains(&choosen_postion){
                return choosen_postion.clone();
            }
        }
    }
    return 0; 
}
fn main() {
    let mut board = [' '; 9];
    let mut turn = ' ';
    
    // The rest of the game logic goes here
    'gameloop: loop {
        display_board(&board);
        if turn == 'X'{
            let position = capture_move(&board);
            play_move(&mut board,turn,position);
        }
        else if turn == 'O'{
            let position = compute_move(&board); 
            play_move(&mut board,turn,position);
        }// checks user input and plays move on board
        if turn == 'O'{
            turn = 'X';
        }else if turn == 'X'{
            turn = 'O';
        }
        else{turn = 'X'}
        let result =  check_winner(&board);
        if result.status{
            println!("The game has ended\n The winner was {}",result.winner);
            break 'gameloop
        }
        else if is_draw(&board){
            println!("The game has ended in a draw");
            break 'gameloop
        }


    }
    display_board(&board);
}
