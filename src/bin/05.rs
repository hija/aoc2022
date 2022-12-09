use std::collections::VecDeque;

struct Containers {
    containers: Vec<Vec<char>>,
}

impl Containers {
    fn execute_command(self, command: &str) {
        // pass
    }
}

fn part_one_with_stacksize(input: &str, stacksize: i32) -> Option<String> {
    let lines = input.lines();

    let mut containers: Vec<VecDeque<char>> = vec![VecDeque::new(); stacksize as usize];

    let mut processmodus = 0;

    for line in lines {
        if line.starts_with(" 1") {
            processmodus = 1;
            continue;
        }

        if processmodus == 0 {
            // Build up containers
            for i in 0..stacksize {
                let containeritem = line.chars().nth(((i * 4) + 1) as usize).unwrap();

                if containeritem != ' ' {
                    containers[i as usize].push_back(containeritem);
                }
            }
        } else if processmodus == 1 {
            // New line
            processmodus = 2;
        } else {
            // Process queries
            let mut parts = line.split(" ");
            let how_many: usize = parts.nth(1).unwrap().parse().unwrap();
            let from_stack: usize = parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            let to_stack: usize = parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;

            for i in 0..how_many {
                let item = containers[from_stack].pop_front().unwrap();
                containers[to_stack].push_front(item);
            }
        }
    }

    let mut list = vec![];
    for i in 0..stacksize {
        list.push(containers[i as usize].pop_front().unwrap());
    }

    return Some(list.into_iter().collect());
}

pub fn part_one(input: &str) -> Option<String> {
    return part_one_with_stacksize(input, 9);
}

pub fn part_two(input: &str) -> Option<String> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one_with_stacksize(&input, 3), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
