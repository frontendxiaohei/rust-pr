use std::str::FromStr;
use std::convert::{TryFrom, From};
use std::num::ParseIntError;

// 定义一个复杂的 IP 地址结构体
#[derive(Debug, PartialEq)]
struct IpAddress {
    octets: [u8; 4],
    port: Option<u16>,
}

// 定义一个简化版的 IP 地址结构体，用于演示 From trait
#[derive(Debug)]
struct SimpleIpAddress {
    address: String,
}

// 定义可能的解析错误
#[derive(Debug)]
enum IpAddressError {
    InvalidFormat,
    InvalidOctet(ParseIntError),
    InvalidPort(ParseIntError),
    OctetOutOfRange(u8),
}

// 实现 FromStr trait
impl FromStr for IpAddress {
    type Err = IpAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 分离 IP 和端口
        let parts: Vec<&str> = s.split(':').collect();
        let ip_str = parts.get(0).ok_or(IpAddressError::InvalidFormat)?;
        let port_str = parts.get(1);

        // 解析 IP 地址部分
        let ip_parts: Vec<&str> = ip_str.split('.').collect();
        if ip_parts.len() != 4 {
            return Err(IpAddressError::InvalidFormat);
        }

        let mut octets = [0u8; 4];
        for (i, part) in ip_parts.iter().enumerate() {
            let octet = part.parse::<u8>().map_err(IpAddressError::InvalidOctet)?;
            if octet > 255 {
                return Err(IpAddressError::OctetOutOfRange(octet));
            }
            octets[i] = octet;
        }

        // 解析端口（如果存在）
        let port = if let Some(p) = port_str {
            Some(p.parse::<u16>().map_err(IpAddressError::InvalidPort)?)
        } else {
            None
        };

        Ok(IpAddress { octets, port })
    }
}

// 实现 TryFrom trait，从字符串切片转换
impl TryFrom<&str> for IpAddress {
    type Error = IpAddressError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

// 实现 From trait，从 SimpleIpAddress 转换到 IpAddress
impl From<SimpleIpAddress> for IpAddress {
    fn from(simple: SimpleIpAddress) -> Self {
        // 对于 From 实现，我们假设输入是有效的，使用默认值处理无效情况
        simple.address.parse().unwrap_or(IpAddress {
            octets: [0, 0, 0, 0],
            port: None,
        })
    }
}

fn main() {
    // 测试 FromStr（通过 parse 方法）
    println!("Testing FromStr trait:");
    let ip1: IpAddress = "192.168.1.1:8080".parse().unwrap();
    println!("Parsed IP with port: {:?}", ip1);

    let ip2: IpAddress = "10.0.0.1".parse().unwrap();
    println!("Parsed IP without port: {:?}", ip2);

    // 测试无效输入
    let invalid = "256.1.2.3".parse::<IpAddress>();
    println!("Invalid IP parse result: {:?}", invalid);

    // 测试 TryFrom
    println!("\nTesting TryFrom trait:");
    let ip3 = IpAddress::try_from("172.16.254.1:443").unwrap();
    println!("IP from TryFrom: {:?}", ip3);

    // 测试 From
    println!("\nTesting From trait:");
    let simple_ip = SimpleIpAddress {
        address: String::from("192.168.0.1"),
    };
    let ip4: IpAddress = simple_ip.into();
    println!("IP from SimpleIpAddress: {:?}", ip4);

    // 展示错误处理
    println!("\nTesting error handling:");
    let error_cases = [
        "256.1.2.3",        // 无效八位字节
        "1.2.3",           // 格式错误
        "1.2.3.4:65536",   // 无效端口
    ];

    for case in error_cases.iter() {
        println!("Parsing {}: {:?}", case, case.parse::<IpAddress>());
    }
}