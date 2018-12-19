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
            rd: false,
            ra: false,
            rcode: resolver::Rcode::NoError,
            qdcount: 1,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        },
        questions: Vec::new(),
    };

    //should yield:
    /* 
    c6 9c 01 00 00 01
    00 00 00 00 00 00 03 77 77 77 08 67 61 6d 65 73
    74 61 72 02 64 65 00 00 01 00 01
    */

}
