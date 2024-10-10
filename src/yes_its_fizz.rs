fn yes_its_fizz(input: &str) -> String {
    let nums = input
        .split_whitespace()
        .filter_map(|num_item|{
            num_item.parse::<u8>().ok()
        })
        .collect::<Vec<u8>>();

    if nums.len() < 2 {
        return String::from("Something went wrong!!")
    }
    let (start, end) = (nums[0], nums[1]);
    let mut final_string = String::with_capacity((end - start + 1) as usize * 10);

    for i in start..=end {
        match (i % 3, i % 5) {
            (0, 0) => final_string.push_str("FizzBuzz\n"),
            (0, _) => final_string.push_str("Fizz\n"),
            (_, 0) => final_string.push_str("Buzz\n"),
            _ => final_string.push_str(&format!("{}\n", i)),
        }
    }
    final_string.trim().to_string()
}


#[cfg(test)]
mod test{
    use super::*;

    const INPUT: &str = r#"8 16"#;

    const OUTPUT: &str = r#"8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
16"#;

    #[test]
    fn test(){
        let output = yes_its_fizz(INPUT);
        assert_eq!(output, OUTPUT)
    }
}