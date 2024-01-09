use std::ops::Div;


fn f(input: &str) -> u64 {
    g(convert(input), -1) + 100 * g(transpose(convert(input)), -1)
}

// This function SUCKS. It clones the whole grid which is not necessary.
// The better way is to branch out and check number of differences. If the number of differences is equal to 1 by the end, it could be the reflection.
fn h(input: &str) -> u64 {
    let og = convert(input);
    let actual = f(input);
    let mut row: i32 = -1;
    let mut col: i32 = -1;
    if actual >= 100 {
        col = actual.div(100) as i32;
    } else {
        row = actual as i32;
    }
    for (k, line) in og.iter().enumerate() {
        for (j, i) in line.iter().enumerate() {

            let mut new_char = i;
            let _nc = new_char;
            if *i == '.' {
                new_char = &'#';
            } else if *i == '#' {
                new_char = &'.';
            } else {
                panic!("jasdlkfhjadjlskf");
            }
            let mut temp = og.clone();
            temp[k][j] = *new_char;
            let p = g(temp.clone(), row) + 100 * g(transpose(temp.clone()), col);
            if p > 0 {
                return p;
            }

        }
    }
    panic!("at the disco!");
}

fn convert(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|row| row.chars().collect()).collect()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn g(input: Vec<Vec<char>>, old: i32) -> u64 {
    let grid = input;
    // println!("{grid:?}");

    let mut ans = 0;
    for i in 0..grid[0].len() - 1 {
        let mut is_sym = true;
        for j in 0..grid[0].len() {
            if (i as i32 - j as i32) >= 0 && i + 1 + j < grid[0].len() {
                for k in 0..grid.len() {
                    if grid[k][i - j] != grid[k][i + 1 + j] {
                        is_sym = false;
                        break;
                    } 

                }
            } else {
                break;
            }
        } 
        if is_sym == true && (i + 1) as i32 != old {
            ans = i + 1;
            // println!("set answer as {ans}");
        }
        
    }
    // println!("{ans}");
    ans as u64
}


#[aoc(day13, part1)]
fn part1(input: &str) -> u64 {

    input.split("\n\n")
        .map(|x| f(x))
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &str) -> u64 {
    input.split("\n\n")
        .map(|x| h(x))
        .sum()
}