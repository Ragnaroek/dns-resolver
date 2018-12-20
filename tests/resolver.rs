extern crate dnsresolver;

use dnsresolver::resolver;

#[test]
fn test_convert_example_query_to_bytes() {
    let msg = resolver::DNSMessage{
        header: resolver::Header{
            id: 0xC69C,
            qr: resolver::QR::Query,
            opcode: resolver::Opcode::StandardQuery,
            aa: false,
            tc: false,
            rd: true,
            ra: false,
            rcode: resolver::Rcode::NoError,
            qdcount: 1,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        },
        questions: vec![
            resolver::Question {
                qname: vec![resolver::Label::from_string("www".to_string()).unwrap(),
                            resolver::Label::from_string("gamestar".to_string()).unwrap(),
                            resolver::Label::from_string("de".to_string()).unwrap()],
                qtype: resolver::QType::A,
                qclass: resolver::QClass::IN,
            }]
    };

    let bytes = msg.to_bytes();
    assert_eq!(bytes, vec![
        0xC6, 0x9C, 0x01, 0x00,
        0x00, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,/*
        0x03, 0x77, 0x77, 0x77,
        0x08, 0x67, 0x61, 0x6d,
        0x65, 0x73, 0x74, 0x61,
        0x72, 0x02, 0x64, 0x65,
        0x00, 0x00, 0x01, 0x00,
        0x01*/])
}
