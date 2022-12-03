use std::str::FromStr;

const WIN_SCORE: u8 = 6;
const DRAW_SCORE: u8 = 3;
const ROCK_SCORE: u8 = 1;
const PAPER_SCORE: u8 = 2;
const SCISSORS_SCORE: u8 = 3;

#[derive(Clone, Copy, PartialEq)]
enum RspChoice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for RspChoice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl RspChoice {
    fn win(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn draw(&self) -> Self {
        *self
    }

    fn lose(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

#[derive(Clone, Copy)]
enum YourChoice {
    X,
    Y,
    Z,
}

impl FromStr for YourChoice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::X),
            "Y" => Ok(Self::Y),
            "Z" => Ok(Self::Z),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
struct Matchup {
    they: RspChoice,
    you: YourChoice,
}

impl FromStr for Matchup {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (they_str, you_str) = s.split_once(' ').ok_or(())?;
        Ok(Matchup {
            they: they_str.parse()?,
            you: you_str.parse()?,
        })
    }
}

fn score_v1(they: RspChoice, you: YourChoice) -> u8 {
    let your = match you {
        YourChoice::X => RspChoice::Rock,
        YourChoice::Y => RspChoice::Paper,
        YourChoice::Z => RspChoice::Scissors,
    };
    score_matchup(they, your)
}

fn score_v2(they: RspChoice, you: YourChoice) -> u8 {
    let your = match you {
        YourChoice::X => they.lose(),
        YourChoice::Y => they.draw(),
        YourChoice::Z => they.win(),
    };
    score_matchup(they, your)
}

fn score_matchup(they: RspChoice, you: RspChoice) -> u8 {
    use RspChoice::*;
    let result_score = if you == they.win() {
        WIN_SCORE
    } else if you == they.draw() {
        DRAW_SCORE
    } else {
        0
    };

    let choice_score = match you {
        Rock => ROCK_SCORE,
        Paper => PAPER_SCORE,
        Scissors => SCISSORS_SCORE,
    };
    result_score + choice_score
}

fn total_score<F>(matchups: &[Matchup], score_fn: F) -> u32
where
    F: Fn(RspChoice, YourChoice) -> u8,
{
    matchups
        .iter()
        .map(|m| score_fn(m.they, m.you) as u32)
        .sum::<u32>()
}

fn parse_input(input: &str) -> Option<Vec<Matchup>> {
    input
        .lines()
        .map(|l| l.parse::<Matchup>().ok())
        .collect::<Option<Vec<Matchup>>>()
}

fn main() {
    let matchups = parse_input(include_str!("input")).expect("Failed to parse all lines");

    println!("Part 1: Total score {}", total_score(&matchups, score_v1));
    println!("Part 2: Total score {}", total_score(&matchups, score_v2));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn test_v1() {
        let matchups = parse_input(INPUT).unwrap();
        let score = total_score(&matchups, score_v1);
        assert_eq!(score, 15);
    }

    #[test]
    fn test_v2() {
        let matchups = parse_input(INPUT).unwrap();
        let score = total_score(&matchups, score_v2);
        assert_eq!(score, 12);
    }
}
