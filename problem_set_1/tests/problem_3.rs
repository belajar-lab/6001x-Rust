use problem_set_1::longest_substring;

#[test]
fn longest_substring_tests() {
    assert_eq!(longest_substring("cpeaggqk"), "aggq".to_string());
    assert_eq!(longest_substring("hkdslkahqoxvuxfx"), "ahq".to_string());
    assert_eq!(longest_substring("vxjrsrbefbvfo"), "jrs".to_string());
    assert_eq!(longest_substring("uhjjnxomeik"), "hjjnx".to_string());
    assert_eq!(longest_substring("nfcbwjomcxul"), "bw".to_string());
    assert_eq!(longest_substring("btzqlbmhvrblgqeq"), "btz".to_string());
    assert_eq!(longest_substring("lisxvihao"), "isx".to_string());
    assert_eq!(longest_substring("abcdefghijklmnopqrstuvwxyz"), "abcdefghijklmnopqrstuvwxyz".to_string());
    assert_eq!(longest_substring("dcwkyaceihyynexltpotjf"), "acei".to_string());
    assert_eq!(longest_substring("uyucrmgxm"), "uy".to_string());
    assert_eq!(longest_substring("xyxrroaogl"), "xy".to_string());
    assert_eq!(longest_substring("zyxwvutsrqponmlkjihgfedcba"), "z".to_string());
    assert_eq!(longest_substring("fsldfunfznqfdn"), "dfu".to_string());
    assert_eq!(longest_substring("twddicgudjvqae"), "ddi".to_string());
    assert_eq!(longest_substring("vicfbuiylecztvem"), "cf".to_string());
    assert_eq!(longest_substring("smzwmnkj"), "mz".to_string());
    assert_eq!(longest_substring("tunvrwowrp"), "tu".to_string());
    assert_eq!(longest_substring("ebwvlvbknhqyhbwzigjqim"), "bkn".to_string());
    assert_eq!(longest_substring("szuzhwjthro"), "sz".to_string());
    assert_eq!(longest_substring("ggxztlhb"), "ggxz".to_string());
}