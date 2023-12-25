// https://adventofcode.com/2023/day/24

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

use itertools::Itertools;
use z3::ast::{Ast, Int};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Trajectory3 {
    pos: [i64; 3],
    vel: [i64; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Trajectory2 {
    pos: [i64; 2],
    vel: [i64; 2],
}

impl Trajectory3 {
    fn project(&self) -> Trajectory2 {
        Trajectory2 {
            pos: [self.pos[0], self.pos[1]],
            vel: [self.vel[0], self.vel[1]],
        }
    }
}

impl From<String> for Trajectory3 {
    fn from(s: String) -> Self {
        let mut parts = s.split(" @ ");
        let pos_part = parts.next().unwrap();
        let vel_part = parts.next().unwrap();
        let pos = pos_part
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let vel = vel_part
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        Self {
            pos: pos.try_into().unwrap(),
            vel: vel.try_into().unwrap(),
        }
    }
}

fn read<R: Read>(io: R) -> Vec<Trajectory3> {
    let br = BufReader::new(io);
    br.lines()
        .map_while(Result::ok)
        .map(Trajectory3::from)
        .collect()
}

fn intersection(t1: &Trajectory2, t2: &Trajectory2) -> Option<([f64; 2], bool, bool)> {
    let x1 = t1.pos[0] as f64;
    let y1 = t1.pos[1] as f64;
    let x2 = (t1.pos[0] + 1000 * t1.vel[0]) as f64;
    let y2 = (t1.pos[1] + 1000 * t1.vel[1]) as f64;

    let x3 = t2.pos[0] as f64;
    let y3 = t2.pos[1] as f64;
    let x4 = (t2.pos[0] + 1000 * t2.vel[0]) as f64;
    let y4 = (t2.pos[1] + 1000 * t2.vel[1]) as f64;

    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if denom == 0.0 {
        return None;
    }

    let px = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / denom;
    let py = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / denom;

    let d1x = (px - x1) / t1.vel[0] as f64;
    let d1y = (py - y1) / t1.vel[1] as f64;
    let is_future_1 = d1x > 0.0 && d1y > 0.0;

    let d2x = (px - x3) / t2.vel[0] as f64;
    let d2y = (py - y3) / t2.vel[1] as f64;
    let is_future_2 = d2x > 0.0 && d2y > 0.0;

    Some(([px, py], is_future_1, is_future_2))
}

fn part_1(input: &[Trajectory3], min: f64, max: f64) -> usize {
    let traj2 = input.iter().map(Trajectory3::project).collect::<Vec<_>>();

    (0..traj2.len())
        .combinations(2)
        .filter_map(|x| {
            let i = x[0];
            let j = x[1];
            let t1 = &traj2[i];
            let t2 = &traj2[j];
            if let Some((is, is_future_1, is_future_2)) = intersection(t1, t2) {
                if is_future_1 && is_future_2 {
                    if is[0] >= min && is[0] <= max && is[1] >= min && is[1] <= max {
                        return Some((i, j));
                    } else {
                        return None;
                    }
                }
            } else {
            }
            None
        })
        .count()
}

fn part_2(input: &[Trajectory3]) -> usize {
    let ctx = z3::Context::new(&z3::Config::new());
    let solver = z3::Solver::new(&ctx);
    let fx = Int::new_const(&ctx, "fx");
    let fy = Int::new_const(&ctx, "fy");
    let fz = Int::new_const(&ctx, "fz");
    let fdx = Int::new_const(&ctx, "fdx");
    let fdy = Int::new_const(&ctx, "fdy");
    let fdz = Int::new_const(&ctx, "fdz");

    let zero = Int::from_i64(&ctx, 0);
    for (i, &traj) in input.iter().enumerate().take(3) {
        let [x, y, z, dx, dy, dz] = [
            traj.pos[0],
            traj.pos[1],
            traj.pos[2],
            traj.vel[0],
            traj.vel[1],
            traj.vel[2],
        ]
        .map(|v| Int::from_i64(&ctx, v as _));
        let t = Int::new_const(&ctx, format!("t{i}"));
        solver.assert(&t.ge(&zero));
        solver.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
        solver.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
        solver.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
    }
    assert_eq!(solver.check(), z3::SatResult::Sat);
    let model = solver.get_model().unwrap();
    let res = model.eval(&(&fx + &fy + &fz), true).unwrap();
    res.as_i64().unwrap() as usize
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    let p1 = part_1(&input, 200_000_000_000_000.0, 400_000_000_000_000.0);
    println!("Part 1: {}", p1);
    let p2 = part_2(&input);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = read(File::open("example1.txt").unwrap());
        assert_eq!(part_1(&input, 7.0, 27.0), 2);
        assert_eq!(part_2(&input), 47);
    }
}
