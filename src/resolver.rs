
/*
struct field names are straight out of
https://tools.ietf.org/html/rfc1035
*/

#[derive(Copy, Clone)]
pub enum Opcode {
    StandardQuery = 0,
    InverseQuery = 1 << 4,
    Status = 2 << 4,
    Reserved = 3 << 4,
}

#[derive(Copy, Clone)]
pub enum QR {
    Query = 0,
    Response = 1 << 7,
}

#[derive(Copy, Clone)]
pub enum Rcode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
    Reserved = 6,
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

impl Label {
    pub fn from_string(str: String) -> Option<Label> {
        if str.is_ascii() {
            Some(Label{data: str.into_bytes()})
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
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

fn push_u16(vec: &mut Vec<u8>, l : usize) {
    if l <= std::u8::MAX as usize {
        vec.push(0);
        vec.push(l as u8);
    } else {
        let u = l as u16;
        let bytes = u.to_be_bytes();
        vec.push(bytes[0]);
        vec.push(bytes[1]);
    }
}

impl DNSMessage {
    pub fn to_bytes(&self) -> Vec<u8> {

        let mut bytes = Vec::with_capacity(self.num_bytes());

/* Header Layout
                               1  1  1  1  1  1
0   1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                      ID                       |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                    QDCOUNT                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                    ANCOUNT                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                    NSCOUNT                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                    ARCOUNT                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
*/

        let h = &self.header;
        let id_bytes = h.id.to_be_bytes();
        bytes.push(id_bytes[0]);
        bytes.push(id_bytes[1]);

        let mut flags1 : u8 = 0;
        flags1 |= h.qr as u8;// qr_flags(h.qr);
        flags1 |= h.opcode as u8;
        flags1 |= (h.aa as u8) << 2;
        flags1 |= (h.tc as u8) << 1;
        flags1 |= h.rd as u8;

        let mut flags2 : u8 = 0;
        flags2 |= (h.ra as u8) << 7;
        flags2 |= h.rcode as u8;

        bytes.push(flags1);
        bytes.push(flags2);

        push_u16(&mut bytes, self.questions.len());
        push_u16(&mut bytes, 0); //TODO ANCOUNT
        push_u16(&mut bytes, 0); //TODO NSCOUNT
        push_u16(&mut bytes, 0); //TODO ARCOUNT

        //TODO fill question data

        return bytes;
    }

    fn num_bytes(&self) -> usize {
        let mut len = 12; //fixed header len
        for q in &self.questions {
            for l in &q.qname {
                len += l.len() + 1
            }
            len += 1 + 4; //zero termination + fixed question len
        }
        len
    }
}
