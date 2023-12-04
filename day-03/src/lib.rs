use regex::Regex;
use std::collections::HashMap;
use aoc_lib::intersects;

pub fn process_p1(input: &str) -> Result<String, String> {
    let mut sum = 0;

    let re_n = Regex::new(r"(\d+)").unwrap();
    let re_p = Regex::new(r"([^\.0-9]+)").unwrap();

    let lines = aoc_lib::rows_to_vector(input);
    let parts = get_islandsmap_from_re(re_p, &lines);
    let numbers = get_islandsmap_from_re(re_n, &lines);

    for number_rowno in numbers.tree.keys() {
        for number_key in numbers.tree.get(number_rowno).unwrap().keys() {
            let parts_x_test_start = if number_key.0 > 0 {
                number_key.0 - 1
            } else {
                0
            };
            let parts_row_test_start = if *number_rowno > 0 {
                number_rowno - 1
            } else {
                0
            };

            'outer: for check_part_row in parts_row_test_start..=(number_rowno + 1) {
                if parts.tree.contains_key(&check_part_row) {
                    for part_key in parts.tree.get(&check_part_row).unwrap().keys() {
                        if intersects(part_key.0, part_key.1, parts_x_test_start, number_key.1 + 1)
                        {
                            let number_to_add = numbers
                                .tree
                                .get(number_rowno)
                                .unwrap()
                                .get(number_key)
                                .unwrap()
                                .parse::<i32>()
                                .unwrap();
                            sum += number_to_add;
                            break 'outer;
                        }
                    }
                }
            }
        }
    }

    Ok(sum.to_string())
}

pub fn process_p2(input: &str) -> Result<String, String> {
    let mut sum = 0;

    let re_n = Regex::new(r"(\d+)").unwrap();
    let re_gears = Regex::new(r"(\*)").unwrap();

    let lines = aoc_lib::rows_to_vector(input);
    let gears = get_islandsmap_from_re(re_gears, &lines);
    let numbers = get_islandsmap_from_re(re_n, &lines);

    for gear_rowno in gears.tree.keys() {
        for gear_key in gears.tree.get(gear_rowno).unwrap().keys() {
            let mut gears_adj_numbers = vec![];
            let number_x_test_start = if gear_key.0 > 0 { gear_key.0 - 1 } else { 0 };
            let number_row_test_start = if *gear_rowno > 0 { gear_rowno - 1 } else { 0 };

            for check_number_row in number_row_test_start..=(gear_rowno + 1) {
                if numbers.tree.contains_key(&check_number_row) {
                    for number_key in numbers.tree.get(&check_number_row).unwrap().keys() {
                        if intersects(
                            number_key.0,
                            number_key.1,
                            number_x_test_start,
                            gear_key.1 + 1,
                        ) {
                            let number_to_add = numbers
                                .tree
                                .get(&check_number_row)
                                .unwrap()
                                .get(number_key)
                                .unwrap()
                                .parse::<i32>()
                                .unwrap();
                            gears_adj_numbers.push(number_to_add);
                        }
                    }
                }
            }

            if gears_adj_numbers.len() == 2 {
                sum += gears_adj_numbers[0] * gears_adj_numbers[1];
            }
        }
    }

    Ok(sum.to_string())
}

#[derive(Debug, Default)]
struct Islands {
    tree: HashMap<usize, HashMap<(usize, usize), String>>,
}

fn get_islandsmap_from_re(re: Regex, lines: &[&str]) -> Islands {
    let mut islands = Islands::default();

    for (row, line) in lines.iter().enumerate() {
        let char_line_map = re.find_iter(line).fold(
            HashMap::new(),
            |mut map: HashMap<(usize, usize), String>, m| {
                map.insert((m.start(), m.end() - 1), m.as_str().to_string());
                map
            },
        );
        if !char_line_map.is_empty() {
            islands.tree.insert(row, char_line_map);
        }
    }
    islands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_p1() {
        let input = include_str!("../input_example.txt");
        assert_eq!("4361", process_p1(input).unwrap());
    }

    #[test]
    fn test_process_p2() {
        let input = include_str!("../input_example.txt");
        assert_eq!("467835", process_p2(input).unwrap());
    }

    #[test]
    fn test_p1() {
        let input = include_str!("../input.txt");
        assert_eq!("532445", process_p1(input).unwrap());
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../input.txt");
        assert_eq!("79842967", process_p2(input).unwrap());
    }
}
