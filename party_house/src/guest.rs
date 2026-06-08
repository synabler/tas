#![allow(dead_code)]

use std::fmt::Debug;

/// TODO: add all types.
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum GuestType {
    // defaults
    WildBuddy,
    OldFriend,
    RichPal,

    // intermediates
    CuteDog,

    // stars
    Mermaid,
    Alien,
    Superhero,
}

impl Debug for GuestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WildBuddy => write!(f, "WIL"),
            Self::OldFriend => write!(f, "OLD"),
            Self::RichPal => write!(f, "RIC"),
            Self::CuteDog => write!(f, "CUT"),
            Self::Mermaid => write!(f, "*MER"),
            Self::Alien => write!(f, "*ALI"),
            Self::Superhero => write!(f, "*SUP"),
        }
    }
}

impl GuestType {
    /// Whether the guest is potentially a trouble.
    pub fn is_trouble(&self) -> bool {
        matches!(self, Self::WildBuddy)
    }

    /// Whether the guest is a white flag (cancels a trouble).
    pub fn is_flag(&self) -> bool {
        matches!(self, Self::CuteDog)
    }

    /// Whether the guest is a star.
    pub fn is_star(&self) -> bool {
        self.cost() >= 20
    }

    /// Base popularity payout by default.
    pub fn default_pop(&self) -> i32 {
        match self {
            Self::WildBuddy => 2,
            Self::OldFriend => 1,
            Self::RichPal => 0,
            Self::CuteDog => 2,
            Self::Mermaid => 0,
            Self::Alien => 0,
            Self::Superhero => 3,
        }
    }

    /// Base cash payout by default.
    pub fn default_cash(&self) -> i32 {
        match self {
            Self::WildBuddy => 0,
            Self::OldFriend => 0,
            Self::RichPal => 1,
            Self::CuteDog => 0,
            Self::Mermaid => 0,
            Self::Alien => 0,
            Self::Superhero => 0,
        }
    }

    /// Popularity cost to add the guest to rolodex.
    pub fn cost(&self) -> u32 {
        match self {
            Self::WildBuddy => 0,
            Self::OldFriend => 2,
            Self::RichPal => 3,
            Self::CuteDog => 7,
            Self::Mermaid => 35,
            Self::Alien => 40,
            Self::Superhero => 50,
        }
    }
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Guest {
    pub gtype: GuestType,
    pub pop: i32,
    pub cash: i32,
    // Whether the guest is *currently* a trouble.
    // This can be removed by a counselor.
    pub trouble: bool,
}

impl Debug for Guest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.gtype)?;
        if self.trouble {
            write!(f, "!")?;
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
            pop: gtype.default_pop(),
            cash: gtype.default_cash(),
            trouble: gtype.is_trouble(),
        }
    }
}
