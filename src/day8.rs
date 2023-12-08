use num;
use std::{collections::{HashMap}};



#[aoc(day8, part1)]
fn part1(input: &str) -> i32 {
    let (path, gs) = input.split_once("\n\n").unwrap();

    let mut lhm: HashMap<&str, &str> = HashMap::new();
    let mut rhm: HashMap<&str, &str> = HashMap::new();

    for ent in gs.lines() {
        let (name, tup) = ent.split_once(" = ").unwrap();
        let (l, r) = tup[1..tup.len() - 1].split_once(", ").unwrap();
        lhm.entry(name).or_insert(l);
        rhm.entry(name).or_insert(r); 
    }

    let mut curr = "AAA";
    let mut ans = 0;

    loop {
        for c in path.chars() {
            if c == 'L' {
                curr = lhm[curr];
            } if c == 'R' {
                curr = rhm[curr];
            }
            ans += 1;
            if curr == "ZZZ" {
                return ans;
            }
        }
    }

}

#[aoc(day8, part2)]
fn part2(input: &str) -> i32 {
    let (path, gs) = input.split_once("\n\n").unwrap();

    let mut lhm: HashMap<&str, &str> = HashMap::new();
    let mut rhm: HashMap<&str, &str> = HashMap::new();

    let mut a = vec![];

    for ent in gs.lines() {
        let (name, tup) = ent.split_once(" = ").unwrap();
        if name.chars().nth(2) == Some('A') {
            a.push(name);
        }
        let (l, r) = tup[1..tup.len() - 1].split_once(", ").unwrap();
        lhm.entry(name).or_insert(l);
        rhm.entry(name).or_insert(r); 
    }


    // a = vec!["AAA"];
    // println!("{a:?}");
    let mut cool = vec![];

    // CYCLE DETECTION CHECK
    // Period is the same as offset... need to do some maths.
    for start in a.clone() {
        let mut new = start;
        let mut temp = vec![];
        let mut ans = 0;
        let mut found_cycle = false;
        let mut seen = HashMap::new();
        loop {

            for (i, c) in path.chars().enumerate() {
                if c == 'L' {
                    new = lhm[new];
                } if c == 'R' {
                    new = rhm[new];
                }
                ans += 1;
                // println!("{new}, {curr}");
                if new.chars().nth(2) == Some('Z') {
                    temp.push((new, ans, c, i));
                }
                // if curr.chars().nth(2) == Some('Z') {
                //     x -= 1;
                // }
                // if x == a.len() {
                //     return ans;
                // }
                // println!("{new:?}");

                if seen.get(&(new, c, i)).is_some() {
                    cool.push(temp.clone());
                    println!("{ans}, {temp:?}, {new}, {c}, {i}");
                    let z = seen.get(&(new, c, i)).unwrap();
                    println!("Cycle starts at : {}, periodicity at : {}", z, ans - z);
                    found_cycle = true;
                    break;
                }  
                seen.insert((new, c, i), ans);

                // 1 2 |----- HBZ-|----- HBZ-|
                
                
            }
            if found_cycle {
                break;
            }
        }

    }
    println!("cool is {cool:?}");

    // WARNING: The following code overflows or some crap... use a manual calculator
    println!("{}", cool.iter().map(|x| x[0].1).reduce(num::integer::lcm).unwrap());
    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let c = 1;
        assert_eq!(c, 1);
    }
}