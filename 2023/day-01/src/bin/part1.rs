fn main() {
let input = include_str!("../../input1.txt");
let output = process(&input);
   println!("{}", output);
}

fn process(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let mut vec = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                vec.push(c);
            }
        }
        if vec.len() > 1 {
            sum += (vec[0].to_string() + &(vec[vec.len() - 1].to_string())).parse::<i32>().unwrap();
            continue;
        } if vec.len() > 0 {
            sum += (vec[0].to_string() + &(vec[0].to_string())).parse::<i32>().unwrap();
            continue;
        } else {
            continue;
        }
    }
    return sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        assert_eq!("142", process(input));
        Ok(())
    }
}