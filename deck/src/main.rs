use rand::{rng, seq::SliceRandom};

#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
        let values = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards } // Vec::new()
    }

    fn shuffle(&mut self) {
        let mut t_rng = rng();
        self.cards.shuffle(&mut t_rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    let cards = deck.deal(3);

    println!("Here is your deck: {:#?}", deck);
    println!("Here is your hand: {:#?}", cards);
}
