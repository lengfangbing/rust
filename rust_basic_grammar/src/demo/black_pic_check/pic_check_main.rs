mod types;
use types::PicVerifyModel;
use types::get_reject_reason_type;
use std::collections::HashMap;

pub fn start_verify(
    product_id: String,
    is_ban: bool,
    reject_reason: u8,
) -> ((String, u8, u8), bool) {
    let val = PicVerifyModel {
        product_id,
        shop_banned: types::ShopBanned::IsBan(is_ban),
        reject_reason: get_reject_reason_type(reject_reason),
    };
    ((val.product_id.clone(), val.get_reject_reason(), val.get_shop_banned_value()), val.get_shop_banned_value() == 1)
}
pub fn start_verify_many(values: &Vec<HashMap<String, String>>) -> (HashMap<String, i64>) {
    let mut res: HashMap<String, i64> = HashMap::new();
    update_map_value(&mut res, "no_ban".to_owned(), 0, true);
    update_map_value(&mut res, "ban".to_owned(), 0, true);
    for i in values.iter() {
        let filter_value = filter_value_to_tuple(&i);
        let increment_key = if filter_value.1 == 1 {
            "ban".to_owned()
        } else {
            "no_ban".to_owned()
        };
        let res = map_value_increment(&mut res, increment_key, true);
        if res.is_none() {
            println!("{:?} is invalid", i);
        }
    }
    res
}
fn filter_value_to_tuple(value: &HashMap<String, String>) -> (String, u8, u8){
    let mut res = ("".to_owned(), 0, 0);
    let product_id = value.get("product_id");
    if product_id.is_some() {
        res.0 = product_id.unwrap().clone();
    }
    let shop_banned = value.get("shop_banned");
    if shop_banned.is_some() {
        let shop_banned = shop_banned.unwrap().parse();
        if shop_banned.is_ok() {
            res.1 = shop_banned.unwrap();
        }
    }
    let reject_reason = value.get("reject_reason");
    if reject_reason.is_some() {
        let reject_reason = reject_reason.unwrap().parse();
        if reject_reason.is_ok() {
            res.2 = reject_reason.unwrap();
        }
    }
    res
}
fn update_map_value(map: &mut HashMap<String, i64>, key: String, value: i64, is_force: bool) {
    if is_force {
        map.insert(key, value);
    } else {
        map.entry(key).or_insert(value);
    }
}
fn map_value_increment(map: &mut HashMap<String, i64>, key: String, is_increment: bool) -> Option<bool> {
    let value = map.get(key.as_str());
    if value.is_none() {
        None
    } else {
        let value = if is_increment {
            value.unwrap() + 1
        } else {
            value.unwrap() - 1
        };
        update_map_value(map, key, value, true);
        Some(true)
    }
}