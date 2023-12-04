use std::collections::{HashMap, HashSet};

#[aoc(day4, part1)]
fn part1(input: &str) -> i32 {

    let mut ans = 0;
    for line in input.split("\n") {
        let line = line.replace("  ", " ");
        let colon = line.find(":").unwrap();
        // let id: i32 = line[5..colon].parse().unwrap();
        let mut x = line[colon + 2..].split(" | ");
        let y = x.next().unwrap();
        let z = x.next().unwrap();
        // println!("{}, {}", y.replace("  ", " "), z.replace("  ", " "));
        let lhs = y.replace("  ", " ").split(" ").map(|x| x.parse().unwrap()).collect::<Vec<u32>>();
        let rhs = z.replace("  ", " ").split(" ").map(|x| x.parse().unwrap()).collect::<Vec<u32>>();

        let lhss: HashSet<u32> = HashSet::from_iter(lhs);
        let rhss: HashSet<u32> = HashSet::from_iter(rhs);
        let comb = lhss.intersection(&rhss);
        let l = comb.count();

        if l == 0 {
            continue;
        } else {
            ans += usize::pow(2, (l - 1 as usize).try_into().unwrap());
        }
    }
    return ans.try_into().unwrap();
}

#[aoc(day4, part2)]
fn part2(input: &str) -> i32 {
    let mut ans = 0;
    const N : usize = 203;
    let mut v = [1; N];
    for line in input.split("\n") {
        // LOL super hacky method
        let mut line = line.replace("  ", " ");
        line = line.replace("  ", " ");
        line = line.replace("  ", " ");
        let colon = line.find(":").unwrap();
        let id: usize = line[5..colon].parse().unwrap();
        let mut x = line[colon + 2..].split(" | ");
        let y = x.next().unwrap();
        let z = x.next().unwrap();
        // println!("{}, {}", y.replace("  ", " "), z.replace("  ", " "));
        let lhs = y.replace("  ", " ").split(" ").map(|x| x.parse().unwrap()).collect::<Vec<u32>>();
        let rhs = z.replace("  ", " ").split(" ").map(|x| x.parse().unwrap()).collect::<Vec<u32>>();

        let lhss: HashSet<u32> = HashSet::from_iter(lhs);
        let rhss: HashSet<u32> = HashSet::from_iter(rhs);
        let comb = lhss.intersection(&rhss);
        let l = comb.count();

        if l == 0 {
            ans += v[(id - 1) as usize];
        } else {
            ans += v[(id - 1) as usize];
            for j in id..=id + l - 1 {
                if j >= N {
                    continue;
                }
                v[j] += v[(id - 1) as usize];
            }
        }
    }
    return v.iter().sum::<usize>() as i32;
}