use colored::*;
use rand::rng;
use rand::seq::IteratorRandom;
use std::io;

#[derive(Clone, Copy)]
enum Mode {
    Normal,
    God,
}

pub struct GameData {
    pub username: String,
    pub player_score: u32,
    pub computer_score: u32,
}

pub fn play_tic_tac_toe() -> GameData {
    let mut player_score = 0u32;
    let mut computer_score = 0u32;
    let mut draws = 0u32;

    println!("Welcome to SuccincTacToe");
    println!("Enter your username: ");
    let mut input_username = String::new();
    io::stdin().read_line(&mut input_username).unwrap();
    let username = input_username.trim().to_string();

    println!("Choose game mode: (1) Normal (2) God Mode 🧠");
    let mut mode_input = String::new();
    io::stdin().read_line(&mut mode_input).unwrap();
    let mode = if mode_input.trim() == "2" {
        Mode::God
    } else {
        Mode::Normal
    };

    loop {
        let mut board = [' '; 9];

        loop {
            println!("Your turn!");
            print_board(&board, None);
            player_move(&mut board);

            if let Some(winning_combo) = check_winner(&board, 'X') {
                print_board(&board, Some(winning_combo));
                println!("🎉 You win! 🎉");
                player_score += 100;
                break;
            }

            if is_full(&board) {
                print_board(&board, None);
                println!("🤝 It's a draw!");
                draws += 1;
                break;
            }

            println!("Computer's turn...");
            match mode {
                Mode::Normal => computer_move_normal(&mut board),
                Mode::God => computer_move_god(&mut board),
            }

            if let Some(winning_combo) = check_winner(&board, 'O') {
                print_board(&board, Some(winning_combo));
                println!("💻 Computer wins! 💥");
                computer_score += 100;
                break;
            }

            if is_full(&board) {
                print_board(&board, None);
                println!("🤝 It's a draw!");
                draws += 1;
                break;
            }
        }

        println!("\n🏁 Scoreboard:");
        println!(
            "You: {} | Computer: {} | Draws: {}\n",
            player_score, computer_score, draws
        );

        println!("Play again? (y/n): ");
        let mut again = String::new();
        io::stdin().read_line(&mut again).unwrap();
        if again.trim().to_lowercase() != "y" {
            println!("Thanks for playing! 👋");
            break;
        }
    }

    GameData {
        username,
        player_score,
        computer_score,
    }
}

fn print_board(board: &[char; 9], winning_combo: Option<[usize; 3]>) {
    println!();
    for i in 0..9 {
        let symbol = match board[i] {
            'X' => "X".to_string(),
            'O' => "O".to_string(),
            _ => (i + 1).to_string(),
        };

        let is_winner = winning_combo.map_or(false, |combo| combo.contains(&i));
        let cell = if is_winner {
            if board[i] == 'X' {
                symbol.red().bold()
            } else {
                symbol.green().bold()
            }
        } else {
            symbol.white()
        };

        print!(" {} ", cell);
        if i % 3 != 2 {
            print!("|");
        } else if i != 8 {
            println!("\n-----------");
        } else {
            println!();
        }
    }
    println!();
}

fn player_move(board: &mut [char; 9]) {
    loop {
        println!("Enter your move (1-9): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(pos) = input.trim().parse::<usize>() {
            if pos >= 1 && pos <= 9 && board[pos - 1] == ' ' {
                board[pos - 1] = 'X';
                break;
            }
        }
        println!("Invalid move. Try again.");
    }
}

fn computer_move_normal(board: &mut [char; 9]) {
    for i in 0..9 {
        if board[i] == ' ' {
            board[i] = 'O';
            if check_winner(board, 'O').is_some() {
                return;
            }
            board[i] = ' ';
        }
    }

    for i in 0..9 {
        if board[i] == ' ' {
            board[i] = 'X';
            if check_winner(board, 'X').is_some() {
                board[i] = 'O';
                return;
            }
            board[i] = ' ';
        }
    }

    if board[4] == ' ' {
        board[4] = 'O';
        return;
    }

    let mut rng = rng();
    let empty_positions: Vec<usize> = board
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v == ' ')
        .map(|(i, _)| i)
        .collect();

    if let Some(&choice) = empty_positions.iter().choose(&mut rng) {
        board[choice] = 'O';
    }
}

fn computer_move_god(board: &mut [char; 9]) {
    let (_, best_move) = minimax(board.clone(), true);
    if let Some(index) = best_move {
        board[index] = 'O';
    }
}

fn minimax(mut board: [char; 9], is_maximizing: bool) -> (i32, Option<usize>) {
    if let Some(_) = check_winner(&board, 'O') {
        return (1, None);
    }
    if let Some(_) = check_winner(&board, 'X') {
        return (-1, None);
    }
    if is_full(&board) {
        return (0, None);
    }

    let mut best_score = if is_maximizing { i32::MIN } else { i32::MAX };
    let mut best_move = None;

    for i in 0..9 {
        if board[i] == ' ' {
            board[i] = if is_maximizing { 'O' } else { 'X' };
            let (score, _) = minimax(board, !is_maximizing);
            board[i] = ' ';
            if is_maximizing && score > best_score {
                best_score = score;
                best_move = Some(i);
            } else if !is_maximizing && score < best_score {
                best_score = score;
                best_move = Some(i);
            }
        }
    }

    (best_score, best_move)
}

fn check_winner(board: &[char; 9], player: char) -> Option<[usize; 3]> {
    let win_combos = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for &combo in win_combos.iter() {
        if combo.iter().all(|&i| board[i] == player) {
            return Some(combo);
        }
    }

    None
}

fn is_full(board: &[char; 9]) -> bool {
    board.iter().all(|&c| c != ' ')
}
