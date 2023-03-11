fn main() {
    let mut code: u64 = 20151125;
    
    for i in 1..decifer_coordinates((2947, 3029)) {
        code = code * 252533 % 33554393;
    }
    
    dbg!(code);
}

fn decifer_coordinates((column, row): (u64, u64)) -> u64 {
    let mut result = 1;
    
    for x in 1..column {
        result += x
    }

    for y in 1..row {
        result += y + column
    }

    result
}
