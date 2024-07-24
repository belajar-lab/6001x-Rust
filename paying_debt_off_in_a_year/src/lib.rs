const EPSILON: f64 = 0.1;

/// Round float to two decimal places
fn round_two(n: f64) -> f64 {
    (n * 100.0).round() / 100.0
}

pub fn calculate_remaining_balance(
    initial_balance: f64,
    annual_interest_rate: f64,
    monthly_payment_rate: f64,
) -> f64 {
    let monthly_interest_rate = annual_interest_rate / 12.0;
    let mut balance = initial_balance;

    for _ in 0..12 {
        balance *= (1.0 - monthly_payment_rate) * (1.0 + monthly_interest_rate);
    }

    round_two(balance)
}

pub fn calculate_lowest_payment(annual_interest_rate: f64, balance: f64) -> f64 {
    let intst = annual_interest_rate / 12.0;
    let mut payment = 0.0;
    let mut bal = 1.0;

    while bal > 0.0 {
        bal = balance;
        payment += 10.0;
        for _ in 0..12 {
            bal = ((bal - payment) * (1.0 + intst)).max(0.0); // ensures balance doesn't go negative
        }
    }

    payment
}

pub fn find_minimum_monthly_payment(balance: f64, annual_interest_rate: f64) -> f64 {
    let mut lower = 0.0;
    let mut upper = (balance * (1.0 + annual_interest_rate / 12.0).powi(12)) / 12.0;

    loop {
        let mut bal = balance;
        let payment = (lower + upper) / 2.0;

        for _ in 0..12 {
            bal = (bal - payment) * (1.0 + annual_interest_rate / 12.0);
        }

        if bal.abs() < EPSILON {
            return round_two(payment);
        } else if bal < -EPSILON {
            upper = payment;
        } else {
            lower = payment;
        }
    }
}