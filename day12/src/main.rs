use json::JsonValue;
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    // let input = "{\"a\": [1, 2, 3]}";

    println!("[Part one]: {}", number_sum(input.to_string()));
    println!("[Part two]: {}", json_sum(input.to_string()));
}

fn number_sum(input: String) -> i64 {
    let re = Regex::new(r"(?:-)?\d+").unwrap();
    re.captures_iter(&input)
        .map(|cap| cap[0].parse::<i64>().unwrap())
        .sum()
}

fn json_sum(input:String) -> i64 {
    let parsed = json::parse(&input).unwrap();
    json_collect(&parsed)
}

fn json_collect(js: &JsonValue) -> i64 {
    if js.is_number() {

        let num: i64 = js.as_number().unwrap().as_fixed_point_i64(0).unwrap();
        return num;
    }

    if js.is_array() {
        return js.members().fold(0, |sum, value| {
            return sum + json_collect(value);
        });
    }

    if js.is_object() {
        if js.entries().any(|entry| entry.0 == "red" || (entry.1.is_string() && entry.1.as_str().unwrap() == "red")) {
            return 0;
        }

        return js.entries().map(|entry| json_collect(entry.1)).sum::<i64>();
    }

    return 0;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_number_sum() {
        assert_eq!(number_sum("[1,2,3]".to_string()), 6);
        assert_eq!(number_sum("{\"a\":2,\"b\":4}".to_string()), 6);
        assert_eq!(number_sum("[[[3]]]".to_string()), 3);
        assert_eq!(number_sum("{\"a\":{\"b\":4},\"c\":-1}".to_string()), 3);
        assert_eq!(number_sum("{\"a\":[-1,1]}".to_string()), 0);
        assert_eq!(number_sum("[-1,{\"a\":1}]".to_string()), 0);
        assert_eq!(number_sum("{}".to_string()), 0);
        assert_eq!(number_sum("[]".to_string()), 0);
    }

    #[test]
    fn test_parse_json() {
        assert_eq!(json_collect(&json::parse("{\"a\":2,\"b\":4}").unwrap()), 6);
        assert_eq!(json_collect(&json::parse("[1,{\"c\":\"red\",\"b\":2},3]").unwrap()), 4);
        assert_eq!(json_collect(&json::parse("[1,\"red\",3]").unwrap()), 4);
        assert_eq!(json_collect(&json::parse("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}").unwrap()), 0);
    }
}

// --- Day 12: JSAbacusFramework.io ---

// Santa's Accounting-Elves need help balancing the books after a recent order. Unfortunately, their accounting software uses a peculiar storage format. That's where you come in.

// They have a JSON document which contains a variety of things: arrays ([1,2,3]), objects ({"a":1, "b":2}), numbers, and strings. Your first job is to simply find all of the numbers throughout the document and add them together.

// For example:

//     [1,2,3] and {"a":2,"b":4} both have a sum of 6.
//     [[[3]]] and {"a":{"b":4},"c":-1} both have a sum of 3.
//     {"a":[-1,1]} and [-1,{"a":1}] both have a sum of 0.
//     [] and {} both have a sum of 0.

// You will not encounter any strings containing numbers.

// What is the sum of all numbers in the document?

//Uh oh - the Accounting-Elves have realized that they double-counted everything red.

// Ignore any object (and all of its children) which has any property with the value "red". Do this only for objects ({...}), not arrays ([...]).

// [1,2,3] still has a sum of 6.
// [1,{"c":"red","b":2},3] now has a sum of 4, because the middle object is ignored.
// {"d":"red","e":[1,2,3,4],"f":5} now has a sum of 0, because the entire structure is ignored.
// [1,"red",5] has a sum of 6, because "red" in an array has no effect.
