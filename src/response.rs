use std::net::Ipv4Addr;

use crate::{domain::findip, question::Question};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Answer {
    A {
        name: String,
        qtype: u16,
        class: u16,
        ttl: u32,
        rlen: u16,
        rdata: Ipv4Addr,
    }, // 1
    CNAME {
        name: String,
        qtype: u16,
        class: u16,
        ttl: u32,
        rlen: u16,
        rdata: Ipv4Addr,
    }, // 5
}

#[allow(dead_code)]
pub struct Response {
    pub ansbuf: [u8; 512],
}

#[allow(dead_code)]
#[allow(unused)]
impl Response {
    pub fn new(&mut self, question: Question, inp: [u8; 512]) {
        let mut res: [u8; 512] = [0; 512];
        let mut ind = 0;

        // Header section
        res[ind..(ind + 2)].copy_from_slice(&inp[ind..(ind + 2)]);
        ind += 2;

        let ansvec: Vec<Answer> = getanswers(question);

        if ansvec.len() == 0 {
            res[ind..(ind + 2)].copy_from_slice(&[0x81, 0x80]);
            ind += 2;
        } else {
            res[ind..(ind + 2)].copy_from_slice(&[0x81, 0x82]);
            ind += 2;
        }

        res[ind..(ind + 2)].copy_from_slice(&inp[ind..(ind + 2)]);
        ind += 2;

        let c: [u8; 2] = [ansvec.len() as u8 >> 4, ansvec.len() as u8 & 0x0F];
        res[ind..(ind + 2)].copy_from_slice(&c[..]);
        ind += 6;

        // Question section
        res[ind..].copy_from_slice(&inp[ind..]);

        while res[ind] != 0 {
            ind += 1;
        }

        ind += 1;
        res[ind..(ind + 2)].copy_from_slice(&inp[ind..(ind + 2)]);

        ind += 2;
        res[ind..(ind + 2)].copy_from_slice(&inp[ind..(ind + 2)]);

        ind += 2;

        //Answer section
        for j in 0..ansvec.len() {
            let val = &ansvec[j];
            println!("{:#?}", val);

            match val {
                Answer::A {
                    name,
                    qtype,
                    class,
                    ttl,
                    rlen,
                    rdata,
                }
                | Answer::CNAME {
                    name,
                    qtype,
                    class,
                    ttl,
                    rlen,
                    rdata,
                } => {
                    let name_bytes = name.as_bytes(); // Convert name to bytes
                    let qtype_bytes = qtype.to_be_bytes(); // Convert qtype to big-endian bytes
                    let class_bytes = class.to_be_bytes(); // Convert class to big-endian bytes
                    let ttl_bytes = ttl.to_be_bytes(); // Convert ttl to big-endian bytes
                    let rlen_bytes = rlen.to_be_bytes(); // Convert rlen to big-endian bytes
                    let rdata_bytes = rdata.octets(); // Convert IPv4Addr to bytes (4 bytes)

                    // Now, concatenate all these bytes into a single byte array
                    let mut response: Vec<u8> = Vec::new();

                    // Add the bytes in the appropriate order
                    response.extend_from_slice(&[0xC0, 0x0C]); // Name
                    response.extend_from_slice(&qtype_bytes); // Qtype
                    response.extend_from_slice(&class_bytes); // Class
                    response.extend_from_slice(&ttl_bytes); // TTL
                    response.extend_from_slice(&rlen_bytes); // Rlen
                    response.extend_from_slice(&rdata_bytes); // Rdata

                    res[ind..(ind + response.len())].copy_from_slice(&response[..]);
                    ind += response.len();
                }
            }
        }

        self.ansbuf = res;
    }
}

fn getanswers(ques: Question) -> Vec<Answer> {
    let mut ans: Vec<Answer> = Vec::new();

    let b: String = findip(&ques.name);

    if b.len() > 0 {
        ans.push(Answer::A {
            name: ques.name.clone(),
            qtype: (1),
            class: (1),
            ttl: (200),
            rlen: (4 as u16),
            rdata: b.parse().unwrap(),
        });
    }

    ans
}
