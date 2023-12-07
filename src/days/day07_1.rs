use std::cmp::{max, Ordering};
use std::collections::BTreeMap;
use std::ops::Index;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum CardType {
    _2, _3, _4, _5, _6, _7, _8, _9,
    _T, _J, _Q, _K, _A,
}

impl CardType {
    pub fn new(symbol: char) -> Self {
        match symbol {
            '2' => Self::_2,
            '3' => Self::_3,
            '4' => Self::_4,
            '5' => Self::_5,
            '6' => Self::_6,
            '7' => Self::_7,
            '8' => Self::_8,
            '9' => Self::_9,
            'T' => Self::_T,
            'J' => Self::_J,
            'Q' => Self::_Q,
            'K' => Self::_K,
            'A' => Self::_A,
            _ => panic!("Invalid character \"{symbol}\"")
        }
    }
}

type HandCards = [CardType; 5];

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum HandRank {
    No,         // no any combination
    HighCard,   // seq like 23456
    OnePair,    // two card with same kind, A236A
    TwoPair,    // two pairs of different kind, 3355Q
    Three,      // three cards of same kind, A2A3A
    FullHouse,  // three and pair cards of different kinds, like AA333
    Four,       // four cards of same kind AQAAA
    Five,       // all five cards have one kind 33333   // most strong
}

impl HandRank {
    pub fn calc_rank(cards: &HandCards) -> HandRank {
        let mut counter = BTreeMap::<CardType, i32>::new();

        for card in cards {
            counter.entry(*card).and_modify(|curr| *curr += 1).or_insert(1);
        }

        let mut counter: Vec<_> = counter.iter().collect();
        counter.sort_by(|(lk, lc), (rk, rc)| (rc, rk).cmp(&(lc, lk)));
        let c1 = counter[0].1;
        if *c1 == 5 {
            return Self::Five;
        }
        let c2 = counter[1].1;

        match (c1, c2) {
            (4, _) => return Self::Four,
            (3, 2) => return Self::FullHouse,
            (3, _) => return Self::Three,
            (2, 2) => return Self::TwoPair,
            (2, _) => return Self::OnePair,
            _ => {}
        }

        Self::No
    }

    pub fn calc_rank_with_jocker(cards: &HandCards) -> HandRank {
        let mut max_rank = Self::calc_rank(cards);

        if max_rank == HandRank::Five {
            return max_rank
        };

        for (idx, card) in cards.iter().enumerate() {
            if card == &CardType::_J {
                for new_card in [CardType::_2, CardType::_3, CardType::_4, CardType::_5, CardType::_6, CardType::_7, CardType::_8, CardType::_9,
                    CardType::_T, CardType::_Q, CardType::_K, CardType::_A] {
                    let mut cards_new = cards.clone();
                    cards_new[idx] = new_card;
                    let rank = Self::calc_rank_with_jocker(&cards_new);
                    max_rank = max_rank.max(rank);
                }

            }

        }

        max_rank
    }

}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: HandCards,
    score: i32,
}


impl Hand {
    pub fn new(cards: HandCards, score: i32) -> Self {
        Self {
            cards,
            score,
        }
    }
}


fn parse_input(input: &str) -> Vec<Hand> {
    let mut games: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let (cards, score) = line.split_once(" ").unwrap();
        let mut cards: Vec<_> = cards.chars().map(|symbol| CardType::new(symbol)).collect();
        let cards = HandCards::try_from(&cards[0..5]).unwrap();
        // println!("{cards:?}");
        assert_eq!(cards.len(), 5);
        let score = score.parse::<i32>().unwrap();

        // println!("{cards:?} {score} {:?}", HandRank::calc_rank_with_jocker(&cards));
        games.push(Hand::new(cards, score));
    }

    games
}

pub fn run1(input: &str) {
    let games = parse_input(input);

    let mut games1: Vec<_> = games.iter().map(|curr| (HandRank::calc_rank(&curr.cards), curr.clone())).collect();

    games1.sort_by(|l, r| {
        let order = l.0.cmp(&r.0);
        if order != Ordering::Equal {
            return order
        }

        l.1.cards.cmp(&r.1.cards)
    });

    let mut sum = 0;
    for (idx, (rank, game)) in games1.iter().enumerate() {
        sum += game.score * (idx+1) as i32;
    }


    println!("{sum}");

}