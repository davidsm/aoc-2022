use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Item(char);

impl Item {
    fn priority(&self) -> u8 {
        const LOWERCASE_A_PRIO: u32 = 1;
        const UPPERCASE_A_PRIO: u32 = 27;

        let prio = if self.0.is_ascii_lowercase() {
            LOWERCASE_A_PRIO + (self.0 as u32) - ('a' as u32)
        } else if self.0.is_ascii_uppercase() {
            UPPERCASE_A_PRIO + (self.0 as u32) - ('A' as u32)
        } else {
            0
        };
        prio as u8
    }
}

struct Compartment(HashSet<Item>);

impl Compartment {
    fn duplicate_item(&self, other: &Compartment) -> Option<Item> {
        self.0.intersection(&other.0).cloned().next()
    }
}

impl FromStr for Compartment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Compartment(s.chars().map(Item).collect::<HashSet<Item>>()))
    }
}

fn duplicate_item(line: &str) -> Option<Item> {
    let (first, second) = line.split_at(line.len() / 2);
    let first: Compartment = first.parse().ok()?;
    let second: Compartment = second.parse().ok()?;
    first.duplicate_item(&second)
}

fn part1(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| duplicate_item(l).map(|i| i.priority() as u32))
        .sum()
}

fn main() {
    println!("Part 1: Priority {}", part1(include_str!("input")).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_duplicate_item() {
        let input1 = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(duplicate_item(input1), Some(Item('p')));

        let input2 = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        assert_eq!(duplicate_item(input2), Some(Item('v')));
    }

    #[test]
    fn test_item_priority() {
        assert_eq!(Item('a').priority(), 1);
        assert_eq!(Item('h').priority(), 8);
        assert_eq!(Item('z').priority(), 26);
        assert_eq!(Item('A').priority(), 27);
        assert_eq!(Item('P').priority(), 42);
        assert_eq!(Item('Z').priority(), 52);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("test_input")).unwrap(), 157);
    }
}
