use problem_set_1::count_bob;

#[test]
fn count_bob_tests() {
    assert_eq!(count_bob("bobbobobiobbobobobooboonoboobiobobobobobhbobbbo"), 11);
    assert_eq!(count_bob("ubobobeoob"), 2);
    assert_eq!(count_bob("uluboaobtbobbwa"), 1);
    assert_eq!(count_bob("uqrocmrobooobobobobboboobosk"), 4);
    assert_eq!(count_bob("oboooornqbobobbbboobobbobobobobobobbobobooboboobob"), 13);
    assert_eq!(count_bob("boenbooboos"), 0);
    assert_eq!(count_bob("nboboboboxrwobobbvbobvboobmvci"), 5);
    assert_eq!(count_bob("zdlrbrsismf"), 0);
    assert_eq!(count_bob("bobobobobobobobobobobobob"), 12);
    assert_eq!(count_bob("obthobobobobobrobobn"), 5);
    assert_eq!(count_bob("bobbbooboboolbobobbubobboboobobjwbobbbobb"), 9);
    assert_eq!(count_bob("aobobotcobobobobobjbobb"), 6);
    assert_eq!(count_bob("teobzobobo"), 1);
    assert_eq!(count_bob("obobbooboblboboobobobobobo"), 7);
    assert_eq!(count_bob("lbooboboobobqbooboggbod"), 2);
    assert_eq!(count_bob("bobbqybdbobbo"), 2);
    assert_eq!(count_bob("boobbobbbooboboiobobboobboqbobobbg"), 5);
    assert_eq!(count_bob("bobbjiobobowtobobbmobob"), 4);
    assert_eq!(count_bob("cbobobboajobobo"), 3);
    assert_eq!(count_bob("boobuobookzboboboboboboobmo"), 5);
}