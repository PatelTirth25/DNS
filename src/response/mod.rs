pub mod answer;
use std::collections::HashMap;

use answer::Answer;

pub mod domain;
use crate::packet::question::Question;
use domain::findip;

#[allow(dead_code)]
pub struct Response {
    pub ansbuf: [u8; 512],
    pub compressname: HashMap<String, Vec<u8>>,
}

#[allow(dead_code)]
#[allow(unused)]
impl Response {
    pub fn resolve(&mut self, question: Question, inp: [u8; 512]) {
        let mut res: [u8; 512] = [0; 512];
        let mut ind = 0;

        // Header section
        res[ind..(ind + 2)].copy_from_slice(&inp[ind..(ind + 2)]);
        ind += 2;

        let ansvec: Vec<Answer> = findip(&question.name.to_string());

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

            match val {
                Answer::A {
                    name,
                    qtype,
                    class,
                    ttl,
                    rlen,
                    rdata,
                } => {
                    let qtype_bytes = qtype.to_be_bytes(); // Convert qtype to big-endian bytes
                    let class_bytes = class.to_be_bytes(); // Convert class to big-endian bytes
                    let ttl_bytes = ttl.to_be_bytes(); // Convert ttl to big-endian bytes
                    let rlen_bytes = rlen.to_be_bytes(); // Convert rlen to big-endian bytes
                    let rdata_bytes = rdata.octets(); // Convert IPv4Addr to bytes (4 bytes)

                    // Now, concatenate all these bytes into a single byte array
                    let mut response: Vec<u8> = Vec::new();

                    // Add the bytes in the appropriate order
                    if *name == question.name.to_string() {
                        response.extend_from_slice(&[0xC0, 0x0C]); // Compression
                    } else if self.compressname.contains_key(name) {
                        response.extend_from_slice(self.compressname.get(name).unwrap());
                    } else {
                        let name_bytes = getnamebytes(name);
                        response.extend_from_slice(&name_bytes);
                        let mut loc: Vec<u8> = vec![0xC0];
                        loc.push(ind as u8);
                        self.compressname.insert(name.to_string(), loc);
                    }
                    response.extend_from_slice(&qtype_bytes); // Qtype
                    response.extend_from_slice(&class_bytes); // Class
                    response.extend_from_slice(&ttl_bytes); // TTL
                    response.extend_from_slice(&rlen_bytes); // Rlen
                    response.extend_from_slice(&rdata_bytes); // Rdata

                    res[ind..(ind + response.len())].copy_from_slice(&response[..]);
                    ind += response.len();
                }
                Answer::CNAME {
                    name,
                    qtype,
                    class,
                    ttl,
                    rlen,
                    rdata,
                } => {
                    let qtype_bytes = qtype.to_be_bytes(); // Convert qtype to big-endian bytes
                    let class_bytes = class.to_be_bytes(); // Convert class to big-endian bytes
                    let ttl_bytes = ttl.to_be_bytes(); // Convert ttl to big-endian bytes
                    let rlen_bytes = rlen.to_be_bytes(); // Convert rlen to big-endian bytes
                    let rdata_bytes = getnamebytes(rdata);

                    // Now, concatenate all these bytes into a single byte array
                    let mut response: Vec<u8> = Vec::new();

                    // Add the bytes in the appropriate order
                    if *name == question.name.to_string() {
                        response.extend_from_slice(&[0xC0, 0x0C]); // Compression
                    } else if self.compressname.contains_key(name) {
                        response.extend_from_slice(self.compressname.get(name).unwrap());
                    } else {
                        let name_bytes = getnamebytes(name);
                        response.extend_from_slice(&name_bytes);
                        let mut loc: Vec<u8> = vec![0xC0];
                        loc.push(ind as u8);
                        self.compressname.insert(name.to_string(), loc);
                    }

                    response.extend_from_slice(&qtype_bytes); // Qtype
                    response.extend_from_slice(&class_bytes); // Class
                    response.extend_from_slice(&ttl_bytes); // TTL
                    response.extend_from_slice(&rlen_bytes); // Rlen

                    let mut loc: Vec<u8> = vec![0xC0];
                    loc.push((ind + response.len()) as u8);
                    self.compressname.insert(rdata.to_string(), loc);

                    response.extend_from_slice(&rdata_bytes); // Rdata

                    res[ind..(ind + response.len())].copy_from_slice(&response[..]);
                    ind += response.len();
                }
                _ => {}
            }
        }

        self.ansbuf = res;
    }
}

fn getnamebytes(name: &str) -> Vec<u8> {
    let mut namebytes: Vec<u8> = Vec::new();
    namebytes.push(0);
    let mut cnt = 0;
    for c in name.chars().rev() {
        if c == '.' {
            namebytes.push(cnt);
            cnt = 0;
        } else {
            cnt += 1;
            namebytes.push(c as u8);
        }
    }

    namebytes.push(cnt);

    namebytes.reverse();

    namebytes
}
