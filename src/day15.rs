
enum Instruction<'a> {
    Remove(&'a str),
    Add(Lens<'a>),
}

struct Lens<'a> {
    label: &'a str,
    focal: u8,
}

// New instructions to parse into...
impl<'a> Instruction<'a> {
    fn new(s: &'a str) -> Self {
        if let Some(label) = s.strip_suffix('-') {
            Self::Remove(label)
        } else {
            let (label, focal) = s.split_once('=').unwrap();
            let focal = focal.parse().unwrap();
            let lens = Lens { label, focal };
            Self::Add(lens)
        }
    }
}

// take byte and then take it as a u64 to get the ascii value... wow!
// constant vector, replicate
// retain and find
// Usage of instruction implementation
// fold syntax
fn f(line: &str) -> u64 {
    line.bytes().fold(0, |acc, x| ((acc + x as u64) * 17) % 256)
}


#[aoc(day15, part1)]
fn part1(input: &str) -> u64 {
    input.split(",")
        .map(|x| f(x))
        .sum()
}

#[aoc(day15, part2)]
fn part2(input: &str) -> usize {

    // Credits to https://nickymeuleman.netlify.app/garden/aoc2023-day15 for this very Rustic solution.
    // Constant vector and then replicate it 256
    const LENS_BOX:Vec<Lens> = Vec::new();
    let mut boxes = [LENS_BOX; 256];
    for line in input.split(",") {
        let ins = Instruction::new(line);
        match ins {
            Instruction::Remove(label) => {
                let hash = f(label);
                // very slow approach that is O(n) but whatever lmao
                boxes[hash as usize].retain(|item| item.label != label);
            }
            Instruction::Add(lens) => {
                let hash = f(lens.label);
                let lenses = &mut boxes[hash as usize];
                // .find implementation
                if let Some(old) = lenses.iter_mut().find(|item| lens.label == item.label) {
                    // update focal length of lens with this label
                    old.focal = lens.focal;
                } else {
                    // add lens to end of box
                    lenses.push(lens);
                }
            }
        }
    }
    boxes
    .into_iter()
    .enumerate()
    .map(|(box_idx, lenses)| {
        let box_focusing_power: usize = lenses
            .into_iter()
            .enumerate()
            .map(|(lens_idx, lens)| (box_idx + 1) * (lens_idx + 1) * lens.focal as usize)
            .sum();
        box_focusing_power
    })
    .sum()


}