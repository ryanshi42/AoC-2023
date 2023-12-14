use std::{collections::{HashMap}};

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

#[aoc(day14, part1)]
fn part1(input: &str) -> u64 {


    input.split("\n\n")
        .map(|x| f(x))
        .sum()
}

#[aoc(day14, part2)]
fn part2(input: &str) -> u64 {
    let mut ans = 0;

    ans as u64

}