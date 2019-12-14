use std::fs::File;
use std::io::{BufRead, BufReader};


fn calc_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn calc_recursive_fuel(mass: i32) -> i32 {
    if mass > 6 {
        let fuel = calc_fuel(mass);
        fuel + calc_recursive_fuel(fuel)
    } else {
        0
    }
}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum_module = 0;
    let mut sum_all = 0;
    for (index, line) in reader.lines().enumerate() {
        let module_mass = line.unwrap().parse::<i32>().unwrap();
        let fuel_module = calc_fuel(module_mass);
        let fuel_extra = calc_recursive_fuel(fuel_module);

        sum_module += fuel_module;
        sum_all += fuel_module + fuel_extra;
        // Show the line and its number.
        println!("{}. {} {} {} {} {}", index + 1, module_mass, fuel_module, fuel_extra, sum_module, sum_all);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1_ex1() {
        assert_eq!(calc_fuel(12), 2);
    }

    #[test]
    fn test_part1_ex2() {
        assert_eq!(calc_fuel(14), 2);
    }

    #[test]
    fn test_part1_ex3() {
        assert_eq!(calc_fuel(1969), 654);
    }

    #[test]
    fn test_part1_ex4() {
        assert_eq!(calc_fuel(100756), 33583);
    }

    #[test]
    fn test_part2_ex1() {
        let mass = 14;
        let fuel = calc_fuel(mass);
        let extra_fuel = calc_recursive_fuel(fuel);
        assert_eq!(fuel + extra_fuel, 2);
    }

    #[test]
    fn test_part2_ex2() {
        let mass = 1969;
        let fuel = calc_fuel(mass);
        let extra_fuel = calc_recursive_fuel(fuel);
        assert_eq!(fuel + extra_fuel, 966);
    }

    #[test]
    fn test_part2_ex3() {
        let mass = 100756;
        let fuel = calc_fuel(mass);
        let extra_fuel = calc_recursive_fuel(fuel);
        assert_eq!(fuel + extra_fuel, 50346);
    }
}
