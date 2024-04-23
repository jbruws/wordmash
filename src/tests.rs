use crate::*;

#[test]
fn conversion_to_decimal() {
    let dog_num: u64 = Masher::from_base36(String::from("DOG"));
    let zinc_num: u64 = Masher::from_base36(String::from("ZINC"));
    let walnut_num: u64 = Masher::from_base36(String::from("WALNUT"));
    assert_eq!(dog_num, 17728);
    assert_eq!(zinc_num, 1657128);
    assert_eq!(walnut_num, 1952724485);
}

#[test]
fn conversion_to_base36() {
    let gibberish_string: String = Masher::to_base36(124111);
    let glide_string: String = Masher::to_base36(27877442);
    assert_eq!(gibberish_string, "2NRJ");
    assert_eq!(glide_string, "GLIDE");
}

#[test]
fn addition() {
    let cat_num: u64 = Masher::from_base36(String::from("CAT"));
    let crux_num: u64 = Masher::from_base36(String::from("CRUX"));
    assert_eq!(cat_num + crux_num, 611918);
    assert_eq!(Masher::to_base36(cat_num + crux_num), "D45Q");
}

#[test]
fn multiplication() {
    let boar_num: u64 = Masher::from_base36(String::from("BOAR"));
    let damned_num: u64 = Masher::from_base36(String::from("DAMNED"));
    assert_eq!(boar_num * damned_num, 437897150155935);
    assert_eq!(Masher::to_base36(boar_num * damned_num), "4B7Z6917DR");
}
