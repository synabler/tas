#![allow(dead_code)]

use crate::guest::{
    Guest,
    GuestType::{self, *},
};
use zoldath::Rng;

#[derive(Clone, Debug)]
pub struct House {
    pub deck: Vec<Guest>,

    pub pop: u32,
    pub cash: u32,
    pub house_size: u32,

    pub rng: Rng,
}

impl House {
    pub fn new(rng: Rng) -> Self {
        let deck = [
            WildBuddy, WildBuddy, WildBuddy, WildBuddy, RichPal, RichPal, OldFriend, OldFriend,
            OldFriend, OldFriend,
        ]
        .into_iter()
        .map(|gtype| Guest::new(gtype))
        .collect();
        Self {
            deck,
            pop: 0,
            cash: 0,
            house_size: 5,
            rng,
        }
    }

    fn shuffle_deck(&mut self) {
        self.rng.shuffle(&mut self.deck);
    }

    pub fn start_day(&mut self) {
        self.shuffle_deck();
        for guest in &mut self.deck {
            guest.trouble = guest.gtype.is_trouble();
        }
    }

    pub fn expansion_cost(&self) -> u32 {
        (self.house_size - 3).min(12)
    }

    pub fn add_pop(&mut self, amt: i32) {
        let new_pop = self.pop.saturating_add_signed(amt);
        self.pop = new_pop.min(65);
    }

    pub fn add_cash(&mut self, amt: i32) {
        let Some(new_cash) = self.cash.checked_add_signed(amt) else {
            self.add_pop(-7);
            self.cash = 0;
            return;
        };
        self.cash = new_cash.min(30);
    }

    pub fn buy_guest(&mut self, gtype: GuestType) -> bool {
        let cost = gtype.cost();
        if self.pop < cost {
            return false;
        }
        self.pop -= cost;
        self.deck.push(Guest::new(gtype));
        self.rng.roll();
        true
    }

    pub fn expand(&mut self) -> bool {
        let cost = self.expansion_cost();
        if self.cash < cost {
            return false;
        }
        self.cash -= cost;
        self.house_size += 1;
        true
    }
}
