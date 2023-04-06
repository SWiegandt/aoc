use std::cmp::Ordering;

use crate::util;

#[derive(Clone, PartialEq, Eq)]
enum PacketItem {
    Value(i32),
    List(Vec<PacketItem>),
}

impl PacketItem {
    fn is_divider(&self) -> bool {
        if let PacketItem::List(ls) = self {
            if let [PacketItem::List(ls)] = &ls[..] {
                return match &ls[..] {
                    &[PacketItem::Value(2)] | &[PacketItem::Value(6)] => true,
                    _ => false,
                };
            }
        }

        false
    }
}

impl core::fmt::Debug for PacketItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::List(ls) => write!(f, "{:?}", ls),
        }
    }
}

impl PartialOrd for PacketItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        fn compare_lists(l1: &Vec<PacketItem>, l2: &Vec<PacketItem>) -> Option<Ordering> {
            for i in 0..l1.len().max(l2.len()) {
                let v1 = l1.get(i);
                let v2 = l2.get(i);

                if let None = v1 {
                    return Some(Ordering::Less);
                } else if let None = v2 {
                    return Some(Ordering::Greater);
                } else if let (Some(v1), Some(v2)) = (v1, v2) {
                    let cmp = v1.partial_cmp(v2);

                    if let Some(Ordering::Equal) = cmp {
                        continue;
                    } else {
                        return cmp;
                    }
                }
            }

            Some(Ordering::Equal)
        }

        let cmp = match (&self, &other) {
            (PacketItem::Value(v1), PacketItem::Value(v2)) => Some(v1.cmp(v2)),
            (PacketItem::List(l1), PacketItem::List(l2)) => compare_lists(l1, l2),
            (PacketItem::Value(v1), l2 @ PacketItem::List(_)) => {
                PacketItem::List(vec![PacketItem::Value(*v1)]).partial_cmp(l2)
            }
            (&l1 @ PacketItem::List(_), PacketItem::Value(v2)) => {
                l1.partial_cmp(&PacketItem::List(vec![PacketItem::Value(*v2)]))
            }
        };

        cmp
    }
}

impl Ord for PacketItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_packet(packet_str: &String) -> PacketItem {
    let mut stack: Vec<PacketItem> = vec![];
    let mut digits = vec![];

    for chr in packet_str.chars() {
        if chr.is_digit(10) {
            digits.push(chr);
            continue;
        }

        let value = digits.iter().collect::<String>();
        digits = vec![];
        let mut curr = stack.pop();

        if !value.is_empty() {
            if let Some(PacketItem::List(ref mut items)) = curr {
                items.push(PacketItem::Value(value.parse().unwrap()))
            }
        }

        if chr == '[' {
            if curr.is_some() {
                stack.push(curr.unwrap());
            }

            curr = Some(PacketItem::List(vec![]));
        } else if chr == ']' {
            let mut parent = stack.pop();

            if let Some(PacketItem::List(ref mut parent_items)) = parent {
                parent_items.push(curr.unwrap());
                curr = parent;
            }
        }

        stack.push(curr.unwrap());
    }

    stack.pop().unwrap()
}

fn one(input: &Vec<String>) -> usize {
    input
        .iter()
        .filter(|&s| s != "")
        .collect::<Vec<_>>()
        .chunks(2)
        .enumerate()
        .filter_map(|(i, ls)| {
            let [s1, s2] = ls[0..2] else { unreachable!() };
            let cmp = parse_packet(s1).partial_cmp(&parse_packet(s2));

            if let Some(Ordering::Less) = cmp {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn two(input: &mut Vec<String>) -> usize {
    input.extend(vec!["[[2]]".to_string(), "[[6]]".to_string()]);

    let mut parsed = input
        .iter()
        .filter(|&s| s != "")
        .map(|s| parse_packet(s))
        .collect::<Vec<_>>();

    parsed.sort();

    parsed
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| if item.is_divider() { Some(idx + 1) } else { None })
        .product()
}

pub fn run() -> (usize, usize) {
    let mut input = util::read_input(13);
    (one(&input), two(&mut input))
}
