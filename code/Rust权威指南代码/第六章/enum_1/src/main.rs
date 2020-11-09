fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loop_back = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home_1 = IpAddr::V4(String::from("127.0.0.1"));
    let loop_back_1 = IpAddr::V6(String::from("::1"));

    let home_2 = IpAddr::V4(127, 0, 0, 1);
    let loop_back_2 = IpAddr::V6(String::from("::1"));
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr_1 {
    V4(String),
    V6(String),
}

enum IpAddr_2 {
    V4(u8, u8, u8, u8),
    V6(String),
}
