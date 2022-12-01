pub fn part_one(input: &str) -> Option<u32> {
    let mut sum_elf: u32 = 0;
    let mut elf_values: Vec<u32> = Vec::new();

    for line in input.lines() {
        if line == "" {
            // Neuer Elf
            elf_values.push(sum_elf);
            sum_elf = 0;
        } else {
            let line_as_int: u32 = line.parse().unwrap();
            sum_elf += line_as_int;
        }
    }

    // Letzten Elf pushen
    elf_values.push(sum_elf);

    let mut max_value: u32 = elf_values[0];

    for x in 0..elf_values.len() {
        if max_value < elf_values[x] {
            max_value = elf_values[x];
        }
    }

    return Some(max_value);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum_elf: u32 = 0;
    let mut elf_values: Vec<u32> = Vec::new();

    for line in input.lines() {
        if line == "" {
            // Neuer Elf
            elf_values.push(sum_elf);
            sum_elf = 0;
        } else {
            let line_as_int: u32 = line.parse().unwrap();
            sum_elf += line_as_int;
        }
    }

    // Letzten Elf pushen
    elf_values.push(sum_elf);

    elf_values.sort_by(|a, b| b.cmp(a));

    return Some(elf_values[0] + elf_values[1] + elf_values[2]);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
