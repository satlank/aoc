use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use] extern crate scan_fmt;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn l1_norm(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn line_segment_intersection(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> Option<Point> {
    // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
    let divisor = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);
    if divisor == 0 {
        // Parallel or coinciding, ignoring the latter for now
        return None;
    }
    let t1 = (p1.x - p3.x) * (p3.y - p4.y) - (p1.y - p3.y) * (p3.x - p4.x);
    let u1 = -((p1.x - p2.x) * (p1.y - p3.y) - (p1.y - p2.y) * (p1.x - p3.x));
    let good_t = t1.signum() == divisor.signum() || t1 == 0;
    let good_u = u1.signum() == divisor.signum() || u1 == 0;

    if good_t && good_u && t1.abs() >= 0 && t1.abs() <= divisor.abs() && u1.abs() >= 0 && u1.abs() <= divisor.abs() {
    //if t1 == 0 || (same_sign && t1.abs() >= 0 && t1.abs() <= divisor.abs()) {
        return Some(Point{
            x: p1.x + t1 * (p2.x - p1.x) / divisor,
            y: p1.y + t1 * (p2.y - p1.y) / divisor
        });
    }
    None
}

fn to_delta(op: String) -> Point {
    let (direction, length) = scan_fmt_some!(&op, "{[RLUD]}{d}", char, i32);
    let length = length.unwrap();
    match direction {
        Some('R') => return Point{x: length, y: 0},
        Some('U') => return Point{x: 0, y: length},
        Some('L') => return Point{x: -length, y: 0},
        Some('D') => return Point{x: 0, y: -length},
        _ => panic!("Don't know what to do")
    }
}

fn path_to_points(path: String) -> Vec<Point> {
    let mut res = Vec::new();
    let mut prev = Point{x:0, y:0};
    res.push(Point{x: prev.x, y: prev.y});
    for op in path.split(',') {
        let delta = to_delta(op.to_string());
        prev.x = prev.x + delta.x;
        prev.y = prev.y + delta.y;
        res.push(Point{x: prev.x, y: prev.y});
    }
    return res;
}

fn find_intersections(path1: &Vec<Point>, path2: &Vec<Point>) -> Vec<Point> {
    let mut res = Vec::new();

    for i in 0..path1.len() - 1 {
        let p1p1 = &path1[i];
        let p1p2 = &path1[i+1];
        for j in 0..path2.len() - 1 {
            let p2p1 = &path2[j];
            let p2p2 = &path2[j+1];
            let common = line_segment_intersection(p1p1, p1p2, p2p1, p2p2);
            match common {
                Some(p) => res.push(p),
                _ => ()
            }
        }
    }

    return res;
}

fn find_min_dist(points: &Vec<Point>) -> Option<i32> {
    let origin = Point{x: 0, y: 0};
    points.iter().map(|p| l1_norm(&origin, &p)).filter(|d| *d != 0).min()
}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let path1 = path_to_points(input[0].clone());
    let path2 = path_to_points(input[1].clone());
    let common = find_intersections(&path1, &path2);
    let dist = find_min_dist(&common);

    println!("{:?}", dist);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_l1() {
        assert_eq!(l1_norm(&Point{x: 10, y: 10}, &Point{x: 20, y: 20}), 20);
        assert_eq!(l1_norm(&Point{x: 0, y: 0}, &Point{x: 10, y: 10}), 20);
    }

    #[test]
    fn test_parse_op() {
        let p = to_delta("R10".to_string());
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 0);
        let p = to_delta("L10".to_string());
        assert_eq!(p.x, -10);
        assert_eq!(p.y, 0);
        let p = to_delta("U10".to_string());
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 10);
        let p = to_delta("D10".to_string());
        assert_eq!(p.x, 0);
        assert_eq!(p.y, -10);
    }

    #[test]
    fn test_path_to_points() {
        let path = path_to_points("R10,U3,L4,D5".to_string());
        assert_eq!(path.len(), 5);
        assert_eq!(path[0].x, 0);
        assert_eq!(path[0].y, 0);
        assert_eq!(path[1].x, 10);
        assert_eq!(path[1].y, 0);
        assert_eq!(path[2].x, 10);
        assert_eq!(path[2].y, 3);
        assert_eq!(path[3].x, 6);
        assert_eq!(path[3].y, 3);
        assert_eq!(path[4].x, 6);
        assert_eq!(path[4].y, -2);
    }

    #[test]
    fn test_intersection_1() {
        let intersection = line_segment_intersection(
            &Point{x:0, y:0}, &Point{x:4, y:0}, &Point{x:2, y:-2}, &Point{x: 2, y: 2}
        );
        let p = intersection.unwrap();
        println!("{} {}", p.x, p.y);
        assert_eq!(p.x, 2);
        assert_eq!(p.y, 0);
    }

    #[test]
    fn test_intersection_2() {
        let intersection = line_segment_intersection(
            &Point{x:0, y:0}, &Point{x:0, y:4}, &Point{x:0, y:0}, &Point{x: 2, y: 0}
        );
        let p = intersection.unwrap();
        println!("{} {}", p.x, p.y);
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);
    }

    #[test]
    fn test_intersection_3() {
        let intersection = line_segment_intersection(
            &Point{x:0, y:0}, &Point{x:0, y:4}, &Point{x: -2, y: 3}, &Point{x: 2, y: 3}
        );
        let p = intersection.unwrap();
        println!("{} {}", p.x, p.y);
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 3);
    }

    #[test]
    fn test_intersection_4() {
        let intersection = line_segment_intersection(
            &Point{x:0, y:0}, &Point{x:1, y:1}, &Point{x: -3, y: 1}, &Point{x: 3, y: 1}
        );
        let p = intersection.unwrap();
        println!("{} {}", p.x, p.y);
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 1);
    }

    #[test]
    fn test_intersection_5() {
        let intersection = line_segment_intersection(
            &Point{x:0, y:0}, &Point{x:8, y:0}, &Point{x: 6, y: 7}, &Point{x: 6, y: 3}
        );
        assert!(intersection.is_none());
    }

    #[test]
    fn test_part1_ex1() {
        let origin = Point{x: 0, y: 0};
        let path1 = path_to_points("R8,U5,L5,D3".to_string());
        let path2 = path_to_points("U7,R6,D4,L4".to_string());
        let common = find_intersections(&path1, &path2);
        let actual_dist = find_min_dist(&common);
        assert_eq!(actual_dist.unwrap(), 6);
    }

    #[test]
    fn test_part1_ex2() {
        let origin = Point{x: 0, y: 0};
        let path1 = path_to_points("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string());
        let path2 = path_to_points("U62,R66,U55,R34,D71,R55,D58,R83".to_string());
        let common = find_intersections(&path1, &path2);
        let actual_dist = find_min_dist(&common);
        assert_eq!(actual_dist.unwrap(), 159);
    }

    #[test]
    fn test_part1_ex3() {
        let origin = Point{x: 0, y: 0};
        let path1 = path_to_points("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string());
        let path2 = path_to_points("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string());
        let common = find_intersections(&path1, &path2);
        let actual_dist = find_min_dist(&common);
        assert_eq!(actual_dist.unwrap(), 135);
    }
}
