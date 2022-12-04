fn get_priority(rucksack: &str) -> u32 {
    let size_compartments = rucksack.len() / 2;
    let (first_compartment, second_compartmentt) = rucksack.split_at(size_compartments);

    for c in first_compartment.chars() {
        if second_compartmentt.contains(c) {
            if c.is_lowercase() {
                return c as u32 - 96;
            } else {
                return (c as u32 - 64) + 26;
            }
        }
    }

    return 0;
}

pub fn part_one(input: &str) -> Option<u32> {
    let rucksacks = input.lines();
    let priorities = rucksacks.map(|x| get_priority(x));
    return Some(priorities.sum());
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
