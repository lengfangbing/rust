// 不过Rust内部实现了内置的IpAddr
enum IpAddrTest {
    V4(u8, u8, u8, u8),
    V6(String),
}
enum RejectReason {
    NoBan,
    SaleFakeProduct,
    ManyViolation,
    Other,
}
enum ShopBanned {
    Ban,
    NoBan,
}
struct VerifyData {
    product_id: String,
    shop_banned: ShopBanned,
    reject_reason: i64,
    ip: IpAddrTest,
}

impl VerifyData {
    fn print_addr (&self) {
        println!("{}", self.product_id);
    }
    fn print_reason (&self) {
        println!("{}", self.reject_reason);
    }
}

// 通过match得到枚举对应的值
fn get_reject_reason_value (target: &RejectReason) -> i64{
    match target {
        RejectReason::NoBan => 0,
        RejectReason::ManyViolation => 1,
        RejectReason::SaleFakeProduct => 2,
        RejectReason::Other => 3,
        // 对应switch的default
        _ => -1,
    }
}

pub fn verify_func (id: &str) -> bool {
    let product_v4 = VerifyData {
        product_id: id.to_owned(),
        shop_banned: ShopBanned::NoBan,
        reject_reason: get_reject_reason_value(&RejectReason::NoBan),
        ip: IpAddrTest::V4(10, 92, 238, 191),
    };
    let product_v6 = VerifyData {
        product_id: id.to_owned(),
        shop_banned: ShopBanned::Ban,
        reject_reason: get_reject_reason_value(&RejectReason::SaleFakeProduct),
        ip: IpAddrTest::V6("::1".to_string()),
    };
    product_v4.print_addr();
    product_v6.print_addr();
    product_v4.print_reason();
    product_v6.print_reason();
    true
}

pub fn options_test () {
    let a = "32";
    let b: Option<&str> = Some("10");
    // 获取Some的值, 如果为空就给个默认值
    let add = a.to_owned() + b.unwrap_or("20");
    println!("{}", add);
    println!("{}", b.unwrap_or("10"));
    // 如果b是有值的
    if b.is_some() {
        // unwrap不建议使用, 可以使用unwrap_or指定一个替代值
        println!("{}", b.unwrap());
    }
    if let Some("10") = b {
        println!("equals");
    }

}
