enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrKind1 {
    V4(i32, i32, i32, i32),
    V6(String),
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("::1"),
    };
    let v4 = IpAddrKind1::V4(127, 12, 0, 1);

    match v4 {
        IpAddrKind1::V4(a, b, c, d) => {
            print!("{},", a);
            print!("{},", b);
            print!("{},", c);
            println!("{}", d);
        }
        IpAddrKind1::V6(s) => {
            println!("{}", s)
        }
    }
    let v4_1 = IpAddrKind1::V4(127, 12, 0, 1);
    if let IpAddrKind1::V4(a, b, c, d) = v4_1 {
        print!("{},", a);
        print!("{},", b);
        print!("{},", c);
        println!("{}", d);
    }

    println!("{:#?}", &v4_1);
}
