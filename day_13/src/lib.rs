use std::cmp::Ordering;
use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
enum PacketItem {
    Item(u16),
    Packet(Vec<PacketItem>)
}


impl Eq for PacketItem {}

impl PartialEq<Self> for PacketItem {
    fn eq(&self, other: &Self) -> bool {
        match is_in_right_order(&self, other) {
            Some(_) => false,
            None => true
        }
    }
}

impl PartialOrd<Self> for PacketItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match is_in_right_order(&self, other) {
            Some(b) => {
                if b {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            None => Some(Ordering::Equal)
        }
    }
}

impl Ord for PacketItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match is_in_right_order(&self, other) {
            Some(b) => {
                if b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            None => Ordering::Equal
        }
    }
}

fn read_in_file_system(file_path: &str) -> Result<(Vec<PacketItem>, Vec<PacketItem>)> {
    let lines = read_lines(file_path)?;
    let mut packet_vec_1 = Vec::new();
    let mut packet_vec_2 = Vec::new();
    let mut first = true;
    for line in lines {
        if let Ok(l) = line{
            if 1 < l.len() {
                if first {
                    packet_vec_1.push(serde_json::from_str(&l)?);
                    first = false;
                } else {
                    packet_vec_2.push(serde_json::from_str(&l)?);
                    first = true;
                }
            }
        }
    }
    Ok((packet_vec_1, packet_vec_2))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_in_right_order(first_item: &PacketItem, second_item: &PacketItem) -> Option<bool> {
    match (first_item, second_item) {
        (PacketItem::Item(val1), PacketItem::Item(val2)) => {
            if val1 < val2 {
                Some(true)
            } else if val2 < val1 {
                Some(false)
            } else {
                None
            }
        }
        (PacketItem::Packet(list1), PacketItem::Packet(list2)) => {
            let list_1_len = list1.len();
            let list_2_len = list2.len();
            if list_1_len == 0 && list_2_len == 0 {
                None
            } else if list_1_len == 0 {
                Some(true)
            } else if list_2_len == 0 {
                Some(false)
            } else {
                if let Some(b) = is_in_right_order(&list1[0], &list2[0]) {
                    Some(b)
                } else {
                    is_in_right_order(
                        &PacketItem::Packet(list1[1..].to_vec()),
                        &PacketItem::Packet(list2[1..].to_vec())
                    )
                }
            }
        }
        (PacketItem::Packet(_), PacketItem::Item(val)) => {
            is_in_right_order(first_item, &PacketItem::Packet(vec![PacketItem::Item(*val)]))
        }
        (PacketItem::Item(val), PacketItem::Packet(_)) => {
            is_in_right_order(&PacketItem::Packet(vec![PacketItem::Item(*val)]), second_item)
        }
    }
}

pub fn solve_routine(file_path: &str) -> Result<usize> {
    let mut sum_of_indices = 0;
    let (packet_item_vec_1, packet_item_vec_2) = read_in_file_system(file_path)?;
    let mut pair_index = 1;
    while pair_index <= packet_item_vec_1.len() && pair_index <= packet_item_vec_2.len() {
        let index = pair_index - 1;
        let packet_item_1 = &packet_item_vec_1[index];
        let packet_item_2 = &packet_item_vec_2[index];
        if is_in_right_order(packet_item_1, packet_item_2).unwrap() {
            sum_of_indices += pair_index;
        }
        pair_index += 1;
    }
    Ok(sum_of_indices)
}

fn is_divider_packet(packet: &PacketItem) -> bool {
    match packet {
        PacketItem::Packet(v1) => {
            if v1.len() == 1 {
                match &v1[0] {
                    PacketItem::Packet(v2) => {
                        if v2.len() == 1 {
                            match &v2[0] {
                                PacketItem::Item(val) => {
                                    if val == &2_u16 || val == &6_u16 {
                                        true
                                    } else {
                                        false
                                    }
                                }
                                _ => false
                            }
                        } else {
                            false
                        }
                    }
                    _ => false
                }
            } else {
                false
            }
        }
        _ => false
    }
}

pub fn solve_routine_2(file_path: &str) -> Result<usize> {
    let (mut packet_item_vec, mut packet_item_vec_2) = read_in_file_system(file_path)?;
    packet_item_vec.append(&mut packet_item_vec_2);
    let mut divider_packets = vec![
        PacketItem::Packet(vec![
            PacketItem::Packet(vec![
                PacketItem::Item(2)
            ])
        ]),
        PacketItem::Packet(vec![
            PacketItem::Packet(vec![
                PacketItem::Item(6)
            ])
        ])
    ];
    packet_item_vec.append(&mut divider_packets);
    packet_item_vec.sort();
    let mut result = 1;
    for i in 0..packet_item_vec.len() {
        if is_divider_packet(&packet_item_vec[i]) {
            result *= i+1;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{read_in_file_system, solve_routine, solve_routine_2};

    #[test]
    fn test_parser() {
        let result = read_in_file_system("input_test");
        match result {
            Ok(_) => assert!(true),
            Err(e) => {
                println!("Error: {}", e);
                assert!(false)
            }
        }
    }

    #[test]
    fn test_puzzle_1() {
        assert_eq!(solve_routine("input_test").unwrap(), 13)
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(solve_routine_2("input_test").unwrap(), 140)
    }
}