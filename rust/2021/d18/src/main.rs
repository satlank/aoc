// https://adventofcode.com/2021/day/18

use itertools::Itertools;
use std::collections::HashSet;
use std::convert::Infallible;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ValueOrNode {
    Value(usize),
    Node(usize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Relation {
    Parent,
    Left,
    Right,
    NoRelation,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Node {
    parent: Option<usize>,
    left: Option<ValueOrNode>,
    right: Option<ValueOrNode>,
}

impl Node {
    fn new() -> Self {
        Self {
            parent: None,
            left: None,
            right: None,
        }
    }

    fn magnitude(&self, nodes: &[Node]) -> usize {
        let left = 3 * match self.left {
            Some(ValueOrNode::Node(nid)) => nodes[nid].magnitude(nodes),
            Some(ValueOrNode::Value(val)) => val,
            _ => unreachable!(),
        };
        let right = 2 * match self.right {
            Some(ValueOrNode::Node(nid)) => nodes[nid].magnitude(nodes),
            Some(ValueOrNode::Value(val)) => val,
            _ => unreachable!(),
        };
        left + right
    }

    fn is_complete(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }

    fn is_root(&self) -> bool {
        self.is_complete() && self.parent.is_none()
    }

    /// Given `self` this function returns whether `other` is the parent node, left node, right
    /// node, or not related at all
    fn get_relation(&self, other: usize) -> Relation {
        if self.parent == Some(other) {
            return Relation::Parent;
        }
        if self.left == Some(ValueOrNode::Node(other)) {
            return Relation::Left;
        }
        if self.right == Some(ValueOrNode::Node(other)) {
            return Relation::Right;
        }
        Relation::NoRelation
    }

    fn set(&mut self, von: ValueOrNode) {
        if self.left.is_none() {
            self.left = Some(von);
        } else {
            assert!(self.right.is_none());
            self.right = Some(von);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SnailfishNumber {
    nodes: Vec<Node>,
}

impl fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = 0;
        let mut gone_left: HashSet<usize> = HashSet::new();
        let mut gone_right: HashSet<usize> = HashSet::new();
        write!(f, "[")?;
        loop {
            if gone_left.contains(&current) && gone_right.contains(&current) {
                if current == 0 {
                    break;
                }
                write!(f, "]")?;
                current = self.nodes[current].parent.unwrap();
            } else {
                if !gone_left.contains(&current) {
                    gone_left.insert(current);
                    match self.nodes[current].left.unwrap() {
                        ValueOrNode::Node(nid) => {
                            current = nid;
                            write!(f, "[")?;
                        }
                        ValueOrNode::Value(val) => write!(f, "{}", val)?,
                    };
                } else {
                    write!(f, ",")?;
                    gone_right.insert(current);
                    match self.nodes[current].right.unwrap() {
                        ValueOrNode::Node(nid) => {
                            current = nid;
                            write!(f, "[")?;
                        }
                        ValueOrNode::Value(val) => write!(f, "{}", val)?,
                    };
                }
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl FromStr for SnailfishNumber {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut nodes: Vec<Node> = Vec::new();
        let mut parent = None;
        for c in s.chars() {
            match c {
                '[' => {
                    let mut n = Node::new();
                    let nid = nodes.len();
                    n.parent = parent;
                    if let Some(pid) = parent {
                        nodes[pid].set(ValueOrNode::Node(nid));
                    }
                    parent = Some(nid);
                    nodes.push(n);
                }
                ']' => {
                    if let Some(pid) = parent {
                        parent = nodes[pid].parent;
                    } else {
                        unreachable!();
                    }
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if let Some(nid) = parent {
                        nodes[nid].set(ValueOrNode::Value(c.to_digit(10).unwrap() as usize));
                    } else {
                        unreachable!();
                    }
                }
                _ => {}
            }
        }
        Ok(SnailfishNumber { nodes })
    }
}

impl<'a, 'b> Add<&'b SnailfishNumber> for &'a SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, other: &'b SnailfishNumber) -> SnailfishNumber {
        let mut nodes = Vec::with_capacity(1 + self.nodes.len() + other.nodes.len());
        nodes.push(Node::new());
        nodes[0].left = Some(ValueOrNode::Node(1));
        nodes[0].right = Some(ValueOrNode::Node(1 + self.nodes.len()));
        for i in 0..self.nodes.len() {
            nodes.push(self.nodes[i]);
            match nodes[1 + i].parent {
                Some(pid) => nodes[1 + i].parent = Some(pid + 1),
                None => nodes[1 + i].parent = Some(0),
            }
            match nodes[1 + i].left {
                Some(ValueOrNode::Node(nid)) => {
                    nodes[1 + i].left = Some(ValueOrNode::Node(nid + 1))
                }
                _ => {}
            }
            match nodes[1 + i].right {
                Some(ValueOrNode::Node(nid)) => {
                    nodes[1 + i].right = Some(ValueOrNode::Node(nid + 1))
                }
                _ => {}
            }
        }
        let offset = 1 + self.nodes.len();
        for i in 0..other.nodes.len() {
            nodes.push(other.nodes[i]);
            match nodes[offset + i].parent {
                Some(pid) => nodes[offset + i].parent = Some(pid + offset),
                None => nodes[offset + i].parent = Some(0),
            }
            match nodes[offset + i].left {
                Some(ValueOrNode::Node(nid)) => {
                    nodes[offset + i].left = Some(ValueOrNode::Node(nid + offset))
                }
                _ => {}
            }
            match nodes[offset + i].right {
                Some(ValueOrNode::Node(nid)) => {
                    nodes[offset + i].right = Some(ValueOrNode::Node(nid + offset))
                }
                _ => {}
            }
        }
        SnailfishNumber { nodes }
    }
}

impl SnailfishNumber {
    fn reduce(&mut self) {
        loop {
            let mut done_something = false;
            if let Some(nid) = self.node_to_explode() {
                self.explode(nid);
                done_something = true;
            } else {
                if let Some(nid) = self.node_to_split() {
                    self.split(nid);
                    done_something = true;
                }
            }
            if !done_something {
                break;
            }
        }
    }

    fn node_to_explode(&self) -> Option<usize> {
        let mut current = 0;
        let mut gone_left: HashSet<usize> = HashSet::new();
        let mut gone_right: HashSet<usize> = HashSet::new();
        let mut depth = 0;
        loop {
            if gone_left.contains(&current) && gone_right.contains(&current) {
                if current == 0 {
                    break;
                }
                current = self.nodes[current].parent.unwrap();
                depth -= 1;
            } else {
                if !gone_left.contains(&current) {
                    gone_left.insert(current);
                    match self.nodes[current].left.unwrap() {
                        ValueOrNode::Node(nid) => {
                            if depth == 3 {
                                return Some(nid);
                            }
                            depth += 1;
                            current = nid;
                        }
                        ValueOrNode::Value(_) => {}
                    };
                } else {
                    gone_right.insert(current);
                    match self.nodes[current].right.unwrap() {
                        ValueOrNode::Node(nid) => {
                            if depth == 3 {
                                return Some(nid);
                            }
                            depth += 1;
                            current = nid;
                        }
                        ValueOrNode::Value(_) => {}
                    };
                }
            }
        }
        None
    }

    fn explode(&mut self, nid: usize) {
        assert!(nid > 0);
        assert!(nid < self.nodes.len());
        let parent = self.nodes[nid].parent.unwrap();
        let relation = self.nodes[parent].get_relation(nid);

        if let Some(ValueOrNode::Value(left)) = self.nodes[nid].left {
            let mut last = nid;
            let mut current = parent;
            loop {
                match self.nodes[current].get_relation(last) {
                    Relation::Left => {
                        // Walk up
                        last = current;
                        if self.nodes[current].is_root() {
                            break;
                        }
                        current = self.nodes[current].parent.unwrap();
                    }
                    Relation::Right => {
                        // Descend into left
                        match self.nodes[current].left {
                            Some(ValueOrNode::Value(val)) => {
                                self.nodes[current].left = Some(ValueOrNode::Value(val + left));
                                break;
                            }
                            Some(ValueOrNode::Node(next)) => {
                                last = current;
                                current = next;
                            }
                            None => {
                                unreachable!();
                            }
                        }
                    }
                    Relation::Parent => {
                        // Descend into right
                        match self.nodes[current].right {
                            Some(ValueOrNode::Value(val)) => {
                                self.nodes[current].right = Some(ValueOrNode::Value(val + left));
                                break;
                            }
                            Some(ValueOrNode::Node(next)) => {
                                last = current;
                                current = next;
                            }
                            None => {
                                unreachable!();
                            }
                        }
                    }
                    Relation::NoRelation => {
                        unreachable!();
                    }
                }
            }
        }
        if let Some(ValueOrNode::Value(right)) = self.nodes[nid].right {
            let mut last = nid;
            let mut current = parent;
            loop {
                match self.nodes[current].get_relation(last) {
                    Relation::Right => {
                        // Walk up
                        last = current;
                        if self.nodes[current].is_root() {
                            break;
                        }
                        current = self.nodes[current].parent.unwrap();
                    }
                    Relation::Left => {
                        // Descend into right
                        match self.nodes[current].right {
                            Some(ValueOrNode::Value(val)) => {
                                self.nodes[current].right = Some(ValueOrNode::Value(val + right));
                                break;
                            }
                            Some(ValueOrNode::Node(next)) => {
                                last = current;
                                current = next;
                            }
                            None => {
                                unreachable!();
                            }
                        }
                    }
                    Relation::Parent => {
                        // Descend into left
                        match self.nodes[current].left {
                            Some(ValueOrNode::Value(val)) => {
                                self.nodes[current].left = Some(ValueOrNode::Value(val + right));
                                break;
                            }
                            Some(ValueOrNode::Node(next)) => {
                                last = current;
                                current = next;
                            }
                            None => {
                                unreachable!();
                            }
                        }
                    }
                    Relation::NoRelation => {
                        unreachable!();
                    }
                }
            }
        }

        if relation == Relation::Left {
            self.nodes[parent].left = Some(ValueOrNode::Value(0));
        } else {
            assert!(relation == Relation::Right);
            self.nodes[parent].right = Some(ValueOrNode::Value(0));
        }
        self.nodes[nid] = Node::new();
    }

    fn node_to_split(&self) -> Option<usize> {
        let mut current = 0;
        let mut gone_left: HashSet<usize> = HashSet::new();
        let mut gone_right: HashSet<usize> = HashSet::new();
        loop {
            if gone_left.contains(&current) && gone_right.contains(&current) {
                if current == 0 {
                    break;
                }
                current = self.nodes[current].parent.unwrap();
            } else {
                if !gone_left.contains(&current) {
                    gone_left.insert(current);
                    match self.nodes[current].left.unwrap() {
                        ValueOrNode::Node(nid) => {
                            current = nid;
                        }
                        ValueOrNode::Value(val) => {
                            if val > 9 {
                                return Some(current);
                            }
                        }
                    };
                } else {
                    gone_right.insert(current);
                    match self.nodes[current].right.unwrap() {
                        ValueOrNode::Node(nid) => {
                            current = nid;
                        }
                        ValueOrNode::Value(val) => {
                            if val > 9 {
                                return Some(current);
                            }
                        }
                    };
                }
            }
        }
        None
    }

    fn split(&mut self, nid: usize) {
        assert!(nid < self.nodes.len());
        if let Some(ValueOrNode::Value(val)) = self.nodes[nid].left {
            if val > 9 {
                self.nodes.push(Node {
                    parent: Some(nid),
                    left: Some(ValueOrNode::Value(val / 2)),
                    right: Some(ValueOrNode::Value(val - val / 2)),
                });
                self.nodes[nid].left = Some(ValueOrNode::Node(self.nodes.len() - 1));
                return;
            }
        }
        if let Some(ValueOrNode::Value(val)) = self.nodes[nid].right {
            assert!(val > 9);
            self.nodes.push(Node {
                parent: Some(nid),
                left: Some(ValueOrNode::Value(val / 2)),
                right: Some(ValueOrNode::Value(val - val / 2)),
            });
            self.nodes[nid].right = Some(ValueOrNode::Node(self.nodes.len() - 1));
        }
    }

    fn magnitude(&self) -> usize {
        self.nodes[0].magnitude(&self.nodes)
    }
}

fn read<R: Read>(io: R) -> Vec<SnailfishNumber> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.unwrap().parse::<SnailfishNumber>().unwrap())
        .collect()
}

fn part_1(nums: &[SnailfishNumber]) -> usize {
    let mut res = &nums[0] + &nums[1];
    res.reduce();
    for i in 2..nums.len() {
        res = &res + &nums[i];
        res.reduce()
    }
    res.magnitude()
}

fn part_2(nums: &[SnailfishNumber]) -> usize {
    nums.iter()
        .combinations(2)
        .map(|com| {
            let mut sum1 = com[0] + com[1];
            sum1.reduce();
            let mut sum2 = com[1] + com[0];
            sum2.reduce();
            usize::max(sum1.magnitude(), sum2.magnitude())
        })
        .max()
        .unwrap()
}

fn main() {
    let input = read(File::open("input.txt").unwrap());
    println!(
        "Little snailfishes homework magnitude is {}",
        part_1(&input)
    );
    println!(
        "Largest magnitude of the sum of two numbers is {}",
        part_2(&input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let input = read(File::open("test1.txt").unwrap());
        assert_eq!(
            input,
            vec![
                "[1,2]".parse::<SnailfishNumber>().unwrap(),
                "[[1,2],3]".parse::<SnailfishNumber>().unwrap(),
                "[9,[8,7]]".parse::<SnailfishNumber>().unwrap(),
                "[[1,9],[8,5]]".parse::<SnailfishNumber>().unwrap(),
                "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]"
                    .parse::<SnailfishNumber>()
                    .unwrap(),
                "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]"
                    .parse::<SnailfishNumber>()
                    .unwrap(),
                "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"
                    .parse::<SnailfishNumber>()
                    .unwrap(),
            ]
        );
    }

    #[test]
    fn test_stringfy_sfn1() {
        let sfn_str = "[1,2]";
        let sfn = sfn_str.parse::<SnailfishNumber>().unwrap();
        assert_eq!(format!("{}", sfn), sfn_str);
    }

    #[test]
    fn test_stringfy_sfn2() {
        let sfn_str = "[1,[2,3]]";
        let sfn = sfn_str.parse::<SnailfishNumber>().unwrap();
        assert_eq!(format!("{}", sfn), sfn_str);
    }

    #[test]
    fn test_stringfy_sfn3() {
        let sfn_str = "[[1,2],3]";
        let sfn = sfn_str.parse::<SnailfishNumber>().unwrap();
        assert_eq!(format!("{}", sfn), sfn_str);
    }

    #[test]
    fn test_stringfy_sfn4() {
        let sfn_str = "[[1,2],[3,4]]";
        let sfn = sfn_str.parse::<SnailfishNumber>().unwrap();
        assert_eq!(format!("{}", sfn), sfn_str);
    }

    #[test]
    fn test_stringfy_sfn5() {
        let sfn_str = "[[1,[2,3]],[[4,5],6]]";
        let sfn = sfn_str.parse::<SnailfishNumber>().unwrap();
        assert_eq!(format!("{}", sfn), sfn_str);
    }

    #[test]
    fn test_add() {
        let a = "[1,2]".parse::<SnailfishNumber>().unwrap();
        let b = "[1,2]".parse::<SnailfishNumber>().unwrap();
        assert_eq!(format!("{}", &a + &b), "[[1,2],[1,2]]");
        let a = "[1,[[3,4],3]]".parse::<SnailfishNumber>().unwrap();
        let b = "[[7,6],[9,8]]".parse::<SnailfishNumber>().unwrap();
        assert_eq!(format!("{}", &a + &b), "[[1,[[3,4],3]],[[7,6],[9,8]]]");
    }

    #[test]
    fn test_explode_1() {
        let mut num = "[[[[[9,8],1],2],3],4]".parse::<SnailfishNumber>().unwrap();
        let nte = num.node_to_explode().unwrap();
        assert_eq!(num.nodes[nte].left, Some(ValueOrNode::Value(9)));
        assert_eq!(num.nodes[nte].right, Some(ValueOrNode::Value(8)));
        num.explode(nte);
        assert_eq!(format!("{}", num), "[[[[0,9],2],3],4]");
    }

    #[test]
    fn test_explode_2() {
        let mut num = "[7,[6,[5,[4,[3,2]]]]]".parse::<SnailfishNumber>().unwrap();
        let nte = num.node_to_explode().unwrap();
        assert_eq!(num.nodes[nte].left, Some(ValueOrNode::Value(3)));
        assert_eq!(num.nodes[nte].right, Some(ValueOrNode::Value(2)));
        num.explode(nte);
        assert_eq!(format!("{}", num), "[7,[6,[5,[7,0]]]]");
    }

    #[test]
    fn test_explode_3() {
        let mut num = "[[6,[5,[4,[3,2]]]],1]".parse::<SnailfishNumber>().unwrap();
        let nte = num.node_to_explode().unwrap();
        assert_eq!(num.nodes[nte].left, Some(ValueOrNode::Value(3)));
        assert_eq!(num.nodes[nte].right, Some(ValueOrNode::Value(2)));
        num.explode(nte);
        assert_eq!(format!("{}", num), "[[6,[5,[7,0]]],3]");
    }

    #[test]
    fn test_explode_4() {
        let mut num = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
            .parse::<SnailfishNumber>()
            .unwrap();
        let nte = num.node_to_explode().unwrap();
        assert_eq!(num.nodes[nte].left, Some(ValueOrNode::Value(7)));
        assert_eq!(num.nodes[nte].right, Some(ValueOrNode::Value(3)));
        num.explode(nte);
        assert_eq!(format!("{}", num), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    }

    #[test]
    fn test_explode_5() {
        let mut num = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
            .parse::<SnailfishNumber>()
            .unwrap();
        let nte = num.node_to_explode().unwrap();
        assert_eq!(num.nodes[nte].left, Some(ValueOrNode::Value(3)));
        assert_eq!(num.nodes[nte].right, Some(ValueOrNode::Value(2)));
        num.explode(nte);
        assert_eq!(format!("{}", num), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn test_split_1() {
        let mut num = SnailfishNumber {
            nodes: vec![
                Node {
                    parent: None,
                    left: Some(ValueOrNode::Value(17)),
                    right: Some(ValueOrNode::Node(1)),
                },
                Node {
                    parent: Some(0),
                    left: Some(ValueOrNode::Value(1)),
                    right: Some(ValueOrNode::Value(11)),
                },
            ],
        };
        let nts = num.node_to_split().unwrap();
        assert_eq!(num.nodes[nts].left, Some(ValueOrNode::Value(17)));
        assert_eq!(num.nodes[nts].right, Some(ValueOrNode::Node(1)));
        num.split(nts);
        assert_eq!(format!("{}", num), "[[8,9],[1,11]]");
        let nts = num.node_to_split().unwrap();
        assert_eq!(num.nodes[nts].left, Some(ValueOrNode::Value(1)));
        assert_eq!(num.nodes[nts].right, Some(ValueOrNode::Value(11)));
        num.split(nts);
        println!("{:?}", num.nodes);
        assert_eq!(format!("{}", num), "[[8,9],[1,[5,6]]]");
    }

    #[test]
    fn test_reduce_1() {
        let mut num = &"[[[[4,3],4],4],[7,[[8,4],9]]]"
            .parse::<SnailfishNumber>()
            .unwrap()
            + &"[1,1]".parse::<SnailfishNumber>().unwrap();
        assert_eq!(format!("{}", num), "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        num.explode(num.node_to_explode().unwrap());
        assert_eq!(format!("{}", num), "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
        num.explode(num.node_to_explode().unwrap());
        assert_eq!(format!("{}", num), "[[[[0,7],4],[15,[0,13]]],[1,1]]");
        num.split(num.node_to_split().unwrap());
        assert_eq!(format!("{}", num), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
        num.split(num.node_to_split().unwrap());
        assert_eq!(format!("{}", num), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
        num.explode(num.node_to_explode().unwrap());
        assert_eq!(format!("{}", num), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_reduce_2() {
        let mut num = &"[[[[4,3],4],4],[7,[[8,4],9]]]"
            .parse::<SnailfishNumber>()
            .unwrap()
            + &"[1,1]".parse::<SnailfishNumber>().unwrap();
        num.reduce();
        assert_eq!(format!("{}", num), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_list_sums_1() {
        let input = read(File::open("test2.txt").unwrap());
        let mut res = &input[0] + &input[1];
        res.reduce();
        res = &res + &input[2];
        res.reduce();
        res = &res + &input[3];
        res.reduce();
        assert_eq!(format!("{}", res), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
        res = &res + &input[4];
        res.reduce();
        assert_eq!(format!("{}", res), "[[[[3,0],[5,3]],[4,4]],[5,5]]");
        res = &res + &input[5];
        res.reduce();
        assert_eq!(format!("{}", res), "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }

    #[test]
    fn test_list_sums_2() {
        let input = read(File::open("test3.txt").unwrap());
        let mut res = &input[0] + &input[1];
        res.reduce();
        for i in 2..input.len() {
            res = &res + &input[i];
            res.reduce();
        }
        assert_eq!(
            format!("{}", res),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );
    }

    #[test]
    fn test_magnitudes() {
        assert_eq!(
            "[[1,2],[[3,4],5]]"
                .parse::<SnailfishNumber>()
                .unwrap()
                .magnitude(),
            143
        );
        assert_eq!(
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
                .parse::<SnailfishNumber>()
                .unwrap()
                .magnitude(),
            1384
        );
        assert_eq!(
            "[[[[1,1],[2,2]],[3,3]],[4,4]]"
                .parse::<SnailfishNumber>()
                .unwrap()
                .magnitude(),
            445
        );
        assert_eq!(
            "[[[[3,0],[5,3]],[4,4]],[5,5]]"
                .parse::<SnailfishNumber>()
                .unwrap()
                .magnitude(),
            791
        );
        assert_eq!(
            "[[[[5,0],[7,4]],[5,5]],[6,6]]"
                .parse::<SnailfishNumber>()
                .unwrap()
                .magnitude(),
            1137
        );
        assert_eq!(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .parse::<SnailfishNumber>()
                .unwrap()
                .magnitude(),
            3488
        );
    }

    #[test]
    fn test_part_1() {
        let input = read(File::open("test4.txt").unwrap());
        assert_eq!(part_1(&input), 4140);
    }

    #[test]
    fn test_part_2() {
        let input = read(File::open("test4.txt").unwrap());
        assert_eq!(part_2(&input), 3993);
    }
}
