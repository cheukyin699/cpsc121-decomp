use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn c_to_hex(c: char) -> u8 {
    match c {
        'a'...'f' => (c as u8 - 'a' as u8) as u8 + 10,
        '0'...'9' => (c as u8 - '0' as u8) as u8,
        _ => 0,
    }
}

fn to_hex(s: &str) -> u8 {
    let l = s.len();
    let mut ret = 0;
    let mut i = l;
    for c in s.chars() {
        i -= 1;
        ret += (16u8).pow(i as u32) * c_to_hex(c);
    }

    ret
}

fn get_bytes(s: String) -> Vec<u8> {
    s.split_whitespace().map(to_hex).collect()
}

fn get_lines(filename: &str) -> std::io::Result<Vec<u8>> {
    let file = File::open(filename)?;
    let mut buf = BufReader::new(file);
    let mut contents = String::new();

    buf.read_to_string(&mut contents)?;
    Ok(get_bytes(contents))
}

fn decompile(bytes: Vec<u8>) {
    let mut instruction = 0;
    let mut index = 0;
    let mut val_c: u32 = 0;
    for (i, b) in bytes.into_iter().enumerate() {
        if index == 0 {
            instruction = b;
            val_c = 0;

            match instruction {
                0x0 => println!("{:02x}: HALT", i),
                0x30 => print!("{:02x}: MOV ", i),
                0x60 => print!("{:02x}: ADD ", i),
                0x61 => print!("{:02x}: SUB ", i),
                0x62 => print!("{:02x}: AND ", i),
                0x70 => print!("{:02x}: JMP ", i),
                0x71 => {
                    print!("{:02x}: JLE ", i);
                }
                _ => {
                    println!("error: don't know what '{:02x}' is", instruction);
                    index = 0
                }
            }

            if instruction != 0x0 {
                index += 1;
            }
        } else if index == 1 {
            match instruction {
                0x30 => {
                    let r_a = b >> 4;
                    let r_b = b & 0xf;
                    print!("r{:x} & r{:x} <= ", r_a, r_b);

                    index += 1;
                }
                0x60 | 0x61 | 0x62 => {
                    let r_a = b >> 4;
                    let r_b = b & 0xf;
                    println!("r{:x}, r{:x}", r_a, r_b);

                    index = 0;
                }
                0x70 | 0x71 => {
                    val_c = b as u32;

                    index += 1;
                }
                _ => index = 0,
            }
        } else if index > 1 {
            val_c = (val_c << 8) | b as u32;
            index += 1;

            if ((instruction & 0x70 == 0x70) && index == 5) || index == 6 {
                println!("{:08x}", val_c);
                index = 0;
            }
        }
    }
}

fn main() {
    if let Some(filename) = env::args().nth(1) {
        match get_lines(&filename) {
            Ok(bytes) => decompile(bytes),
            Err(e) => panic!(e),
        }
    } else {
        panic!("error: not enough arguments");
    }
}
