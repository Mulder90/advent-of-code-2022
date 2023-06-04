fn main() {
    let file = std::fs::read_to_string("./input.txt").unwrap();
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
