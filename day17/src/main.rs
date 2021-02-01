use itertools::Itertools;

fn main() {
    let containers: [u32; 20] = [
        50, 44, 11, 49, 42, 46, 18, 32, 26, 40, 21, 7, 18, 43, 10, 47, 36, 24, 22, 40,
    ];

    let mut sums = 0;
    let mut minimum_size: Option<usize> = None;
    let mut minimum_size_ways = 0;

    for i in 0..containers.len() {
        for perm in containers.iter().combinations(i) {
            if perm.iter().map(|&&n| n).sum::<u32>() == 150 {
                match minimum_size {
                    None => {
                        minimum_size = Some(i);
                        minimum_size_ways += 1;
                    }
                    Some(size) => {
                        if i == size {
                            minimum_size_ways += 1;
                        }
                    }
                }
                sums += 1;
            }
        }
    }

    println!("[Part one]: {}", sums);
    println!("[Part two]: {}", minimum_size_ways);
}

// --- Day 17: No Such Thing as Too Much ---

// The elves bought too much eggnog again - 150 liters this time. To fit it all into your refrigerator, you'll need to move it into smaller containers. You take an inventory of the capacities of the available containers.

// For example, suppose you have containers of size 20, 15, 10, 5, and 5 liters. If you need to store 25 liters, there are four ways to do it:

//     15 and 10
//     20 and 5 (the first 5)
//     20 and 5 (the second 5)
//     15, 5, and 5

// Filling all containers entirely, how many different combinations of containers can exactly fit all 150 liters of eggnog?

// 20, 15, 10, 5, and 5

// --- Part Two ---

// While playing with all the containers in the kitchen, another load of eggnog arrives! The shipping and receiving department is requesting as many containers as you can spare.

// Find the minimum number of containers that can exactly fit all 150 liters of eggnog. How many different ways can you fill that number of containers and still hold exactly 150 litres?

// In the example above, the minimum number of containers was two. There were three ways to use that many containers, and so the answer there would be 3.
