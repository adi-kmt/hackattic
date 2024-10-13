fn a_case_of_snakes(input: &str) -> String {
    input.lines()
        .map(|line_item| {
            line_item.chars().fold((String::new(), false), |(mut acc, mut found_first_upper), c| {
                if c.is_uppercase() {
                    if found_first_upper {
                        acc.push('_');
                    }
                    acc.push(c.to_ascii_lowercase());
                    found_first_upper = true;
                } else if found_first_upper {
                    acc.push(c);
                }
                (acc, found_first_upper)
            }).0
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod test {
    use super::a_case_of_snakes;

    const INPUT: &str = r#"szWindowContents
iAirflowParameter
fMixtureRatio"#;

    const OUTPUT: &str = r#"window_contents
airflow_parameter
mixture_ratio"#;

    #[test]
    fn test() {
        let output = a_case_of_snakes(INPUT);
        assert_eq!(OUTPUT, output);
    }
}
