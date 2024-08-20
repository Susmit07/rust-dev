use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]  // Deriving the Debug trait for the Deck struct

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    // associated function.
    fn create_standard_deck() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = Vec::new();

        for suit in suits {
            for value in values {
                let card = format!("{} <-> {}", value, suit);
                cards.push(card)
            }
        }
        let deck = Deck { cards };
        return deck;
    }

    //method.
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn display(&self) {
        println!("Deck: {:#?}", self);
    }


    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }


}

// fn push_cards() -> Deck {
//     let suits = ["Hearts", "Spades", "Diamonds"];
//     let values = ["Ace", "Two", "Three"];

//     let mut cards = Vec::new();

//     for suit in suits {
//         for value in values {
//             let card = format!("{} <-> {}", value, suit);
//             cards.push(card)
//         }
//     }
//      return Deck { cards };
// }


 fn main() {
    let mut standard_deck = Deck::create_standard_deck();
    standard_deck.shuffle();
    standard_deck.display();

    let cards = standard_deck.deal(6);
    standard_deck.display();
    
}
