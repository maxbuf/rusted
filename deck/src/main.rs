use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits: [&str; 4] = ["Hearts", "Spades", "Diamnonds", "Clubs"];
        let values: [&str; 13] = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];
        
        let mut cards: Vec<String> = vec![];

        for suit in suits {
            for value in values {
                let card: String = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck: Deck = Deck::new();
    deck.shuffle();

    let hand = deck.deal(1);
    
    println!("Here's your hand: {:#?}", hand);
    println!("Here's your deck: {:#?}", deck);
}
