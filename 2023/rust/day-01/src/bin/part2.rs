use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = answer(input);
    dbg!(output);
}

fn translate_spell(spell: &str) -> String {
    match spell {
        "one" => "1".to_string(),
        "two" => "2".to_string(),
        "three" => "3".to_string(),
        "four" => "4".to_string(),
        "five" => "5".to_string(),
        "six" => "6".to_string(),
        "seven" => "7".to_string(),
        "eight" => "8".to_string(),
        "nine" => "9".to_string(),
        _ => spell.to_string(),
    }
}

fn translate_reversed_spell(spell: &str) -> String {
    match spell {
        "eno" => "1".to_string(),
        "owt" => "2".to_string(),
        "eerht" => "3".to_string(),
        "ruof" => "4".to_string(),
        "evif" => "5".to_string(),
        "xis" => "6".to_string(),
        "neves" => "7".to_string(),
        "thgie" => "8".to_string(),
        "enin" => "9".to_string(),
        _ => spell.to_string(),
    }
}

fn answer(input: &str) -> String {
    let regex = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)");
    let reversed_regex = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)");

    input.split("\n")
        .map(|line| {
            let reversed = line.chars().rev().collect::<String>();

            let mut first = regex.as_ref().expect("Regex should exist").captures_iter(line).next().unwrap()[0].to_string();
            first = translate_spell(&first);

            let mut last = reversed_regex.as_ref().expect("Regex should exist").captures_iter(&reversed).next().unwrap()[0].to_string();
            last = translate_reversed_spell(&last);

            first.push_str(&last);
            let concat = first.parse::<u32>().unwrap();

            concat
        }).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphabet_regex_should_match_both_ways() {
        let result = answer(r"one
two
three
four
five
six
seven
eight
nine");
        assert_eq!(result, "495");
    }
    
    #[test]
    fn given_example_should_match() {
        let result = answer(r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
        assert_eq!(result, "281");
    }
}
