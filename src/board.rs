use crate::util::vector2::Vector2;

pub struct Board {
    values: [[u8; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Board {
            values: [[0; 3]; 3],
        }
    }

    pub fn get_value(&self, x: usize, y: usize) -> u8 {
        self.values[y][x]
    }

    pub fn set_value(&mut self, position: &Vector2, value: u8) {
        self.values[position.y][position.x] = value;
    }

    pub fn print(&self) {
        // Ignore this, I did some hacky things for formatting

        println!("\n\n");

        for (i, y) in self.values.iter().enumerate() {
            for (j, x) in y.iter().enumerate() {
                print!(
                    "{}",
                    if *x == 1 {
                        "X"
                    } else if *x == 2 {
                        "O"
                    } else {
                        " "
                    }
                );

                if j != 2 {
                    print!(" | ")
                }
            }

            if i != 2 {
                println!("\n----------");
            }
        }
        println!("");
    }

    pub fn is_board_full(&self) -> bool {
        for y in self.values {
            for x in y {
                if x == 0 {
                    return false;
                }
            }
        }

        return true;
    }

    fn get_y(&self, y: usize) -> [u8; 3] {
        self.values[y]
    }

    fn get_x(&self, x: usize) -> [u8; 3] {
        let mut ret = [0; 3];

        for (i, y) in self.values.iter().enumerate() {
            ret[i] = y[x];
        }

        ret
    }

    fn get_diag(&self, start_y: i8, slope: i8) -> [u8; 3] {
        let mut ret = [0; 3];

        let mut y: i8 = start_y;
        for x in 0..3 {
            ret[x] = self.values[y as usize][x];
            y += slope;
        }

        ret
    }

    fn has_winner(&self, list: [u8; 3]) -> bool {
        if list[0] == 0 {
            return false;
        }

        for v in list {
            if v != list[0] {
                return false;
            }
        }

        true
    }

    pub fn get_winner(&self) -> u8 {
        for y in 0..3 {
            if self.has_winner(self.get_y(y)) {
                return self.values[y][0];
            }
        }

        for x in 0..3 {
            if self.has_winner(self.get_x(x)) {
                return self.values[0][x];
            }
        }

        if self.has_winner(self.get_diag(0, 1)) {
            return self.values[0][0];
        }

        if self.has_winner(self.get_diag(2, -1)) {
            return self.values[2][0];
        }

        0
    }
}
