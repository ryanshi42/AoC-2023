
#[derive(Hash, PartialEq, Eq, Clone)]
struct Module<'a> {
    name: &'a str,
    readers: Vec<&'a str>
}

fn parse_modules(input: &str) -> Vec<Module> {
    input
        .lines()
        .map(|line| match line.chars().nth(0).unwrap() {
            'b' => 
        })
        .collect()
}

#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
    let modules = parse_modules(input);
    0
}

#[aoc(day20, part2)]
fn part2(input: &str) -> usize {
    // should chain
    0
}