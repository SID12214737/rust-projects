enum IpAddrKind {
    v4(String),
    v6(String),
}

fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;
    
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));


}

fn route(ip_kind: IpAddrKind) {}