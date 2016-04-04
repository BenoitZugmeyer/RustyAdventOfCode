use std::fmt;
use std::io;
use std::io::Read;
use std::io::BufRead;


struct LightIterator<'a> {
    index: usize,
    grid: &'a Grid,
}

impl<'a> LightIterator<'a> {
    fn new(grid: &'a Grid) -> Self {
        LightIterator { index: 0, grid: grid }
    }
}

impl<'a> Iterator for LightIterator<'a> {
    type Item = (usize, usize, &'a bool);

    fn next(&mut self) -> Option<Self::Item> {
        self.grid.data.get(self.index).map(|light| {
            let i = self.index;
            self.index += 1;
            (i % self.grid.size, i / self.grid.size, light)
        })
    }
}

#[derive(Debug)]
struct Grid {
    size: usize,
    is_bogus: bool,
    data: Vec<bool>,
}

impl Grid {
    fn new() -> Self {
        Grid {
            size: 0,
            is_bogus: false,
            data: Vec::new(),
        }
    }

    fn set_bogus(&mut self) {
        self.is_bogus = true;
        self.set_bogus_lights();
    }

    fn set_size(&mut self, size: usize) {
        self.size = size;
        self.data.resize(size * size, false);
    }

    fn turn_on(&mut self, x: usize, y: usize) {
        if x < self.size && y < self.size {
            self.data[x + y * self.size] = true;
        }
    }

    fn iter(&self) -> LightIterator {
        LightIterator::new(self)
    }

    fn count_neighbours(&self, x: usize, y: usize) -> u32 {
        // this is very time consuming, let's optimize by hand
        macro_rules! get {
            ($cond: expr, $x: expr, $y: expr) => (
                if $cond {
                    self.data[$x + $y * self.size] as u32
                }
                else {
                    0
                }
            )
        }

        let has_left = x > 0;
        let has_top = y > 0;
        let has_right = x < self.size - 1;
        let has_bottom = y < self.size - 1;

        get!(has_left,   x - 1, y) +
        get!(has_right,  x + 1, y) +
        get!(has_top,    x, y - 1) +
        get!(has_bottom, x, y + 1) +

        // corners
        get!(has_left && has_bottom,  x - 1, y + 1) +
        get!(has_left && has_top,     x - 1, y - 1) +
        get!(has_right && has_bottom, x + 1, y + 1) +
        get!(has_right && has_top,    x + 1, y - 1)
    }

    fn set_bogus_lights(&mut self) {
        let m = self.size - 1;
        self.turn_on(0, 0);
        self.turn_on(0, m);
        self.turn_on(m, 0);
        self.turn_on(m, m);
    }

    fn next_step(&self) -> Self {
        let mut result = Grid::new();
        result.set_size(self.size);

        for (x, y, light) in self.iter() {
            let count = self.count_neighbours(x, y);
            let should_turn_on = if *light { count == 2 || count == 3 } else { count == 3 };

            if should_turn_on {
                result.turn_on(x, y);
            }
        }

        if self.is_bogus { result.set_bogus() }

        result
    }

    fn count_lights_on(&self) -> usize {
        self.data.iter().filter(|&l| *l).count()
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (x, _, light) in self.iter() {
            try!(write!(f, "{}", if *light { '#' } else { '.' }));
            if x as usize == self.size - 1 {
                try!(write!(f, "\n"));
            }
        }
        Ok(())
    }
}

fn main() {
    let mut original_grid = Grid::new();

    let stdin = io::stdin();

    for (y, line) in stdin.lock().lines().filter_map(|l| l.ok()).enumerate() {
        if original_grid.size == 0 {
            original_grid.set_size(line.len());
        }
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                original_grid.turn_on(x, y);
            }
        }
    }

    let mut grid = original_grid.next_step();

    original_grid.set_bogus();
    let mut bogus_grid = original_grid.next_step();

    for _ in 1..100 {
        grid = grid.next_step();
        bogus_grid = bogus_grid.next_step();
    }

    println!("Working grid lights count: {}", grid.count_lights_on());
    println!("Bogus grid lights count: {}", bogus_grid.count_lights_on());
}
