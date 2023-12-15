pub struct Car {
    pub x: i16,
    pub y: i16,
    pub display: &'static str,
}

impl Car {
    pub fn new_road_car(x: i16, y: i16) -> Car {
        return Car {
            x: x,
            y: y,
            display: "=>",
        };
    }

    pub fn new_truck(x: i16, y: i16) -> Car {
        return Car {
            x: x,
            y: y,
            display: "[====]=>",
        };
    }

    pub fn step(&mut self) -> () {
        self.x += 1;
    }
}
