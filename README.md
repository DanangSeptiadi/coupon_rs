# coupon_rs

## Description

This package countains of **PRICE** function and **YIELD** function, similar in LibreOffice Calc or Microsoft Excel. 
The package also countains some function to calculate days of coupons.

## Syntax and usage

Firstly, add package to `Cargo.toml`

```toml
[dependencies]
coupon_rs = "0.1.0"
```

Then in rust code:

```rs
use coupon_rs::{get_coupdaybs, get_coupdays, get_coupdaysnc, get_coupnum, get_price_, get_yield_, lcl_get_couppcd};

// PRICE(settlement, maturity, rate, yld, redemption, frequency, basis)
get_price_(n_settle, n_mat, f_rate, f_yield, f_redemp, n_freq, n_base)

// YIELD(settlement, maturity, coupon_rate, price, redemption, frequency, basis)
get_yield_(n_settle, n_mat, f_coup, f_price, f_redemp, n_freq, n_base)

// return payment date before settlement
lcl_get_couppcd(r_settle, r_mat, n_freq)

// return payment date after settlement
lcl_get_coupncd(r_settle, r_mat, n_freq)

// return number of days in one period
get_coupdays(n_settle, n_mat, n_freq, n_base)

// return number of days in period before settlement
get_coupdaybs(n_settle, n_mat, n_freq, n_base)

// return number of days after settlement until the next payment date
get_coupdaysnc(n_settle, n_mat, n_freq, n_base)

// return number of payment in one year
get_coupnum(n_settle, n_mat, n_freq, n_base)
```

## License

This project is licensed under 

* [MIT License](https://opensource.org/licenses/MIT)
