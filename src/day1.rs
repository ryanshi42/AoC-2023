use std::collections::HashMap;

#[aoc(day1, part1)]
fn part1(_input: &str) -> i32 {
    // println!("Hello World!");
    // let v = bytes.chars().clone().filter(|c| "0123456789".contains(*c) ).collect::<Vec<char>>(); 
    // let n = v.len();
    // if n == 0 {
    //     panic!("help");
    // } else if n == 1 {
    //     x += 11 * v[0].to_digit(10).unwrap();
    // } else {
    //     x += 10 * v[0].to_digit(10).unwrap() + v[n-1].to_digit(10).unwrap();

    // }
    return 0;
}

#[aoc(day1, part2)]
fn part2(input: &str) -> i32 {
    let mut x = 0;
    for bytes in input.split('\n') {
        // println!("{}", line?);
        let mut first_best = -1;
        let mut first_idx = 1000000;
        let mut last_idx = 0;
        let mut last_best = -1;
        let mut ktv = HashMap::new();
        ktv.insert("1", 1);
        ktv.insert("one", 1);
        ktv.insert("2", 2);
        ktv.insert("two", 2);
        ktv.insert("3", 3);
        ktv.insert("three", 3);
        ktv.insert("4", 4);
        ktv.insert("four", 4);
        ktv.insert("5", 5);
        ktv.insert("five", 5);
        ktv.insert("6", 6);
        ktv.insert("six", 6);
        ktv.insert("7", 7);
        ktv.insert("seven", 7);
        ktv.insert("8", 8);
        ktv.insert("eight", 8);
        ktv.insert("9", 9);
        ktv.insert("nine", 9);
        for i in ["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"] {

            let v: Vec<_> = bytes.match_indices(i).collect();
            
            let n = v.len();
            if n == 0 {
                ();
                // println!("{:?}", bytes);
                // assert!(false);
            } else if n >= 1 {
                let first = v.first().unwrap();
                if first.0 <= first_idx {
                    first_idx = first.0;
                    first_best = *ktv.get(first.1).unwrap();
                }
                let last = v.last().unwrap();
                if last.0 >= last_idx {
                    last_idx = last.0;
                    last_best = *ktv.get(last.1).unwrap();
                }
            } 

        }
        assert!(first_best != -1);
        assert!(last_best != -1);
        x += 10 * first_best + last_best;
    }
    println!("{}", x);
    return x;
}