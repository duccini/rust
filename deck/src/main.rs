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

  fn shuffle(&mut self) {
    let mut rng = rng();
    self.cards.shuffle(&mut rng); // recursive function?
  }

  fn deal(&mut self, num_cards: usize) -> Vec<String> {
    // split_off: creates a new vector with the elements extracted from the elments
    // at index given to the end of the array
    self.cards.split_off(self.cards.len() - num_cards)
  }
}

fn main() {
  let mut deck = Deck::new();

  println!("Heres your deck: {:#?}", deck);

  deck.shuffle();

  let cards = deck.deal(3);

  println!("Heres your deck: {:#?}", deck);
  println!("Heres your hand: {:#?}", cards);
}

