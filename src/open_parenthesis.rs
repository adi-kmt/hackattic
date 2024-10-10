fn open_parenthesis(input: &str) -> String{
    input
        .lines()
        .map(|line_item|{
        let mut parenthesis_call:i8 = 0;
            line_item.chars().for_each(|c|{
                match c {
                    '(' => parenthesis_call += 1,
                    ')' => parenthesis_call -= 1,
                    _ => panic!("Not a valid parenthesis")
                }
                if parenthesis_call < 0{
                    return;
                }
            });
            if parenthesis_call == 0 {
                return "yes"
            } else {
                return "no"
            }
        })
        .collect::<Vec<&str>>()
        .join("\n")
}



#[cfg(test)]
mod test{
    use super::*;

    const INPUT: &str = r#"(())
()))
(()((())))
(()(()(()))"#;

    const OUTPUT: &str = r#"yes
no
yes
no"#;

    #[test]
    fn test1(){
        let output = open_parenthesis(INPUT);
        assert_eq!(output, OUTPUT);
    }
}