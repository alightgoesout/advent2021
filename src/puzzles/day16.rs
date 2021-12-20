use lazy_static::lazy_static;
use std::collections::VecDeque;

use super::{input, Puzzle};

lazy_static! {
    static ref INPUT: String = input::read_file("day16");
}

pub struct Day16;

impl Puzzle for Day16 {
    fn number(&self) -> u8 {
        16
    }

    fn part_one(&self) -> String {
        let mut parser = PacketParser::new(&INPUT);
        let packet = parser.parse().unwrap();
        format!("Sum of all version: {}", packet.sum_versions())
    }

    fn part_two(&self) -> String {
        let mut parser = PacketParser::new(&INPUT);
        let packet = parser.parse().unwrap();
        format!("Result of evaluation: {}", packet.evaluate())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Packet {
    version: u8,
    kind: PacketKind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum PacketKind {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan {
        first: Box<Packet>,
        second: Box<Packet>,
    },
    LessThan {
        first: Box<Packet>,
        second: Box<Packet>,
    },
    Equal {
        first: Box<Packet>,
        second: Box<Packet>,
    },
}

impl Packet {
    fn sum_versions(&self) -> u32 {
        match &self.kind {
            PacketKind::Literal(_) => self.version as u32,
            PacketKind::Sum(sub_packets)
            | PacketKind::Product(sub_packets)
            | PacketKind::Minimum(sub_packets)
            | PacketKind::Maximum(sub_packets) => {
                self.version as u32 + sub_packets.iter().map(Packet::sum_versions).sum::<u32>()
            }
            PacketKind::GreaterThan { first, second }
            | PacketKind::LessThan { first, second }
            | PacketKind::Equal { first, second } => {
                self.version as u32 + first.sum_versions() + second.sum_versions()
            }
        }
    }

    fn evaluate(&self) -> u64 {
        match &self.kind {
            PacketKind::Literal(value) => *value,
            PacketKind::Sum(sub_packets) => sub_packets.iter().map(Packet::evaluate).sum(),
            PacketKind::Product(sub_packets) => sub_packets.iter().map(Packet::evaluate).product(),
            PacketKind::Minimum(sub_packets) => {
                sub_packets.iter().map(Packet::evaluate).min().unwrap_or(0)
            }
            PacketKind::Maximum(sub_packets) => {
                sub_packets.iter().map(Packet::evaluate).max().unwrap_or(0)
            }
            PacketKind::GreaterThan { first, second } => {
                if first.evaluate() > second.evaluate() {
                    1
                } else {
                    0
                }
            }
            PacketKind::LessThan { first, second } => {
                if first.evaluate() < second.evaluate() {
                    1
                } else {
                    0
                }
            }
            PacketKind::Equal { first, second } => {
                if first.evaluate() == second.evaluate() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

struct PacketParser<'a> {
    lexer: PacketLexer<'a>,
}

impl<'a> PacketParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: PacketLexer::new(source),
        }
    }

    pub fn parse(&mut self) -> Option<Packet> {
        self.parse_packet().map(|(packet, _)| packet)
    }

    fn parse_packet(&mut self) -> Option<(Packet, usize)> {
        let version = self.lexer.read_number(3)? as u8;
        let type_id = self.lexer.read_number(3)? as u8;

        let (kind, size) = match type_id {
            4 => self.parse_literal()?,
            _ => self.parse_operator(type_id)?,
        };

        Some((Packet { version, kind }, 6 + size))
    }

    fn parse_literal(&mut self) -> Option<(PacketKind, usize)> {
        let (value, size) = self.lexer.read_grouped_number()?;
        Some((PacketKind::Literal(value as u64), size))
    }

    fn parse_operator(&mut self, operator_id: u8) -> Option<(PacketKind, usize)> {
        let (mut sub_packets, size) = self.parse_sub_packets()?;
        match operator_id {
            0 => Some((PacketKind::Sum(sub_packets), size)),
            1 => Some((PacketKind::Product(sub_packets), size)),
            2 => Some((PacketKind::Minimum(sub_packets), size)),
            3 => Some((PacketKind::Maximum(sub_packets), size)),
            5 => Some((
                PacketKind::GreaterThan {
                    first: Box::new(sub_packets.remove(0)),
                    second: Box::new(sub_packets.remove(0)),
                },
                size,
            )),
            6 => Some((
                PacketKind::LessThan {
                    first: Box::new(sub_packets.remove(0)),
                    second: Box::new(sub_packets.remove(0)),
                },
                size,
            )),
            7 => Some((
                PacketKind::Equal {
                    first: Box::new(sub_packets.remove(0)),
                    second: Box::new(sub_packets.remove(0)),
                },
                size,
            )),
            _ => None,
        }
    }

    fn parse_sub_packets(&mut self) -> Option<(Vec<Packet>, usize)> {
        let length_type = self.lexer.read_number(1)?;
        match length_type {
            0 => self.parse_type_0_sub_packets(),
            1 => self.parse_type_1_sub_packets(),
            _ => None,
        }
        .map(|(content, size)| (content, size + 1))
    }

    fn parse_type_0_sub_packets(&mut self) -> Option<(Vec<Packet>, usize)> {
        let sub_packets_length = self.lexer.read_number(15)? as usize;
        let mut sub_packets = Vec::new();
        let mut current_length = 0;
        while current_length < sub_packets_length {
            let (packet, size) = self.parse_packet()?;
            sub_packets.push(packet);
            current_length += size;
        }
        Some((sub_packets, 15 + sub_packets_length))
    }

    fn parse_type_1_sub_packets(&mut self) -> Option<(Vec<Packet>, usize)> {
        let nb_packets = self.lexer.read_number(11)?;
        let mut sub_packets = Vec::new();
        let mut sub_packets_length = 0;
        for _ in 0..nb_packets {
            let (packet, size) = self.parse_packet()?;
            sub_packets.push(packet);
            sub_packets_length += size;
        }
        Some((sub_packets, 11 + sub_packets_length))
    }
}

struct PacketLexer<'a> {
    source: &'a [u8],
    next_byte: usize,
    buffer: VecDeque<u8>,
}

impl<'a> PacketLexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            next_byte: 0,
            buffer: VecDeque::new(),
        }
    }

    pub fn read_number(&mut self, bits: u8) -> Option<u64> {
        let mut number = 0;
        for _ in 0..bits {
            number = number * 2 + self.next_bit()? as u64;
        }
        Some(number)
    }

    pub fn read_grouped_number(&mut self) -> Option<(u64, usize)> {
        let mut prefix = self.next_bit()?;
        let mut number = self.read_number(4)?;
        let mut total_bits = 5;
        while prefix != 0 {
            prefix = self.next_bit()?;
            number = (number << 4) + self.read_number(4)?;
            total_bits += 5;
        }
        Some((number, total_bits))
    }

    pub fn next_bit(&mut self) -> Option<u8> {
        if self.buffer.is_empty() {
            self.fill_buffer();
        }
        self.buffer.pop_front()
    }

    fn fill_buffer(&mut self) {
        for _ in 0..4 {
            if let Some(n) = self.source.get(self.next_byte) {
                let bits = (*n as char).to_digit(16).unwrap() as u8;

                self.buffer.push_back((bits & 0b1000) >> 3);
                self.buffer.push_back((bits & 0b0100) >> 2);
                self.buffer.push_back((bits & 0b0010) >> 1);
                self.buffer.push_back(bits & 0b0001);

                self.next_byte += 1;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn packet_parser_should_parse_a_literal_packet() {
        let mut parser = PacketParser::new("D2FE28");

        let result = parser.parse();

        assert_eq!(
            result,
            Some(Packet {
                version: 6,
                kind: PacketKind::Literal(2021),
            }),
        );
    }

    #[test]
    fn packet_parser_should_parse_a_type_0_operator() {
        let mut parser = PacketParser::new("38006F45291200");

        let result = parser.parse().unwrap();

        assert_eq!(
            result,
            Packet {
                version: 1,
                kind: PacketKind::LessThan {
                    first: Box::new(Packet {
                        version: 6,
                        kind: PacketKind::Literal(10),
                    }),
                    second: Box::new(Packet {
                        version: 2,
                        kind: PacketKind::Literal(20),
                    }),
                },
            },
        )
    }

    #[test]
    fn packet_parser_should_parse_a_type_1_operator() {
        let mut parser = PacketParser::new("EE00D40C823060");

        let result = parser.parse().unwrap();

        assert_eq!(
            result,
            Packet {
                version: 7,
                kind: PacketKind::Maximum(vec![
                    Packet {
                        version: 2,
                        kind: PacketKind::Literal(1)
                    },
                    Packet {
                        version: 4,
                        kind: PacketKind::Literal(2)
                    },
                    Packet {
                        version: 1,
                        kind: PacketKind::Literal(3)
                    },
                ],),
            },
        )
    }

    #[test]
    fn sum_versions_should_return_16_for_8a004a801a8002f478() {
        let mut parser = PacketParser::new("8A004A801A8002F478");

        let packet = parser.parse().unwrap();

        assert_eq!(packet.sum_versions(), 16);
    }

    #[test]
    fn sum_versions_should_return_12_for_620080001611562c8802118e34() {
        let mut parser = PacketParser::new("620080001611562C8802118E34");

        let packet = parser.parse().unwrap();

        assert_eq!(packet.sum_versions(), 12);
    }

    #[test]
    fn sum_versions_should_return_23_for_c0015000016115a2e0802f182340() {
        let mut parser = PacketParser::new("C0015000016115A2E0802F182340");

        let packet = parser.parse().unwrap();

        assert_eq!(packet.sum_versions(), 23);
    }

    #[test]
    fn sum_versions_should_return_32_for_a0016c880162017c3686b18a3d4780() {
        let mut parser = PacketParser::new("A0016C880162017C3686B18A3D4780");

        let packet = parser.parse().unwrap();

        assert_eq!(packet.sum_versions(), 31);
    }
}
