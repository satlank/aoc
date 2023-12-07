use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};


pub fn read<R: Read>(io: R) -> Vec<(Hand, usize)> {
    let br = BufReader::new(io);
    br.lines()
        .filter_map(Result::ok)
        .map(|line| {
            let mut it = line.split_whitespace();
            let cards = it
                .next()
                .unwrap()
                .chars()
                .map(Card::from)
                .collect::<Vec<_>>();
            let bid = it.next().unwrap().parse::<usize>().unwrap();
            let cards: [Card; 5] = cards.try_into().unwrap();
            (cards.into(), bid)
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    cnts: HashMap<Card, u8>,
}

impl Hand {
    pub fn get_type(&self) -> HandType {
        match self.cnts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if self.cnts.values().any(|&v| v == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if self.cnts.values().any(|&v| v == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unreachable!("Only 5 cards in a hand allowed!"),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.get_type() != other.get_type() {
            return self.get_type().cmp(&other.get_type());
        }
        for i in 0..5 {
            if self.cards[i] < other.cards[i] {
                return Ordering::Less;
            } else if self.cards[i] > other.cards[i] {
                return Ordering::Greater;
            }
        }

        unreachable!()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<[Card; 5]> for Hand {
    fn from(cards: [Card; 5]) -> Self {
        let mut cnts = HashMap::new();
        for card in cards {
            *cnts.entry(card).or_insert(0) += 1;
        }
        Hand { cards, cnts }
    }
}
