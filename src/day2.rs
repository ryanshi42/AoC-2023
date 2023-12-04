
use std::cmp::max;
use std::collections::HashMap;
#[aoc(day2, part1)]
fn part1(input: &str) -> i32 {
    let mut ans = 0;
    for line in input.split("\n") {
        let colon = line.find(":").unwrap();
        let id: i32 = line[5..colon].parse().unwrap();
        let mut maybe = true;
        for game in line[colon + 2..].split("; ") {
            for colours in game.split(", ") {
                let mut v = colours.split(" ");
                // println!("{v:?}");
                // println!("{colours:?}");
                let cnt : i32 = v.next().unwrap().parse().unwrap();
                let col = v.next().unwrap();
                if col == "red" && cnt > 12 {
                    maybe = false;
                    break;
                } 
                if col == "green" && cnt > 13 {
                    maybe = false;
                    break;
                } 
                if col == "blue" && cnt > 14 {
                    maybe = false;
                    break;
                } 

            }
            if !maybe {
                break;
            }
        } 
        if maybe {
            ans += id;
        }
    }
    return ans;
}

#[aoc(day2, part2)]
fn part2(input: &str) -> i32 {
    let mut ans = 0;
    for line in input.split("\n") {
        let colon = line.find(":").unwrap();
        let id: i32 = line[5..colon].parse().unwrap();

        let mut chm : HashMap<&str, i32> = HashMap::new();
        for game in line[colon + 2..].split("; ") {
            for colours in game.split(", ") {
                let mut v = colours.split(" ");
                // println!("{v:?}");
                // println!("{colours:?}");
                let cnt : i32 = v.next().unwrap().parse().unwrap();
                let col = v.next().unwrap();
                chm.entry(col).or_insert(cnt);
                chm.insert(col, max(chm[col], cnt));

            }
        } 
        ans += *chm.get("red").unwrap() * *chm.get("blue").unwrap() * *chm.get("green").unwrap();
    }
    return ans;
}