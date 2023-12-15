pub struct Player {
    pub name: &'static str,
    pub x: i16,
    pub y: i16,
}

impl Player {
    pub fn new(name: &'static str, x: i16, y: i16) -> Player {
        Player { name, x, y }
    }

    pub fn get_cursor() -> &'static str {
        return "#";
    }

    pub fn forward(&mut self) {
        self.y -= 1;
    }

    pub fn backwards(&mut self) {
        self.y += 1;
    }

    pub fn left(&mut self) {
        self.x -= 1;
    }

    pub fn right(&mut self) {
        self.x += 1;
    }
}