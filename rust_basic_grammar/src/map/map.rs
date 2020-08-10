use std::collections::HashMap;

pub fn map_demo() {
    let mut map: HashMap<String, String> = HashMap::new();
    map_add_value(&mut map, "name".to_owned(), "冷方冰".to_owned());
    println!("{:?}", map.get("name"));
    map_update_value(&mut map, "name".to_owned(), "方冰同学".to_owned(), true);
    println!("{:#?}", map.get("name"));
    map_remove_value(&mut map, "name2");
    println!("{:#?}", map.get("name"));
}

fn map_add_value(map: &mut HashMap<String, String>, key: String, value: String) {
    map.insert(key, value);
}

fn map_update_value(map: &mut HashMap<String, String>,
                    key: String,
                    value: String,
                    force_update: bool) {
    if force_update {
        map_add_value(map, key, value);
    } else {
        map.entry(key).or_insert(value);
    }
}

fn map_remove_value(map: &mut HashMap<String, String>, key: &str) {
    map.remove(key);
}