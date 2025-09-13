use chrono::NaiveDate;

pub fn parse_command_line() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        Some(args[1].clone())
    } else {
        None
    }
}

pub fn prompt_for_string(message: &String) -> String {
    println!("{}", message);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn prompt_for_date(message: &String) -> NaiveDate {
    let s = prompt_for_string(message);
    let s = s.trim();
    date_from_str(&s)
}

pub fn date_from_str(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
}
