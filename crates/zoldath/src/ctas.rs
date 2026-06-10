use std::ops::RangeBounds;

pub const MAGIC_BYTES: [u8; 4] = [b'C', b'T', b'A', b'S'];

pub const HEADER_SIZE: usize = 0x400;
pub const PADDING_SIZE_V1_5: usize = 1000;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ButtonPress {
    pub held: bool,
    pub pressed: bool,
    pub released: bool,
}

impl ButtonPress {
    // TODO: verify
    pub fn from_byte(byte: u8) -> Result<Self, ()> {
        assert!(byte < 8);
        Ok(Self {
            held: (byte & 1) != 0,
            pressed: (byte & 2) != 0,
            released: (byte & 4) != 0,
        })
    }

    pub fn to_byte(&self) -> u8 {
        self.held as u8 | ((self.pressed as u8) << 1) | ((self.released as u8) << 2)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlayerInput {
    pub up: ButtonPress,
    pub down: ButtonPress,
    pub left: ButtonPress,
    pub right: ButtonPress,
    pub action1: ButtonPress,
    pub action2: ButtonPress,
    pub start_pressed: bool,
}

impl PlayerInput {
    // TODO: verify
    pub fn from_bytes(slice: [u8; 4]) -> Result<Self, ()> {
        Self::from_u32(u32::from_le_bytes(slice))
    }

    pub fn from_u32(val: u32) -> Result<Self, ()> {
        assert_eq!(val >> 19, 0);
        Ok(Self {
            up: ButtonPress::from_byte((val & 7) as u8)?,
            down: ButtonPress::from_byte(((val >> 3) & 7) as u8)?,
            left: ButtonPress::from_byte(((val >> 6) & 7) as u8)?,
            right: ButtonPress::from_byte(((val >> 9) & 7) as u8)?,
            action1: ButtonPress::from_byte(((val >> 12) & 7) as u8)?,
            action2: ButtonPress::from_byte(((val >> 15) & 7) as u8)?,
            start_pressed: (val >> 18) != 0,
        })
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        let mut val = 0u32;
        val |= self.up.to_byte() as u32;
        val |= (self.down.to_byte() as u32) << 3;
        val |= (self.left.to_byte() as u32) << 6;
        val |= (self.right.to_byte() as u32) << 9;
        val |= (self.action1.to_byte() as u32) << 12;
        val |= (self.action2.to_byte() as u32) << 15;
        val |= (self.start_pressed as u32) << 18;
        val.to_le_bytes()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Input {
    pub p1: PlayerInput,
    pub p2: PlayerInput,
}

impl Input {
    // TODO: verify
    pub fn from_bytes(slice: [u8; 8]) -> Result<Self, ()> {
        let (p1, slice) = slice.split_first_chunk::<4>().unwrap();
        let p1 = PlayerInput::from_bytes(*p1)?;
        let (p2, _) = slice.split_first_chunk::<4>().unwrap();
        let p2 = PlayerInput::from_bytes(*p2)?;
        Ok(Self { p1, p2 })
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        let mut output = [0u8; 8];
        let (p1, p2) = output.split_at_mut(4);
        p1.copy_from_slice(&self.p1.to_bytes());
        p2.copy_from_slice(&self.p2.to_bytes());
        output
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Randomize {
    pub frame: u32,
    pub seed: f64,
}

impl Randomize {
    // TODO: verify
    pub fn from_bytes(slice: [u8; 12]) -> Result<Self, ()> {
        let (frame, slice) = slice.split_first_chunk::<4>().unwrap();
        let frame = u32::from_le_bytes(*frame);
        let (seed, _) = slice.split_first_chunk::<8>().unwrap();
        let seed = f64::from_le_bytes(*seed);
        Ok(Self { frame, seed })
    }

    pub fn to_bytes(&self) -> [u8; 12] {
        let mut output = [0u8; 12];
        let (frame, seed) = output.split_at_mut(4);
        frame.copy_from_slice(&self.frame.to_le_bytes());
        seed.copy_from_slice(&self.seed.to_le_bytes());
        output
    }
}

#[derive(Debug)]
pub enum CtasParseError {
    InsufficientLength(usize),
    BadMagicNumber([u8; 4]),
    UnsupportedVersion(u32),
    BadInputLength(i32),
    BadRandomizeLength(i32),
    BodyLengthMismatch { expected: usize, actual: usize },
    RandomizeUnsorted { read: Vec<Randomize> },
    BadPadding,

    // TODO
    BadInput(()),
    BadRandomize(()),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Ctas {
    pub version_number: u32,
    pub inputs: Vec<Input>,
    pub randomizes: Vec<Randomize>,
    /// 0 on v1.4
    pub num_rerecords: u32,
    /// 0 on v1.5
    pub timer: u32,
}

impl Ctas {
    pub fn from_bytes(slice: &[u8]) -> Result<Self, CtasParseError> {
        use CtasParseError::*;

        if slice.len() < HEADER_SIZE {
            return Err(InsufficientLength(slice.len()));
        }

        // Magic number
        let (magic, slice) = slice.split_first_chunk::<4>().unwrap();
        if magic != &MAGIC_BYTES {
            return Err(BadMagicNumber(*magic));
        }
        // Version
        let (version_number, slice) = slice.split_first_chunk::<4>().unwrap();
        let version_number = u32::from_le_bytes(*version_number);
        if ![3, 4].contains(&version_number) {
            return Err(UnsupportedVersion(version_number));
        }
        // Number of inputs
        let (num_inputs, slice) = slice.split_first_chunk::<4>().unwrap();
        let num_inputs = i32::from_le_bytes(*num_inputs);
        let Ok(num_inputs) = usize::try_from(num_inputs) else {
            return Err(BadInputLength(num_inputs));
        };
        // Number of randomizes
        let (num_randomizes, slice) = slice.split_first_chunk::<4>().unwrap();
        let num_randomizes = i32::from_le_bytes(*num_randomizes);
        let Ok(num_randomizes) = usize::try_from(num_randomizes) else {
            return Err(BadInputLength(num_randomizes));
        };
        // Number of rerecords
        let (num_rerecords, slice) = slice.split_first_chunk::<4>().unwrap();
        let num_rerecords = u32::from_le_bytes(*num_rerecords);
        if version_number == 3 {
            // TODO: return err instead
            assert_eq!(num_rerecords, 0);
        }
        // Timer in milliseconds
        let (timer, slice) = slice.split_first_chunk::<4>().unwrap();
        let timer = u32::from_le_bytes(*timer);
        if version_number == 3 {
            // TODO: return err instead
            assert_eq!(timer, 0);
        }
        // Padding
        let (padding, slice) = slice.split_first_chunk::<PADDING_SIZE_V1_5>().unwrap();
        if padding.iter().any(|byte| byte != &0) {
            return Err(BadPadding);
        }

        // Check body length
        let expected_length = 8 * num_inputs + 12 * num_randomizes;
        if expected_length != slice.len() {
            return Err(BodyLengthMismatch {
                expected: expected_length,
                actual: slice.len(),
            });
        }

        // Body
        let (inputs, randomizes) = slice.split_at(8 * num_inputs);
        let inputs = match inputs
            .chunks(8)
            .take(num_inputs)
            .map(|bytes| Input::from_bytes(bytes.try_into().unwrap()))
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(inputs) => inputs,
            Err(e) => {
                return Err(BadInput(e));
            }
        };
        let randomizes = match randomizes
            .chunks(12)
            .take(num_randomizes)
            .map(|bytes| Randomize::from_bytes(bytes.try_into().unwrap()))
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(randomizes) => randomizes,
            Err(e) => {
                return Err(BadInput(e));
            }
        };

        // Ok
        Ok(Self {
            version_number,
            inputs,
            randomizes,
            num_rerecords,
            timer,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut output = vec![];

        // header
        output.extend(MAGIC_BYTES);
        output.extend(self.version_number.to_le_bytes());
        let num_inputs = i32::try_from(self.inputs.len()).unwrap();
        output.extend(num_inputs.to_le_bytes());
        let num_randomizes = i32::try_from(self.randomizes.len()).unwrap();
        output.extend(num_randomizes.to_le_bytes());
        output.extend(self.num_rerecords.to_le_bytes());
        output.extend(self.timer.to_le_bytes());
        output.extend([0; PADDING_SIZE_V1_5]);

        // body
        for input in &self.inputs {
            output.extend(input.to_bytes());
        }
        for randomize in &self.randomizes {
            output.extend(randomize.to_bytes());
        }

        output
    }

    pub fn num_inputs(&self) -> usize {
        self.inputs.len()
    }

    pub fn fix_incongruences(&mut self) {
        fn fix(before: &mut ButtonPress, after: &mut ButtonPress, is_arrow: bool) -> bool {
            let mut fixed = false;

            // held -> not held, but did not release (actions only)
            // In CoffeeTools source code:
            // (heldprev and !held and !released) && (input > 3)
            if before.held && !after.held && !after.released && !is_arrow {
                after.released = true;
                fixed = true;
            }

            // tapped, but did not press (actions only)
            // (!pressed && !held && released && !heldprev) && (input > 3)
            if !after.pressed && !after.held && after.released && !before.held && !is_arrow {
                after.pressed = true;
                fixed = true;
            }

            // not held -> held, but did not press (actions only)
            // (!pressed && held && !heldprev) ... (input > 3)
            if !before.held && after.held && !after.pressed && !is_arrow {
                after.pressed = true;
                fixed = true;
            }

            // TODO opposite arrows

            fixed
        }

        for i in 1..self.inputs.len() {
            let j = i - 1;
            let inp = &mut self.inputs[j..=i];
            let (bef, aft) = inp.split_at_mut(1);
            let bef = &mut bef[0];
            let aft = &mut aft[0];
            if fix(&mut bef.p1.up, &mut aft.p1.up, true) {
                println!("fixed {j}-{i} p1.up");
            }
            if fix(&mut bef.p1.down, &mut aft.p1.down, true) {
                println!("fixed {j}-{i} p1.down");
            }
            if fix(&mut bef.p1.left, &mut aft.p1.left, true) {
                println!("fixed {j}-{i} p1.left");
            }
            if fix(&mut bef.p1.right, &mut aft.p1.right, true) {
                println!("fixed {j}-{i} p1.right");
            }
            if fix(&mut bef.p1.action1, &mut aft.p1.action1, false) {
                println!("fixed {j}-{i} p1.action1");
            }
            if fix(&mut bef.p1.action2, &mut aft.p1.action2, false) {
                println!("fixed {j}-{i} p1.action2");
            }
        }
    }

    pub fn add_empty_frames(&mut self, amount: usize) {
        self.inputs.extend(vec![Input::default(); amount]);
        // Since this has to be measured again, reset it to avoid mistakes
        self.timer = 0;
    }

    pub fn slice(&mut self, range: impl RangeBounds<usize>) {
        use std::ops::Bound::*;
        let start = u32::try_from(match range.start_bound() {
            Included(s) => *s,
            Excluded(s) => *s + 1,
            Unbounded => 0,
        })
        .unwrap();

        self.randomizes = self
            .randomizes
            .iter()
            .filter_map(|rand| {
                if range.contains(&usize::try_from(rand.frame).unwrap()) {
                    Some(Randomize {
                        frame: rand.frame - start,
                        seed: rand.seed,
                    })
                } else {
                    None
                }
            })
            .collect();
        self.inputs = self.inputs.drain(range).collect();
        // Since this has to be measured again, reset it to avoid mistakes
        self.timer = 0;
    }

    pub fn append(&mut self, other: Self) {
        let self_frames = u32::try_from(self.num_inputs()).unwrap();

        self.inputs.extend(other.inputs);
        for other_randomize in other.randomizes {
            self.randomizes.push(Randomize {
                frame: other_randomize.frame + self_frames,
                seed: other_randomize.seed,
            });
        }
        // Since this has to be measured again, reset it to avoid mistakes
        self.timer = 0;
    }

    pub fn insert_empty_frames(&mut self, at: usize, amount: usize) {
        let empty_frames = vec![Input::default(); amount];
        self.inputs.splice(at..at, empty_frames);
        for rng in &mut self.randomizes {
            if rng.frame >= at as u32 {
                rng.frame += amount as u32;
            }
        }
        // Since this has to be measured again, reset it to avoid mistakes
        self.timer = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// WIP of Kick Club TAS made in Game v1.8.9 and CoffeeTools v1.4
    #[test]
    fn test_io_v1_4() {
        let bytes = std::fs::read("test-movies/kickclub-1-4.ctas").unwrap();
        let ctas = Ctas::from_bytes(&bytes).unwrap();
        assert_eq!(ctas.version_number, 3);
        assert_eq!(ctas.num_inputs(), 2045);
        assert_eq!(ctas.randomizes[0].frame, 0);
        assert_eq!(ctas.randomizes[1].frame, 811);
        assert!(ctas.to_bytes() == bytes);
    }

    #[test]
    fn test_manipulate() {
        let bytes = std::fs::read("test-movies/kickclub-1-4.ctas").unwrap();

        let mut ctas = Ctas::from_bytes(&bytes).unwrap();
        ctas.slice(120..);
        assert_eq!(ctas.randomizes.len(), 1);
        assert_eq!(ctas.randomizes[0].frame, 811 - 120);

        let mut ctas = Ctas::from_bytes(&bytes).unwrap();
        ctas.slice(..333);
        assert_eq!(ctas.randomizes.len(), 1);
        assert_eq!(ctas.randomizes[0].frame, 0);

        let mut ctas = Ctas::from_bytes(&bytes).unwrap();
        ctas.append(ctas.clone());
        assert_eq!(ctas.num_inputs(), 2045 * 2);
        assert_eq!(ctas.randomizes.len(), 4);
        assert_eq!(ctas.randomizes[0].frame, 0);
        assert_eq!(ctas.randomizes[1].frame, 811);
        assert_eq!(ctas.randomizes[2].frame, 2045);
        assert_eq!(ctas.randomizes[3].frame, 2045 + 811);
    }

    // TODO error tests
}
