use std::{cmp::Ordering, collections::{HashMap}};

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum BaseHandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    High
}

fn get_type(a: (&str, &str)) -> BaseHandType {
    let mut h = a.0.chars().collect::<Vec<_>>();
    h.sort();

    let v = h.clone();
    // print!("{a:?}, {v:?}");
    let mut hs = HashMap::new();
    
    let mut js = 0;
    for c in h.clone() {
        if c == 'J' {
            js += 1;
        }
        *hs.entry(c).or_insert(0) += 1;
    }

    if h.iter().all(|x| *x == v[0]) {
        return BaseHandType::Five;
    } else if h.iter().skip(1).all(|x| *x == v[1]) {
        if js > 0 {
            return BaseHandType::Five;
        }
        return BaseHandType::Four;
    } else if h.iter().take(4).all(|x| *x == v[0]) {
        if js > 0 {
            return BaseHandType::Five;
        }
        return BaseHandType::Four;
    } else if h.clone().iter().take(3).all(|x| *x == v[0]) && h.clone().iter().skip(3).all(|x| *x == v[3]) {
        if js > 0 {
            return BaseHandType::Five;
        }
        return BaseHandType::FullHouse;
    } else if h.clone().iter().take(2).all(|x| *x == v[0]) && h.clone().iter().skip(2).all(|x| *x == v[2]) {
        if js > 0 {
            return BaseHandType::Five;
        }
        return BaseHandType::FullHouse;
    } else if h.iter().take(3).all(|x| *x == v[0]) {
        if js > 0 {
            return BaseHandType::Four;
        }
        return BaseHandType::Three;
    } else if h.iter().skip(1).take(3).all(|x| *x == v[1]) {
        if js > 0 {
            return BaseHandType::Four;
        }
        return BaseHandType::Three;
    } else if h.iter().skip(2).take(3).all(|x| *x == v[2]) {
        if js > 0 {
            return BaseHandType::Four;
        }
        return BaseHandType::Three;
    } else {
        let mut res = vec![];
        for (s, t) in hs {
            if t == 2 {
                res.push(s);
            } 
        }
        if res.len() == 0 {
            if js == 0 {
                return BaseHandType::High; 
            }
            else {
                return BaseHandType::OnePair;
            }
        } if res.len() == 1 {
            if js > 0 {
                return BaseHandType::Three; 
            }
            else {
                return BaseHandType::OnePair;
            }
        } if res.len() == 2 {
            if js == 2 {
                return BaseHandType::Four; 
            } else if js == 1 {
                return BaseHandType::FullHouse;
            } else {
                return BaseHandType::TwoPair;
            }
        }
        panic!("hello");
    }

}

fn compare_char(a: char, b: char) -> std::cmp::Ordering {
    // let x = "AKQJT98765432";
    let x = "AKQT98765432J";
    if x.find(a) < x.find(b) {
        Ordering::Greater
    } else if x.find(a) > x.find(b) {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn compare_fn(a: (&str, &str), b: (&str, &str)) -> std::cmp::Ordering {
    let ta = get_type(a); 
    let tb = get_type(b); 

    // println!("{a:?} {b:?} {:?} {:?}", ta, tb);

    if ta < tb {
        Ordering::Less
    } else if ta > tb {
        Ordering::Greater
    } else {
        for i in 0..5 {
            if compare_char(a.0.chars().nth(i).unwrap(), b.0.chars().nth(i).unwrap()) == Ordering::Less {
                return Ordering::Greater;
            } 
            if compare_char(a.0.chars().nth(i).unwrap(), b.0.chars().nth(i).unwrap()) == Ordering::Greater {
                return Ordering::Less;
            } 
        }
        panic!("same ???");
    }
}


#[aoc(day7, part1)]
fn part1(input: &str) -> i32 {
    // better: lines, map (into hand type), sorted, enumerate, map, sum
    let mut si = input
        .split("\n")
        .map(|x| x.split_once(" ").unwrap())
        .collect::<Vec<_>>();
    si.sort_by(|a, b| compare_fn(*a, *b));
    let f = si.iter().zip((1..=si.len()).rev());

    println!("{f:?}, {:?}", f.size_hint());
    f.into_iter()
        .map(|(x, y)| (x.1.parse::<usize>().unwrap() * y) as i32)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> i32 {
    let mut si = input
        .split("\n")
        .map(|x| x.split_once(" ").unwrap())
        .collect::<Vec<_>>();
    si.sort_by(|a, b| compare_fn(*a, *b));
    let f = si.iter().zip((1..=si.len()).rev());

    println!("{f:?}, {:?}", f.size_hint());
    f.into_iter()
        .map(|(x, y)| (x.1.parse::<usize>().unwrap() * y) as i32)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day7::part1;

    #[test]
    fn it_works() {
        let x = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part1(x), 5905);
    }
}