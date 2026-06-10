mod guest;
mod house;

use crate::{guest::GuestType::*, house::House};
use zoldath::rng::Rng;

/// Returns the estimated number of frames until winning,
/// or `None` if you don't win.
fn sim(seed: u64, do_debug: bool) -> Option<usize> {
    let mut rng = Rng::new(seed);
    rng.roll_onto(45);
    let mut house = House::new(rng);

    // number of frames
    let mut time = 0usize;
    let mut bought_star = 0usize;

    for day in 0..25 {
        house.start_day();
        if do_debug {
            println!("D{day} {house:?}");
        }

        let mut delta_time = 0;
        let mut invited = 0usize;
        let mut trouble = 0;
        let mut star = 0;

        // greedy invitation
        for i in 0..house.deck.len().min(house.house_size as usize) {
            let guest = &house.deck[i];
            if (i + 1 == house.house_size as usize || (bought_star <= 3 && star >= 0))
                && guest.gtype == Mermaid
            {
                break;
            }
            if guest.trouble && trouble == 2 {
                break;
            }

            invited += 1;
            delta_time += 1;
            if guest.trouble {
                trouble += 1;
            }
            if guest.gtype == Mermaid {
                delta_time += 100;
            }
            if guest.gtype.is_star() {
                star += 1;
                if star == 4 {
                    break;
                }
            }
        }

        // close party
        delta_time += 100;
        if do_debug {
            if invited == house.house_size as usize {
                println!("invite full {invited}");
            } else {
                println!("invite {invited}");
            }
        }

        // win
        if star >= 4 {
            time += delta_time;
            if do_debug {
                println!("estimated +{delta_time}f => {time}f");
            }
            return Some(time);
        }

        // payout
        for i in 0..invited {
            let guest = house.deck[i];
            if guest.pop != 0 {
                delta_time += 8;
                house.add_pop(guest.pop);
            } else {
                delta_time += 1;
            }
            if guest.cash != 0 {
                delta_time += 8;
                house.add_cash(guest.cash);
            } else {
                delta_time += 1;
            }
        }
        delta_time += 9 + 3 + 3 + 60 + 36 + 7;

        // shop
        let mut cursor = "exit";
        delta_time += 1;
        if bought_star == 0 && house.pop >= 50 {
            // move cursor
            if cursor == "exit" {
                delta_time += 2;
            }
            cursor = "superhero";
            // buy
            house.buy_guest(Superhero);
            bought_star += 1;
            delta_time += 1;
        } else if bought_star == 1 && house.pop >= 57 {
            // move cursor
            if cursor == "exit" {
                delta_time += 3;
            }
            cursor = "mermaid";
            // buy
            house.buy_guest(Mermaid);
            bought_star += 1;
            delta_time += 1;
        } else if bought_star == 2 && house.pop >= 57 {
            // move cursor
            if cursor == "exit" {
                delta_time += 3;
            }
            cursor = "mermaid";
            // buy
            house.buy_guest(Mermaid);
            bought_star += 1;
            delta_time += 1;
        } else if bought_star >= 3 && house.pop >= 35 {
            // move cursor
            if cursor == "exit" {
                delta_time += 3;
            }
            cursor = "mermaid";
            // buy
            house.buy_guest(Mermaid);
            bought_star += 1;
            delta_time += 1;
        }
        while house.cash >= house.expansion_cost() {
            // move cursor
            if cursor == "exit" {
                delta_time += 1;
            } else if cursor == "mermaid" {
                delta_time += 1;
            }
            cursor = "expand";
            // buy
            house.expand();
            delta_time += 1;
        }
        delta_time += 30;

        time += delta_time;
        if do_debug {
            println!("estimated +{delta_time}f => {time}f");
        }
    }
    None
}

const SEED_START: u64 = 0;
const TRIES: u64 = 100_000_000;

fn main() {
    let mut optimum = (999999, 0);
    for seed in SEED_START..SEED_START + TRIES {
        if seed % 2_000_000 == 0 {
            println!("done {seed}");
        }
        let Some(time) = sim(seed, false) else {
            continue;
        };
        if optimum.0 <= time {
            continue;
        }
        optimum = (time, seed);
        println!("{seed} -> {time}");
    }
    let (time, seed) = optimum;
    sim(seed, true);
    println!("BEST: {time}f with {seed}");

    for b in (seed as f64).to_le_bytes() {
        print!("{b:02x} ");
    }
    println!();
}
