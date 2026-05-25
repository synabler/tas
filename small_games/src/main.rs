use util::bfs;

/// Toggles a mask tool and returns the new list of equipped masks.
///
/// If the mask cannot be toggled, just clones `equipped`.
///
/// Parameters:
/// - `equipped`: the list of currently equipped masks,
///   from innermost to outermost, each represented by a bitmask that
///   denotes which areas are being masked.
/// - `mask`: the mask to toggle, represented by the list of areas
///   that are being masked.
/// - `on_condition`: assuming the mask is currently not equipped,
///   the function that returns whether the mask can be equipped.
/// - `off_condition`: assuming the mask is currently equipped,
///   the function that returns whether the mask can be unequipped.
fn toggle_mask(
    equipped: &[u32],
    mask: &[u32],
    on_condition: impl Fn(&[u32]) -> bool,
    off_condition: impl Fn(&[u32]) -> bool,
) -> Vec<u32> {
    let mask = mask.iter().map(|x| 1 << x).sum::<u32>();
    let mut new_ball = equipped.to_vec();
    if let Some(i) = equipped.iter().position(|m| *m == mask) {
        if off_condition(equipped) {
            new_ball.remove(i);
        }
    } else if on_condition(equipped) {
        new_ball.push(mask);
    }
    new_ball
}

/// A factory ball, divided into `N` areas.
///
/// Fields:
/// - `color`: the color of each area. 0 is reserved for uncolored.
///   if one of the paints (or brushes) are white, use 1 instead
///   (and the starting states should be all 1 as well).
/// - `masks`: the list of currently equipped masks,
///   from innermost to outermost, each represented by a bitmask that
///   denotes which areas are being masked.
///
/// If rotation is enabled, follow this indexing:
/// ```notest
///  07
/// 1  6
/// 2  5
///  34
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AreaBall<const N: usize> {
    color: [u8; N],
    masks: Vec<u32>,
}

impl<const N: usize> AreaBall<N> {
    /// Paints the ball with paint (or brush) and returns the new ball.
    /// 0 is reserved for uncolored, which means the area will not be
    /// colored regardless of masks.
    fn paint(&self, new_color: [u8; N]) -> Self {
        let mut new_ball = self.clone();
        for (i, new_col) in new_color.iter().copied().enumerate() {
            if new_col == 0 {
                continue;
            }
            if self.masks.iter().any(|mask| mask & (1 << i) != 0) {
                continue;
            }
            new_ball.color[i] = new_col;
        }
        new_ball
    }

    /// Toggles a mask tool and returns the new list of equipped masks.
    ///
    /// If the mask cannot be toggled, just clones `self`.
    ///
    /// Parameters:
    /// - `mask`: the mask to toggle, represented by the list of areas
    ///   that are being masked.
    /// - `on_condition`: assuming the mask is currently not equipped,
    ///   the function that returns whether the mask can be equipped.
    /// - `off_condition`: assuming the mask is currently equipped,
    ///   the function that returns whether the mask can be unequipped.
    fn toggle_mask(
        &self,
        mask: &[u32],
        on_condition: impl Fn(&[u32]) -> bool,
        off_condition: impl Fn(&[u32]) -> bool,
    ) -> Self {
        Self {
            color: self.color,
            masks: toggle_mask(&self.masks, mask, on_condition, off_condition),
        }
    }

    /// Rotates a ball counterclockwise.
    ///
    /// If the ball cannot be rotated, just clones `self`.
    fn rotate_ccw(&self) -> Self {
        let mut new_ball = self.clone();
        if self.masks.is_empty() {
            new_ball.color.rotate_right(1);
        }
        new_ball
    }
}

/// A factory ball that uses plants, divided into `N` areas.
///
/// Fields:
/// - `grass`: the state of grass in each area.
///   0, 1, 2, 3, 4 denotes unseeded, seeded, green, dark green, and brown,
///   respectively.
/// - `yellow`: the state of yellow flowers in each area.
///   0, 1, 2, 3 denotes unseeded, seeded, small bloom, and large bloom,
///   respectively.
/// - `blue`: the state of blue flowers in each area, similarly.
/// - `masks`: the list of currently equipped masks,
///   from innermost to outermost, each represented by a bitmask that
///   denotes which areas are being masked.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GrassBall<const N: usize> {
    grass: [u8; N],
    yellow: [u8; N],
    blue: [u8; N],
    masks: Vec<u32>,
}

impl<const N: usize> GrassBall<N> {
    /// Plants seeds of given species in the unmasked areas.
    /// 0, 1, 2 correspond to grass, yellow flowers, and blue flowers,
    /// respectively.
    fn plant(&self, species: u8) -> Self {
        let mut new_ball = self.clone();
        for i in 0..N {
            if self.masks.iter().any(|mask| mask & (1 << i) != 0) {
                continue;
            }
            if species == 0 {
                new_ball.grass[i] = self.grass[i].max(1);
            } else if species == 1 {
                new_ball.yellow[i] = self.yellow[i].max(1);
            } else {
                new_ball.blue[i] = self.blue[i].max(1);
            }
        }
        new_ball
    }

    /// Waters the unmasked areas.
    fn water(&self) -> Self {
        let mut new_ball = self.clone();
        for i in 0..N {
            if self.masks.iter().any(|mask| mask & (1 << i) != 0) {
                continue;
            }
            new_ball.grass[i] = [0, 2, 3, 4, 4][self.grass[i] as usize];
            new_ball.yellow[i] = [0, 2, 3, 0][self.yellow[i] as usize];
            new_ball.blue[i] = [0, 2, 3, 0][self.blue[i] as usize];
        }
        new_ball
    }

    /// Toggles a mask tool and returns the new list of equipped masks.
    ///
    /// If the mask cannot be toggled, just clones `self`.
    ///
    /// Parameters:
    /// - `mask`: the mask to toggle, represented by the list of areas
    ///   that are being masked.
    /// - `on_condition`: assuming the mask is currently not equipped,
    ///   the function that returns whether the mask can be equipped.
    /// - `off_condition`: assuming the mask is currently equipped,
    ///   the function that returns whether the mask can be unequipped.
    fn toggle_mask(
        &self,
        mask: &[u32],
        on_condition: impl Fn(&[u32]) -> bool,
        off_condition: impl Fn(&[u32]) -> bool,
    ) -> Self {
        Self {
            masks: toggle_mask(&self.masks, mask, on_condition, off_condition),
            ..self.clone()
        }
    }
}

/// - AREA:  0 upper bg, 1 upper circle, 2 lower bg, 3 lower circle
/// - COLOR: 1 pink, 2 orange, 3 yellow
/// - MASK:  1 3 spot
fn f1l12() -> Vec<&'static str> {
    let start = AreaBall {
        color: [0; 4],
        masks: vec![],
    };
    let end = AreaBall {
        color: [1, 0, 3, 2],
        masks: vec![],
    };
    bfs(
        &[start],
        |ball| {
            vec![
                ("pink", ball.paint([1, 1, 1, 1])),
                ("orange", ball.paint([0, 0, 2, 2])),
                ("yellow", ball.paint([0, 0, 3, 3])),
                ("spot", ball.toggle_mask(&[1, 3], |_| true, |_| true)),
            ]
        },
        |ball| ball == &end,
    )
}

/// - AREA:  0-8
/// - COLOR: 1 pink, 2 cyan
/// - MASK:  3 4 5 hbelt, 1 4 7 vbelt
fn f2l13() -> Vec<&'static str> {
    let start = AreaBall {
        color: [0; 9],
        masks: vec![],
    };
    let end = AreaBall {
        color: [2, 1, 2, 1, 0, 1, 2, 1, 2],
        masks: vec![],
    };
    bfs(
        &[start],
        |ball| {
            vec![
                ("pink", ball.paint([1; 9])),
                ("orange", ball.paint([2; 9])),
                (
                    "hbelt",
                    ball.toggle_mask(
                        &[3, 4, 5],
                        |_| true,
                        |masks| masks.last() == Some(&(8 + 16 + 32)),
                    ),
                ),
                (
                    "vbelt",
                    ball.toggle_mask(
                        &[1, 4, 7],
                        |_| true,
                        |masks| masks.last() == Some(&(2 + 16 + 128)),
                    ),
                ),
            ]
        },
        |ball| ball == &end,
    )
}

/// - AREA:  0 upper bg, 1 upper ear, 2 lower bg, 3 lower ear
/// - COLOR: 1 yellow, 2 blue, 3 green, 4 red
/// - MASK:  1 3 headset, 0 1 cap
fn f2l18() -> Vec<&'static str> {
    let start = AreaBall {
        color: [0; 4],
        masks: vec![],
    };
    let end = AreaBall {
        color: [1, 3, 2, 4],
        masks: vec![],
    };
    bfs(
        &[start],
        |ball| {
            vec![
                ("yellow", ball.paint([1; 4])),
                ("blue", ball.paint([2; 4])),
                ("green", ball.paint([3; 4])),
                ("red", ball.paint([4; 4])),
                ("headset", ball.toggle_mask(&[1, 3], |_| true, |_| true)),
                (
                    "cap",
                    ball.toggle_mask(&[0, 1], |masks| masks.is_empty(), |masks| masks.len() == 1),
                ),
            ]
        },
        |ball| ball == &end,
    )
}

/// - AREA:  0 upper, 1 lower
/// - MASK:  0 cap
fn f2l19() -> Vec<&'static str> {
    let start = GrassBall {
        grass: [0; 2],
        yellow: [0; 2],
        blue: [0; 2],
        masks: vec![],
    };
    let end = GrassBall {
        grass: [2, 2],
        yellow: [3, 0],
        blue: [0, 0],
        masks: vec![],
    };
    bfs(
        &[start],
        |ball| {
            vec![
                ("grass", ball.plant(0)),
                ("yellow", ball.plant(1)),
                ("water", ball.water()),
                ("cap", ball.toggle_mask(&[0], |_| true, |_| true)),
            ]
        },
        |ball| ball == &end,
    )
}

/// - AREA:  rotation
/// - COLOR: 1 yellow, 2 black
/// - MASK:  0 1 6 7 cap
fn f2l20() -> Vec<&'static str> {
    let start = AreaBall {
        color: [0; 8],
        masks: vec![],
    };
    let end = AreaBall {
        color: [1, 1, 2, 1, 1, 1, 1, 1],
        masks: vec![],
    };
    bfs(
        &[start],
        |ball| {
            vec![
                ("yellow", ball.paint([1; 8])),
                ("black", ball.paint([2; 8])),
                ("rotate", ball.rotate_ccw()),
                ("cap", ball.toggle_mask(&[0, 1, 6, 7], |_| true, |_| true)),
            ]
        },
        |ball| ball == &end,
    )
}

/// - AREA:  0 upper bg, 1 upper circle, 2 lower bg, 3 lower circle
/// - COLOR: 1 white, 2 black
/// - MASK:  1 3 spot, 0 1 cap
fn f2l23() -> Vec<&'static str> {
    let start = AreaBall {
        color: [1; 4],
        masks: vec![],
    };
    let end = AreaBall {
        color: [2, 1, 1, 2],
        masks: vec![],
    };
    bfs(
        &[start],
        |ball| {
            vec![
                ("white", ball.paint([0, 0, 1, 1])),
                ("black", ball.paint([2; 4])),
                ("spot", ball.toggle_mask(&[1, 3], |_| true, |_| true)),
                (
                    "cap",
                    ball.toggle_mask(&[0, 1], |masks| masks.is_empty(), |masks| masks.len() == 1),
                ),
            ]
        },
        |ball| ball == &end,
    )
}

/// - AREA:  0 upper, 1 lower
/// - MASK:  0 cap
fn f2l24() -> Vec<&'static str> {
    let start = GrassBall {
        grass: [0; 2],
        yellow: [0; 2],
        blue: [0; 2],
        masks: vec![],
    };
    let end = GrassBall {
        grass: [3, 3],
        yellow: [2, 2],
        blue: [0, 3],
        masks: vec![],
    };
    bfs(
        &[start],
        |ball| {
            vec![
                ("grass", ball.plant(0)),
                ("yellow", ball.plant(1)),
                ("blue", ball.plant(2)),
                ("water", ball.water()),
                ("cap", ball.toggle_mask(&[0], |_| true, |_| true)),
            ]
        },
        |ball| ball == &end,
    )
}

/// - AREA:  rotation
/// - COLOR: 1 orange, 2 green, 3 yellow
/// - MASK:  0 1 6 7 cap
fn f2l25() -> Vec<&'static str> {
    let start = AreaBall {
        color: [0; 8],
        masks: vec![],
    };
    let end = AreaBall {
        color: [2, 1, 1, 1, 1, 1, 1, 3],
        masks: vec![],
    };
    bfs(
        &[start],
        |ball| {
            vec![
                ("orange", ball.paint([1; 8])),
                ("green", ball.paint([2; 8])),
                ("yellow", ball.paint([3; 8])),
                ("rotate", ball.rotate_ccw()),
                ("cap", ball.toggle_mask(&[0, 1, 6, 7], |_| true, |_| true)),
            ]
        },
        |ball| ball == &end,
    )
}

/// - AREA:  0-8
/// - MASK:  3 4 5 hbelt, 1 4 7 vbelt
fn f2l28() -> Vec<&'static str> {
    let start = GrassBall {
        grass: [0; 9],
        yellow: [0; 9],
        blue: [0; 9],
        masks: vec![],
    };
    let end = GrassBall {
        grass: [4, 3, 4, 3, 0, 3, 4, 3, 4],
        yellow: [0; 9],
        blue: [0; 9],
        masks: vec![],
    };
    bfs(
        &[start],
        |ball| {
            vec![
                ("grass", ball.plant(0)),
                ("water", ball.water()),
                (
                    "hbelt",
                    ball.toggle_mask(
                        &[3, 4, 5],
                        |_| true,
                        |masks| masks.last() == Some(&(8 + 16 + 32)),
                    ),
                ),
                (
                    "vbelt",
                    ball.toggle_mask(
                        &[1, 4, 7],
                        |_| true,
                        |masks| masks.last() == Some(&(2 + 16 + 128)),
                    ),
                ),
            ]
        },
        |ball| ball == &end,
    )
}

/// - AREA:  rotation
/// - COLOR: 1 black, 2 blue, 3 red
/// - MASK:  0 1 6 7 cap
fn f2l29() -> Vec<&'static str> {
    let start = AreaBall {
        color: [0; 8],
        masks: vec![],
    };
    let end = AreaBall {
        color: [2, 0, 0, 1, 3, 3, 3, 3],
        masks: vec![],
    };
    bfs(
        &[start],
        |ball| {
            vec![
                ("black", ball.paint([1; 8])),
                ("blue", ball.paint([2; 8])),
                ("red", ball.paint([3; 8])),
                ("rotate", ball.rotate_ccw()),
                ("cap", ball.toggle_mask(&[0, 1, 6, 7], |_| true, |_| true)),
            ]
        },
        |ball| ball == &end,
    )
}

fn main() {
    println!("F1L12: {:?}", f1l12());
    println!();
    println!("F2L13: {:?}", f2l13());
    println!("F2L18: {:?}", f2l18());
    println!("F2L19: {:?}", f2l19());
    println!("F2L20: {:?}", f2l20());
    println!("F2L23: {:?}", f2l23());
    println!("F2L24: {:?}", f2l24());
    println!("F2L25: {:?}", f2l25());
    println!("F2L28: {:?}", f2l28());
    println!("F2L29: {:?}", f2l29());
}
