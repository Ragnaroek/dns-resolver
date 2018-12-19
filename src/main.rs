
extern crate dnsresolver;

use std::net::UdpSocket;
use std::net::ToSocketAddrs;
use std::io;

fn main() -> io::Result<()> {

    println!("test dns query");

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    //println!("bind ok");

    println!("to_socket_addr");
    let mut addr = "8.8.8.8:53".to_socket_addrs()?;
    println!("to addr ok");

    //TODO send real dns query to google DNS
    socket.send_to(&[0; 10], addr.next().unwrap())?;
    println!("send ok");

    Ok(())
}
