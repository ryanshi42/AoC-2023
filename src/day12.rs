use std::collections::HashMap;


fn satisfy(spring: &str, po: Vec<u32>) -> bool {
    let mut ptr = 0;
    let mut curr = 0;
    for c in spring.chars() {
        if c == '.' {
            if curr != 0 {
                if ptr < po.len() && po[ptr] == curr {
                    ();
                } else {
                    return false;
                }
                ptr += 1;
                curr = 0;
            }
        } else if c == '#' {
            curr += 1;
        } else {
            panic!("wtf");
        } 
    } 
    if curr != 0 {
        if ptr == po.len() - 1 && po[ptr] == curr {
            ();
        } else {
            return false;
        }
    } else if ptr != po.len() {
        return false;
    }
    return true;
}

fn find_ways(springs: &str, po: Vec<u32>) -> u32 {

    let mut waiting: Vec<String> = vec![];

    for (i, elem) in springs.chars().enumerate() {
        if i == 0 {
            match elem {
                '?' => {
                    waiting.push(".".to_owned()); 
                    waiting.push("#".to_owned()); 
                },
                '.' => {
                    waiting.push(".".to_owned()); 
                },
                '#' => {
                    waiting.push("#".to_string()); 
                },
                _ => panic!("at the disco!"),
            }
        } else {
            let mut nw = vec![];
            for w in waiting {

                match elem {
                    '?' => {
                        nw.push(w.to_owned() + "."); 
                        nw.push(w.to_owned() + "#"); 
                    },
                    '.' => {
                        nw.push(w.to_owned() + "."); 
                    },
                    '#' => {
                        nw.push(w.to_owned() + "#"); 
                    },
                    _ => panic!("at the disco!"),
                }
            }
            waiting = nw;
            
        }
    }

    // println!("{:?}", waiting.iter().filter(|line| satisfy(line, po.clone())).collect::<Vec<_>>());
    return waiting.iter().filter(|line| satisfy(line, po.clone())).count() as u32;
}

#[aoc(day12, part1)]
fn part1(input: &str) -> u32 {

    let mut ans = 0;
    for line in input.lines() {
        let (springs, order) = line.split_once(" ").unwrap();
        let po: Vec<u32> = order.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
        // println!("{}", find_ways(springs, po.clone()));
        ans += find_ways(springs, po);
        
    }
    return ans;
}

// Stolen off someone on CSESoc. Thanks! I recognised it was DP but was too dum to figure out the smart way to do it.
// A slightly dumber way is to keep track of where we are in ONE BLOCK, which requires three states. Going block by block is easier.
// Add number of ways to both... etc. etc. 
// Be careful of integer overflow... they really hurt in a Rust solution vs Python.
fn dp(curr_s: usize, curr_b: usize, springs: &str, blocks: Vec<u32>, storage: &mut HashMap<(usize, usize), u64>) -> u64 {
    // curr_s = curr spring, curr_b = block

    match storage.get(&(curr_s, curr_b)) {
        Some(x) => return *x,
        None => (),
    }

    if curr_s >= springs.len().try_into().unwrap() {
        return match curr_b == blocks.len().try_into().unwrap() {
            true => 1, 
            false => 0, 
        }
    }

    let mut ways = 0;

    if springs.chars().nth(curr_s).unwrap() == '?' || springs.chars().nth(curr_s).unwrap() == '.' {

        // keep track of which spring you're up to, and which block.
        // If skip this one, consider case where just increasing next block
        ways += dp(curr_s + 1, curr_b, springs, blocks.clone(), storage);
    }

    if curr_b >= blocks.len() {
        // storage.insert((curr_s, curr_b), ways);
        return ways;
    }

    if springs.chars().nth(curr_s).unwrap() == '?' || springs.chars().nth(curr_s).unwrap() == '#' {
        let end = curr_s + blocks[curr_b] as usize;
        if end <= springs.len() {
            let mut sl = springs.chars().skip(curr_s).take(end - curr_s);
            if !sl.any(|x| x == '.') && (end == springs.len() || springs.chars().nth(end).unwrap() != '#') {
                ways += dp(end + 1, curr_b + 1, springs, blocks, storage);
            }
        }
    }

    storage.insert((curr_s, curr_b), ways);

    ways
}

#[aoc(day12, part2)]
fn part2(input: &str) -> u64 {

    let mut ans:u64 = 0;
    for line in input.lines() {
        let (springs, order) = line.split_once(" ").unwrap();
        let new_springs = [springs, springs, springs, springs, springs].join("?"); 
        let mut storage: HashMap<(usize, usize), u64> = HashMap::new();
        let po: Vec<u32> = order.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
        let mut new_po = vec![]; 
        for _ in 0..5 {
            new_po.append(&mut po.clone());
        }
        // println!("{}", find_ways(springs, po.clone()));
        ans += dp(0, 0, &new_springs, new_po, &mut storage) as u64;
        
    }
    println!("{ans}");
    return ans;
}