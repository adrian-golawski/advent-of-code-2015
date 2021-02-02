use std::{
    convert::TryInto,
    fmt::{self, Debug, Display},
};

fn main() {
    let lights: Vec<bool> = include_str!("input.txt")
        .lines()
        .collect::<String>()
        .chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("Unknown char!"),
        })
        .collect();

    let mut grid = Grid::new(&lights, 100);

    for _ in 0..100 {
        grid = Grid::from(&grid);
    }

    println!("[Part one] {}", grid.get_light_count());

    let mut grid = Grid::new(&lights, 100);

    grid.set(0, 0, true);
    grid.set(0, grid.size - 1, true);
    grid.set(grid.size - 1, 0, true);
    grid.set(grid.size - 1, grid.size - 1, true);

    for _ in 0..100 {
        grid = Grid::from_with_broken_lights(&grid);
    }

    println!("[Part two] {}", grid.get_light_count());
}

#[derive(Clone)]
struct Grid {
    lights: Vec<bool>,
    size: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.lights.chunks(100) {
            for &light in line {
                let symbol = if light { '#' } else { '.' };
                write!(f, "{} ", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.lights.chunks(100) {
            for &light in line {
                let symbol = if light { '#' } else { '.' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Grid {
    fn new(lights: &[bool], size: usize) -> Self {
        Grid {
            lights: lights.try_into().unwrap(),
            size,
        }
    }

    fn from(old_grid: &Grid) -> Self {
        let mut new_grid = Grid {
            lights: vec![false; old_grid.size * old_grid.size],
            size: old_grid.size,
        };

        for x in 0..old_grid.size {
            for y in 0..old_grid.size {
                if old_grid.get(x, y) {
                    new_grid.set(
                        x,
                        y,
                        match old_grid.get_neighbours(x, y) {
                            2 | 3 => true,
                            _ => false,
                        },
                    );
                } else {
                    new_grid.set(
                        x,
                        y,
                        match old_grid.get_neighbours(x, y) {
                            3 => true,
                            _ => false,
                        },
                    );
                }
            }
        }

        new_grid
    }

    fn from_with_broken_lights(old_grid: &Grid) -> Self {
        let mut new_grid = Grid {
            lights: vec![false; old_grid.size * old_grid.size],
            size: old_grid.size,
        };

        new_grid.set(0, 0, true);
        new_grid.set(0, old_grid.size - 1, true);
        new_grid.set(old_grid.size - 1, 0, true);
        new_grid.set(old_grid.size - 1, old_grid.size - 1, true);

        for x in 0..=old_grid.size - 1 {
            for y in 0..=old_grid.size - 1 {
                if (x == 0 && y == 0)
                    || (x == 0 && y == old_grid.size - 1)
                    || (x == old_grid.size - 1 && y == 0)
                    || (x == old_grid.size - 1 && y == old_grid.size - 1)
                {
                    new_grid.set(x, y, true);
                } else if old_grid.get(x, y) {
                    new_grid.set(
                        x,
                        y,
                        match old_grid.get_neighbours(x, y) {
                            2 | 3 => true,
                            _ => false,
                        },
                    );
                } else {
                    new_grid.set(
                        x,
                        y,
                        match old_grid.get_neighbours(x, y) {
                            3 => true,
                            _ => false,
                        },
                    );
                }
            }
        }

        new_grid
    }

    fn get(&self, x: usize, y: usize) -> bool {
        if (0..self.size).contains(&x) || (0..self.size).contains(&y) {
            return self.lights[x + y * self.size];
        }

        panic!("Wrong size, {} {}", x, y);
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        if (0..self.size).contains(&x) || (0..self.size).contains(&y) {
            return self.lights[x + y * self.size] = value;
        }

        panic!("Wrong size, {} {}", x, y);
    }

    fn get_neighbours(&self, x: usize, y: usize) -> u8 {
        let neighbours = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        neighbours
            .iter()
            .map(|(d_x, d_y)| {
                if (0..self.size as i32).contains(&(x as i32 + d_x))
                    && (0..self.size as i32).contains(&(y as i32 + d_y))
                {
                    if self.get((x as i32 + d_x) as usize, (y as i32 + d_y) as usize) {
                        return 1;
                    }
                }
                0
            })
            .sum()
    }

    fn get_light_count(&self) -> usize {
        self.lights.iter().filter(|&&l| l).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let lights: Vec<bool> = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####.."
            .lines()
            .collect::<String>()
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => panic!("Unknown char!"),
            })
            .collect();

        let grid = Grid::new(&lights, 6);
        assert_eq!(grid.get_light_count(), 15);
    }

    #[test]
    fn test_value() {
        let lights: Vec<bool> = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####.."
            .lines()
            .collect::<String>()
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => panic!("Unknown char!"),
            })
            .collect();

        let mut grid = Grid::new(&lights, 6);

        for _ in 0..4 {
            grid = Grid::from(&grid);
        }

        assert_eq!(grid.get_light_count(), 4);
    }

    #[test]
    fn test_broken_value() {
        let lights: Vec<bool> = "##.#.#\n...##.\n#....#\n..#...\n#.#..#\n####.#"
            .lines()
            .collect::<String>()
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => panic!("Unknown char!"),
            })
            .collect();

        let mut grid = Grid::new(&lights, 6);

        let expected = [18, 18, 18, 14, 17];

        for i in 0..5 {
            grid = Grid::from_with_broken_lights(&grid);

            dbg!(&grid);

            assert_eq!(grid.get_light_count(), expected[i]);
        }

        dbg!(&grid.lights);
    }
}

// --- Day 18: Like a GIF For Your Yard ---

// After the million lights incident, the fire code has gotten stricter: now, at most ten thousand lights are allowed. You arrange them in a 100x100 grid.

// Never one to let you down, Santa again mails you instructions on the ideal lighting configuration. With so few lights, he says, you'll have to resort to animation.

// Start by setting your lights to the included initial configuration (your puzzle input). A # means "on", and a . means "off".

// Then, animate your grid in steps, where each step decides the next configuration based on the current one. Each light's next state (either on or off) depends on its current state and the current states of the eight lights adjacent to it (including diagonals). Lights on the edge of the grid might have fewer than eight neighbors; the missing ones always count as "off".

// For example, in a simplified 6x6 grid, the light marked A has the neighbors numbered 1 through 8, and the light marked B, which is on an edge, only has the neighbors marked 1 through 5:

// 1B5...
// 234...
// ......
// ..123.
// ..8A4.
// ..765.

// The state a light should have next is based on its current state (on or off) plus the number of neighbors that are on:

//     A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
//     A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.

// All of the lights update simultaneously; they all consider the same current state before moving to the next.

// Here's a few steps from an example configuration of another 6x6 grid:

// Initial state:
// .#.#.#
// ...##.
// #....#
// ..#...
// #.#..#
// ####..

// After 1 step:
// ..##..
// ..##.#
// ...##.
// ......
// #.....
// #.##..

// After 2 steps:
// ..###.
// ......
// ..###.
// ......
// .#....
// .#....

// After 3 steps:
// ...#..
// ......
// ...#..
// ..##..
// ......
// ......

// After 4 steps:
// ......
// ......
// ..##..
// ..##..
// ......
// ......

// After 4 steps, this example has four lights on.

// In your grid of 100x100 lights, given your initial configuration, how many lights are on after 100 steps?

// --- Part Two ---

// You flip the instructions over; Santa goes on to point out that this is all just an implementation of Conway's Game of Life. At least, it was, until you notice that something's wrong with the grid of lights you bought: four lights, one in each corner, are stuck on and can't be turned off. The example above will actually run like this:

// Initial state:
// ##.#.#
// ...##.
// #....#
// ..#...
// #.#..#
// ####.#

// After 1 step:
// #.##.#
// ####.#
// ...##.
// ......
// #...#.
// #.####

// After 2 steps:
// #..#.#
// #....#
// .#.##.
// ...##.
// .#..##
// ##.###

// After 3 steps:
// #...##
// ####.#
// ..##.#
// ......
// ##....
// ####.#

// After 4 steps:
// #.####
// #....#
// ...#..
// .##...
// #.....
// #.#..#

// After 5 steps:
// ##.###
// .##..#
// .##...
// .##...
// #.#...
// ##...#

// After 5 steps, this example now has 17 lights on.

// In your grid of 100x100 lights, given your initial configuration, but with the four corners always in the on state, how many lights are on after 100 steps?
