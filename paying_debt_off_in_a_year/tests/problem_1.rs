use paying_debt_off_in_a_year::calculate_remaining_balance;

#[test]
fn p1_test_case_1() {
    let initial_balance = 42.0;
    let annual_interest_rate = 0.2;
    let monthly_payment_rate = 0.04;

    let result = calculate_remaining_balance(
        initial_balance, 
        annual_interest_rate, 
        monthly_payment_rate
    );
    assert_eq!(result, 31.38)
}

#[test]
fn p1_test_case_2() {
    let initial_balance = 484.0;
    let annual_interest_rate = 0.2;
    let monthly_payment_rate = 0.04;

    let result = calculate_remaining_balance(
        initial_balance, 
        annual_interest_rate, 
        monthly_payment_rate
    );
    assert_eq!(result, 361.61)
}

#[test]
fn p1_randomized_test_case_1() {
    let initial_balance = 325.0;
    let annual_interest_rate = 0.18;
    let monthly_payment_rate = 0.04;

    let result = calculate_remaining_balance(
        initial_balance, 
        annual_interest_rate, 
        monthly_payment_rate
    );
    assert_eq!(result, 238.08)
}

#[test]
fn p1_randomized_test_case_2() {
    let initial_balance = 427.0;
    let annual_interest_rate = 0.12;
    let monthly_payment_rate = 0.06;

    let result = calculate_remaining_balance(
        initial_balance, 
        annual_interest_rate, 
        monthly_payment_rate
    );
    assert_eq!(result, 228.99)
}

#[test]
fn p1_randomized_test_case_3() {
    let initial_balance = 68.0;
    let annual_interest_rate = 0.18;
    let monthly_payment_rate = 0.08;

    let result = calculate_remaining_balance(
        initial_balance, 
        annual_interest_rate, 
        monthly_payment_rate
    );
    assert_eq!(result, 29.89)
}

#[test]
fn p1_randomized_test_case_4() {
    let initial_balance = 86.0;
    let annual_interest_rate = 0.2;
    let monthly_payment_rate = 0.05;

    let result = calculate_remaining_balance(
        initial_balance, 
        annual_interest_rate, 
        monthly_payment_rate
    );
    assert_eq!(result, 56.67)
}