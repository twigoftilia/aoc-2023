use std::{
    collections::{HashMap, HashSet},
    usize,
};

pub fn process_p1(input: &str) -> Result<String, String> {
    let mut sum = 0;
    let cards = cards(input);
    // dbg!(&cards);
    for card in cards.values() {
        let overlaps = card.numbers.intersection(&card.correct).clone().count();
        if overlaps > 0 {
            sum += u32::pow(2, overlaps as u32 - 1);
        }
    }
    Ok(sum.to_string())
}

pub fn process_p2(input: &str) -> Result<String, String> {
    let cards = cards(input);
    // dbg!(&cards);
    let no_cards = cards.len();
    let mut rev = cards.values().map(|s| s.id).collect::<Vec<usize>>();
    rev.sort();
    let mut cardworths: HashMap<usize, usize> = HashMap::new();

    for iter_no in 0..(no_cards) {
        let card_pos = no_cards - 1 - iter_no;
        // dbg!(iter_no, card_pos, card_id);
        let mut cardworth = 1;
        let card = cards.get(&rev[card_pos]).unwrap();
        let overlaps = card.numbers.intersection(&card.correct).clone().count();
        // dbg!(overlaps, start_add_card_pos, end_add_card_pos);
        for added_card_pos in (card_pos + 1)..((card_pos + 1 + overlaps).min(no_cards)) {
            // dbg!(added_card_pos);
            cardworth += cardworths.get(&added_card_pos).unwrap();
        }
        cardworths.insert(card_pos, cardworth);
    }
    dbg!(&cardworths);
    Ok(cardworths.values().sum::<usize>().to_string())
}

fn cards(input: &str) -> HashMap<usize, Card> {
    let mut cards = HashMap::new();
    for line in input.lines() {
        //        dbg!(&line);
        let mut part = line.split([':', '|']);

        let card = Card {
            id: part
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap(),

            // dbg!(part.next().unwrap().split(" ").);
            numbers: part
                .next()
                .unwrap()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    // dbg!(s);
                    s.parse().unwrap()
                })
                .collect(),
            correct: part
                .next()
                .unwrap()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    // dbg!(s);
                    s.parse().unwrap()
                })
                .collect(),
        };

        cards.insert(card.id, card);
        // dbg!(&cards);
    }
    cards
}

#[derive(Debug, Default)]
struct Card {
    id: usize,
    numbers: HashSet<usize>,
    correct: HashSet<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_p1() {
        let input = include_str!("../input_example.txt");
        assert_eq!("13", process_p1(input).unwrap());
    }

    #[test]
    fn test_process_p2() {
        let input = include_str!("../input_example.txt");
        assert_eq!("30", process_p2(input).unwrap());
    }

    #[test]
    fn test_p1() {
        let input = include_str!("../input.txt");
        assert_eq!("33950", process_p1(input).unwrap());
    }

    #[test]
    fn test_p2() {
        let input = include_str!("../input.txt");
        assert_eq!("14814534", process_p2(input).unwrap());
    }
}
