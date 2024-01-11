use core::panic;
use std::{collections::HashMap, hash::Hash};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Which {
    X,
    M,
    A,
    S
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Condition<'a> {
    name: Which,
    op: char,
    num: i64,
    where_next: &'a str
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Rule<'a> {
    conditions: Vec<Condition<'a>>,
    end: &'a str
} 

#[derive(Hash, PartialEq, Eq, Clone, Default)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
} 

fn parse_cond<'a>(cond: &'a str) -> Condition<'a> {
    let (con, where_next) = cond.split_once(":").unwrap();
    if con.chars().any(|c| c == '<') {
        let (name, num) = con.split_once("<").unwrap();
        let name = match name {
            "x" => Which::X,
            "m" => Which::M,
            "a" => Which::A,
            "s" => Which::S,
            _ => panic!("at the disco"),
        }; 
        Condition {
            name: name,
            op: '<',
            num: num.parse::<i64>().unwrap(),
            where_next: where_next
        }
    } else {
        let (name, num) = con.split_once(">").unwrap();
        let name = match name {
            "x" => Which::X,
            "m" => Which::M,
            "a" => Which::A,
            "s" => Which::S,
            _ => panic!("at the disco0"),
        }; 
        Condition {
            name: name, 
            op: '>',
            num: num.parse::<i64>().unwrap(),
            where_next: where_next
        }
    }
}

fn parse_rest(rest: &str) -> Rule {
    let mut rules_vec: Vec<_> = rest.split(",").collect();
    let unparsed_end = rules_vec.pop().unwrap();
    let end = &unparsed_end[0..unparsed_end.len() - 1];

    let conds_vec = rules_vec
        .iter()
        .map(|word| parse_cond(word))
        .collect();

    Rule {
        conditions: conds_vec,
        end: end
    }
}

fn parse_rules(rules: &str) -> HashMap<&str, Rule> {
    let mapped_rules: Vec<(&str, Rule<'_>)> = rules
        .lines()
        .map(|line| line.split_once("{").unwrap())
        .map(|(x, rest)| (x, parse_rest(rest)))
        .collect();

    let mut hm = HashMap::new();
    for (name, rule) in mapped_rules {
        hm.insert(name, rule);
    } 
    hm
}

fn parse_part(part: &str) -> Part {
    let mut p = Part::default();
    for kv in part.split(",") {
        let (key, val) = kv.split_once("=").unwrap();
        match key {
            "x" => p.x = val.parse::<i64>().unwrap(), 
            "m" => p.m = val.parse::<i64>().unwrap(),
            "a" => p.a = val.parse::<i64>().unwrap(),
            "s" => p.s = val.parse::<i64>().unwrap(),
            _ => panic!("at the disco1"),
        }
    } 
    p
}

fn parse_parts(parts: &str) -> Vec<Part> {
    parts.lines().map(|line| parse_part(&line[1..&line.len() - 1])).collect()
}


#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    
    let (rules, parts) = input.split_once("\n\n").unwrap(); 
    let pr = parse_rules(rules);
    let pp = parse_parts(parts);
    
    let mut ans = 0;

    for p in pp {
        let mut curr_loc = "in";
        while curr_loc != "A" && curr_loc != "R" {
            let og_loc = curr_loc;
            let rule = pr.get(curr_loc).unwrap();
            for cond in &rule.conditions {
                let field = match cond.name {
                    Which::X => p.x,
                    Which::M => p.m,
                    Which::A => p.a,
                    Which::S => p.s,
                };
                let b = match cond.op {
                    '<' => {
                        field < cond.num
                    },
                    '>' => {
                        field > cond.num
                    },
                    _ => panic!("at the disco2"),
                };
                if b {
                    curr_loc = cond.where_next;
                    break;
                }
            }
            if og_loc == curr_loc {
                curr_loc = rule.end;
            }
            // println!("{curr_loc}");
        } 
        if curr_loc == "A" {
            ans += p.x + p.m + p.a + p.s
        }
    }
    ans as usize

}

// Incomplete. Solution is long. 
// https://nickymeuleman.netlify.app/garden/aoc2023-day19
#[aoc(day19, part2)]
fn part2(_input: &str) -> usize {
    0
}