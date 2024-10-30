use std::fs;

use crate::response::answer::Answer;

fn getcname(fans: &mut Vec<Answer>, d: String, eachrow: &Vec<Vec<&str>>) {
    for r in eachrow.iter() {
        if d == r[0] {
            match r[1] {
                "A" => {
                    fans.push(Answer::A {
                        name: d.clone(),
                        qtype: (1),
                        class: (1),
                        ttl: (200),
                        rlen: (4 as u16),
                        rdata: r[2].parse().unwrap(),
                    });
                }
                "CNAME" => {
                    fans.push(Answer::CNAME {
                        name: d.clone(),
                        qtype: (1),
                        class: (1),
                        ttl: (200),
                        rlen: (r[2].len() + 2) as u16,
                        rdata: r[2].parse().unwrap(),
                    });
                    getcname(fans, r[2].to_string(), &eachrow);
                }
                _ => {}
            }
        }
    }
}

pub fn findip(domain: &String) -> Vec<Answer> {
    // Reading file data
    let data = fs::read_to_string("name.txt").expect("Should have been able to read the file");
    let row: Vec<_> = data.split(",\n").collect();
    let mut eachrow: Vec<Vec<_>> = Vec::new();
    for x in row.iter() {
        eachrow.push(x.split(';').collect());
    }

    eachrow.pop();

    let mut fans: Vec<Answer> = Vec::new();

    // Finally finding domain name
    for r in eachrow.iter() {
        if domain == r[0] {
            match r[1] {
                "A" => {
                    fans.push(Answer::A {
                        name: domain.clone(),
                        qtype: (1),
                        class: (1),
                        ttl: (200),
                        rlen: (4 as u16),
                        rdata: r[2].parse().unwrap(),
                    });
                }
                "CNAME" => {
                    fans.push(Answer::CNAME {
                        name: domain.clone(),
                        qtype: (5),
                        class: (1),
                        ttl: (200),
                        rlen: (r[2].len() + 2) as u16,
                        rdata: r[2].to_string(),
                    });
                    getcname(&mut fans, r[2].to_string(), &eachrow);
                }
                _ => {}
            }
        }
    }

    fans
}

// use std::collections::HashMap;
//
// fn create_ip_map() -> HashMap<String, String> {
//     let mut ip_map = HashMap::new();
//     ip_map.insert("example.com".to_string(), "1.3.1.1".to_string());
//     ip_map.insert("example.com".to_string(), "1.1.1.1".to_string());
//     ip_map.insert("example.net".to_string(), "2.2.2.2".to_string());
//     ip_map
// }
//
// pub fn findip(domain: &String) -> String {
//     let ip = create_ip_map();
//     ip.get(domain).cloned().unwrap().to_string()
// }
