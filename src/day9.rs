
fn extrapolate(numbers: &str) -> i32 {
    let v: Vec<i32> = numbers.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
    
    let mut curr = v;
    let mut vs = vec![];
    while !curr.iter().all(|x| *x == 0) {
        vs.push(curr.clone());
        let new = curr.iter().skip(1).enumerate().map(|(i, elem)| elem - curr[i]).collect(); 
        curr = new;
    }
    println!("{vs:?}");
    let l = vs.len();
    let mut guess = 0;

    for i in (0..l).rev() {
        guess = vs[i].last().unwrap() + guess;
    }
    println!("{guess}");
    return guess;
}


fn extrapolate2(numbers: &str) -> i32 {
    let v: Vec<i32> = numbers.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
    
    let mut curr = v;
    let mut vs = vec![];
    while !curr.iter().all(|x| *x == 0) {
        vs.push(curr.clone());
        let new = curr.iter().skip(1).enumerate().map(|(i, elem)| elem - curr[i]).collect(); 
        curr = new;
    }
    println!("{vs:?}");
    let l = vs.len();
    let mut guess = 0;

    for i in (0..l).rev() {
        guess = vs[i].first().unwrap() - guess;
    }
    println!("{guess}");
    return guess;
}

#[aoc(day9, part1)]
fn part1(input: &str) -> i32 {
    // let mut ans = 0;
    input.lines()
        .map(|x| extrapolate(x))
        .sum()


}

#[aoc(day9, part2)]
fn part2(input: &str) -> i32 {
    input.lines()
        .map(|x| extrapolate2(x))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day9::{part1, part2};

    #[test]
    fn it_works() {
        let c = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part1(c), 114);

        let c = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part2(c), 2);
    }
}