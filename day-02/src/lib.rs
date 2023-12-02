pub fn process_p1(input: &str) -> Result<String, String> {
    process(input, false)
}

pub fn process_p2(input: &str) -> Result<String, String> {
    process(input, true)
}

fn process(input: &str, p2: bool) -> Result<String, String> {
    let mut sum = 0;

    for line in aoc_lib::rows_to_vector(input) {
        let c: (&str, &str) = line.split_once(':').unwrap();
        let game: i32 = c.0.split_once(' ').unwrap().1.parse::<i32>().unwrap();

        let max_rgb =
            c.1.split(&[';', ','])
                .fold((0, 0, 0), |mut acc, c_and_val| {
                    let v = c_and_val.trim().split_once(' ').unwrap();
                    let val = v.0.parse::<i32>().unwrap();

                    match v.1 {
                        "red" => acc.0 = acc.0.max(val),
                        "green" => acc.1 = acc.1.max(val),
                        "blue" => acc.2 = acc.2.max(val),
                        _ => {}
                    }
                    acc
                });
        if p2 {
            sum += max_rgb.0 * max_rgb.1 * max_rgb.2
        } else if max_rgb.0 <= 12 && max_rgb.1 <= 13 && max_rgb.2 <= 14 {
            sum += game;
        }
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_p1() {
        let input = include_str!("../input_example.txt");
        assert_eq!("8", process_p1(input).unwrap());
    }

    #[test]
    fn test_process_p2() {
        let input = include_str!("../input_example.txt");
        assert_eq!("2286", process_p2(input).unwrap());
    }

    #[test]
    fn test_p1() {
        let input = include_str!("../input.txt");
        assert_eq!("2348", process_p1(input).unwrap());
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../input.txt");
        assert_eq!("76008", process_p2(input).unwrap());
    }
}
