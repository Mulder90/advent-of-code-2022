fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();
    part1(&file);
    part2(&file);
}

fn part1(file: &str) {
    let result = file
        .split("\n\n")
        .map(|load| {
            load.lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    println!("{}", result.to_string());
}

fn part2(file: &str) {
    let mut grouped_loads: Vec<u32> = file
        .split("\n\n")
        .map(|load| {
            load.lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();
    grouped_loads.sort();
    let total_calories = grouped_loads.iter().rev().take(3).sum::<u32>();
    println!("{}", total_calories);
}
