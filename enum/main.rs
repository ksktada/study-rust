// # pattern 1
// enum IpAddrKind {
//     V4,
//     V6
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// }

// # pattern 2

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// let home = IpAddr::V4(String::from("127.0.0.1"));

// let loopback = IpAddr::V6(String::from("::1"));

// # pattern 3
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// # pattern 4
// standard library?

// struct Ipv4Addr {
//     // --snip--
// }

// struct Ipv6Addr {
//     // --snip--
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// Message
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String)
    ChangeColor(i32, i32, i32)
}

// ↑can describe same object↓
struct QuitMessage; // unit struct 
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String) // tuple struct
struct ChangeColor(i32, i32, i32) // tuple struct

impl Message {
    fn call(&self) {
        // implements
    }
}

// doesn't work
fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}