use std::env;
use std::fs;

fn to_bits(input: &str) -> Vec<bool> {
    let mut out = Vec::new();
    for c in input.chars() {
        let nybble = c.to_digit(16).unwrap();
        for i in (0..4).rev() {
            out.push(((nybble >> i) & 1) == 1);
        }
    }
    out
}

fn bits_to_num(bits: &[bool]) -> usize {
    let mut out = 0;
    for b in bits.iter() {
        out *= 2;
        out += *b as usize;
    }
    out
}

#[derive(Clone, Debug, PartialEq)]
enum Payload {
    Value(usize),
    Subpackets(Vec<Packet>),
}

#[derive(Clone, Debug, PartialEq)]
struct Packet {
    version: usize,
    packet_type: usize,
    payload: Payload,
}

fn parse_packet(bits: &[bool]) -> (Packet, usize) {
    let mut i = 0;
    let ver = bits_to_num(&bits[i..i + 3]);
    i += 3;
    let packet_type = bits_to_num(&bits[i..i + 3]);
    i += 3;
    if packet_type == 4 {
        // literal
        let mut num_bits = Vec::new();
        let mut more = bits[i];
        i += 1;
        num_bits.extend_from_slice(&bits[i..i + 4]);
        i += 4;
        while more {
            more = bits[i];
            i += 1;
            num_bits.extend_from_slice(&bits[i..i + 4]);
            i += 4;
        }
        let num = bits_to_num(&num_bits);
        return (
            Packet {
                version: ver,
                packet_type,
                payload: Payload::Value(num),
            },
            i,
        );
    }
    // otherwise, sub-packets.
    let length_type = bits[i];
    i += 1;
    let mut subpackets = Vec::new();
    if length_type {
        let num_subpackets = bits_to_num(&bits[i..i + 11]);
        i += 11;
        for _ in 0..num_subpackets {
            let (sub, len) = parse_packet(&bits[i..]);
            subpackets.push(sub);
            i += len;
        }
    } else {
        let subpackets_len = bits_to_num(&bits[i..i + 15]);
        i += 15;
        let mut sub_len = 0;
        while sub_len < subpackets_len {
            let (sub, len) = parse_packet(&bits[i..]);
            subpackets.push(sub);
            sub_len += len;
            i += len;
        }
    }
    (
        Packet {
            version: ver,
            packet_type,
            payload: Payload::Subpackets(subpackets),
        },
        i,
    )
}

fn sum_versions(packet: &Packet) -> usize {
    let mut sum = packet.version;
    if let Payload::Subpackets(subs) = &packet.payload {
        for sub in subs.iter() {
            sum += sum_versions(sub);
        }
    }
    sum
}

fn evaluate(packet: &Packet) -> usize {
    match &packet.payload {
        Payload::Value(x) => *x,
        Payload::Subpackets(packets) => match packet.packet_type {
            0 => packets.iter().map(evaluate).sum(),
            1 => packets.iter().map(evaluate).product(),
            2 => packets.iter().map(evaluate).min().unwrap(),
            3 => packets.iter().map(evaluate).max().unwrap(),
            5 => {
                assert_eq!(packets.len(), 2);
                (evaluate(&packets[0]) > evaluate(&packets[1])) as usize
            }
            6 => {
                assert_eq!(packets.len(), 2);
                (evaluate(&packets[0]) < evaluate(&packets[1])) as usize
            }
            7 => {
                assert_eq!(packets.len(), 2);
                (evaluate(&packets[0]) == evaluate(&packets[1])) as usize
            }
            _ => panic!("invalid packet type {}", packet.packet_type),
        },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input_str = fs::read_to_string(&args[1]).expect("couldn't read file");
    let input = input_str.trim();
    let bits = to_bits(input);
    let packet = parse_packet(&bits).0;
    println!("{:?}", sum_versions(&packet));
    println!("{:?}", evaluate(&packet));
}
