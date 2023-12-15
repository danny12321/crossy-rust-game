use std::{time::{SystemTime, Duration}, io,};

use crossterm::event::{poll, read, KeyEvent, KeyCode, KeyEventKind, Event};

use crate::{renderer, Car, player, Player};

pub struct GameInfo {
    pub start_time: SystemTime,
}

pub fn run(game_info: GameInfo) {
    let view = renderer::View::new(20, 10);
    let mut player = player::Player::new("Danny", view.width / 2, 0);

    let mut cars = vec![Car::new_road_car(3, 3), Car::new_truck(10, 5)];

    // Move to game file
    loop {
        let playing = handle_input(&mut player);
        if playing.unwrap() == false {
            break;
        }

        renderer::render(
            &view,
            &player,
            &cars,
            game_info.start_time
        );
        cars.iter_mut().for_each(|car| car.step());
    }

}


fn handle_input(player: &mut Player) -> io::Result<bool> {
    if poll(Duration::from_secs(0))? {
        loop {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                }) => {
                    player.forward();
                    return Ok(true);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                }) => {
                    player.backwards();
                    return Ok(true);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                }) => {
                    player.left();
                    return Ok(true);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                }) => {
                    player.right();
                    return Ok(true);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                }) => {
                    println!();
                    return Ok(false);
                }
                _ => {
                    return Ok(true);
                }
            }
        }
    } else {
        return Ok(true);
    }
}
