use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Debug)]
struct Card {
    pub id: i32,
    pub winning: BTreeSet<i32>,
    pub numbers: Vec<i32>,
}

impl Card {
    pub fn new(id: i32, winning: BTreeSet<i32>, numbers: Vec<i32>) -> Self {
        Self {
            id,
            winning,
            numbers,
        }
    }
}

fn parse_input(input: &str) -> Vec<Card> {
    let mut result = Vec::new();

    for line in input.lines() {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let (card_str, num_info_str) = line.split_once(": ").unwrap();
        let (_, card_id) = card_str.trim().split_once(" ").unwrap();
        let id = card_id.trim().parse::<i32>().unwrap();
        let (winning, numbers) = num_info_str.split_once(" | ").unwrap();
        let mut winning_set = BTreeSet::new();
        for n in winning.trim().split_whitespace() {
            let n = n.trim().parse::<i32>().unwrap();
            if !winning_set.insert(n) {
                panic!("duplicate winning number! {n} in {line}")
            }
        }
        let mut numbers_vec = Vec::new();
        for n in numbers.trim().split_whitespace() {
            let n = n.trim().parse::<i32>().unwrap();
            numbers_vec.push(n);
        }

        result.push(Card::new(id, winning_set, numbers_vec));
    }

    result
}

pub fn run(input: &String) {
    let cards = parse_input(input);

    let mut sum = 0;
    let mut cache = BTreeMap::new();

    for card in &cards {
        let mut count = 0;
        for n in card.numbers.iter() {
            if card.winning.contains(&n) {
                count += 1;
            }
        }

        cache.insert(card.id, count);

        if count > 0 {
            sum += 2i32.pow(count-1)
        }
    }

    println!("first stage - {sum}");


    let mut sum = 0;
    let mut queue = VecDeque::<i32>::new();

    for k in cache.keys() {
        queue.push_back(*k);
    }

    while queue.len() > 0 {
        let item: i32 = queue.pop_front().unwrap();
        sum += 1;

        let winning = cache.get(&item).unwrap();
        for i in item+1..=(item+(*winning as i32)) {
            if cache.contains_key(&i) {
                queue.push_back(i);
            }
        }
    }

    println!("second stage - {sum}");
}

