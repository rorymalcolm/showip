use dns_lookup::{getaddrinfo, AddrInfoHints, SockType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let hostname = &args[1];
    let servname = &args[2];
    println!("Hostname: {}", hostname);
    println!("Servname: {}", servname);
    let hints = AddrInfoHints {
        socktype: SockType::Stream.into(),
        ..AddrInfoHints::default()
    };
    let res = getaddrinfo(Some(hostname), Some(servname), Some(hints));
    let res = match res {
        Ok(res) => res,
        Err(_error) => panic!("Problem getting address info Hostname or Service incorrect"),
    };
    let sockets = res.collect::<std::io::Result<Vec<_>>>().unwrap();
    print!("IP Addresses: ");
    let sockraddr: Vec<String> = sockets
        .into_iter()
        .map(|x| x.sockaddr.to_string())
        .collect();
    println!("{}", sockraddr.join(", "))
}
