use crate::cards::Card;

pub struct Player {
    hand: Vec<Card>,
}

impl Player {
    pub fn new(hand: Vec<Card>) -> Self {
        Self { hand }
    }

    pub fn bust(&self) -> bool {
        let mut total = 0;
        for card in &self.hand {
            if card.value() > 10 {
                total += 10;
            } else {
                total += card.value();
            }
        }
        if total > 21 {
            true
        } else {
            false
        }
    }
}
