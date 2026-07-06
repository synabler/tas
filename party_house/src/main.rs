use zoldath::{
    game::party_house::{Guest, GuestType, House},
    rng::Rng,
};

#[derive(Default)]
struct HouseResult {
	delta_time: usize,
	worth: u32,
	invited: u32,
	
	magician: u32,
	bought_stars: u32,
	star: u32,
}

fn do_party(house: &mut House) -> HouseResult {
	let mut delta_time = 0usize;

	let mut invited = 0;
	let mut star = 0;
	let mut flag = 0;
	let mut trouble = 0;
	
	let mut introv = 0;
	let mut magician = 0;
	
	let bought_stars = house.cheat_deck().iter()
		.filter(|guest| guest.gtype().is_star())
		.count() as u32;

	//////////////////////////// greedy invitation

	for i in 0..house.deck_size() as u32 {
		let gtype = house.cheat_guest(i as usize).gtype();
		// too much trouble, end party
		if gtype.is_trouble() {
			if trouble == 2 {
				break;
			}
			trouble += 1;
		}

		// invite otherwise
		invited += 1;
		if gtype == GuestType::Ghost {
			invited -= 1;
		}
		if gtype == GuestType::Introvert {
			introv += 1;
			if introv == 4 && bought_stars < 4 { break; }
		}
		if gtype == GuestType::Magician {
			magician += 1;
		}
		if gtype.is_star() {
			star += 1;
			if star == 4 {
				break;
			}
		}
		
		// full
		if invited == house.house_size() {
			break;
		}
	}

	//////////////////////////// decide magic
	
	if bought_stars >= 4 && star + magician >= 4 {
		star = 4;	
	}

	//////////////////////////// close party

	delta_time += 100;

	// win
	if star >= 4 {
		return HouseResult {
			delta_time,
			worth: 999999,
			..Default::default()
		};
	}

	// payout
	for i in 0..(invited + star) as usize {
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
	if introv > 0 {
		let blank = (house.house_size() - invited) as i32;
		house.add_pop(blank * introv);
		delta_time += 30;
	}

	// payment
	for i in 0..(invited + star) as usize {
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
	
	let mut worth = house.pop() + 3*house.cash();
	if bought_stars != star && magician == 0{
		worth = 0;
	}
	HouseResult {
		delta_time,
		worth,
		invited,
		magician,
		bought_stars,
		star,
	}
}

/// Returns the estimated number of frames until winning,
/// or `None` if you don't win.
fn sim(seed: u64, do_debug: bool) -> Option<usize> {
    let mut rng = Rng::new(seed);
    rng.roll_onto(45);
    let mut house = House::new(rng);

    let mut time = 0usize; // number of frames

    // when pop reaches this value, buy this guest
    let strategy = [
        (7, GuestType::Gambler),
        (5, GuestType::Magician),
        (7, GuestType::Gambler),
        (4, GuestType::Introvert),
        (4, GuestType::Introvert),
        (4, GuestType::Introvert),
        (4, GuestType::Introvert),
        (45, GuestType::Ghost),
        (45, GuestType::Ghost),
        (45, GuestType::Ghost),
        (45, GuestType::Ghost),
    ];
    let mut strat_ptr = 0;
    let mut bought_star = 0;

    for day in 0..15 {
		house.start_day();
		if do_debug {
			println!("D{} {house}", 25-day);
		}
		
		let house_result = do_party(&mut house);
		let mut delta_time = house_result.delta_time;
		
		if do_debug {
			println!("invite {}", house_result.invited);
		}
		
		if house_result.worth >= 999999 {
			time += delta_time;
			return Some(time);
		}
		
        //////////////////////////// shop

		//println!("NOW {house}");
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

        //////////////////////////// magician rng
        
        let mut best = (0, 0); // (worth, #rolls)
        for magic_roll in 0..50 {
			let mut sim_house = house.clone();
			for _ in 0..magic_roll {
				sim_house.roll_magician();
			}
			sim_house.start_day();
			let next_result = do_party(&mut sim_house);
			best = best.max((next_result.worth, magic_roll));
			
			// cannot actually roll
			if house_result.magician == 0 { break; }
			if house_result.bought_stars != house_result.star {
				break;
			}
		}
		
		let num_rolls = best.1;
		for _ in 0..num_rolls {
			house.roll_magician();
		}
		time += num_rolls;
        if do_debug {
            println!("{num_rolls} magician rolls");
        }
    }
    None
}

const SEED_START: u64 = 0;
const TRIES: u64 = 10_000_000;

fn main() {
    let mut optimum = (999999, 0);
    for seed in SEED_START..SEED_START + TRIES {
        if seed % 200_000 == 0 {
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
