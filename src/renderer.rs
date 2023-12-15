use std::time::SystemTime;

use crate::camera::get_offset;
use crate::{Car, Player};

pub struct View {
    pub width: i16,
    pub height: i16,
}

impl View {
    pub fn new(width: i16, height: i16) -> View {
        View { width, height }
    }
}

pub fn render(
    view: &View,
    player: &Player,
    cars: &Vec<Car>,
    start_time: SystemTime
) {
    // Clear the screen
    print!("{esc}c", esc = 27 as char);
    render_stats(player, start_time);

    let y_offset = get_offset(player);
    let y_range = y_offset..(y_offset + view.height);
    for (_i, _y) in y_range.enumerate() {
        println!("");

        let mut _x = 0;
        while _x < view.width {
            if player.x == _x && player.y == _y {
                print!("{}", Player::get_cursor());
                _x += 1;
                continue;
            } else if cars.iter().any(|car| car.x == _x && car.y == _y) {
                let car = cars.iter().find(|car| car.x == _x && car.y == _y).unwrap();
                print!("{}", car.display);
                _x += car.display.len() as i16;
                continue;
            }

            print!(" ");
            _x += 1;
        }
    }
}

fn render_stats(player: &Player, start_time: SystemTime) {
    println!(
        "Player: {}, Distance: {}, time elapsed: {}s",
        player.name,
        player.y,
        start_time.elapsed().unwrap().as_secs()
    );
}
