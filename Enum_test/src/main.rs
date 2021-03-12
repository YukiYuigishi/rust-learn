#![allow(unused)]
fn main() {
    enum IpAddrKind {
        V4(String),
        V6(String),
    }

    fn route(ip_type: IpAddrKind){}

    struct Ipv4Addr {
        kind: IpAddrKind,
        address: String,
    };

    struct Ipv6Addr {
        kind: IpAddrKind,
        address: String,

    };

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }




    

}
