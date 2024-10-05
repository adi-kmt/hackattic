fn almost_binary(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let chars = line.chars()
                .filter_map(|c| match c {
                    '#' => Some('1'),
                    '.' => Some('0'),
                    _ => None,
                })
                .collect::<String>();

            u32::from_str_radix(&chars, 2)
                .map_err(|e| format!("Failed to parse binary: {}", e))
                .unwrap_or(0)

        })
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join("\n")

}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"#.#.#.###.#.##.#
##.##.......#..#
#..#####..#.#...
###..#....###.##
#..#..#.#....##
#......#.##....
#############.#."#;

    const OUTPUT: &str = r#"43949
55305
40744
58427
18755
16560
65530"#;

    #[test]
    fn check() {
        let output = almost_binary(&INPUT);
        assert_eq!(output, OUTPUT);
    }
}
