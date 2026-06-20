use crate::rng::Rng;

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum GuestType {
    WildBuddy,
    OldFriend,
    RichPal,

    // 3
    Driver,
    // 4
    TicketTaker,
    Hippy,
    // 5
    Rockstar,
    Comedian,
    Caterer,
    Cheerleader,
    // 6
    Gangster,
    Athlete,
    // 7
    CuteDog,
    Gambler,
    Stylist,
    Counselor,
    // 8
    Writer,
    // 9
    Auctioneer,
    // 11
    Bartender,

    // stars
    Dinosaur,
    Mermaid,
    Alien,
    Superhero,
}

impl GuestType {
    /// Returns an emoji that depicts this guest.
    ///
    /// **Note:** it's arbitrarily chosen by the crate's author.
    /// Not the official source.
    pub fn emoji(&self) -> &str {
        match self {
            Self::WildBuddy => "🤪",
            Self::OldFriend => "👨",
            Self::RichPal => "💵",
            Self::Driver => "🚕",
            Self::TicketTaker => "🎫",
            Self::Hippy => "😎",
            Self::Rockstar => "🎸",
            Self::Comedian => "🤣",
            Self::Caterer => "🍽️",
            Self::Cheerleader => "🎉",
            Self::Gangster => "🔫",
            Self::Athlete => "🏀",
            Self::CuteDog => "🐩",
            Self::Gambler => "🎰",
            Self::Stylist => "💈",
            Self::Counselor => "🙂",
            Self::Writer => "📝",
            Self::Auctioneer => "💰",
            Self::Bartender => "🍷",
            Self::Dinosaur => "🦖",
            Self::Mermaid => "🧜‍",
            Self::Alien => "👽",
            Self::Superhero => "🦸",
        }
    }

    /// Returns the popularity cost to buy this guest.
    /// Wild Buddy is assumed to have cost 0, reflecting the in-game implementation.
    pub fn cost(&self) -> u32 {
        match self {
            Self::WildBuddy => 0,
            Self::OldFriend => 2,
            Self::RichPal => 3,
            Self::Driver => 3,
            Self::TicketTaker => 4,
            Self::Hippy => 4,
            Self::Rockstar => 5,
            Self::Comedian => 5,
            Self::Caterer => 5,
            Self::Cheerleader => 5,
            Self::Gangster => 6,
            Self::Athlete => 6,
            Self::CuteDog => 7,
            Self::Gambler => 7,
            Self::Stylist => 7,
            Self::Counselor => 7,
            Self::Writer => 8,
            Self::Auctioneer => 9,
            Self::Bartender => 11,
            Self::Dinosaur => 25,
            Self::Mermaid => 35,
            Self::Alien => 40,
            Self::Superhero => 50,
        }
    }

    /// Returns the base popularity this guest gives by default.
    /// Does not include bonus pop (e.g. Dancer).
    pub fn base_pop(&self) -> i32 {
        match self {
            Self::WildBuddy => 2,
            Self::OldFriend => 1,
            Self::RichPal => 0,
            Self::Driver => 0,
            Self::TicketTaker => -1,
            Self::Hippy => 1,
            Self::Rockstar => 3,
            Self::Comedian => 0,
            Self::Caterer => 4,
            Self::Cheerleader => 1,
            Self::Gangster => 0,
            Self::Athlete => 1,
            Self::CuteDog => 2,
            Self::Gambler => 2,
            Self::Stylist => 0,
            Self::Counselor => 0,
            Self::Writer => 1,
            Self::Auctioneer => 0,
            Self::Bartender => 1,
            Self::Dinosaur => 0,
            Self::Mermaid => 0,
            Self::Alien => 0,
            Self::Superhero => 3,
        }
    }

    /// Returns the base cash this guest gives by default.
    /// Does not include bonus cash (e.g. Bartender).
    pub fn base_cash(&self) -> i32 {
        match self {
            Self::WildBuddy => 0,
            Self::OldFriend => 0,
            Self::RichPal => 1,
            Self::Driver => 0,
            Self::TicketTaker => 2,
            Self::Hippy => 0,
            Self::Rockstar => 2,
            Self::Comedian => -1,
            Self::Caterer => -1,
            Self::Cheerleader => 0,
            Self::Gangster => 4,
            Self::Athlete => 1,
            Self::CuteDog => 0,
            Self::Gambler => 3,
            Self::Stylist => -1,
            Self::Counselor => 0,
            Self::Writer => 0,
            Self::Auctioneer => 3,
            Self::Bartender => 0,
            Self::Dinosaur => 0,
            Self::Mermaid => 0,
            Self::Alien => 0,
            Self::Superhero => 0,
        }
    }

    /// Returns whether this guest is trouble by default.
    /// Does not include Werewolf.
    pub fn is_trouble(&self) -> bool {
        match self {
            Self::WildBuddy => true,
            Self::OldFriend => false,
            Self::RichPal => false,
            Self::Driver => false,
            Self::TicketTaker => false,
            Self::Hippy => false,
            Self::Rockstar => true,
            Self::Comedian => false,
            Self::Caterer => false,
            Self::Cheerleader => false,
            Self::Gangster => true,
            Self::Athlete => false,
            Self::CuteDog => false,
            Self::Gambler => true,
            Self::Stylist => false,
            Self::Counselor => false,
            Self::Writer => false,
            Self::Auctioneer => false,
            Self::Bartender => false,
            Self::Dinosaur => true,
            Self::Mermaid => false,
            Self::Alien => false,
            Self::Superhero => false,
        }
    }

    /// Returns whether this guest is white flag (cancels 1 trouble).
    pub fn is_white_flag(&self) -> bool {
        match self {
            Self::WildBuddy => false,
            Self::OldFriend => false,
            Self::RichPal => false,
            Self::Driver => false,
            Self::TicketTaker => false,
            Self::Hippy => true,
            Self::Rockstar => false,
            Self::Comedian => false,
            Self::Caterer => false,
            Self::Cheerleader => false,
            Self::Gangster => false,
            Self::Athlete => false,
            Self::CuteDog => true,
            Self::Gambler => false,
            Self::Stylist => false,
            Self::Counselor => false,
            Self::Writer => false,
            Self::Auctioneer => false,
            Self::Bartender => false,
            Self::Dinosaur => false,
            Self::Mermaid => false,
            Self::Alien => false,
            Self::Superhero => false,
        }
    }

    /// Returns whether this guest is a star.
    pub fn is_star(&self) -> bool {
        self.cost() >= 25
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Guest {
    gtype: GuestType,
    pop: i32,
    cash: i32,

    // Whether the guest is *currently* a trouble.
    // This can be removed by a counselor.
    trouble: bool,
    invited: bool,
    // Only used by Werewolf.
    morphed: bool,
}

impl std::fmt::Display for Guest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.gtype.emoji())?;
        if self.trouble {
            write!(f, "!")?;
        }
        if self.gtype.is_star() {
            write!(f, "⭐")?;
        }
        if self.pop != 0 {
            write!(f, "^{}", self.pop)?;
        }
        if self.cash != 0 {
            write!(f, "${}", self.cash)?;
        }
        Ok(())
    }
}

impl Guest {
    pub fn new(gtype: GuestType) -> Self {
        Self {
            gtype,
            pop: gtype.base_pop(),
            cash: gtype.base_cash(),
            trouble: gtype.is_trouble(),
            invited: false,
            morphed: false,
        }
    }

    pub fn gtype(&self) -> GuestType {
        self.gtype
    }

    pub fn pop(&self) -> i32 {
        self.pop
    }

    pub fn cash(&self) -> i32 {
        self.cash
    }

    pub fn trouble(&self) -> i32 {
        self.trouble as i32
    }

    /// Resets the guest status for a new day.
    pub fn reset(&mut self) {
        self.trouble = self.gtype.is_trouble();
        self.invited = false;
    }

    /// Increments its base pop by 1 (by Stylist or Climber), up to 9.
    pub fn increment_base_pop(&mut self) {
        self.pop = (self.pop + 1).min(9);
    }
}

#[derive(Clone, Debug)]
pub struct House {
    rng: Rng,
    deck: Vec<Guest>,

    pop: u32,
    cash: u32,
    house_size: u32,

    // internal variables
    ptr: usize,
    total_invited: u32,
    total_trouble: i32,
}

impl std::fmt::Display for House {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "^{} ${} [{}] [", self.pop, self.cash, self.house_size)?;
        for guest in &self.deck {
            write!(f, "{guest} ")?;
        }
        write!(f, "] {:?}", self.rng)?;
        Ok(())
    }
}

impl House {
    pub fn new(rng: Rng) -> Self {
        use GuestType::*;
        let deck = [
            WildBuddy, WildBuddy, WildBuddy, WildBuddy, RichPal, RichPal, OldFriend, OldFriend,
            OldFriend, OldFriend,
        ]
        .into_iter()
        .map(|gtype| Guest::new(gtype))
        .collect();
        Self {
            rng,
            deck,
            pop: 0,
            cash: 0,
            house_size: 5,
            ptr: 0,
            total_invited: 0,
            total_trouble: 0,
        }
    }

    pub fn pop(&self) -> u32 {
        self.pop
    }

    pub fn cash(&self) -> u32 {
        self.cash
    }

    pub fn house_size(&self) -> u32 {
        self.house_size
    }

    pub fn deck_size(&self) -> usize {
        self.deck.len()
    }

    /// Adds `amt` pop, clamped into range `[0, 65]`.
    pub fn add_pop(&mut self, amt: i32) {
        self.pop = self.pop.saturating_add_signed(amt).min(65);
    }

    /// Adds `cash` pop, clamped into range `[0, 30]`,
    /// and deducting 7 pop if cash tries to go negative.
    pub fn add_cash(&mut self, amt: i32) {
        let Some(new_cash) = self.cash.checked_add_signed(amt) else {
            self.cash = 0;
            self.add_pop(-7);
            return;
        };
        self.cash = new_cash.min(30);
    }

    /// Returns whether the house can expand (smaller than 34).
    pub fn can_expand(&self) -> bool {
        self.house_size < 34
    }

    /// Returns cash cost to expand the house by 1.
    /// This does not reflect the fact that the house cannot expand beyond 34.
    pub fn expansion_cost(&self) -> u32 {
        (self.house_size - 3).min(12)
    }

    /// Tries to purchase this guest type.
    /// Returns whether it succeeded, i.e. there was enough pop.
    /// Does not consider whether there were already 4 of the same type.
    pub fn buy_guest(&mut self, gtype: GuestType) -> bool {
        let cost = gtype.cost();
        if self.pop < cost {
            return false;
        }
        self.pop -= cost;
        self.deck.push(Guest::new(gtype));
        // TODO: fix this for old friends, etc.
        self.rng.roll();
        true
    }

    /// Tries to expand the house.
    /// Returns whether it succeede, i.e. there was enough cash and space.
    pub fn expand(&mut self) -> bool {
        let cost = self.expansion_cost();
        if self.cash < cost || !self.can_expand() {
            return false;
        }
        self.cash -= cost;
        self.house_size += 1;
        true
    }

    fn shuffle_deck(&mut self) {
        self.rng.shuffle(&mut self.deck);
    }

    pub fn start_day(&mut self) {
        self.shuffle_deck();
        for guest in &mut self.deck {
            guest.reset();
        }
        self.ptr = 0;
        self.total_invited = 0;
        self.total_trouble = 0;
    }

    // -------- INTERNAL UTILITIES ---------

    // -------- TAS UTILITIES ---------

    /// Returns the deck in order. This should only be used for TAS.
    pub fn cheat_deck(&self) -> &[Guest] {
        &self.deck
    }

    /// Returns the guest at position `i`. This should only be used for TAS.
    pub fn cheat_guest(&self, i: usize) -> &Guest {
        &self.deck[i]
    }

    /// Returns the guest at position `i`. This should only be used for TAS.
    pub fn cheat_guest_mut(&mut self, i: usize) -> &mut Guest {
        &mut self.deck[i]
    }
}
