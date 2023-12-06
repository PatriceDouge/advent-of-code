fn main() {
    let input = include_str!("../../input2.txt");
    let output = process(&input);
    println!("{}", output);
}

fn process(input: &str) -> String {
    return "1".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!("1", process(input));
        Ok(())
    }
}