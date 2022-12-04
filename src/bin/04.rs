use array_tool::vec::Intersect;

fn is_contained(s1: Vec<u32>, s2: Vec<u32>) -> bool {
    let s1_len = s1.len();
    let s2_len = s2.len();
    let intersection = s1.intersect(s2);
    return intersection.len() == s1_len || intersection.len() == s2_len;
}

fn do_overlap(s1: Vec<u32>, s2: Vec<u32>) -> bool {
    let intersection = s1.intersect(s2);
    return intersection.len() > 0;
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.lines().map(|l| {
        let mut elf_vectors = l.split(",").map(|p| {
            let mut iterator = p.split("-");
            let start_vec = iterator.next().unwrap().parse::<u32>().unwrap();
            let end_vec = iterator.last().unwrap().parse::<u32>().unwrap();
            let elf_vector = start_vec..=end_vec;
            return elf_vector.collect::<Vec<u32>>();
        });

        let first_range = elf_vectors.next().unwrap();
        let second_range = elf_vectors.next().unwrap();

        return match is_contained(first_range, second_range) {
            true => 1 as u32,
            _ => 0 as u32,
        };
    });
    let sum_result: Option<u32> = Some(result.sum());
    return sum_result;
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input.lines().map(|l| {
        let mut elf_vectors = l.split(",").map(|p| {
            let mut iterator = p.split("-");
            let start_vec = iterator.next().unwrap().parse::<u32>().unwrap();
            let end_vec = iterator.last().unwrap().parse::<u32>().unwrap();
            let elf_vector = start_vec..=end_vec;
            return elf_vector.collect::<Vec<u32>>();
        });

        let first_range = elf_vectors.next().unwrap();
        let second_range = elf_vectors.next().unwrap();

        return match do_overlap(first_range, second_range) {
            true => 1 as u32,
            _ => 0 as u32,
        };
    });
    let sum_result: Option<u32> = Some(result.sum());
    return sum_result;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
