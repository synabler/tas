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
    Alien,
}

impl Debug for GuestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WildBuddy => write!(f, "WIL"),
            Self::OldFriend => write!(f, "OLD"),
            Self::RichPal => write!(f, "RIC"),
            Self::CuteDog => write!(f, "CUT"),
            Self::Alien => write!(f, "*ALI"),
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
        matches!(self, Self::Alien)
    }

    /// Base popularity payout by default.
    pub fn default_pop(&self) -> i32 {
        match self {
            GuestType::WildBuddy => 2,
            GuestType::OldFriend => 1,
            GuestType::RichPal => 0,
            GuestType::CuteDog => 2,
            GuestType::Alien => 0,
        }
    }

    /// Base cash payout by default.
    pub fn default_cash(&self) -> i32 {
        match self {
            GuestType::WildBuddy => 0,
            GuestType::OldFriend => 0,
            GuestType::RichPal => 1,
            GuestType::CuteDog => 0,
            GuestType::Alien => 0,
        }
    }

    /// Popularity cost to add the guest to rolodex.
    pub fn cost(&self) -> u32 {
        match self {
            GuestType::WildBuddy => 0,
            GuestType::OldFriend => 2,
            GuestType::RichPal => 3,
            GuestType::CuteDog => 7,
            GuestType::Alien => 40,
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
