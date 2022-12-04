fn get_priority_for_character(c: char) -> u32 {
    if c.is_lowercase() {
        return c as u32 - 96;
    } else {
        return (c as u32 - 64) + 26;
    }
}

fn get_priority(data: Vec<&str>) -> u32 {
    let first_vec = data.first().copied().unwrap();

    for c in first_vec.chars() {
        let mut is_common_letter = true;
        for other_vector in data.iter().copied() {
            if !other_vector.contains(c) {
                is_common_letter = false;
                break;
            }
        }

        if is_common_letter {
            return get_priority_for_character(c);
        }
    }

    return 0;
}

pub fn part_one(input: &str) -> Option<u32> {
    let rucksacks = input.lines();
    let mut sum_priorities = 0;
    for rucksack in rucksacks {
        let (p1, p2) = rucksack.split_at(rucksack.len() / 2);
        sum_priorities += get_priority(vec![p1, p2]);
    }
    return Some(sum_priorities);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines().into_iter();
    let mut sum_priorities = 0;
    loop {
        let p1 = lines.next();
        let p2 = lines.next();
        let p3 = lines.next();

        if p1.is_none() {
            break;
        }

        sum_priorities += get_priority(vec![p1.unwrap(), p2.unwrap(), p3.unwrap()]);
    }
    return Some(sum_priorities);
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
        assert_eq!(part_two(&input), Some(70));
    }
}
