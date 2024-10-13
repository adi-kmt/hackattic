fn sum_of_things(input: &str) -> String {
    input
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            let sum = line.split_whitespace()
                .filter_map(|word| {
                    let value = if word.len() == 1 {
                        let ascii_value = word.chars().next().unwrap() as i32;
                        Some(ascii_value)
                    } else if word.starts_with("0x") {
                        if let Ok(num) = i32::from_str_radix(word.trim_start_matches("0x"), 16) {
                            Some(num)
                        } else {
                            None
                        }
                    } else if word.starts_with("0o") {
                        if let Ok(num) = i32::from_str_radix(word.trim_start_matches("0o"), 8) {
                            Some(num)
                        } else {
                            None
                        }
                    } else if word.starts_with("0b") {
                        if let Ok(num) = i32::from_str_radix(word.trim_start_matches("0b"), 2) {
                            Some(num)
                        } else {
                            None
                        }
                    } else {
                        if let Ok(num) = word.parse::<i32>() {
                            Some(num)
                        } else {
                            None
                        }
                    };
                    value
                })
                .sum::<i32>();
            sum.to_string()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"110 0x187 300 T / d
180 A 0x10e 0x18c N 95
423 0xac 417 0o20 q &
0x14e 0b10000 247 284 0o447 268"#;

    const OUTPUT: &str = r#"1032
1084
1179
1444"#;

    #[test]
    fn test() {
        let output = sum_of_things(INPUT);
        println!("Final output:\n{}", output);
        assert_eq!(OUTPUT, output);
    }
}