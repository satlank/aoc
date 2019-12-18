use std::fs;
use std::io::{stdout, Write};

#[macro_use] extern crate scan_fmt;

fn mem_from_string(content: String) -> Vec<i32> {
    content.split(',').map(|item| item.trim().parse::<i32>().unwrap()).collect()
}

fn print_memory(memory: &Vec<i32>) {
    println!("{:?}", memory);
}

fn get_value(memory: &Vec<i32>, address: usize, mode: i32) -> i32 {
    if mode == 1 {
        memory[address]
    } else {
        memory[memory[address] as usize]
    }
}

fn get_parameter_modes(modes: i32, num_params: u32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut tmp = modes;
    result.push(tmp % 10);
    tmp /= 10;
    for _ in 0..num_params-1 {
        result.push(tmp % 10);
        tmp /= 10;
    }
    return result;
}

fn run_program(memory: &mut Vec<i32>, initial_op_counter: usize) {
    let mut op_counter = initial_op_counter;
    let mut input_counter = 0;
    let mut output_counter = 0;
    while memory[op_counter] != 99 {
        let opcode = memory[op_counter] % 100;
        let parametermode = memory[op_counter] / 100;
        match opcode {
            1 => {
                let num_params = 3;
                let modes = get_parameter_modes(parametermode, num_params);
                if modes[2] != 0 {
                    println!(
                        "op_counter: {:?}, opcode: {:?}, modes: {:?}",
                        op_counter, opcode, modes
                    );
                    panic!("Expected mode 0 for result parameter")
                }
                let x = get_value(memory, op_counter + 1 as usize, modes[0]);
                let y = get_value(memory, op_counter + 2 as usize, modes[1]);
                let pr = memory[op_counter + 3] as usize;
                memory[pr] = x + y;
                op_counter += (1 + num_params) as usize;
            },
            2 => {
                let num_params = 3;
                let modes = get_parameter_modes(parametermode, num_params);
                if modes[2] != 0 {
                    println!(
                        "op_counter: {:?}, opcode: {:?}, modes: {:?}",
                        op_counter, opcode, modes
                    );
                    panic!("Expected mode 0 for result parameter")
                }
                let x = get_value(memory, op_counter + 1 as usize, modes[0]);
                let y = get_value(memory, op_counter + 2 as usize, modes[1]);
                let pr = memory[op_counter + 3] as usize;
                memory[pr] = x * y;
                op_counter += (1 + num_params) as usize;
            },
            3 => {
                let num_params = 1;
                let modes = get_parameter_modes(parametermode, num_params);
                if modes[0] != 0 {
                    println!(
                        "op_counter: {:?}, opcode: {:?}, modes: {:?}",
                        op_counter, opcode, modes
                    );
                    panic!("Expected mode 0 for result parameter")
                }
                print!("i{}: ", input_counter);
                stdout().flush().unwrap();
                let val = scanln_fmt!("{}", i32).unwrap();
                let pr = memory[op_counter + 1] as usize;
                memory[pr] = val;
                input_counter += 1;
                op_counter += (1 + num_params) as usize;
            },
            4 => {
                let num_params = 1;
                let modes = get_parameter_modes(parametermode, num_params);
                let x = get_value(memory, op_counter + 1 as usize, modes[0]);
                println!("o{}: {}", output_counter, x);
                output_counter += 1;
                op_counter += (1 + num_params) as usize;
            },
            5 => {
                let num_params = 2;
                let modes = get_parameter_modes(parametermode, num_params);
                let con = get_value(memory, op_counter + 1 as usize, modes[0]);
                if con != 0 {
                    let new_opcounter = get_value(memory, op_counter + 2 as usize, modes[1]) as usize;
                    op_counter = new_opcounter;
                } else {
                    op_counter += (1 + num_params) as usize;
                }
            },
            6 => {
                let num_params = 2;
                let modes = get_parameter_modes(parametermode, num_params);
                let con = get_value(memory, op_counter + 1 as usize, modes[0]);
                if con == 0 {
                    let new_opcounter = get_value(memory, op_counter + 2 as usize, modes[1]) as usize;
                    op_counter = new_opcounter;
                } else {
                    op_counter += (1 + num_params) as usize;
                }
            },
            7 => {
                let num_params = 3;
                let modes = get_parameter_modes(parametermode, num_params);
                if modes[2] != 0 {
                    println!(
                        "op_counter: {:?}, opcode: {:?}, modes: {:?}",
                        op_counter, opcode, modes
                    );
                    panic!("Expected mode 0 for result parameter")
                }
                let first = get_value(memory, op_counter + 1 as usize, modes[0]);
                let second = get_value(memory, op_counter + 2 as usize, modes[1]);
                let pr = memory[op_counter + 3] as usize;
                memory[pr] = if first < second { 1 } else { 0 };
                op_counter += (1 + num_params) as usize;
            },
            8 => {
                let num_params = 3;
                let modes = get_parameter_modes(parametermode, num_params);
                if modes[2] != 0 {
                    println!(
                        "op_counter: {:?}, opcode: {:?}, modes: {:?}",
                        op_counter, opcode, modes
                    );
                    panic!("Expected mode 0 for result parameter")
                }
                let first = get_value(memory, op_counter + 1 as usize, modes[0]);
                let second = get_value(memory, op_counter + 2 as usize, modes[1]);
                let pr = memory[op_counter + 3] as usize;
                memory[pr] = if first == second { 1 } else { 0 };
                op_counter += (1 + num_params) as usize;
            },
            _ => {
                println!("op_counter: {:?}, op_code: {:?}", op_counter, opcode);
                panic!("Unsupported opcode")
            }
        }
    }
}

fn main() {
    let filename = "input.txt";

    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let memory = mem_from_string(content);

    let mut instance = memory.to_vec();
    run_program(&mut instance, 0);
    print_memory(&instance);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_parameter_mode() {
        assert_eq!(get_parameter_modes(2/100, 3), [0, 0, 0]);
        assert_eq!(get_parameter_modes(102/100, 3), [1, 0, 0]);
        assert_eq!(get_parameter_modes(1102/100, 3), [1, 1, 0]);
    }
}
