use chrono::NaiveDate;
use coupon_rs::{get_coupdaybs, get_coupdays, get_coupdaysnc, get_coupnum, get_price_, get_yield_, lcl_get_couppcd};

fn main() {
    let n_settle = NaiveDate::parse_from_str("2024-02-29", "%Y-%m-%d").unwrap();
    let n_mat = NaiveDate::parse_from_str("2045-01-15", "%Y-%m-%d").unwrap();
    let f_price = 99.378;
    let f_coup = 0.04125;
    let f_rate = 0.04125;
    let f_redemp = 100.0;
    let n_freq: i32 = 2;
    let n_base: i32 = 0;

    let couppcd: NaiveDate = lcl_get_couppcd(n_settle, n_mat, n_freq);

    println!("CoupDays = {} ", get_coupdays(n_settle, n_mat, n_freq, n_base));
    println!("CoupDayBS = {} ", get_coupdaybs(n_settle, n_mat, n_freq, n_base));
    println!("CoupDaySNC = {} ", get_coupdaysnc(n_settle, n_mat, n_freq, n_base));
    println!("CoupPcd = {} ", couppcd.format("%Y-%m-%d").to_string());
    println!("CoupNum = {} ", get_coupnum(n_settle, n_mat, n_freq, n_base));

    println!("Price yield 0 = {}", get_price_(n_settle, n_mat, f_rate, 0.0, f_redemp, n_freq, n_base));
    println!("Price yield 0 = {}", get_price_(n_settle, n_mat, f_rate, 1.0, f_redemp, n_freq, n_base));

    println!("Yield = {}", get_yield_(n_settle, n_mat, f_coup, f_price, f_redemp, n_freq, n_base));
}
