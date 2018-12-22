
extern crate dnsresolver;

use std::net::UdpSocket;
use std::net::ToSocketAddrs;
use std::io;

use dnsresolver::*;

fn main() -> io::Result<()> {

    println!("test dns query");

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    //println!("bind ok");

    println!("to_socket_addr");
    let mut addr = "8.8.8.8:53".to_socket_addrs()?;
    println!("to addr ok");

    let msg = DNSMessage{
        header: Header{
            id: 0xC69C,
            qr: QR::Query,
            opcode: Opcode::StandardQuery,
            aa: false,
            tc: false,
            rd: true,
            ra: false,
            rcode: Rcode::NoError,
            qdcount: 1,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        },
        questions: vec![
            Question {
                qname: vec![Label::from_string("www".to_string()).unwrap(),
                            Label::from_string("gamestar".to_string()).unwrap(),
                            Label::from_string("de".to_string()).unwrap()],
                qtype: QType::A,
                qclass: QClass::IN,
            }]
    };

    //TODO send real dns query to google DNS
    socket.send_to(&msg.to_bytes(), addr.next().unwrap())?;
    println!("send ok");

    Ok(())
}
