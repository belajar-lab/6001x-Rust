use paying_debt_off_in_a_year::calculate_lowest_payment;

#[test]
fn p2_test_case_1() {
    let balance = 3329.0;
    let annual_interest_rate = 0.2;

    let result = calculate_lowest_payment(annual_interest_rate, balance);
    assert_eq!(result, 310.0)
}

#[test]
fn p2_test_case_2() {
    let balance = 4773.0;
    let annual_interest_rate = 0.2;

    let result = calculate_lowest_payment(annual_interest_rate, balance);
    assert_eq!(result, 440.0)
}

#[test]
fn p2_test_case_3() {
    let balance = 3926.0;
    let annual_interest_rate = 0.2;

    let result = calculate_lowest_payment(annual_interest_rate, balance);
    assert_eq!(result, 360.0)
}

#[test]
fn p2_randomized_test_case_1() {
    let balance = 819.0;
    let annual_interest_rate = 0.2;

    let result = calculate_lowest_payment(annual_interest_rate, balance);
    assert_eq!(result, 80.0)
}

#[test]
fn p2_randomized_test_case_2() {
    let balance = 215.0;
    let annual_interest_rate = 0.18;

    let result = calculate_lowest_payment(annual_interest_rate, balance);
    assert_eq!(result, 20.0)
}

#[test]
fn p2_randomized_test_case_3() {
    let balance = 27.0;
    let annual_interest_rate = 0.18;

    let result = calculate_lowest_payment(annual_interest_rate, balance);
    assert_eq!(result, 10.0)
}

#[test]
fn p2_randomized_test_case_4() {
    let balance = 925.0;
    let annual_interest_rate = 0.18;

    let result = calculate_lowest_payment(annual_interest_rate, balance);
    assert_eq!(result, 90.0)
}

#[test]
fn p2_randomized_test_case_5() {
    let balance = 3957.0;
    let annual_interest_rate = 0.2;

    let result = calculate_lowest_payment(annual_interest_rate, balance);
    assert_eq!(result, 370.0)
}

#[test]
fn p2_randomized_test_case_6() {
    let balance = 4852.0;
    let annual_interest_rate = 0.18;

    let result = calculate_lowest_payment(annual_interest_rate, balance);
    assert_eq!(result, 440.0)
}

#[test]
fn p2_randomized_test_case_7() {
    let balance = 4168.0;
    let annual_interest_rate = 0.2;

    let result = calculate_lowest_payment(annual_interest_rate, balance);
    assert_eq!(result, 380.0)
}

#[test]
fn p2_randomized_test_case_8() {
    let balance = 3876.0;
    let annual_interest_rate = 0.2;

    let result = calculate_lowest_payment(annual_interest_rate, balance);
    assert_eq!(result, 360.0)
}

#[test]
fn p2_randomized_test_case_9() {
    let balance = 4131.0;
    let annual_interest_rate = 0.15;

    let result = calculate_lowest_payment(annual_interest_rate, balance);
    assert_eq!(result, 370.0)
}

#[test]
fn p2_randomized_test_case_10() {
    let balance = 3091.0;
    let annual_interest_rate = 0.2;

    let result = calculate_lowest_payment(annual_interest_rate, balance);
    assert_eq!(result, 290.0)
}

#[test]
fn p2_randomized_test_case_11() {
    let balance = 3761.0;
    let annual_interest_rate = 0.2;

    let result = calculate_lowest_payment(annual_interest_rate, balance);
    assert_eq!(result, 350.0)
}

#[test]
fn p2_randomized_test_case_12() {
    let balance = 3930.0;
    let annual_interest_rate = 0.18;

    let result = calculate_lowest_payment(annual_interest_rate, balance);
    assert_eq!(result, 360.0)
}