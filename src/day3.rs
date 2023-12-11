use std::{collections::HashMap};

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let mut grid : HashMap<(usize, usize), char> = HashMap::new();
    let mut ints: Vec<(u32, usize, usize, usize)> = vec![];
    let mut i = 0 ;
    let mut ans = 0;
    for lines in input.split("\n") {
        let mut start = 0;
        let mut cs: Vec<char> = vec![];
        for (j, item) in lines.chars().enumerate() {
            if item != '.' && !item.is_numeric() {
                grid.insert((j, i), item);
            }
            if item.is_numeric() {
                if cs.len() == 0 {
                    start = j;
                }
                cs.push(item);

                if j == lines.len() - 1 {
                    let a : u32 = cs
                        .iter()
                        .map(|x| (x).to_digit(10).unwrap())
                        .fold(0, |acc, elem: u32| acc * 10 + elem
                    );
                
                    ints.push((a, start, j, i));

                    cs = vec![];
                } 

            } else if cs.len() > 0 {

                let a : u32 = cs
                    .iter()
                    .map(|x| (x).to_digit(10).unwrap())
                    .fold(0, |acc, elem: u32| acc * 10 + elem
                );
                // println!("{a}");
            
                ints.push((a, start, j - 1, i));

                cs = vec![];
            }
        }
        i += 1;
    }

    // println!("{:?}", ints);
    // println!("{:?}", grid);
    for (u, us, ue, lat) in ints {
        let mut seen = false;
        let b = if us == 0 { 0 } else {us - 1};
        let c = if lat == 0 { 0 } else {lat - 1};
        for x in b..=ue + 1 {
            for y in c..=lat + 1 {
                // if y == lat && (x >= us && x <= ue) {
                //     continue;
                // }
                // guarantee now that this is a neighbour
                // println!("{u} {} {}", x, y);
                if grid.get(&(x, y)).is_some() {
                    seen = true;
                    // println!("{u};");
                    ans += u;
                    break;
                }
            }
            if seen {
                break;
            }

        } 
    }

    return ans;
}

#[aoc(day3, part2)]
fn part2(input: &str) -> i32 {
    let mut grid : HashMap<(usize, usize), char> = HashMap::new();
    let mut ints: Vec<(u32, usize, usize, usize)> = vec![];
    let mut ans : HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut i = 0 ;
    for lines in input.split("\n") {
        let mut start = 0;
        let mut cs: Vec<char> = vec![];
        for (j, item) in lines.chars().enumerate() {
            if item != '.' && !item.is_numeric() {
                grid.insert((j, i), item);
            }
            if item.is_numeric() {
                if cs.len() == 0 {
                    start = j;
                }
                cs.push(item);

                if j == lines.len() - 1 {
                    let a : u32 = cs
                        .iter()
                        .map(|x| (x).to_digit(10).unwrap())
                        .fold(0, |acc, elem: u32| acc * 10 + elem
                    );
                
                    ints.push((a, start, j, i));

                    cs = vec![];
                } 

            } else if cs.len() > 0 {

                let a : u32 = cs
                    .iter()
                    .map(|x| (x).to_digit(10).unwrap())
                    .fold(0, |acc, elem: u32| acc * 10 + elem
                );
                // println!("{a}");
            
                ints.push((a, start, j - 1, i));

                cs = vec![];
            }
        }
        i += 1;
    }

    // println!("{:?}", ints);
    // println!("{:?}", grid);
    for (u, us, ue, lat) in ints {
        let b = if us == 0 { 0 } else {us - 1};
        let c = if lat == 0 { 0 } else {lat - 1};
        for x in b..=ue + 1 {
            for y in c..=lat + 1 {
                // if y == lat && (x >= us && x <= ue) {
                //     continue;
                // }
                // guarantee now that this is a neighbour
                // println!("{u} {} {}", x, y);
                if let Some(ch) = grid.get(&(x, y)) {
                    if *ch != '*' {
                        continue;
                    }
                    ans.entry((x, y)).or_insert(vec![]);
                    ans.get_mut(&(x, y)).unwrap().push(u);
                    // println!("{u};");
                }
            }

        } 
    }
    let mut z = 0;

    for (_ch, v) in ans {
        if v.len() == 2 {
            z += v[0] * v[1];
        }
    } 
    return z.try_into().unwrap();

}