#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rng {
    pub seed: u64,
    pub num_rolls: usize,
    s1: u64,
    s2: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        let mask = 1431655765;
        let s1 = 1253089769 ^ (seed & mask);
        let s2 = 2342871706 ^ (seed & !mask);
        let mut rng = Rng {
            seed,
            num_rolls: 0,
            s1,
            s2,
        };
        for _ in 0..20 {
            rng.roll();
        }
        rng
    }

    pub fn roll(&mut self) -> u64 {
        self.num_rolls += 1;
        self.s1 = (65192 * (self.s1 & 65535)) + ((self.s1 & 4294901760) >> 16);
        self.s2 = (64473 * (self.s2 & 65535)) + ((self.s2 & 4294901760) >> 16);
        (((self.s1 & 65535) << 16) + self.s2) & 4294967295
    }

    pub fn roll_int(&mut self, l: u64, r: u64) -> u64 {
        let val = self.roll();
        let numerator = (r - l + 1) * val;
        l + numerator / 4294967296
    }

    pub fn shuffle<T>(&mut self, vals: &mut [T]) {
        let mut i = vals.len();
        while i > 1 {
            i -= 1;
            let j = self.roll_int(0, i as u64) as usize;
            vals.swap(i, j);
        }
    }

    pub fn roll_onto(&mut self, target_num: usize) {
        assert!(self.num_rolls <= target_num);
        for _ in 0..(target_num - self.num_rolls) {
            self.roll();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mortol_4b_rng() {
        let mut rng = Rng::new(4471);
        rng.roll_onto(3027 + 48);

        // 4th bost nest rng
        rng.roll();
        let timer = 720 - rng.roll_int(0, 360);
        let mut objects = ['F', 'F', 'L', 'L', 'E', 'E', 'E', 'R', 'R', 'S'];
        rng.shuffle(&mut objects);
        assert_eq!(timer, 360);
        assert_eq!(objects[0], 'F');
    }

    #[test]
    fn test_party_house_rng() {
        let mut rng = Rng::new(58071757);
        rng.roll_onto(45);

        let mut dex = vec!['W', 'W', 'W', 'W', 'R', 'R', 'O', 'O', 'O', 'O'];
        rng.shuffle(&mut dex);
        println!("{dex:?} {}", rng.num_rolls);
        rng.shuffle(&mut dex);
        println!("{dex:?} {}", rng.num_rolls);
        dex.push('H');
        dex.push('H');
        rng.roll();
        rng.roll();
        rng.shuffle(&mut dex);
        println!("{dex:?} {}", rng.num_rolls);
    }
}
