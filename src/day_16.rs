use std::any::TypeId;
use std::collections::HashMap;
use std::iter::Peekable;
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

struct BitsStream(Peekable<Box<dyn Iterator<Item = char>>>);

impl BitsInput {
    fn into_bits_stream(self) -> BitsStream {
        let iter: Box<dyn Iterator<Item = char>> = Box::new(self.0.into_iter());
        BitsStream(iter.peekable())
    }
}

impl BitsStream {
    fn take(&mut self, n: usize) -> Option<String> {
        let mut res = Vec::new();
        for _ in 0..n {
            res.push(self.0.next()?);
        }
        Some(res.into_iter().collect())
    }

    fn discard(&mut self, n: usize) {
        for _ in 0..n {
            self.0.next().expect("Discarding empty stream");
        }
    }

    fn from_chars(iter: Vec<char>) -> Self {
        let iter: Box<dyn Iterator<Item = char>> = Box::new(iter.into_iter());
        Self(iter.peekable())
    }
}

trait Decode {
    type Output;
    fn decode(stream: &mut BitsStream) -> Option<Self::Output>;
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

#[derive(Debug)]
struct LiteralPackage {
    number: usize,
}

#[derive(Debug)]
struct OperatorPackage {
    subpackages: Packages,
}

#[derive(Debug)]
enum Package {
    Operator((Header, OperatorPackage)),
    Literal((Header, LiteralPackage)),
}

impl Package {
    fn version_sum(&self) -> usize {
        match self {
            Package::Operator((header, other)) => {
                header.version as usize + other.subpackages.version_sum() as usize
            }
            Package::Literal((header, _)) => header.version as usize,
        }
    }

    fn sum(&self) -> usize {
        match self {
            Package::Operator((_, operator)) => operator.subpackages.sum(),
            Package::Literal((_, literal)) => literal.number,
        }
    }

    fn mul(&self) -> usize {
        match self {
            Package::Operator((_, operator)) => operator.subpackages.mul(),
            Package::Literal((_, literal)) => literal.number,
        }
    }

    fn min(&self) -> usize {
        match self {
            Package::Operator((_, operator)) => operator.subpackages.min(),
            Package::Literal((_, literal)) => literal.number,
        }
    }

    fn max(&self) -> usize {
        match self {
            Package::Operator((_, operator)) => operator.subpackages.max(),
            Package::Literal((_, literal)) => literal.number,
        }
    }

    fn cond(&self, op: impl FnOnce(usize, usize) -> usize) -> usize {
        match self {
            Package::Operator((_, operator)) => {
                if let (a, b) = (
                    operator.subpackages.0[0].compute(),
                    operator.subpackages.0[1].compute(),
                ) {
                    op(a, b)
                } else {
                    unreachable!()
                }
            }
            Package::Literal(_) => unreachable!(),
        }
    }

    fn greater(&self) -> usize {
        self.cond(|a, b| (a > b).then_some(1).unwrap_or(0))
    }

    fn less(&self) -> usize {
        self.cond(|a, b| (a < b).then_some(1).unwrap_or(0))
    }

    fn equal(&self) -> usize {
        self.cond(|a, b| (a == b).then_some(1).unwrap_or(0))
    }

    fn compute(&self) -> usize {
        match self {
            Package::Operator((header, _)) => self.compute_with_type(&header.package_type),
            Package::Literal((_, LiteralPackage { number })) => *number,
        }
    }

    fn compute_with_type(&self, package_type: &PackageType) -> usize {
        match package_type {
            PackageType::Literal => unreachable!(),
            PackageType::Operator(0) => self.sum(),
            PackageType::Operator(1) => self.mul(),
            PackageType::Operator(2) => self.min(),
            PackageType::Operator(3) => self.max(),
            PackageType::Operator(5) => self.greater(),
            PackageType::Operator(6) => self.less(),
            PackageType::Operator(7) => self.equal(),
            _ => unreachable!(),
        }
    }
}

impl Decode for Header {
    type Output = Header;

    fn decode(stream: &mut BitsStream) -> Option<Self::Output> {
        let version = u8::from_str_radix(&stream.take(3)?, 2).expect("A parseable version");
        let package_type = PackageType::from_u8(
            u8::from_str_radix(&stream.take(3)?, 2).expect("A parseable package type"),
        );

        Some(Self {
            version,
            package_type,
        })
    }
}

impl Decode for LiteralPackage {
    type Output = LiteralPackage;

    fn decode(stream: &mut BitsStream) -> Option<Self::Output> {
        let mut bits: Vec<String> = Vec::new();
        loop {
            let next = stream.take(5)?;
            let breaking = next.starts_with('0');
            bits.push(next.chars().skip(1).collect());
            if breaking {
                break;
            }
        }
        let number_bits: String = bits.join("");
        let number = usize::from_str_radix(&number_bits, 2).expect("Parseable number");
        Some(Self { number })
    }
}

impl OperatorPackage {
    fn decode_length(stream: &mut BitsStream) -> Option<Self> {
        let length = usize::from_str_radix(&stream.take(15)?, 2).expect("Parseable number");
        let chars = stream.take(length)?.chars().collect();
        let mut substream: BitsStream = BitsStream::from_chars(chars);
        Some(Self {
            subpackages: Packages::decode(&mut substream)?,
        })
    }

    fn decode_n(stream: &mut BitsStream) -> Option<Self> {
        let n = usize::from_str_radix(&stream.take(11)?, 2).expect("Parseable number");
        let mut packages = Vec::new();
        for _ in 0..n {
            packages.push(Package::decode(stream)?);
        }
        Some(Self {
            subpackages: Packages(packages),
        })
    }
}

impl Decode for OperatorPackage {
    type Output = OperatorPackage;

    fn decode(stream: &mut BitsStream) -> Option<Self::Output> {
        let length_type_id = stream.take(1)?;
        match length_type_id.as_str() {
            "0" => Self::decode_length(stream),
            "1" => Self::decode_n(stream),
            _ => unreachable!(),
        }
    }
}

impl Decode for Package {
    type Output = Package;

    fn decode(stream: &mut BitsStream) -> Option<Self::Output> {
        let header = Header::decode(stream)?;
        Some(match header.package_type {
            PackageType::Literal => Self::Literal((header, LiteralPackage::decode(stream)?)),
            PackageType::Operator(_) => Self::Operator((header, OperatorPackage::decode(stream)?)),
        })
    }
}

#[derive(Debug)]
struct Packages(Vec<Package>);

impl Packages {
    fn version_sum(&self) -> usize {
        self.0.iter().map(|p| p.version_sum()).sum()
    }

    fn sum(&self) -> usize {
        self.0.iter().map(|p| p.compute()).sum()
    }

    fn mul(&self) -> usize {
        self.0.iter().map(|p| p.compute()).product()
    }

    fn min(&self) -> usize {
        self.0.iter().map(|p| p.compute()).min().unwrap()
    }

    fn max(&self) -> usize {
        self.0.iter().map(|p| p.compute()).max().unwrap()
    }
}

impl Decode for Packages {
    type Output = Packages;

    fn decode(stream: &mut BitsStream) -> Option<Self::Output> {
        let mut packages = Vec::new();
        while let Some(package) = Package::decode(stream) {
            packages.push(package);
        }
        if packages.is_empty() {
            None
        } else {
            Some(Self(packages))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day_16::{
        BitsInput, Decode, Header, LiteralPackage, OperatorPackage, Package, PackageType, Packages,
    };
    use crate::utils::io;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn parse_literal_with_header() {
        let input = "D2FE28";
        let bits_input: BitsInput = input.parse().expect("Bitsinput should parse");
        let mut stream = bits_input.into_bits_stream();
        let header = Header::decode(&mut stream).unwrap();
        let expected_header = Header {
            version: 6,
            package_type: PackageType::Literal,
        };
        assert_eq!(header, expected_header);
        let literal = LiteralPackage::decode(&mut stream).unwrap();
        assert_eq!(literal.number, 2021);
    }

    #[test]
    fn parse_operator_with_header() {
        let input = "38006F45291200";
        let bits_input: BitsInput = input.parse().expect("Bitsinput should parse");
        let mut stream = bits_input.into_bits_stream();
        let header = Header::decode(&mut stream).unwrap();
        let expected_header = Header {
            version: 1,
            package_type: PackageType::Operator(6),
        };
        assert_eq!(header, expected_header);
        let operator = OperatorPackage::decode(&mut stream).unwrap();
        assert_eq!(operator.subpackages.0.len(), 2);
        for package in &operator.subpackages.0 {
            assert!(matches!(package, Package::Literal(_)));
        }
        println!("{:?}", operator);
    }

    #[test]
    fn parse_operator_with_header_other() {
        let input = "EE00D40C823060";
        let bits_input: BitsInput = input.parse().expect("Bitsinput should parse");
        let mut stream = bits_input.into_bits_stream();
        let header = Header::decode(&mut stream).unwrap();
        let expected_header = Header {
            version: 7,
            package_type: PackageType::Operator(3),
        };
        assert_eq!(header, expected_header);
        let operator = OperatorPackage::decode(&mut stream).unwrap();
        assert_eq!(operator.subpackages.0.len(), 3);
        for package in &operator.subpackages.0 {
            assert!(matches!(package, Package::Literal(_)));
        }
        println!("{:?}", operator);
    }

    #[test]
    fn part_1() -> std::io::Result<()> {
        let bits_input: BitsInput =
            io::read_object_from_file(&PathBuf::from_str("./inputs/day_16.txt").unwrap())?;
        let mut stream = bits_input.into_bits_stream();
        let packages = Packages::decode(&mut stream).unwrap();
        println!("Day 16 part 1 solution: {}", packages.version_sum());
        Ok(())
    }

    #[test]
    fn part_2() -> std::io::Result<()> {
        let bits_input: BitsInput =
            io::read_object_from_file(&PathBuf::from_str("./inputs/day_16.txt").unwrap())?;
        let mut stream = bits_input.into_bits_stream();
        let package = Package::decode(&mut stream).unwrap();
        println!("{:?}", package);
        println!("Day 16 part 2 solution: {}", package.compute());
        Ok(())
    }
}
