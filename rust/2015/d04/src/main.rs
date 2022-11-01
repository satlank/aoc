use md5::{Digest, Md5};

fn part_1(input: &str) -> usize {
    let mut current = 0;
    let mut hasher = Md5::new();
    hasher.update(input);
    loop {
        let mut mh = hasher.clone();
        mh.update(format!("{}", current));
        let hash = mh.finalize();
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            return current;
        }
        current += 1;
    }
}

fn part_2(input: &str) -> usize {
    let mut current = 0;
    let mut hasher = Md5::new();
    hasher.update(input);
    loop {
        let mut mh = hasher.clone();
        mh.update(format!("{}", current));
        let hash = mh.finalize();
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return current;
        }
        current += 1;
    }
}

fn main() {
    let num = part_1("bgvyzdsv");
    println!(
        "Lowest number to get an MD5 hash with at least 5 leading zeroes is: {}",
        num
    );
    let num = part_2("bgvyzdsv");
    println!(
        "Lowest number to get an MD5 hash with at least 6 leading zeroes is: {}",
        num
    );
}
