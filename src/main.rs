use std::env;
use dns_lookup::{getaddrinfo, AddrInfoHints, SockType};

fn main() {
    let args : Vec<String> = env::args().collect();
    let hostname = &args[1];
    let servname = &args[2];
    println!("Hostname: {}", hostname);
    println!("Servname: {}", servname);
    let hints = AddrInfoHints {
        socktype : SockType::Stream.into(),
        .. AddrInfoHints::default()
    };
    let sockets = getaddrinfo(Some(hostname), Some(servname), Some(hints))
            .unwrap().collect::<std::io::Result<Vec<_>>>().unwrap();
    print!("IP Addresses:");
    let sockraddr : Vec<String> = sockets.into_iter().map(|x| { x.sockaddr.to_string()}).collect();
    println!("{}", sockraddr.join(", "))
}
