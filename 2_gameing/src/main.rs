use std::io::{Read, Write};

use std::cmp::{max, min};

use termion::{color, cursor};
use termion::event::{Key, Event};
use termion::input::{TermRead};
use termion::raw::IntoRawMode;

use std::thread;
use std::time::Duration;

use std::net;

#[derive(Clone, Copy)]
struct GameState {
    board: [[i8; 3]; 3],
    cursor: [i8; 2]
}

// Server loop
fn main() {
    let listener = net::TcpListener::bind("0.0.0.0:9999").expect("Failed to bind TCP listener");
    for connection in listener.incoming() {
        let mut stream = connection.expect("Failed to accept connection");
        println!("Accepted new connection from {}", stream.peer_addr().expect("Could not get peer address"));
        let mut stream_clone = stream.try_clone().expect("Could not clone stream"); // this placates the borrow checker
        thread::spawn(move || { run_game(&mut stream, &mut stream_clone); });
    }
}

// Game main loop
fn run_game(instream: &mut impl Read, outstream: &mut impl Write) {
    let mut outstream = outstream.into_raw_mode().expect("Couldn't set stream to raw mode");

    writeln!(outstream, "{}", termion::clear::All).expect("Write error");
    write!(outstream, "{}", cursor::Goto(1, 1)).expect("Write error");
    write!(outstream, "{}", cursor::Hide).expect("Write error");

    let mut state: GameState = GameState {
        board: [[0; 3]; 3],
        cursor: [0; 2]
    };

    print_field_state(&state, &mut outstream);
    for event in instream.events() {
        let event = event.unwrap();
        match event {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Left) => state.cursor[0] = max(state.cursor[0] - 1, 0),
            Event::Key(Key::Right) => state.cursor[0] = min(state.cursor[0] + 1, 2),
            Event::Key(Key::Up) => state.cursor[1] = max(state.cursor[1] - 1, 0),
            Event::Key(Key::Down) => state.cursor[1] = min(state.cursor[1] + 1, 2),
            Event::Key(Key::Char(' ')) | Event::Key(Key::Char('\n')) => {
                if valid_turn(&state, state.cursor) {
                    state.board[state.cursor[1] as usize][state.cursor[0] as usize] = 1;
                    if check_win(&state) == 0 && !game_over(&state) {
                        print_field_state(&state, &mut outstream);
                        thread::sleep(Duration::from_millis(100));
                        state = ai_turn(&state);
                    }
                }
            }
            _ => {}
        }
        print_field_state(&state, &mut outstream);
        let win_state = check_win(&state);
        if win_state != 0 {
            if win_state == 1 {
                writeln!(outstream, "You win!").expect("Write error");
            }
            else {
                writeln!(outstream, "Computer wins! Beep boop!").expect("Write error");
            }
            break;
        }
        if game_over(&state) {
            break;
        }
    }
    writeln!(outstream, "\rGame over!\n\r").expect("Write error");
    write!(outstream, "{}", cursor::Show).expect("Write error");
}

// Check if there are no valid turns left
fn game_over(state: &GameState) -> bool {
    for row_idx in 0..state.board.len() {
        for cell_idx in 0..state.board[0].len() {
            if state.board[row_idx][cell_idx] == 0 {
                return false;
            }
        }
    }
    return true;
}

// Check if a turn can be executed
fn valid_turn(state: &GameState, proposed: [i8; 2]) -> bool {
    if state.board[proposed[1] as usize][proposed[0] as usize] == 0 {
        return true;
    }
    return false;
}

// Figure out best AI turn
fn ai_turn(state: &GameState) -> GameState {
    // Check for AI win
    for row_idx in 0..state.board.len() {
        for cell_idx in 0..state.board[0].len() {
            if valid_turn(state, [cell_idx as i8, row_idx as i8]) {
                let mut proposed_state = state.clone();
                proposed_state.board[row_idx][cell_idx] = -1;
                if check_win(&proposed_state) == -1 {
                    return proposed_state;
                }
            }
        }
    }

    // Check for player win
    for row_idx in 0..state.board.len() {
        for cell_idx in 0..state.board[0].len() {
            if valid_turn(state, [cell_idx as i8, row_idx as i8]) {
                let mut proposed_state = state.clone();
                proposed_state.board[row_idx][cell_idx] = 1;
                if check_win(&proposed_state) == 1 {
                    proposed_state.board[row_idx][cell_idx] = -1;
                    return proposed_state;
                }
            }
        }
    }

    // Check central
    if valid_turn(state, [1, 1]) {
        let mut proposed_state = state.clone();
        proposed_state.board[1][1] = -1;
        return proposed_state;
    }

    // Just go first free
    for row_idx in 0..state.board.len() {
        for cell_idx in 0..state.board[0].len() {
            if valid_turn(state, [cell_idx as i8, row_idx as i8]) {
                let mut proposed_state = state.clone();
                proposed_state.board[row_idx][cell_idx] = -1;
                return proposed_state;
            }
        }
    }

    // Should never be reached
    return *state;
}

// Draw the field using ~fancy terminal stuff~
fn print_field_state(state: &GameState, outstream: &mut impl Write) {
    let color_ui = color::Fg(color::Rgb(255, 255, 255));
    let color_player1 = color::Fg(color::Rgb(91, 206, 250));
    let color_player2 = color::Fg(color::Rgb(245, 169, 184));

    writeln!(outstream, "{}", termion::clear::CurrentLine).expect("Write error");
    write!(outstream, "{}", cursor::Goto(1, 1)).expect("Write error");

    for (row_idx, row) in state.board.iter().enumerate() {
        write!(outstream, "\r").expect("Write error");
        for (cell_idx, &cell) in row.iter().enumerate() {
            let mut sel_str_left: String = String::from(" ");
            let mut sel_str_right: String = String::from(" ");
            if cell_idx as i8 == state.cursor[0] && row_idx as i8 == state.cursor[1] {
                sel_str_left = String::from(">");
                sel_str_right = String::from("<");
            }
            if cell == 0 {
                write!(outstream, "{sel_str_left} {sel_str_right}").expect("Write error");
            }
            else if cell == 1 {
                write!(outstream, "{color_ui}{sel_str_left}{color_player1}X{color_ui}{sel_str_right}").expect("Write error");
            }
            else if cell == -1 {
                write!(outstream, "{color_ui}{sel_str_left}{color_player2}O{color_ui}{sel_str_right}").expect("Write error");
            }
            
            if cell_idx < 2 {
                write!(outstream, "{color_ui}|").expect("Write error");
            }
        }
        if row_idx < 2 {
            write!(outstream, "\n\r{color_ui}-----------").expect("Write error");
        }

        write!(outstream, "\n\r").expect("Write error");
    }
}

// See if the  board state is a winning state
fn check_win(state: &GameState) -> i8 {    
    // check rows
    for row_idx in 0..state.board.len() {
        let mut win_state = 0;
        for cell_idx in 0..state.board[0].len() {
            win_state += state.board[row_idx][cell_idx];
        }
        if win_state == 3 {
            return 1;
        }
        if win_state == -3 {
            return -1;
        }
    }

     // check cols
    for cell_idx in 0..state.board[0].len() {
        let mut win_state = 0;
        for row_idx in 0..state.board.len() {
            win_state += state.board[row_idx][cell_idx];
        }
        if win_state == 3 {
            return 1;
        }
        if win_state == -3 {
            return -1;
        }
    }

    // Check diagonals
    let mut win_state = 0;
    for diag_idx in 0..state.board.len() {
        win_state += state.board[diag_idx][diag_idx];
        if win_state == 3 {
            return 1;
        }
        if win_state == -3 {
            return -1;
        }
    }

    let mut win_state = 0;
    for diag_idx in 0..state.board.len() {
        win_state += state.board[diag_idx][2 - diag_idx];
        if win_state == 3 {
            return 1;
        }
        if win_state == -3 {
            return -1;
        }
    }

    return 0;
}
