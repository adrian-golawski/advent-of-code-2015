use std::vec;

fn main() {

    println!("{}", count_presents(36000000));
    println!("{}", count_presents_2(36000000));
}

fn count_presents(n: usize) -> usize {
    let mut houses = vec![0; 1 + n/10];

    for i in 1..=houses.len() {
        for j in (i..=houses.len()).step_by(i) {
            if let Some(value) = houses.get_mut(j) {
                *value += i * 10;
            }
        }
    }

    houses.iter().position(|&presents| presents >= n).unwrap()
}

fn count_presents_2(n: usize) -> usize {
    let mut houses = vec![0; 1 + n/10];

    for i in 1..=houses.len() {
        for j in (i..).step_by(i).take(50) {
            if let Some(value) = houses.get_mut(j) {
                *value += i * 11;
            }
        }
    }

    houses.iter().position(|&presents| presents >= n).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(count_presents(10), 1);
        assert_eq!(count_presents(20), 2);
        assert_eq!(count_presents(35), 3);
        assert_eq!(count_presents(100), 6);
        assert_eq!(count_presents(150), 8);
    }
}


// --- Day 20: Infinite Elves and Infinite Houses ---

// To keep the Elves busy, Santa has them deliver some presents by hand, door-to-door. He sends them down a street with infinite houses numbered sequentially: 1, 2, 3, 4, 5, and so on.

// Each Elf is assigned a number, too, and delivers presents to houses based on that number:

//     The first Elf (number 1) delivers presents to every house: 1, 2, 3, 4, 5, ....
//     The second Elf (number 2) delivers presents to every second house: 2, 4, 6, 8, 10, ....
//     Elf number 3 delivers presents to every third house: 3, 6, 9, 12, 15, ....

// There are infinitely many Elves, numbered starting with 1. Each Elf delivers presents equal to ten times his or her number at each house.

// So, the first nine houses on the street end up like this:

// House 1 got 10 presents.
// House 2 got 30 presents.
// House 3 got 40 presents.
// House 4 got 70 presents.
// House 5 got 60 presents.
// House 6 got 120 presents.
// House 7 got 80 presents.
// House 8 got 150 presents.
// House 9 got 130 presents.

// The first house gets 10 presents: it is visited only by Elf 1, which delivers 1 * 10 = 10 presents. The fourth house gets 70 presents, because it is visited by Elves 1, 2, and 4, for a total of 10 + 20 + 40 = 70 presents.

// What is the lowest house number of the house to get at least as many presents as the number in your puzzle input?

// Your puzzle input is 36000000.
 
// --- Part Two ---

// The Elves decide they don't want to visit an infinite number of houses. Instead, each Elf will stop after delivering presents to 50 houses. To make up for it, they decide to deliver presents equal to eleven times their number at each house.

// With these changes, what is the new lowest house number of the house to get at least as many presents as the number in your puzzle input?
