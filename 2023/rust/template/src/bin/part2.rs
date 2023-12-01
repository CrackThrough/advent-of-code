fn main() {
    let input = include_str!("./input.txt");
    let output = answer(input);
    dbg!(output);
}

fn answer(input: &str) -> String {
    "test".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = answer("test");
        assert_eq!(result, "test");
    }
}
