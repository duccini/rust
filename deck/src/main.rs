#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

fn main() {
    // List of 'suits' - 'hearts', 'spades'
    let suits = ["Hearts", "Spades", "Diamonds"];

    // List of 'values' - 'ace', 'two'
    let values = ["Ace", "Two", "Three"];

    let mut cards = vec![];

    // Double nested for loop
    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    let deck = Deck { cards };

    println!("Heres your deck: {:#?}", deck);
}

