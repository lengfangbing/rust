// 暴露公共模块 black_pic_check, 因为black_pic_check也是一个mod, 所以可以暴露出来
pub mod black_pic_check;

// 暴露出公共的使用的方法等数据类型, 外部引入了这个模块就可以直接使用
pub use black_pic_check::pic_check_main::start_verify_many;

pub mod actix_http;

pub mod io_grep;

pub mod rocket_http;