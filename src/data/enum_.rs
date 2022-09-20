use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct Ipv4Address {
    kind: IpAddr,
}
struct Ipv6Address {
    kind: IpAddr,
}

impl Message {
    fn call(&self) -> () {
        println!("{:?}", &self);
    }
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

    let message = Message::Write(String::from("Hello, guys"));
    message.call();
}
