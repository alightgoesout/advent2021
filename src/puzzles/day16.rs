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
        format!("Sum of all version: {}", packet.versions_sum())
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
    content: PacketContent,
}

impl Packet {
    fn versions_sum(&self) -> u32 {
        match &self.content {
            PacketContent::Literal(_) => self.version as u32,
            PacketContent::Operator { sub_packets, .. } => {
                (self.version as u32) + sub_packets.iter().map(Packet::versions_sum).sum::<u32>()
            }
        }
    }

    fn evaluate(&self) -> u64 {
        match &self.content {
            PacketContent::Literal(v) => *v,
            PacketContent::Operator {
                operation,
                sub_packets,
            } => operation.apply(sub_packets),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum PacketContent {
    Literal(u64),
    Operator {
        operation: Operation,
        sub_packets: Vec<Packet>,
    },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

impl Operation {
    fn apply(&self, parameters: &[Packet]) -> u64 {
        match self {
            Operation::Sum => parameters.iter().map(Packet::evaluate).sum(),
            Operation::Product => parameters.iter().map(Packet::evaluate).product(),
            Operation::Minimum => parameters.iter().map(Packet::evaluate).min().unwrap(),
            Operation::Maximum => parameters.iter().map(Packet::evaluate).max().unwrap(),
            Operation::GreaterThan => (parameters[0].evaluate() > parameters[1].evaluate()) as u64,
            Operation::LessThan => (parameters[0].evaluate() < parameters[1].evaluate()) as u64,
            Operation::Equal => (parameters[0].evaluate() == parameters[1].evaluate()) as u64,
        }
    }
}

impl Operation {
    fn from_id(operator_id: u8) -> Self {
        match operator_id {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::Equal,
            _ => panic!("Unknown operator id: {}", operator_id),
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
        self.read_packet().map(|(packet, _)| packet)
    }

    fn read_packet(&mut self) -> Option<(Packet, u32)> {
        let version = self.lexer.read_number(3)? as u8;

        let (content, content_size) = match self.lexer.read_number(3)? {
            4 => self.parse_literal()?,
            operator_id => self.parse_operator(operator_id as u8)?,
        };

        Some((Packet { version, content }, 6 + content_size))
    }

    fn parse_literal(&mut self) -> Option<(PacketContent, u32)> {
        let (value, size) = self.lexer.read_grouped_number()?;
        Some((PacketContent::Literal(value as u64), size))
    }

    fn parse_operator(&mut self, operator_id: u8) -> Option<(PacketContent, u32)> {
        let operation = Operation::from_id(operator_id);
        let length_type = self.lexer.next_bit()?;
        match length_type {
            0 => self.parse_type_0_operator(operation),
            1 => self.parse_type_1_operator(operation),
            _ => None,
        }
        .map(|(content, size)| (content, size + 1))
    }

    fn parse_type_0_operator(&mut self, operation: Operation) -> Option<(PacketContent, u32)> {
        let sub_packets_length = self.lexer.read_number(15)?;
        let mut sub_packets = Vec::new();
        let mut current_length = 0;
        while current_length < sub_packets_length {
            let (packet, size) = self.read_packet()?;
            sub_packets.push(packet);
            current_length += size as u64;
        }
        Some((
            PacketContent::Operator {
                operation,
                sub_packets,
            },
            15 + sub_packets_length as u32,
        ))
    }

    fn parse_type_1_operator(&mut self, operation: Operation) -> Option<(PacketContent, u32)> {
        let nb_packets = self.lexer.read_number(11)?;
        let mut sub_packets = Vec::new();
        let mut sub_packets_length = 0;
        for _ in 0..nb_packets {
            let (packet, size) = self.read_packet()?;
            sub_packets.push(packet);
            sub_packets_length += size;
        }
        Some((
            PacketContent::Operator {
                operation,
                sub_packets,
            },
            11 + sub_packets_length,
        ))
    }
}

struct PacketLexer<'a> {
    source: &'a [u8],
    next: usize,
    buffer: VecDeque<u8>,
}

impl<'a> PacketLexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            next: 0,
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

    pub fn read_grouped_number(&mut self) -> Option<(u64, u32)> {
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
        if let Some(n) = self.source.get(self.next) {
            let mut bits = (*n as char).to_digit(16).unwrap() as u8;

            let n0 = bits % 2;
            bits /= 2;
            let n1 = bits % 2;
            bits /= 2;
            let n2 = bits % 2;
            let n3 = bits / 2;
            self.buffer.extend([n3, n2, n1, n0]);

            self.next += 1;
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
                content: PacketContent::Literal(2021),
            }),
        )
    }

    #[test]
    fn packet_parser_should_parse_a_type_0_operator() {
        let mut parser = PacketParser::new("38006F45291200");

        let result = parser.parse().unwrap();

        assert_eq!(
            result,
            Packet {
                version: 1,
                content: PacketContent::Operator {
                    operation: Operation::LessThan,
                    sub_packets: vec![
                        Packet {
                            version: 6,
                            content: PacketContent::Literal(10)
                        },
                        Packet {
                            version: 2,
                            content: PacketContent::Literal(20)
                        },
                    ],
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
                content: PacketContent::Operator {
                    operation: Operation::Maximum,
                    sub_packets: vec![
                        Packet {
                            version: 2,
                            content: PacketContent::Literal(1)
                        },
                        Packet {
                            version: 4,
                            content: PacketContent::Literal(2)
                        },
                        Packet {
                            version: 1,
                            content: PacketContent::Literal(3)
                        },
                    ],
                },
            },
        )
    }

    #[test]
    fn versions_sum_for_8a004a801a8002f478_should_be_16() {
        let mut parser = PacketParser::new("8A004A801A8002F478");

        let packet = parser.parse().unwrap();

        assert_eq!(packet.versions_sum(), 16);
    }

    #[test]
    fn versions_sum_for_620080001611562c8802118e34_should_be_12() {
        let mut parser = PacketParser::new("620080001611562C8802118E34");

        let packet = parser.parse().unwrap();

        assert_eq!(packet.versions_sum(), 12);
    }

    #[test]
    fn versions_sum_for_c0015000016115a2e0802f182340_should_be_23() {
        let mut parser = PacketParser::new("C0015000016115A2E0802F182340");

        let packet = parser.parse().unwrap();

        assert_eq!(packet.versions_sum(), 23);
    }

    #[test]
    fn versions_sum_for_a0016c880162017c3686b18a3d4780_should_be_31() {
        let mut parser = PacketParser::new("A0016C880162017C3686B18A3D4780");

        let packet = parser.parse().unwrap();

        assert_eq!(packet.versions_sum(), 31);
    }
}
