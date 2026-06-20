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

    // when pop reaches this value, buy this guest
    let strategy = [
        (5, GuestType::Rockstar),
        (7, GuestType::Stylist),
        (5, GuestType::Cheerleader),
        (7, GuestType::Stylist),
        (5, GuestType::Cheerleader),
        (57, GuestType::Dinosaur),
        (32, GuestType::Dinosaur),
        (7, GuestType::Counselor),
        (50, GuestType::Dinosaur),
        (25, GuestType::Dinosaur),
    ];
    let mut strat_ptr = 0;
    let mut bought_star = 0;

    for day in 0..13 {
        house.start_day();
        if do_debug {
            println!("D{} {house}", 25 - day);
        }

        let mut delta_time = 0;

        let mut invited = 0;
        let mut star = 0;
        let mut flag = 0;
        let mut trouble = 0;

        let mut counselor = 0;
        let mut counselor_active = 0;
        let mut stylist = 0;
        let mut stylist_active = 0;
        let mut cheerleader_active = 0;
        let mut bartender = 0;

        //////////////////////////// greedy invitation

        for i in 0..house.house_size().min(house.deck_size() as u32) {
            let gtype = house.cheat_guest(i as usize).gtype();
            // too much trouble, use counselor or end party
            if gtype.is_trouble() {
                if trouble - flag == 2 {
                    if counselor_active == 0 {
                        break;
                    }
                    if do_debug {
                        println!("Counsel at {i}");
                    }
                    counselor_active -= 1;
                    trouble = 0;
                }
                trouble += 1;
            }

            // invite otherwise
            invited += 1;
            if gtype == GuestType::Cheerleader {
                cheerleader_active += 1;
            }
            if gtype == GuestType::Counselor {
                counselor += 1;
                counselor_active += 1;
            }
            if gtype == GuestType::Stylist {
                stylist += 1;
                stylist_active += 1;
            }
            if gtype == GuestType::Bartender {
                bartender += 1;
            }
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

        //////////////////////////// decide styling

        loop {
            if stylist == 0 {
                break;
            }
            if stylist_active == 0 && cheerleader_active == 0 {
                break;
            }

            // find guest
            let best_i = house.cheat_deck()[..invited]
                .iter()
                .enumerate()
                .filter(|(_, guest)| guest.pop() < 9)
                .max_by_key(|(_, guest)| {
                    2 * guest.pop() + guest.cash() - 3 * guest.trouble()
                        + 10 * ((guest.gtype() == GuestType::Bartender) as i32)
                        + ((guest.gtype() == GuestType::Cheerleader) as i32)
                })
                .map(|(i, _)| i);
            let Some(best_i) = best_i else {
                break;
            };

            // do style
            let guest = house.cheat_guest_mut(best_i);
            while guest.pop() < 9 {
                if stylist_active == 0 {
                    if cheerleader_active == 0 {
                        break;
                    }
                    cheerleader_active -= 1;
                    counselor_active = counselor;
                    stylist_active = stylist;
                    delta_time += 4;
                    if do_debug {
                        println!("Cheer to {counselor_active} {stylist_active}");
                    }
                }
                guest.increment_base_pop();
                stylist_active -= 1;
                delta_time += 7;
                if do_debug {
                    println!("Style {best_i} to {guest}");
                }
            }
        }

        //////////////////////////// close party

        delta_time += 100;
        if do_debug {
            let net_invited = invited as u32;
            if net_invited == house.house_size() {
                println!("invite full {net_invited}");
            } else {
                println!("invite {net_invited}");
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
        for i in 0..invited as usize {
            let guest = *house.cheat_guest(i);
            delta_time += 2;
            if guest.pop() > 0 {
                house.add_pop(guest.pop());
                delta_time += 7;
            }
            if guest.cash() > 0 {
                house.add_cash(guest.cash());
                delta_time += 7;
            }
        }
        if bartender > 0 {
            house.add_cash(2 * trouble * bartender);
            delta_time += 30;
        }

        // payment
        for i in 0..invited as usize {
            let guest = *house.cheat_guest(i);
            if guest.pop() < 0 {
                house.add_pop(guest.pop());
                delta_time += 7;
            }
            if guest.cash() < 0 {
                house.add_cash(guest.cash());
                delta_time += 7;
            }
        }

        // transition time
        delta_time += 9 + 3 + 3 + 60 + 36 + 7;

        //////////////////////////// shop

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

        //////////////////////////// end day

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
    assert_ne!(time, 999999);

    sim(seed, true);
    println!("BEST: {time}f with {seed}");

    for b in (seed as f64).to_le_bytes() {
        print!("{b:02x} ");
    }
    println!();
}
