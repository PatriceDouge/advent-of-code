fn main() {
    let input = include_str!("../../input1.txt");
    let output: i32 = process(&input);
    println!("{}", output);
}

fn process(input: &str) -> i32 {
    let mut sum: i32 = 0;

    for line in input.lines() {
        let game_info: Vec<&str> = line.split(':').collect();
        let game_cubes = game_info[1].trim();

        let mut max_red: i32 = 0;
        let mut max_green: i32 = 0;
        let mut max_blue: i32 = 0;

        for turn in game_cubes.split(';') {
            let mut red: i32 = 0;
            let mut green: i32 = 0;
            let mut blue: i32 = 0;

            let color_info = turn.trim().split(',').collect::<Vec<&str>>();
            
            for color in color_info  {
                let color_map = color.trim().split_whitespace().collect::<Vec<&str>>();

                let color_count: i32 = color_map[0].trim().parse::<i32>().unwrap();
                let color_name = color_map[1].trim();
                
                match color_name {
                    "red" => red = color_count,
                    "green" => green = color_count,
                    "blue" => blue = color_count,
                    _ => continue,
                };
            }

            if red > max_red {
                max_red = red;
            } if green > max_green {
                max_green = green;
            } if blue > max_blue {
                max_blue = blue;
            }
        }
        
        let power = max_red * max_green * max_blue;
        sum += power;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, process(input));
        Ok(())
    }
}