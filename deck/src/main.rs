use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
  cards: Vec<String>
}

// Inherent Implementation
// self is a reference to whatever type is mentioned in the parent implementation block
impl Deck {
  fn new() -> Self {
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
    return deck;
  }

  fn shuffle(&self) {

  }
}

fn main() {
  let deck = Deck::new();

  deck.shuffle();

  println!("Heres your deck: {:#?}", deck);
}

