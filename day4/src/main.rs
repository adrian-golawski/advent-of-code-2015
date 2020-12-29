fn main() {
    let s = "bgvyzdsv";

    println!("[Part one]: {}", hash(s, false));
    println!("[Part two]: {}", hash(s, true));
}

fn hash(seed: &str, harder: bool) -> i32 {
    for i in 0.. {
        let test_string = [String::from(seed), i.to_string()].join("");
        let hash = md5::compute(test_string);

        if hash[0] == 0 && hash[1] == 0 && hash[2] <= if harder { 0 } else { 15 } {
            return i;
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("abcdef", false), 609043);
        assert_eq!(hash("pqrstuv", false), 1048970);

        assert_eq!(hash("abcdef", true), 6742839);
        assert_eq!(hash("pqrstuv", true), 5714438);
    }

}

// --- Day 4: The Ideal Stocking Stuffer ---

// Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts
//  for all the economically forward-thinking little girls and boys.

// To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes. 
// The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a number in decimal. 
// To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.

// For example:

//     If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts with five zeroes (000001dbbfa...), 
//     and it is the lowest such number to do so.
//     If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting with five zeroes is 1048970; 
//     that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....

// Your puzzle input is bgvyzdsv.