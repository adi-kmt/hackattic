use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Extra {
    balance: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Account {
    balance: i64,
    account_no: i64,
    #[serde(default)]
    extra: Option<Extra>,
}

fn sort_json(input: &str) -> String {
    let mut final_resp = String::new();
    let mut item_vec: Vec<(String, Account)> = Vec::new();

    for line_item in input.lines() {
        match serde_json::from_str::<HashMap<String, Account>>(line_item) {
            Ok(record) => {
                if let Some((account_name, account_info)) = record.into_iter().next() {
                    item_vec.push((account_name, account_info));
                }
            }
            Err(e) => {
                println!("Something went wrong: {}", e);
            }
        }
    }

    item_vec.sort_by(|a, b| {
        let a_balance = a.1.extra.as_ref().map(|e| e.balance).unwrap_or(a.1.balance);
        let b_balance = b.1.extra.as_ref().map(|e| e.balance).unwrap_or(b.1.balance);
        a_balance.cmp(&b_balance)
    });

    for (name, account) in item_vec {
        let balance = account.extra.as_ref().map_or(account.balance, |extra| extra.balance);
        final_resp.push_str(&format!("{}: {}\n", name, balance));
    }

    final_resp.trim_end().to_string() // Remove the trailing newline
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"{"Bentley.G":{"balance":2134,"account_no":233831255}}
{"Barclay.E":{"balance":1123,"account_no":312333321}}
{"Alton.K":{"balance":9315,"account_no":203123613,"extra":{"balance":131}}}
{"Bancroft.M":{"balance":233,"account_no":287655771101,"extra":{"balance":98}}}"#;

    const OUTPUT: &str = r#"Bancroft.M: 98
Alton.K: 131
Barclay.E: 1123
Bentley.G: 2134"#;

    #[test]
    fn test() {
        let output = sort_json(INPUT);
        assert_eq!(OUTPUT, output);
    }
}
