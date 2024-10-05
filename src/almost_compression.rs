use itertools::Itertools;

fn almost_compression(input: &str) -> String{
    input
        .lines()
        .map(|line_item|{
            line_item
                .chars()
                .chunk_by(|c| *c) // Grouping by chars
                .into_iter()
                .map(|(c, group)|{
                    let count = group.count();

                    match count {
                        1 => c.to_string(),
                        2 => c.to_string(),
                        _ => format!("{}{}", count, c)
                    }
                })
                .collect()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn almost_compression_alt(input: &str) -> String {
    input
        .lines()
        .map(|input_line|{
            let mut result = String::new();
            let mut chars = input_line.chars().peekable();

            while let Some(current_char) = chars.next() {
                let mut count = 1;

                // Count consecutive occurrences
                while chars.peek() == Some(&current_char) {
                    count += 1;
                    chars.next();
                }

                // Only add the count if it's more than 1
                if count > 2 {
                    result.push_str(&count.to_string());
                }
                result.push(current_char);
            }

            result
        })
        .collect::<Vec<String>>()
        .join("\n")
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"aaaaaiiiixqvsm
rrdkuuuuyyyrrrrgghc
xhzzzccccvvsssqppc
jbiiiulllllvvvvtttttxxxxxs"#;

    const OUTPUT: &str = r#"5a4ixqvsm
rdk4u3y4rghc
xh3z4cv3sqpc
jb3iu5l4v5t5xs"#;

    #[test]
    fn check() {
        let output = almost_compression_alt(&INPUT);
        assert_eq!(output, OUTPUT);
    }
}