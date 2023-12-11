use std::{collections::HashMap};

#[aoc(day5, part1)]
fn part1(input: &str) -> i32 {

    let mut it = input.split("\n\n");
    let seeds = it
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut curr = seeds;
    loop {
        let x = it.next(); 
        let mut new_curr = vec![];
        if x.is_none() {
            break;
        } 
        let y = x.unwrap();
        // println!("{y:?}");
        let mut hm = HashMap::new();
        for mapping in y.split("\n").skip(1) {
            let mut d = mapping.split(" ");
            let a = d.next().unwrap().parse::<usize>().unwrap();
            let b = d.next().unwrap().parse::<usize>().unwrap();
            let c = d.next().unwrap().parse::<usize>().unwrap();
            hm.insert((a, b), c);
        }
        for c in curr {
            let mut found = false;
            for key in hm.keys() {
                let (v, k) = *key;
                if k <= c && c < (k + hm[key]) {
                    new_curr.push(v + (c - k));
                    found = true;
                    break;
                }
            } 
            if !found {
                new_curr.push(c);
            }
        }
        curr = new_curr;

        // println!("{curr:?}");
    }
    println!("{curr:?}");
    println!("{:?}", curr.iter().min().unwrap());
    return *curr.iter().min().unwrap() as i32;
}

#[aoc(day5, part2)]
fn part2(input: &str) -> i32 {

    let mut it = input.split("\n\n");
    let seeds = it
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut curr = seeds.iter();
    let mut ns = vec![];
    
    loop {
        let a = curr.next();
        let b = curr.next();

        if a.is_none() {
            break;
        }

        ns.push((a.unwrap(), b.unwrap()));
    }

    let mut hms = vec![];

    loop {
        let x = it.next(); 
        if x.is_none() {
            break;
        } 
        let y = x.unwrap();
        let mut hm = HashMap::new();
        for mapping in y.split("\n").skip(1) {
            let mut d = mapping.split(" ");
            let a = d.next().unwrap().parse::<usize>().unwrap();
            let b = d.next().unwrap().parse::<usize>().unwrap();
            let c = d.next().unwrap().parse::<usize>().unwrap();
            hm.insert((a, b), c);
        }
        hms.push(hm);
    }

    for i in 0..10000000000 as usize {
        // iterate backwards through hashmap to see if mapping exists, should only need to do 10^10 operations :skull:
        let mut z = i;
        let mut j:i32 = (hms.len() - 1).try_into().unwrap();
        while j >= 0 {
            let temp_hm = &hms[j as usize];
            for key in temp_hm.keys() {
                let (y, x) = key;
                if *y + temp_hm[key] > z && z >= *y {
                    z = *x + (z - *y); 
                    break;
                } 
            }
            j -= 1;
        }
        // Check if in new seeds
        for (s, t) in &ns {
            if **s <= z && z < **s + **t {
                println!("{i}");
                return i as i32;
            }
        }
    }

    return -1;
}