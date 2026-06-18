use zoldath::{
    game::party_house::{Guest, GuestType, House},
    rng::Rng,
};

/// Returns the estimated number of frames until winning,
/// or `None` if you don't win.
fn sim(seed: u64, do_debug: bool) -> Option<usize> {
    let mut rng = Rng::new(seed);
    rng.roll_onto(45);
    let mut house = House::new(rng);

    let mut time = 0usize; // number of frames
    let mut bought_star = 0;

    // when pop reaches this value, buy this guest
    let strategy = [
        (7, GuestType::Gambler),
        (7, GuestType::CuteDog),
        (8, GuestType::Writer),
        (7, GuestType::Gambler),
        (7, GuestType::CuteDog),
        (8, GuestType::Writer),
        (8, GuestType::Writer),
        (8, GuestType::Writer),
        (50, GuestType::Mermaid),
        (50, GuestType::Mermaid),
        (50, GuestType::Mermaid),
        (35, GuestType::Mermaid),
    ];
    let mut strat_ptr = 0;

    for day in 0..25 {
        house.start_day();
        if do_debug {
            println!("D{} {house}", 25 - day);
        }

        let mut delta_time = 0;

        let mut invited = 0;
        let mut star = 0;
        let mut trouble = 0;
        let mut flag = 0;

        // greedy invitation
        for i in 0..house.house_size() {
            let gtype = house.cheat_guest(i as usize).gtype();
            // too much trouble, end party
            if gtype.is_trouble() {
                if trouble - flag == 2 {
                    break;
                }
                trouble += 1;
            }
            // stop 100f delay or overflow
            if gtype == GuestType::Mermaid {
                if bought_star < 4 || i == house.house_size() - 1 {
                    break;
                }
                delta_time += 100;
            }

            // invite otherwise
            invited += 1;
            if gtype.is_white_flag() {
                flag += 1;
            }
            if gtype.is_star() {
                star += 1;
                if star == 4 {
                    break;
                }
            }
        }

        // close party
        delta_time += 100;
        if do_debug {
            if invited == house.house_size() {
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
        let mut writers = 0;
        for i in 0..invited as usize {
            let guest = *house.cheat_guest(i);
            delta_time += 2;
            if guest.pop() != 0 {
                house.add_pop(guest.pop());
                delta_time += 7;
            }
            if guest.cash() != 0 {
                house.add_cash(guest.cash());
                delta_time += 7;
            }
            if guest.gtype() == GuestType::Writer {
                writers += 1;
            }
        }
        if writers > 0 {
            house.add_pop(4 * trouble);
            delta_time += 30;
        }

        // transition time
        delta_time += 9 + 3 + 3 + 60 + 36 + 7;

        // shop
        while let Some((threshold, gtype)) = strategy.get(strat_ptr) {
            if house.pop() < *threshold {
                break;
            }
            house.buy_guest(*gtype);
            if gtype.is_star() {
                bought_star += 1;
            }
            if do_debug {
                println!("buy {gtype:?}");
            }
            strat_ptr += 1;
        }
        while house.expand() {
            if do_debug {
                println!("expand");
            }
        }
        delta_time += 30;

        // update time
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
