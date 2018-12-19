
/*
struct field names are straight out of
https://tools.ietf.org/html/rfc1035
*/

pub enum Opcode {
    StandardQuery,
    InverseQuery,
    Status,
    Reserved,
}

pub enum QR {
    Query,
    Response,
}

pub enum Rcode {
    NoError,
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused,
    Reserved,
}

pub struct Header {
    pub id: u16,
    pub qr: QR,
    pub opcode: Opcode,
    pub aa: bool,
    pub tc: bool,
    pub rd: bool,
    pub ra: bool,
    pub rcode: Rcode,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

pub enum QType {
    A,
}

pub enum QClass {
    IN,
}

pub struct Label {
    data: Vec<u8>
}

pub struct Question {
    pub qname: Vec<Label>,
    pub qtype: QType,
    pub qclass: QClass,
}

//very low-level struct for a DNSMessage
pub struct DNSMessage {
    pub header: Header,
    pub questions: Vec<Question>,
}
