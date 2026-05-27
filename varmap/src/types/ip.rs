use crate::*;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

impl VarMapValue for Ipv4Addr {
    type Decoded<'a> = Ipv4Addr;
    const TYPE_ID: u32 = 0;

    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
        Value::new(ValueKind::IpV4(*self), builder.arena())
    }

    fn from_value<'a>(value: &Value<'a>) -> Option<Ipv4Addr> {
        match value.kind() {
            ValueKind::IpV4(ip) => Some(*ip),
            _ => None,
        }
    }
}

impl VarMapValue for Ipv6Addr {
    type Decoded<'a> = Ipv6Addr;
    const TYPE_ID: u32 = 0;

    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
        Value::new(
            ValueKind::Ipv6(builder.arena_mut().store(self.octets().as_slice(), MemAlign::Bits8)),
            builder.arena(),
        )
    }

    fn from_value<'a>(value: &Value<'a>) -> Option<Ipv6Addr> {
        match value.kind() {
            ValueKind::Ipv6(index) => value
                .arena()
                .get(*index)
                .map(|bytes| Ipv6Addr::from_octets(bytes.try_into().unwrap())),
            _ => None,
        }
    }
}

impl VarMapValue for IpAddr {
    type Decoded<'a> = IpAddr;
    const TYPE_ID: u32 = 0;

    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
        match self {
            IpAddr::V4(ip) => Value::new(ValueKind::IpV4(*ip), builder.arena()),
            IpAddr::V6(ip) => Value::new(
                ValueKind::Ipv6(builder.arena_mut().store(ip.octets().as_slice(), MemAlign::Bits8)),
                builder.arena(),
            ),
        }
    }

    fn from_value<'a>(value: &Value<'a>) -> Option<IpAddr> {
        match value.kind() {
            ValueKind::IpV4(ip) => Some(IpAddr::V4(*ip)),
            ValueKind::Ipv6(ip) => Some(IpAddr::V6(Ipv6Addr::from_octets(
                value.arena().get(*ip)?.try_into().unwrap(),
            ))),
            _ => None,
        }
    }
}
