use std::collections::HashSet;

fn fix_tail(
    head_x: u32,
    head_y: u32,
    mut tail_x: u32,
    mut tail_y: u32,
    mut visitedPositions: HashSet<Vec<u32>>,
) -> (u32, u32, u32, u32, HashSet<Vec<u32>>) {
    visitedPositions.insert(vec![tail_x, tail_y]);

    // Check if we need to adjust
    if head_x.abs_diff(tail_x) > 1 || head_y.abs_diff(tail_y) > 1 {
        // We need to move towards x
        if head_x > tail_x {
            tail_x += 1;
        } else if head_x < tail_x {
            tail_x -= 1;
        }

        // Diagonally
        if head_y > tail_y {
            tail_y += 1;
        } else if head_y < tail_y {
            tail_y -= 1;
        }
    }
    visitedPositions.insert(vec![tail_x, tail_y]);
    return (head_x, head_y, tail_x, tail_y, visitedPositions);
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut head_x: u32 = 1000;
    let mut head_y: u32 = 1000;
    let mut tail_x: u32 = 1000;
    let mut tail_y: u32 = 1000;

    let mut visitedPositions: HashSet<Vec<u32>> = HashSet::new();

    for line in input.lines() {
        let mut command_parts = line.split(" ");
        let direction = command_parts.nth(0).unwrap();
        let mut amount: u32 = command_parts.nth(0).unwrap().parse().unwrap();

        while amount > 0 {
            if direction == "U" {
                head_x -= 1;
            } else if direction == "D" {
                head_x += 1;
            } else if direction == "R" {
                head_y += 1;
            } else if direction == "L" {
                head_y -= 1;
            }

            (head_x, head_y, tail_x, tail_y, visitedPositions) =
                fix_tail(head_x, head_y, tail_x, tail_y, visitedPositions);
            amount -= 1;
        }
    }
    return Some(visitedPositions.len() as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
