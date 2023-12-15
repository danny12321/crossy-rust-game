use crate::Player;

pub fn get_offset(player: &Player) -> i16 {
    let offset = player.y - 2;

    if offset < 0 {
        return 0;
    }

    return offset;
}