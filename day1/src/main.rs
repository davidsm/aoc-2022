fn main() {
    let input = include_str!("input").lines().map(|l| l.parse::<u32>().ok());
    let mut elves = vec![];
    let mut sum_cal = 0;
    for cal_entry in input {
        match cal_entry {
            Some(cal) => sum_cal += cal,
            None => {
                elves.push(sum_cal);
                sum_cal = 0;
            }
        }
    }
    elves.sort();
    println!("Part 1: Most calories: {}", elves.last().unwrap());
    let top_3 = elves.iter().rev().take(3);
    println!(
        "Part 2: Top 3 most calories combined: {}",
        top_3.sum::<u32>()
    );
}
