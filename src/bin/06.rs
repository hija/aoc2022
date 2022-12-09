fn first_unique(input: &str, size: i8) -> Option<u32> {
    let mut x = input.as_bytes().windows(size as usize);

    for i in 0..x.len() {
        let mut sub_window = x.nth(0).unwrap().to_vec();
        sub_window.sort();
        sub_window.dedup();
        if sub_window.len() == size as usize {
            return Some(i as u32);
        }
    }
    return Some(0);
}

pub fn part_one(input: &str) -> Option<u32> {
    return Some(first_unique(input, 4).unwrap() + 4);
}

pub fn part_two(input: &str) -> Option<u32> {
    return Some(first_unique(input, 14).unwrap() + 14);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(23));
    }
}
