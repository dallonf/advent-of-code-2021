use std::borrow::Cow;
use std::fmt::Debug;
use std::str::FromStr;

// Day 16: Packet Decoder
use crate::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: Vec<u8> =
        parse_hexadecimal(include_str!("day16_input.txt").trim()).unwrap();
}

pub fn part_one() -> Result<u32, String> {
    let mut bit_stream = into_bit_stream(&PUZZLE_INPUT);
    let packet = Packet::read(&mut bit_stream)?;
    Ok(packet.version_sum())
}

pub fn part_two() -> Result<u64, String> {
    let mut bit_stream = into_bit_stream(&PUZZLE_INPUT);
    let packet = Packet::read(&mut bit_stream)?;
    packet.evaluate()
}

fn parse_hexadecimal(input: &str) -> Result<Vec<u8>, String> {
    let input = if input.len() % 2 == 0 {
        Cow::from(input)
    } else {
        Cow::from(input.to_owned() + "0")
    };
    hex::decode(input.as_ref()).map_err(|err| err.to_string())
}

type BitStream<'a> = Box<dyn Iterator<Item = bool> + 'a>;

fn into_bit_stream(bytes: &[u8]) -> BitStream {
    Box::new(bytes.into_iter().flat_map(|&byte| {
        (0..u8::BITS as u8).rev().map(move |bit_i| {
            let bit = 1 << bit_i;
            byte & bit == bit
        })
    }))
}

fn read_int(iter: &mut impl Iterator<Item = bool>, bits: usize) -> Result<u64, String> {
    let mut result = 0;
    for bit_i in (0..bits).rev() {
        let bit_i: u64 = bit_i
            .try_into()
            .map_err(|_| "can't parse bit_i into an Integer")
            .unwrap();
        let bit = 1 << bit_i;

        let bit_on = iter
            .next()
            .ok_or("Bit stream ran out while reading an int")?;

        if bit_on {
            result |= bit;
        }
    }
    Ok(result)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum OperatorPacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl OperatorPacketType {
    fn from_id(id: u8) -> Result<Self, String> {
        match id {
            0 => Ok(Self::Sum),
            1 => Ok(Self::Product),
            2 => Ok(Self::Minimum),
            3 => Ok(Self::Maximum),
            4 => Err("This is a literal packet".to_string()),
            5 => Ok(Self::GreaterThan),
            6 => Ok(Self::LessThan),
            7 => Ok(Self::EqualTo),
            _ => Err(format!("Unrecognized operator type ID: {}", id)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

#[derive(Debug, PartialEq, Eq)]
struct LiteralPacket {
    version: u8,
    value: u64,
}
#[derive(Debug, PartialEq, Eq)]
struct OperatorPacket {
    version: u8,
    packet_type: OperatorPacketType,
    sub_packets: Vec<Packet>,
}

impl OperatorPacket {
    fn evaluate(&self) -> Result<u64, String> {
        let evaluated_sub_packets = || {
            self.sub_packets
                .iter()
                .map(|it| it.evaluate())
                .collect::<Result<Vec<_>, _>>()
        };
        match self.packet_type {
            OperatorPacketType::Sum => Ok(evaluated_sub_packets()?.into_iter().sum()),
            OperatorPacketType::Product => Ok(evaluated_sub_packets()?.into_iter().product()),
            OperatorPacketType::Minimum => evaluated_sub_packets()?
                .into_iter()
                .min()
                .ok_or("Can't get the minimum of 0 sub-packets".to_string()),
            OperatorPacketType::Maximum => evaluated_sub_packets()?
                .into_iter()
                .max()
                .ok_or("Can't get the minimum of 0 sub-packets".to_string()),
            OperatorPacketType::GreaterThan => {
                if self.sub_packets.len() != 2 {
                    return Err(format!(
                        "Expected 2 subpackets for GreaterThan operation, but got {}",
                        self.sub_packets.len()
                    ));
                }
                let a = self.sub_packets[0].evaluate();
                let b = self.sub_packets[1].evaluate();
                if a > b {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            OperatorPacketType::LessThan => {
                if self.sub_packets.len() != 2 {
                    return Err(format!(
                        "Expected 2 subpackets for LessThan operation, but got {}",
                        self.sub_packets.len()
                    ));
                }
                let a = self.sub_packets[0].evaluate();
                let b = self.sub_packets[1].evaluate();
                if a < b {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            OperatorPacketType::EqualTo => {
                if self.sub_packets.len() != 2 {
                    return Err(format!(
                        "Expected 2 subpackets for EqualTo operation, but got {}",
                        self.sub_packets.len()
                    ));
                }
                let a = self.sub_packets[0].evaluate();
                let b = self.sub_packets[1].evaluate();
                if a == b {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
        }
    }
}

const PACKET_TYPE_LITERAL: u8 = 4;

impl Packet {
    #[allow(dead_code)]
    fn new_literal(version: u8, value: u64) -> Self {
        Packet::Literal(LiteralPacket { version, value })
    }

    fn read(bit_stream: &mut BitStream) -> Result<Self, String> {
        let version: u8 = read_int(bit_stream, 3)? as u8;
        let packet_type: u8 = read_int(bit_stream, 3)? as u8;

        if packet_type == PACKET_TYPE_LITERAL {
            let mut value_bits = vec![];
            loop {
                let last_group = !bit_stream
                    .next()
                    .ok_or("Ran out of stream while parsing literal packet")?;
                value_bits.extend(bit_stream.take(4));
                if last_group {
                    break;
                }
            }

            let value_bits_count = value_bits.len();
            let value: u64 = read_int(&mut value_bits.into_iter(), value_bits_count)?;

            Ok(Packet::Literal(LiteralPacket { version, value }))
        } else {
            let length_type_1 = bit_stream
                .next()
                .ok_or("Ran out of stream while parsing operator packet")?;

            let sub_packets = if length_type_1 {
                let sub_packet_count = read_int(bit_stream, 11)?;
                (0..sub_packet_count)
                    .map(|_| Packet::read(bit_stream))
                    .collect::<Result<_, _>>()?
            } else {
                let sub_packet_length = read_int(bit_stream, 15)?;
                let mut sub_packets_stream = bit_stream.take(sub_packet_length as usize).peekable();
                let mut sub_packets = vec![];
                while let Some(_) = sub_packets_stream.peek() {
                    let mut boxed_stream: BitStream = Box::new(&mut sub_packets_stream);
                    sub_packets.push(Packet::read(&mut boxed_stream)?);
                }
                sub_packets
            };

            Ok(Packet::Operator(OperatorPacket {
                version,
                packet_type: OperatorPacketType::from_id(packet_type)?,
                sub_packets,
            }))
        }
    }

    fn version_sum(&self) -> u32 {
        match self {
            Packet::Literal(packet) => packet.version as u32,
            Packet::Operator(packet) => {
                let sub_packet_version_sum: u32 =
                    packet.sub_packets.iter().map(|it| it.version_sum()).sum();
                sub_packet_version_sum + packet.version as u32
            }
        }
    }

    fn evaluate(&self) -> Result<u64, String> {
        match self {
            Packet::Literal(packet) => Ok(packet.value),
            Packet::Operator(packet) => packet.evaluate(),
        }
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = parse_hexadecimal(s)?;
        let mut bit_stream = into_bit_stream(&bytes);
        Packet::read(&mut bit_stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex() {
        let result = parse_hexadecimal("D2FE28");
        assert_eq!(result, Ok(vec![0b11010010, 0b11111110, 0b00101000]));
    }

    #[test]
    fn test_parse_hex_longer() {
        let result = parse_hexadecimal("38006F45291200");
        assert_eq!(
            result,
            Ok(vec![
                0b00111000, 0b00000000, 0b01101111, 0b01000101, 0b00101001, 0b00010010, 0b00000000
            ])
        );
    }

    #[test]
    fn test_bit_stream() {
        let expected = "110100101111111000101000"
            .chars()
            .map(|it| it == '1')
            .collect_vec();
        let result = into_bit_stream(&vec![0b11010010, 0b11111110, 0b00101000]).collect_vec();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bit_stream_short() {
        let expected = "110".chars().map(|it| it == '1').collect_vec();
        let result = into_bit_stream(&parse_hexadecimal("D2").unwrap())
            .take(3)
            .collect_vec();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_read_int() {
        fn read_int_util<Number>(bytes: Vec<u8>, bits: usize) -> u64 {
            let mut bit_stream = into_bit_stream(&bytes);
            read_int(&mut bit_stream, bits).unwrap()
        }
        assert_eq!(read_int_util::<u8>(vec![0b11000000], 3), 6);
        assert_eq!(read_int_util::<u8>(vec![0b10000000], 3), 4);
        assert_eq!(read_int_util::<u16>(vec![0b01111110, 0b01010000], 12), 2021);
    }

    #[test]
    fn parse_packet() {
        let bits = into_bit_stream(&vec![0b11010010, 0b11111110, 0b00101000]).collect_vec();
        let mut bit_stream: BitStream = Box::new(bits.iter().copied());
        let packet = Packet::read(&mut bit_stream);
        assert_eq!(
            packet,
            Ok(Packet::Literal(LiteralPacket {
                version: 6,
                value: 2021
            }))
        );
    }

    #[test]
    fn parse_operator_packet_length_type_0() {
        let expected = Packet::Operator(OperatorPacket {
            version: 1,
            packet_type: OperatorPacketType::LessThan,
            sub_packets: vec![Packet::new_literal(6, 10), Packet::new_literal(2, 20)],
        });
        let bytes = parse_hexadecimal("38006F45291200").unwrap();

        let packet = Packet::read(&mut into_bit_stream(&bytes));
        assert_eq!(packet, Ok(expected));
    }

    #[test]
    fn parse_operator_packet_length_type_1() {
        let expected = Packet::Operator(OperatorPacket {
            version: 7,
            packet_type: OperatorPacketType::Maximum,
            sub_packets: vec![
                Packet::new_literal(2, 1),
                Packet::new_literal(4, 2),
                Packet::new_literal(1, 3),
            ],
        });
        let bytes = parse_hexadecimal("EE00D40C823060").unwrap();

        let packet = Packet::read(&mut into_bit_stream(&bytes));
        assert_eq!(packet, Ok(expected));
    }

    #[test]
    fn test_version_sum() {
        assert_eq!(
            Packet::from_str("8A004A801A8002F478")
                .unwrap()
                .version_sum(),
            16
        );
        assert_eq!(
            Packet::from_str("620080001611562C8802118E34")
                .unwrap()
                .version_sum(),
            12
        );
        assert_eq!(
            Packet::from_str("C0015000016115A2E0802F182340")
                .unwrap()
                .version_sum(),
            23
        );
        assert_eq!(
            Packet::from_str("A0016C880162017C3686B18A3D4780")
                .unwrap()
                .version_sum(),
            31
        );
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, Ok(925));
    }

    #[test]
    fn test_evaluate() {
        assert_eq!(Packet::from_str("C200B40A82").unwrap().evaluate(), Ok(3));
        assert_eq!(Packet::from_str("04005AC33890").unwrap().evaluate(), Ok(54));
        assert_eq!(
            Packet::from_str("880086C3E88112").unwrap().evaluate(),
            Ok(7)
        );
        assert_eq!(
            Packet::from_str("CE00C43D881120").unwrap().evaluate(),
            Ok(9)
        );
        assert_eq!(Packet::from_str("D8005AC2A8F0").unwrap().evaluate(), Ok(1));
        assert_eq!(Packet::from_str("F600BC2D8F").unwrap().evaluate(), Ok(0));
        assert_eq!(Packet::from_str("9C005AC2F8F0").unwrap().evaluate(), Ok(0));
        assert_eq!(
            Packet::from_str("9C0141080250320F1802104A08")
                .unwrap()
                .evaluate(),
            Ok(1)
        );
    }

    #[test]
    fn part_two_answer() {
        let result = part_two();
        assert_eq!(result, Ok(342997120375));
    }
}
