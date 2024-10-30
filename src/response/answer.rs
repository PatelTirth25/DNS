use std::net::Ipv4Addr;

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
        rdata: String,
    }, // 5
}
