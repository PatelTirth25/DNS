use std::collections::HashMap;

fn create_ip_map() -> HashMap<String, String> {
    let mut ip_map = HashMap::new();
    ip_map.insert("example.com".to_string(), "1.1.1.1".to_string());
    ip_map.insert("example.net".to_string(), "2.2.2.2".to_string());
    ip_map
}

pub fn findip(domain: &String) -> String {
    let ip = create_ip_map();
    ip.get(domain).cloned().unwrap().to_string()
}
