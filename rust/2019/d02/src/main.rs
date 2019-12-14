use std::fs;

fn mem_from_string(content: String) -> Vec<i32> {
    content.split(',').map(|item| item.trim().parse::<i32>().unwrap()).collect()
}

fn print_memory(memory: &Vec<i32>) {
    println!("{:?}", memory);
}

fn run_program(memory: &mut Vec<i32>, initial_op_counter: usize) {
    let mut op_counter = initial_op_counter;
    while memory[op_counter] != 99 {
        let opcode = memory[op_counter];
        match opcode {
            1 => {
                let px = memory[op_counter + 1] as usize;
                let py = memory[op_counter + 2] as usize;
                let pr = memory[op_counter + 3] as usize;
                memory[pr] = memory[px] + memory[py];
            },
            2 => {
                let px = memory[op_counter + 1] as usize;
                let py = memory[op_counter + 2] as usize;
                let pr = memory[op_counter + 3] as usize;
                memory[pr] = memory[px] * memory[py];
            },
            _ => {
                panic!("Unsupported opcode")
            }
        }
        op_counter += 4;
    }
}

fn main() {
    let filename = "input.txt";

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let memory = mem_from_string(content);

    for x in 0..100 {
        for y in 0..100 {
            let mut instance = memory.to_vec();
            instance[1] = x;
            instance[2] = y;
            run_program(&mut instance, 0);
            if (instance[0] == 19690720) || (x == 12 && y == 2) {
                println!("x = {}, y = {}, 100 * x + y = {}", x, y, 100 * x + y);
                print_memory(&instance);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_memory() {
        assert_eq!(mem_from_string("1,2,\n3,4".to_string()), [1, 2, 3, 4])
    }

    #[test]
    fn test_part1_ex1() {
        let mut prog = vec![1, 0, 0, 0, 99];
        run_program(&mut prog, 0);
        print_memory(&prog);
        assert_eq!(prog, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_part1_ex2() {
        let mut prog = vec![2, 3, 0, 3, 99];
        run_program(&mut prog, 0);
        print_memory(&prog);
        assert_eq!(prog, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_part1_ex3() {
        let mut prog = vec![2, 4, 4, 5, 99, 0];
        run_program(&mut prog, 0);
        print_memory(&prog);
        assert_eq!(prog, [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_part1_ex4() {
        let mut prog = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run_program(&mut prog, 0);
        print_memory(&prog);
        assert_eq!(prog, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
