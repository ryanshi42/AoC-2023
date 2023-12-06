
#[aoc(day6, part1)]
fn part1(input: &str) -> i32 {
    let (times, distances) = input.split_once("\n").unwrap();
    let times = times.split_whitespace().skip(1);
    let mut distances = distances.split_whitespace().skip(1);
    // println!("{times:?} {distances:?}");

    let mut res = 1;

    for t in times {
        let d = distances.next().unwrap();
        let mut x = 0;
        for i in 1..=t.parse().unwrap() {
            if (t.parse::<i32>().unwrap() - i) * i >= d.parse().unwrap() {
                x += 1;
            } 
        }
        res *= x;

    }

    return res;

}

#[aoc(day6, part2)]
fn part2(input: &str) -> i32 {
    let (times, distances) = input.split_once("\n").unwrap();
    let times = times.split_whitespace().skip(1);
    let distances = distances.split_whitespace().skip(1);
    let times = times.collect::<Vec<_>>().join("");
    let distances = distances.collect::<Vec<_>>().join("");
    // println!("{times:?} {distances:?}");

    let d:i64 = distances.parse().unwrap();
    let mut x = 0;
    for i in 1..=times.parse().unwrap() {
        if (times.parse::<i64>().unwrap() - i) * i >= d {
            x += 1;
        } 
    }

    return x;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let c = 1;
        assert_eq!(c, 1);
    }
}