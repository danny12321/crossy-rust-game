use std::{io, process::exit, time::{Duration, SystemTime}};

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::game::{run, GameInfo};

pub fn menu() -> () {
    let mut cursor = 0;

    loop {
        print!("{esc}c", esc = 27 as char);
        println!("Welcome to the game!");
        println!("");
        println!("Use the arrow keys to navigate the menu.");
        println!("Press enter to select an option.");
        println!("");
        println!("Options:");
        
        for (i, option) in get_menu_options().iter().enumerate() {
            if cursor == i {
                println!("> {}", option);
            } else {
                println!("  {}", option);
            }
        }

        if handle_input(&mut cursor).unwrap() {
            handle_enter(cursor);
        }
    }

    
}

fn handle_enter(cursor: usize) {
    match cursor {
        0 => {
            run(GameInfo {
                start_time: SystemTime::now(),
            });
            println!("Game over!");
        }
        1 => {
            println!("Bye!");
            exit(0);
        }
        _ => {
            println!("Invalid option!");
        }
    }
}

fn get_menu_options() -> [&'static str; 2] {
    return ["Start", "Exit"];
}

fn handle_input(cursor: &mut usize) -> io::Result<bool> {
    if poll(Duration::from_secs(0))? {
        loop {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                }) => {
                    *cursor -= 1;
                    return Ok(false);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                }) => {
                    *cursor += 1;
                    return Ok(false);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                }) => {
                    return Ok(true);
                }
                _ => {
                    return Ok(false);
                }
            }
        }
    } else {
        return Ok(false);
    }
}
