use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
}

fn part_1() {
    let file = File::open("./input.txt").unwrap();
    let lines = BufReader::new(file).lines().map(Result::unwrap);

    let mut memory: HashMap<i32, u64> = HashMap::new();
    let mut mask_ones: u64 = 0x11111;
    let mut mask_zeroes: u64 = 0x00000;
    for line in lines {
        if &line[..4] == "mask" {
            let mask_new = line[7..].to_string();
            mask_ones = 0b0;
            mask_zeroes = 0b1;

            // convert to binary
            // 10X0110X01100X00111XX00001X011101001 - input
            // 100011000110000011100000010011101001 - ones
            // 101011010110010011111000011011101001 - zeroes
            for bit in mask_new.chars() {
                match bit {
                    '1' => {
                        mask_ones |= 0b1;
                        mask_zeroes |= 0b1;
                    }
                    '0' => (),
                    'X' => {
                        mask_zeroes |= 0b1;
                    }
                    _ => panic!(),
                }

                // could move these before the match
                // then wouldn't have to undo after loop
                mask_ones <<= 1;
                mask_zeroes <<= 1;
            }

            mask_ones >>= 1;
            mask_zeroes >>= 1;
        } else if &line[..3] == "mem" {
            let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
            let captures = re.captures(&line).unwrap();

            let mem_loc = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let mut mem_val = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();

            // combine mem_val with mask
            mem_val |= mask_ones;
            mem_val &= mask_zeroes;

            memory.insert(mem_loc, mem_val);
        } else {
            panic!();
        }
    }

    println!("{}", memory.iter().fold(0, |sum, (_, value)| sum + value))
}
