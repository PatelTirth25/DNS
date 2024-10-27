mod rscode;
use rscode::ResultCode;

#[allow(unused_variables)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub qr: bool,
    pub opcode: u16,
    pub aa: bool,
    pub tc: bool,
    pub rd: bool,
    pub ra: bool,
    pub z: bool,
    pub rcode: ResultCode,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl Header {
    pub fn new(inp: &[u8; 512]) -> Header {
        let headslice = &inp[0..12];
        let header: [u8; 12] = headslice.try_into().unwrap();

        // Transaction ID (16 bits, 2 bytes)
        let transaction_id = u16::from_be_bytes([header[0], header[1]]);

        // Flags (16 bits, 2 bytes)
        let flags = u16::from_be_bytes([header[2], header[3]]);
        let qr = (flags >> 15) & 0x1; // 1-bit for QR
        let opcode = (flags >> 11) & 0xF; // 4-bit for Opcode
        let aa = (flags >> 10) & 0x1; // 1-bit for AA
        let tc = (flags >> 9) & 0x1; // 1-bit for TC
        let rd = (flags >> 8) & 0x1; // 1-bit for RD
        let ra = (flags >> 7) & 0x1; // 1-bit for RA
        let z = (flags >> 4) & 0x7; // 3-bit for Z (reserved, must be 0)
        let rcode = flags & 0xF; // 4-bit for RCODE

        // Questions (16 bits, 2 bytes)
        let questions = u16::from_be_bytes([header[4], header[5]]);

        // Answer RRs (16 bits, 2 bytes)its
        let answer_rrs = u16::from_be_bytes([header[6], header[7]]);

        // Authority RRs (16 bits, 2 bytes)
        let authority_rrs = u16::from_be_bytes([header[8], header[9]]);

        // Additional RRs (16 bits, 2 bytes)
        let additional_rrs = u16::from_be_bytes([header[10], header[11]]);

        Header {
            id: transaction_id,
            qr: qr == 1,
            opcode,
            aa: aa == 1,
            tc: tc == 1,
            rd: rd == 1,
            ra: ra == 1,
            z: z == 0,
            rcode: ResultCode::from_num(rcode),
            qdcount: questions,
            ancount: answer_rrs,
            nscount: authority_rrs,
            arcount: additional_rrs,
        }
    }
}
