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

    balance
}
