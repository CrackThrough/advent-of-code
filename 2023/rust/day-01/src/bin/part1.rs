use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = answer(input);
    dbg!(output);
}

fn answer(input: &str) -> String {
    let regex = Regex::new(r"\d");

    input.split("\n")
        .map(|line| {
            let reversed = line.chars().rev().collect::<String>();
            
            let mut first = regex.as_ref().expect("Regex should exist").captures_iter(line).next().unwrap()[0].to_string();
            let last = regex.as_ref().expect("Regex should exist").captures_iter(&reversed).next().unwrap()[0].to_string();
            
            first.push_str(&last);
            let concat = first.parse::<u32>().unwrap();
            
            concat
        }).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = answer(r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, "142");
    }
}
