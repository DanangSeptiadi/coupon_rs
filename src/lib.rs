/*!
 * 'coupon_rs'
 * provide function same as Excel and LibreOffice Calc for Coupon.
 * Build from LibreOffice Calc's code base
*/

use std::panic;

use chrono::{Datelike, NaiveDate};
use chronoutil::shift_months;

pub fn lcl_get_couppcd(r_settle: NaiveDate, r_mat: NaiveDate, n_freq:i32) -> NaiveDate {
    let mut r_date: NaiveDate;
    r_date = r_mat.clone();

    if r_date.with_year(r_settle.year()).is_none() {
        r_date = r_date.with_day(28).unwrap().with_year(r_settle.year()).unwrap();
    } else {
        r_date = r_date.with_year(r_settle.year()).unwrap();
    }

    if r_date < r_settle {
        let year = r_date.year().clone();
        r_date = r_date.with_year(year+1).unwrap();
    }

    let sub_freq = -12 / n_freq as i32;
    while r_date > r_settle {
        r_date = shift_months(r_date, sub_freq);
    }
    r_date
}

pub fn get_coupdaybs(n_settle: NaiveDate, n_mat: NaiveDate, n_freq:i32, n_base: i32) -> f64 {
    if n_settle >= n_mat {
        panic!("panic settle >= mat");
    }

    let a_date = lcl_get_couppcd(n_settle, n_mat, n_freq);

    let day_diff: f64;
    if n_base == 0 || n_base == 4 {
        let month_diff = (n_settle.year() - a_date.year()) * 12 + (n_settle.month() as i32 - a_date.month() as i32);
        let delta = NaiveDate::signed_duration_since(n_settle, shift_months(a_date, month_diff));
        day_diff = delta.num_days() as f64 + 30.0 * month_diff as f64;
    }
    else {
        day_diff = NaiveDate::signed_duration_since(n_settle, a_date).num_days() as f64;
    }
    
    day_diff
}

pub fn lcl_get_coupncd(r_settle: NaiveDate, r_mat: NaiveDate, n_freq:i32) -> NaiveDate {
    let mut r_date: NaiveDate;
    r_date = r_mat.clone();

    if r_date.with_year(r_settle.year()).is_none() {
        r_date = r_date.with_day(28).unwrap().with_year(r_settle.year()).unwrap();
    } else {
        r_date = r_date.with_year(r_settle.year()).unwrap();
    }

    if r_date > r_settle {
        let year = r_date.year().clone();
        r_date = r_date.with_year(year-1).unwrap();
    }

    let sub_freq = 12 / n_freq as i32;
    while r_date <= r_settle {
        r_date = shift_months(r_date, sub_freq);
    }
    r_date
}

pub fn get_coupdays(n_settle: NaiveDate, n_mat: NaiveDate, n_freq:i32, n_base: i32) -> f64 {
    if n_settle >= n_mat {
        panic!("panic settle >= mat");
    }

    let day_diff: f64;
    if n_base == 1 {
        let a_date = lcl_get_couppcd(n_settle, n_mat, n_freq);
        let a_nextdate = shift_months(a_date, 12 / n_freq);
        day_diff = NaiveDate::signed_duration_since(a_nextdate, a_date).num_days() as f64;
    }
    else if n_base == 3 {
        day_diff = 365.0 / n_freq as f64;
    }
    else {
        day_diff = 360.0 / n_freq as f64;
    }
    day_diff
}

pub fn get_coupdaysnc(n_settle: NaiveDate, n_mat: NaiveDate, n_freq:i32, n_base: i32) -> f64 {
    if n_settle >= n_mat {
        panic!("panic settle >= mat");
    }

    let day_diff: f64;
    if n_base != 0 && n_base != 4 {
        let a_date = lcl_get_coupncd(n_settle, n_mat, n_freq);
        day_diff = NaiveDate::signed_duration_since(a_date, n_settle).num_days() as f64;
    }
    else {
        day_diff = get_coupdays(n_settle, n_mat, n_freq, n_base) - get_coupdaybs(n_settle, n_mat, n_freq, n_base);
    }
    day_diff
}

pub fn get_coupnum(n_settle: NaiveDate, n_mat: NaiveDate, n_freq:i32, _n_base: i32) -> f64 {
    if n_settle >= n_mat {
        panic!("panic settle >= mat");
    }

    let a_mat = n_mat.clone();
    let a_date = lcl_get_couppcd(n_settle, a_mat, n_freq);
    let n_months = (a_mat.year() - a_date.year()) * 12 + (a_mat.month() as i32 - a_date.month() as i32);

    n_months as f64 * n_freq as f64 / 12.0
}

pub fn get_price_(n_settle: NaiveDate, n_mat: NaiveDate, f_rate: f64, f_yield: f64, f_redemp: f64, n_freq:i32, n_base: i32) -> f64 {
    let f_freq = n_freq as f64;

    let f_e = get_coupdays(n_settle, n_mat, n_freq, n_base);
    let f_dsc_e = get_coupdaysnc(n_settle, n_mat, n_freq, n_base) / f_e;
    let f_n = get_coupnum(n_settle, n_mat, n_freq, n_base);
    let f_a = get_coupdaybs(n_settle, n_mat, n_freq, n_base);

    let mut f_ret = f_redemp / ( f64::powf(1.0 + f_yield / f_freq, f_n - 1.0 + f_dsc_e));
    f_ret = f_ret - ( 100.0 * f_rate / f_freq * f_a / f_e );

    let f_t1 = 100.0 * f_rate / f_freq;
    let f_t2 = 1.0 + f_yield / f_freq;

    let mut f_k:f64 = 0.0;
    while f_k < f_n {
        f_ret += f_t1 / f64::powf(f_t2, f_k + f_dsc_e);
        f_k += 1.0;
    }
    f_ret
}

pub fn get_yield_(n_settle: NaiveDate, n_mat: NaiveDate, f_coup:f64, f_price:f64, f_redemp: f64, n_freq:i32, n_base: i32) -> f64 {
    let f_rate = f_coup;
    let mut f_pricen = 0.0 as f64;
    let mut f_yield1 = 0.0 as f64;
    let mut f_yield2 = 1.0 as f64;

    let mut f_price1 = get_price_(n_settle, n_mat, f_rate, f_yield1, f_redemp, n_freq, n_base);
    let mut f_price2 = get_price_(n_settle, n_mat, f_rate, f_yield2, f_redemp, n_freq, n_base);
    let mut f_yieldn = (f_yield2 - f_yield1) * 0.5;

    let mut f_final_yield: f64 = 0.0;
    let mut n_iter: i32 = 0;
    let mut is_break = false;
    while n_iter < 100 && f_pricen != f_price {
        f_pricen = get_price_(n_settle, n_mat, f_rate, f_yieldn, f_redemp, n_freq, n_base);

        if f_price == f_price1{
            f_final_yield = f_yield1;
            is_break = true;
            break;
        }
        else if f_price == f_price2 {
            f_final_yield = f_yield2;
            is_break = true;
            break;
        }
        else if f_price == f_pricen {
            f_final_yield = f_yieldn;
            is_break = true;
            break;
        }
        else if f_price < f_price2 {
            f_yield2 = f_yield2 * 2.0;
            f_price2 = get_price_(n_settle, n_mat, f_rate, f_yield2, f_redemp, n_freq, n_base);

            f_yieldn = (f_yield2 - f_yield1) * 0.5;
        }
        else {
            if f_price < f_pricen {
                f_yield1 = f_yieldn;
                f_price1 = f_pricen;
            }
            else {
                f_yield2 = f_yieldn;
                f_price2 = f_pricen;
            }
            f_yieldn = f_yield2 - (f_yield2-f_yield1) * ( (f_price - f_price2) / (f_price1 - f_price2) );
        }
        n_iter += 1;
    }

    if is_break == false {
        if f64::abs(f_price - f_pricen) > f_price / 100.0 {
            panic!("result not precise enough");
        }
        f_final_yield = f_yieldn;
    }

    f_final_yield
}