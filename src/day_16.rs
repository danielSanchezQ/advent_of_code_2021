use std::collections::HashMap;
use std::lazy::Lazy;
use std::str::FromStr;

const HEXMAP: Lazy<HashMap<char, &str>> = Lazy::new(|| {
    [
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]
    .into_iter()
    .collect()
});

const LITERAL_ID: u8 = 4;

struct BitsInput(Vec<char>);

impl FromStr for BitsInput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().flat_map(|c| HEXMAP[&c].chars()).collect()))
    }
}

struct BitsStream(Box<dyn Iterator<Item = char>>);

impl BitsInput {
    fn to_bits_stream(self) -> BitsStream {
        BitsStream(Box::new(self.0.into_iter()))
    }
}

impl BitsStream {
    fn take(&mut self, n: usize) -> String {
        (0..n)
            .map(|_| self.0.next().expect("Not empty value"))
            .collect()
    }

    fn discard(&mut self, n: usize) {
        for _ in 0..n {
            self.0.next().expect("Discarding empty stream");
        }
    }
}

trait Decode {
    type Output;
    fn decode(stream: &mut BitsStream) -> Self::Output;
}

#[derive(Debug, Eq, PartialEq)]
enum PackageType {
    Literal,
    Operator(u8),
}

impl PackageType {
    fn from_u8(id: u8) -> Self {
        match id {
            LITERAL_ID => Self::Literal,
            id => Self::Operator(id),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Header {
    version: u8,
    package_type: PackageType,
}

struct LiteralPackage {
    number: usize,
}

struct OperatorPackage {
    subpackages: Vec<Package>,
}

enum Package {
    Operator((Header, OperatorPackage)),
    Literal((Header, LiteralPackage)),
}

impl Decode for Header {
    type Output = Header;

    fn decode(stream: &mut BitsStream) -> Self::Output {
        let version = u8::from_str_radix(&stream.take(3), 2).expect("Parseable number");
        let package_type =
            PackageType::from_u8(u8::from_str_radix(&stream.take(3), 2).expect("Parseable number"));

        Self {
            version,
            package_type,
        }
    }
}

impl Decode for LiteralPackage {
    type Output = LiteralPackage;

    fn decode(stream: &mut BitsStream) -> Self::Output {
        let mut bits: Vec<String> = Vec::new();
        loop {
            let next = stream.take(5);
            let breaking = next.starts_with('0');
            bits.push(next.chars().skip(1).collect());
            if breaking {
                break;
            }
        }
        let number_bits: String = bits.join("");
        let number = usize::from_str_radix(&number_bits, 2).expect("Parseable number");
        Self { number }
    }
}

impl Decode for OperatorPackage {
    type Output = OperatorPackage;

    fn decode(stream: &mut BitsStream) -> Self::Output {
        let length_type_id = stream.take(1);
        match length_type_id.as_str() {
            "0" => Self {
                subpackages: vec![],
            },
            "1" => Self {
                subpackages: vec![],
            },
            _ => unreachable!(),
        }
    }
}

impl Decode for Package {
    type Output = Package;

    fn decode(stream: &mut BitsStream) -> (Self::Output) {
        let header = Header::decode(stream);
        match header.package_type {
            PackageType::Literal => Self::Literal((header, LiteralPackage::decode(stream))),
            PackageType::Operator(_) => Self::Operator((header, OperatorPackage::decode(stream))),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day_16::{BitsInput, Decode, Header, LiteralPackage, PackageType};

    #[test]
    fn parse_literal_with_header() {
        let input = "D2FE28";
        let bits_input: BitsInput = input.parse().expect("Bitsinput should parse");
        let mut stream = bits_input.to_bits_stream();
        let header = Header::decode(&mut stream);
        let expected_header = Header {
            version: 6,
            package_type: PackageType::Literal,
        };
        assert_eq!(header, expected_header);
        let literal = LiteralPackage::decode(&mut stream);
        assert_eq!(literal.number, 2021);
    }
}
