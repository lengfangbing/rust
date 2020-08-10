pub enum RejectReason {
    NoReject,
    SaleFakeProducts,
    ManyViolations,
    Other,
}
pub enum ShopBanned {
    IsBan(bool),
}

pub struct PicVerifyModel {
    pub product_id: String,
    pub shop_banned: ShopBanned,
    pub reject_reason: RejectReason,
}

impl PicVerifyModel {
    pub fn get_reject_reason(&self) -> u8{
        match self.reject_reason {
            RejectReason::NoReject => 0,
            RejectReason:: SaleFakeProducts => 1,
            RejectReason::ManyViolations => 2,
            RejectReason:: Other => 3,
        }
    }
    pub fn get_shop_banned_value(&self) -> u8 {
        match self.shop_banned {
            ShopBanned::IsBan(true) => 1,
            ShopBanned::IsBan(false) => 2,
        }
    }
}

pub fn get_reject_reason_type (value: u8) -> RejectReason {
    match value {
        0 => RejectReason::NoReject,
        1 => RejectReason::SaleFakeProducts,
        2 => RejectReason::ManyViolations,
        _ => RejectReason::Other,
    }
}