use paying_debt_off_in_a_year::find_minimum_monthly_payment;

#[test]
fn p3_test_case_1() {
    let balance = 320000.0;
    let annual_interest_rate = 0.2;

    let result = find_minimum_monthly_payment(balance, annual_interest_rate);
    assert_eq!(result, 29157.08)
}

#[test]
fn p3_test_case_2() {
    let balance = 999999.0;
    let annual_interest_rate = 0.18;

    let result = find_minimum_monthly_payment(balance, annual_interest_rate);
    assert_eq!(result, 90325.03)
}

#[test]
fn p3_randomized_test_case_1() {
    let balance = 293048.0;
    let annual_interest_rate = 0.21;

    let result = find_minimum_monthly_payment(balance, annual_interest_rate);
    assert_eq!(result, 26817.50)
}

#[test]
fn p3_randomized_test_case_2() {
    let balance = 45866.0;
    let annual_interest_rate = 0.15;

    let result = find_minimum_monthly_payment(balance, annual_interest_rate);
    assert_eq!(result, 4088.68)
}

#[test]
fn p3_randomized_test_case_3() {
    let balance = 467946.0;
    let annual_interest_rate = 0.22;

    let result = find_minimum_monthly_payment(balance, annual_interest_rate);
    assert_eq!(result, 43008.63)
}

#[test]
fn p3_randomized_test_case_4() {
    let balance = 416914.0;
    let annual_interest_rate = 0.21;

    let result = find_minimum_monthly_payment(balance, annual_interest_rate);
    assert_eq!(result, 38152.76)
}

#[test]
fn p3_randomized_test_case_5() {
    let balance = 408670.0;
    let annual_interest_rate = 0.15;

    let result = find_minimum_monthly_payment(balance, annual_interest_rate);
    assert_eq!(result, 36430.48)
}

#[test]
fn p3_randomized_test_case_6() {
    let balance = 58503.0;
    let annual_interest_rate = 0.2;

    let result = find_minimum_monthly_payment(balance, annual_interest_rate);
    assert_eq!(result, 5330.55)
}

#[test]
fn p3_randomized_test_case_7() {
    let balance = 205907.0;
    let annual_interest_rate = 0.22;

    let result = find_minimum_monthly_payment(balance, annual_interest_rate);
    assert_eq!(result, 18924.78)
}

#[test]
fn p3_randomized_test_case_8() {
    let balance = 91139.0;
    let annual_interest_rate = 0.18;

    let result = find_minimum_monthly_payment(balance, annual_interest_rate);
    assert_eq!(result, 8232.14)
}

#[test]
fn p3_randomized_test_case_9() {
    let balance = 155484.0;
    let annual_interest_rate = 0.22;

    let result = find_minimum_monthly_payment(balance, annual_interest_rate);
    assert_eq!(result, 14290.44)
}

#[test]
fn p3_randomized_test_case_10() {
    let balance = 149136.0;
    let annual_interest_rate = 0.15;

    let result = find_minimum_monthly_payment(balance, annual_interest_rate);
    assert_eq!(result, 13294.59)
}