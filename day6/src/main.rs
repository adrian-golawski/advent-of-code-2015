use std::{ops::RangeInclusive};

fn main() -> anyhow::Result<()> {
    let s: Vec<Instruction> = include_str!("input.txt")
        .lines()
        .map(parse_instruction)
        .collect();

    
    let mut lights = Lights::new();
    let lights = s.iter().fold(&mut lights, |lights, instruction| {
        lights.switch_lights(instruction)
    });

    let mut analog_lights = AnalogLights::new();
    let analog_lights = s.iter().fold(&mut analog_lights, |lights, instruction| {
        lights.switch_lights(instruction)
    });

    println!("[Part one]: {}", lights.get_turned_on());
    println!("[Part two]: {}", analog_lights.get_brightness());

    Ok(())
}

fn parse_instruction(input: &str) -> Instruction {
    peg::parser! {
      grammar instruction_parser() for str {
        rule light() -> Light = f:$("toggle" / "turn off" / "turn on") {
            match f {
                "toggle" => Light::Toggle,
                "turn off" => Light::Off,
                "turn on" => Light::On,
                _ => panic!("Unexpected input")
            }
        }

        rule num() -> usize = s:$(['0'..='9']+) { s.parse().unwrap() }

        pub(crate) rule parse() -> Instruction
          = light:light() " " x_0:num() "," y_0:num() " through " x_1:num() "," y_1:num() {
              Instruction {
                  light,
                  x: (x_0..=x_1),
                  y: (y_0..=y_1)
              }
           }
      }
    }

    return instruction_parser::parse(input).unwrap();
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Light {
    On,
    Off,
    Toggle,
}

#[derive(PartialEq, Eq, Debug)]
struct Instruction {
    light: Light,
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
}

#[derive(Clone)]
struct Lights([bool; 1000 * 1000]);

impl Lights {
    fn new() -> Self {
        return Lights([false; 1000 * 1000]);
    }

    fn set_x_y(&mut self, x: usize, y: usize, value: bool) {
        self.0[x + y * 1000] = value;
    }

    fn toggle_x_y(&mut self, x: usize, y: usize) {
        self.0[x + y * 1000] = !self.0[x + y * 1000];
    }

    fn switch_lights(&mut self, instruction: &Instruction) -> &mut Lights {
        for x in instruction.x.clone() {
            for y in instruction.y.clone() {
                match instruction.light {
                    Light::Toggle => self.toggle_x_y(x, y),
                    Light::On => self.set_x_y(x, y, true),
                    Light::Off => self.set_x_y(x, y, false),
                }
            }
        }
        return self;
    }

    fn get_turned_on(&self) -> usize {
        self.0.iter().filter(|&x| *x).count()
    }
}

#[derive(Clone)]
struct AnalogLights([i16; 1000 * 1000]);

impl AnalogLights {
    fn new() -> Self {
        return AnalogLights([0; 1000 * 1000]);
    }

    fn set_x_y(&mut self, x: usize, y: usize, value: i16) {
        self.0[x + y * 1000] += value;

        if self.0[x + y * 1000] == -1 { self.0[x + y * 1000] = 0; }
    }

    fn switch_lights(&mut self, instruction: &Instruction) -> &mut AnalogLights {
        for x in instruction.x.clone() {
            for y in instruction.y.clone() {
                match instruction.light {
                    Light::Toggle => self.set_x_y(x, y, 2),
                    Light::On => self.set_x_y(x, y, 1),
                    Light::Off => self.set_x_y(x, y, -1),
                }
            }
        }
        return self;
    }

    fn get_brightness(&self) -> i32 {
        self.0.iter().map(|i| *i as i32).sum::<i32>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_switch_all() {
        let mut lights = Lights::new();
        let lights = lights.switch_lights(&Instruction {
            light: Light::Toggle,
            x: (0..=999),
            y: (0..=999),
        });

        for &x in lights.0.iter() {
            assert_eq!(x, true);
        }

        assert_eq!(lights.get_turned_on(), 1000000);

        lights.switch_lights(&Instruction {
            light: Light::Off,
            x: (0..=999),
            y: (0..=999),
        });

        for &x in lights.0.iter() {
            assert_eq!(x, false);
        }

        assert_eq!(lights.get_turned_on(), 0);

        lights.switch_lights(&Instruction {
            light: Light::On,
            x: (0..=999),
            y: (0..=999),
        });

        for &x in lights.0.iter() {
            assert_eq!(x, true);
        }

        assert_eq!(lights.get_turned_on(), 1000000);
    }

    #[test]
    fn test_light_switch_some() {
        let mut lights = Lights::new();

        let lights = lights.switch_lights(&Instruction {
            light: Light::On,
            x: (0..=999),
            y: (0..=0),
        });

        for x in 0..=999 {
            for y in 0..=999 {
                let light = lights.0[x + y * 1000];
                if y == 0 {
                    assert_eq!(light, true);
                } else {
                    assert_eq!(light, false);
                }
            }
        }

        assert_eq!(lights.get_turned_on(), 1000);
    }

    #[test]
    fn test_parser() {
        let tests = [
            (
                "turn on 606,361 through 892,600",
                Instruction {
                    light: Light::On,
                    x: (606..=892),
                    y: (361..=600),
                },
            ),
            (
                "turn off 448,208 through 645,684",
                Instruction {
                    light: Light::Off,
                    x: (448..=645),
                    y: (208..=684),
                },
            ),
            (
                "toggle 50,472 through 452,788",
                Instruction {
                    light: Light::Toggle,
                    x: (50..=452),
                    y: (472..=788),
                },
            ),
        ];

        for (input, expected) in tests.iter() {
            assert_eq!(&parse_instruction(input), expected);
        }
    }
}

// --- Day 6: Probably a Fire Hazard ---

// Because your neighbors keep defeating you in the holiday house decorating contest year after year,
// you've decided to deploy one million lights in a 1000x1000 grid.

// Furthermore, because you've been especially nice this year, Santa has mailed you instructions
// on how to display the ideal lighting configuration.

// Lights in your grid are numbered from 0 to 999 in each direction; the lights
// at each corner are at 0,0, 0,999, 999,999, and 999,0.
// The instructions include whether to turn on, turn off, or toggle various inclusive ranges given as coordinate pairs.
// Each coordinate pair represents opposite corners of a rectangle, inclusive;
// a coordinate pair like 0,0 through 2,2 therefore refers to 9 lights in a 3x3 square.
// The lights all start turned off.

// To defeat your neighbors this year, all you have to do is set up your lights by doing the instructions Santa sent you in order.

// For example:

//     turn on 0,0 through 999,999 would turn on (or leave on) every light.
//     toggle 0,0 through 999,0 would toggle the first line of 1000 lights,
//         turning off the ones that were on, and turning on the ones that were off.
//     turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights.

// After following the instructions, how many lights are lit?

// --- Part Two ---

// You just finish implementing your winning light pattern when you realize you mistranslated Santa's message from Ancient Nordic Elvish.

// The light grid you bought actually has individual brightness controls; each light can have a brightness of zero or more. The lights all start at zero.

// The phrase turn on actually means that you should increase the brightness of those lights by 1.

// The phrase turn off actually means that you should decrease the brightness of those lights by 1, to a minimum of zero.

// The phrase toggle actually means that you should increase the brightness of those lights by 2.

// What is the total brightness of all lights combined after following Santa's instructions?

// For example:

//     turn on 0,0 through 0,0 would increase the total brightness by 1.
//     toggle 0,0 through 999,999 would increase the total brightness by 2000000.

