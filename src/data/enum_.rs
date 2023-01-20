use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

struct Ipv4Address {
    kind: IpAddr,
}
struct Ipv6Address {
    kind: IpAddr,
}


impl Ipv4Address {
    fn route(&self) -> () {
        println!("IP: {:?}", self.kind);
    }   
}
impl Ipv6Address {
    fn route(&self) -> () {
        println!("IP: {:?}", self.kind);
    }
}


pub fn main_enum() {
    let home = Ipv4Address {
        kind: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
    };
    let loopback = Ipv6Address {
        kind: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
    };

    home.route();
    loopback.route();

    let some_number = Some(5);
    let some_string = Some("String");
    println!("{:?}, {:?}", some_number, some_string);
}
