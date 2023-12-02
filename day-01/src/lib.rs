use phf::phf_map;
use regex::Regex;

pub fn process_p1(input: &str) -> Result<String, String> {
    let re = Regex::new(r"\D*(\d).*?(\d)?\D*$").unwrap();
    let mut sum = 0;
    let v = aoc_lib::rows_to_vector(input);

    for r in v {
        let caps = re.captures(r).unwrap();
        let first = &caps[1];
        let first_int = first.parse::<i32>().unwrap();
        sum += 10 * first_int;
        let last = caps.get(2);
        // dbg!(r, first, last);

        if let Some(l) = last {
            let last = l.as_str();
            sum += last.parse::<i32>().unwrap();
            // dbg!(sum, first, last);
        } else {
            sum += first_int;
        }
    }
    Ok(sum.to_string())
}

pub fn process_p2(input: &str) -> Result<String, String> {
    let mut sum = 0;
    let v = aoc_lib::rows_to_vector(input);

    for r in v {
        let (f, l) = find_first_and_last(r);
        sum += 10 * f;
        sum += l;
    }
    Ok(sum.to_string())
}

static KNOWN_NUMBERS: phf::Map<&'static str, i32> = phf_map! {
   "one" => 1,
    "two" =>2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

fn get_as_int(input: &str) -> Option<i32> {
    let ch = input.chars().next().unwrap();
    if ch.is_ascii_digit() {
        return Some(ch as i32 - '0' as i32);
    }

    for key in KNOWN_NUMBERS.keys() {
        if input.starts_with(key) {
            return Some(*KNOWN_NUMBERS.get(key).unwrap());
        }
    }
    None
}

fn find_first_and_last(input: &str) -> (i32, i32) {
    let mut first_index = 0;
    let mut first: Option<i32> = None;
    let mut last: Option<i32> = None;

    let len = input.len();

    for (i, _) in input.chars().enumerate() {
        let end_offset: usize = len - 1 - i;
        let start_slice = &input[i..];
        let end_slice = &input[end_offset..];

        if end_offset < first_index {
            break;
        }

        if first.is_none() {
            first = get_as_int(start_slice);
            if first.is_some() {
                first_index = i;
            }
        }

        if last.is_none() {
            last = get_as_int(end_slice);
        }

        if first.is_some() && last.is_some() {
            break;
        }
    }

    if first.is_none() {
        return (last.unwrap(), last.unwrap());
    }

    if last.is_none() {
        return (first.unwrap(), first.unwrap());
    }

    (first.unwrap(), last.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_p1() {
        let input = include_str!("../input_example.txt");
        assert_eq!("142", process_p1(input).unwrap());
    }

    #[test]
    fn test_process_p2() {
        let input = include_str!("../input_example2.txt");
        assert_eq!("281", process_p2(input).unwrap());
    }

    #[test]
    fn test_p1() {
        let input = include_str!("../input.txt");
        assert_eq!("53334", process_p1(input).unwrap());
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../input.txt");
        assert_eq!("52834", process_p2(input).unwrap());
    }
}
