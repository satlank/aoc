use std::fmt::{Display, Formatter};

fn value(c: char) -> usize {
    match c {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => 0,
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct Home<const D: usize> {
    pub(crate) owner: char,
    pub(crate) id: usize,
    pub(crate) hallway: usize,
    pub(crate) places: [char; D],
}

impl<const D: usize> Home<D> {
    fn new(owner: char, id: usize, hallway: usize) -> Self {
        Self {
            owner,
            id,
            hallway,
            places: [' '; D],
        }
    }

    /// For a given home check if the top most needs to leave and if so, return which one it is,
    /// how far it needs to move to reach the exit and how the home looks like when it has left.
    fn get_mover(&self) -> Option<(char, usize, Self)> {
        let mut loc = D;
        loop {
            loc -= 1;
            if self.places[loc] != ' ' {
                break;
            }

            if loc == 0 {
                // We are empty!
                return None;
            }
        }
        // Maybe move the thing at `loc`
        if self.places[0..=loc].iter().all(|&c| c == self.owner) {
            // No, we are all already where we want to be
            return None;
        }
        let mut emptier_home = *self;
        emptier_home.places[loc] = ' ';
        Some((self.places[loc], D - 1 - loc, emptier_home))
    }

    /// Trying to enter a home, if we can, then we will return the step we need to take internally
    /// and a new home with the position filled
    fn try_enter(&self) -> Option<(usize, Self)> {
        if self.places[D - 1] == ' ' && self.places.iter().all(|&c| c == self.owner || c == ' ') {
            let mut new_home = *self;
            for i in 0..D {
                if new_home.places[i] == ' ' {
                    new_home.places[i] = self.owner;
                    return Some((D - i, new_home));
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct Burrow<const D: usize> {
    pub(crate) hallway: [char; 11],
    pub(crate) homes: [Home<D>; 4],
}

impl<const D: usize> Default for Burrow<D> {
    fn default() -> Self {
        Self {
            hallway: [' '; 11],
            homes: [
                Home::<D>::new('A', 0, 2),
                Home::<D>::new('B', 1, 4),
                Home::<D>::new('C', 2, 6),
                Home::<D>::new('D', 3, 8),
            ],
        }
    }
}

impl<const D: usize> Burrow<D> {
    pub fn is_complete(&self) -> bool {
        self.homes
            .iter()
            .all(|h| h.places.iter().all(|&e| e == h.owner))
    }

    pub fn valid_moves(&self) -> Vec<(usize, Burrow<D>)> {
        if self.is_complete() {
            return Vec::new();
        }

        let mut res = Vec::new();
        // Try to move out
        for h in &self.homes {
            if let Some((mover, steps, new_home)) = h.get_mover() {
                for left in (0..new_home.hallway).rev() {
                    if left > 0 && left % 2 == 0 {
                        // Can't stop in front of a home
                        continue;
                    }
                    if self.hallway[left] != ' ' {
                        // Cannot move further left
                        break;
                    }
                    let mut burrow = *self;
                    burrow.homes[new_home.id] = new_home;
                    burrow.hallway[left] = mover;
                    res.push((
                        (steps + 1 + (new_home.hallway - left)) * value(mover),
                        burrow,
                    ));
                }
                for right in new_home.hallway + 1..11 {
                    if right < 10 && right % 2 == 0 {
                        // Can't stop in front of a home
                        continue;
                    }
                    if self.hallway[right] != ' ' {
                        // Cannot move further left
                        break;
                    }
                    let mut burrow = *self;
                    burrow.homes[new_home.id] = new_home;
                    burrow.hallway[right] = mover;
                    res.push((
                        (steps + 1 + right - new_home.hallway) * value(mover),
                        burrow,
                    ));
                }
            }
        }

        // Try to move back in
        for loc in 0..11 {
            let mover = self.hallway[loc];
            if mover != ' ' {
                let to_home = match mover {
                    'A' => 0,
                    'B' => 1,
                    'C' => 2,
                    'D' => 3,
                    _ => unreachable!(),
                };
                let hallway_steps;
                let can_reach = if loc < self.homes[to_home].hallway {
                    // Walk right
                    hallway_steps = self.homes[to_home].hallway - loc;
                    self.hallway[loc + 1..self.homes[to_home].hallway]
                        .iter()
                        .all(|&c| c == ' ')
                } else {
                    // Walk left
                    hallway_steps = loc - self.homes[to_home].hallway;
                    self.hallway[self.homes[to_home].hallway..loc]
                        .iter()
                        .all(|&c| c == ' ')
                };
                if can_reach {
                    if let Some((steps, new_home)) = self.homes[to_home].try_enter() {
                        let mut burrow = *self;
                        burrow.homes[new_home.id] = new_home;
                        burrow.hallway[loc] = ' ';
                        res.push(((steps + hallway_steps) * value(mover), burrow));
                    }
                }
            }
        }

        res
    }
}

impl<const D: usize> Display for Burrow<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for c in &self.hallway {
            write!(f, "{}", c)?;
        }
        writeln!(f, "#")?;
        writeln!(
            f,
            "###{}#{}#{}#{}###",
            self.homes[0].places[D - 1],
            self.homes[1].places[D - 1],
            self.homes[2].places[D - 1],
            self.homes[3].places[D - 1]
        )?;
        for i in 2..=D {
            writeln!(
                f,
                "  #{}#{}#{}#{}#  ",
                self.homes[0].places[D - i],
                self.homes[1].places[D - i],
                self.homes[2].places[D - i],
                self.homes[3].places[D - i]
            )?;
        }
        writeln!(f, "  #########  ")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_complete() {
        let mut burrow = Burrow::<4>::default();
        assert!(!burrow.is_complete());
        burrow.homes[0].places = ['A'; 4];
        burrow.homes[1].places = ['B'; 4];
        burrow.homes[2].places = ['C'; 4];
        burrow.homes[3].places = ['D'; 4];
        assert!(burrow.is_complete());
    }

    #[test]
    fn test_get_moves_out() {
        let burrow = Burrow::<4> {
            hallway: [' '; 11],
            homes: [
                Home {
                    owner: 'A',
                    id: 0,
                    hallway: 2,
                    places: ['A', 'B', 'C', 'D'],
                },
                Home {
                    owner: 'B',
                    id: 1,
                    hallway: 4,
                    places: ['A', 'B', 'C', 'D'],
                },
                Home {
                    owner: 'C',
                    id: 2,
                    hallway: 6,
                    places: ['A', 'B', 'C', 'D'],
                },
                Home {
                    owner: 'D',
                    id: 3,
                    hallway: 8,
                    places: ['A', 'B', 'C', 'D'],
                },
            ],
        };

        let moves = burrow.valid_moves();
        for m in &moves {
            println!("---");
            println!("{}", m.1);
            println!("---");
        }
        assert_eq!(moves.len(), 28);
    }

    #[test]
    fn test_get_moves_in() {
        let burrow = Burrow::<4> {
            hallway: ['A', 'B', ' ', 'A', ' ', 'D', ' ', ' ', ' ', 'C', ' '],
            homes: [
                Home {
                    owner: 'A',
                    id: 0,
                    hallway: 2,
                    places: [' ', ' ', ' ', ' '],
                },
                Home {
                    owner: 'B',
                    id: 1,
                    hallway: 4,
                    places: ['B', ' ', ' ', ' '],
                },
                Home {
                    owner: 'C',
                    id: 2,
                    hallway: 6,
                    places: ['B', 'B', ' ', ' '],
                },
                Home {
                    owner: 'D',
                    id: 3,
                    hallway: 8,
                    places: ['D', 'D', 'D', ' '],
                },
            ],
        };

        let moves = burrow.valid_moves();
        for m in &moves {
            println!("---");
            println!("{}", m.1);
            println!("---");
        }
        assert_eq!(moves.len(), 3);
        assert_eq!(
            moves.iter().map(|(c, _)| *c).collect::<Vec<usize>>(),
            vec![4 * 10, 5, 4 * 1000]
        );
    }
}
