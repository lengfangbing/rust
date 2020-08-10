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
    reject_reason: RejectReason,
    ip: IpAddrTest,
}

impl VerifyData {
    fn print_addr(&self) {
        println!("{}", self.product_id);
    }
    fn get_shop_banned_reason(&self) -> i64 {
        match self.shop_banned {
            ShopBanned::NoBan => 0,
            ShopBanned::Ban => 1,
        }
    }
    fn get_reject_reason_value(&self) -> i64 {
        match self.reject_reason {
            RejectReason::NoBan => 0,
            RejectReason::ManyViolation => 1,
            RejectReason::SaleFakeProduct | RejectReason::Other => 2,
        }
    }
}

pub fn verify_func(id: &str) -> bool {
    let product_v4 = VerifyData {
        product_id: id.to_owned(),
        shop_banned: ShopBanned::NoBan,
        reject_reason: RejectReason::NoBan,
        ip: IpAddrTest::V4(10, 92, 238, 191),
    };
    let product_v6 = VerifyData {
        product_id: id.to_owned(),
        shop_banned: ShopBanned::Ban,
        reject_reason: RejectReason::SaleFakeProduct,
        ip: IpAddrTest::V6("::1".to_string()),
    };
    product_v4.print_addr();
    product_v6.print_addr();
    println!("v4 reject reason: {}", product_v4.get_reject_reason_value());
    println!("v6 reject reason: {}", product_v6.get_reject_reason_value());
    println!("v4 shop banned reason: {}", product_v4.get_shop_banned_reason());
    println!("v6 shop banned reason: {}", product_v6.get_shop_banned_reason());
    true
}

pub fn options_test() {
    let a = "32";
    // Rust提供的可直接用的枚举
    let b: Option<&str> = Some("10");
    let c: Result<i64, i64> = Ok(10);
    if let Ok(20) = c {
        println!("c is not equals 20");
    } else {
        println!("c is {}", c.unwrap());
    }
    if c.unwrap() == 10 {
        println!("You guess right: c is 10~");
    }
    println!("{}", c.unwrap());
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
