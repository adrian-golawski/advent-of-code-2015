fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(parse_ingredient)
        .collect::<Vec<Ingredient>>();

    let all_combinations_summing_to_100 = get_all_combinations_summing_to_n(100);
    let max_score = all_combinations_summing_to_100
        .iter()
        .map(|multipliers: &[i32; 4]| {
            let ingredients = [
                (*input.get(0).unwrap(), multipliers[0]),
                (*input.get(1).unwrap(), multipliers[1]),
                (*input.get(2).unwrap(), multipliers[2]),
                (*input.get(3).unwrap(), multipliers[3]),
            ];
            calculate_score(&ingredients)
        })
        .max()
        .unwrap();

    println!("[Part one]: {}", max_score);

    let max_score_with_500_calories = all_combinations_summing_to_100
        .iter()
        .filter(|multipliers| {
            let ingredients = [
                (*input.get(0).unwrap(), multipliers[0]),
                (*input.get(1).unwrap(), multipliers[1]),
                (*input.get(2).unwrap(), multipliers[2]),
                (*input.get(3).unwrap(), multipliers[3]),
            ];
            calculate_calories(&ingredients) == 500
        })
        .map(|multipliers| {
            let ingredients = [
                (*input.get(0).unwrap(), multipliers[0]),
                (*input.get(1).unwrap(), multipliers[1]),
                (*input.get(2).unwrap(), multipliers[2]),
                (*input.get(3).unwrap(), multipliers[3]),
            ];
            calculate_score(&ingredients)
        })
        .max()
        .unwrap();

    println!("[Part two]: {}", max_score_with_500_calories);
}

fn get_all_combinations_summing_to_n(n: i32) -> Vec<[i32; 4]> {
    let mut sums = Vec::new();

    // God, have mercy on all of us
    for a in 0..=n {
        for b in 0..=n {
            for c in 0..=n {
                for d in 0..=n {
                    if (a + b + c + d) == n {
                        sums.push([a, b, c, d]);
                    }
                }
            }
        }
    }

    return sums;
}

fn calculate_calories(ingredients: &[(Ingredient, i32)]) -> i32 {
    ingredients.iter().fold(0, |sum, (ingredient, amount)| {
        return sum + ingredient.calories * amount;
    })
}

fn calculate_score(ingredients: &[(Ingredient, i32)]) -> i32 {
    let capacity = ingredients.iter().fold(0, |sum, (ingredient, amount)| {
        return sum + ingredient.capacity * amount;
    });
    let durability = ingredients.iter().fold(0, |sum, (ingredient, amount)| {
        return sum + ingredient.durability * amount;
    });
    let flavor = ingredients.iter().fold(0, |sum, (ingredient, amount)| {
        return sum + ingredient.flavor * amount;
    });
    let texture = ingredients.iter().fold(0, |sum, (ingredient, amount)| {
        return sum + ingredient.texture * amount;
    });

    if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
        return 0;
    }

    capacity * durability * flavor * texture
}

fn parse_ingredient<'a>(input: &'a str) -> Ingredient<'a> {
    peg::parser! {
        grammar ingredient_parser() for str {
            rule name() -> &'input str = n:$(['a'..='z' | 'A'..='Z']+) {
                n
            }

            rule num() -> i32 = n:$(['-']*['0'..='9']+) { n.parse().unwrap() }

            pub rule parse() -> Ingredient<'input> = name:name() ": capacity " capacity:num() ", durability " durability:num() ", flavor " flavor:num() ", texture " texture:num() ", calories " calories:num() {
                Ingredient {
                    name,
                    capacity,
                    durability,
                    flavor,
                    texture,
                    calories
                }
            }
        }
    }

    ingredient_parser::parse(input).unwrap()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ingredient<'a> {
    name: &'a str,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_ingredient(
                "Frosting: capacity 4, durability -2, flavor 0, texture 0, calories 5"
            ),
            Ingredient {
                name: "Frosting",
                capacity: 4,
                durability: -2,
                flavor: 0,
                texture: 0,
                calories: 5
            }
        );
    }

    #[test]
    fn test_calculate_calories() {
        assert_eq!(
            calculate_calories(&[
                (
                    Ingredient {
                        name: "Butterscotch",
                        capacity: -1,
                        durability: -2,
                        flavor: 6,
                        texture: 3,
                        calories: 8
                    },
                    40
                ),
                (
                    Ingredient {
                        name: "Cinammon",
                        capacity: 2,
                        durability: 3,
                        flavor: -2,
                        texture: -1,
                        calories: 3
                    },
                    60
                )
            ]),
            500
        );
    }

    #[test]
    fn test_calculate_score() {
        assert_eq!(
            calculate_score(&[
                (
                    Ingredient {
                        name: "Butterscotch",
                        capacity: -1,
                        durability: -2,
                        flavor: 6,
                        texture: 3,
                        calories: 8
                    },
                    44
                ),
                (
                    Ingredient {
                        name: "Cinammon",
                        capacity: 2,
                        durability: 3,
                        flavor: -2,
                        texture: -1,
                        calories: 3
                    },
                    56
                )
            ]),
            62842880
        );

        assert_eq!(
            calculate_score(&[
                (
                    Ingredient {
                        name: "Frosting",
                        capacity: 4,
                        durability: -2,
                        flavor: 0,
                        texture: 0,
                        calories: 5,
                    },
                    49,
                ),
                (
                    Ingredient {
                        name: "Candy",
                        capacity: 0,
                        durability: 5,
                        flavor: -1,
                        texture: 0,
                        calories: 8,
                    },
                    1,
                ),
                (
                    Ingredient {
                        name: "Butterscotch",
                        capacity: -1,
                        durability: 0,
                        flavor: 5,
                        texture: 0,
                        calories: 6,
                    },
                    1,
                ),
                (
                    Ingredient {
                        name: "Sugar",
                        capacity: 0,
                        durability: 0,
                        flavor: -2,
                        texture: 2,
                        calories: 1,
                    },
                    49,
                )
            ]),
            0
        );
    }

    #[test]
    fn test_get_sums() {
        assert_eq!(
            get_all_combinations_summing_to_n(4),
            vec!(
                [0, 0, 0, 4],
                [0, 0, 1, 3],
                [0, 0, 2, 2],
                [0, 0, 3, 1],
                [0, 0, 4, 0],
                [0, 1, 0, 3],
                [0, 1, 1, 2],
                [0, 1, 2, 1],
                [0, 1, 3, 0],
                [0, 2, 0, 2],
                [0, 2, 1, 1],
                [0, 2, 2, 0],
                [0, 3, 0, 1],
                [0, 3, 1, 0],
                [0, 4, 0, 0],
                [1, 0, 0, 3],
                [1, 0, 1, 2],
                [1, 0, 2, 1],
                [1, 0, 3, 0],
                [1, 1, 0, 2],
                [1, 1, 1, 1],
                [1, 1, 2, 0],
                [1, 2, 0, 1],
                [1, 2, 1, 0],
                [1, 3, 0, 0],
                [2, 0, 0, 2],
                [2, 0, 1, 1],
                [2, 0, 2, 0],
                [2, 1, 0, 1],
                [2, 1, 1, 0],
                [2, 2, 0, 0],
                [3, 0, 0, 1],
                [3, 0, 1, 0],
                [3, 1, 0, 0],
                [4, 0, 0, 0]
            )
        );

        assert_eq!(get_all_combinations_summing_to_n(100).len(), 176851);
    }
}

// --- Day 15: Science for Hungry People ---

// Today, you set out on the task of perfecting your milk-dunking cookie recipe. All you have to do is find the right balance of ingredients.

// Your recipe leaves room for exactly 100 teaspoons of ingredients. You make a list of the remaining ingredients you could use to finish the recipe (your puzzle input) and their properties per teaspoon:

//     capacity (how well it helps the cookie absorb milk)
//     durability (how well it keeps the cookie intact when full of milk)
//     flavor (how tasty it makes the cookie)
//     texture (how it improves the feel of the cookie)
//     calories (how many calories it adds to the cookie)

// You can only measure ingredients in whole-teaspoon amounts accurately, and you have to be accurate so you can reproduce your results in the future. The total score of a cookie can be found by adding up each of the properties (negative totals become 0) and then multiplying together everything except calories.

// For instance, suppose you have these two ingredients:

// Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
// Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3

// Then, choosing to use 44 teaspoons of butterscotch and 56 teaspoons of cinnamon (because the amounts of each ingredient must add up to 100) would result in a cookie with the following properties:

//     A capacity of 44*-1 + 56*2 = 68
//     A durability of 44*-2 + 56*3 = 80
//     A flavor of 44*6 + 56*-2 = 152
//     A texture of 44*3 + 56*-1 = 76

// Multiplying these together (68 * 80 * 152 * 76, ignoring calories for now) results in a total score of 62842880, which happens to be the best score possible given these ingredients. If any properties had produced a negative total, it would have instead become zero, causing the whole score to multiply to zero.

// Given the ingredients in your kitchen and their properties, what is the total score of the highest-scoring cookie you can make?

// --- Part Two ---

// Your cookie recipe becomes wildly popular! Someone asks if you can make another recipe that has exactly 500 calories per cookie (so they can use it as a meal replacement). Keep the rest of your award-winning process the same (100 teaspoons, same ingredients, same scoring system).

// For example, given the ingredients above, if you had instead selected 40 teaspoons of butterscotch and 60 teaspoons of cinnamon (which still adds to 100), the total calorie count would be 40*8 + 60*3 = 500. The total score would go down, though: only 57600000, the best you can do in such trying circumstances.

// Given the ingredients in your kitchen and their properties, what is the total score of the highest-scoring cookie you can make with a calorie total of 500?
