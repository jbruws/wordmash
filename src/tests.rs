use crate::*;
use masher::Masher;

#[test]
fn conversion_to_decimal() {
    let dog_num: u128 = Masher::from_base36(String::from("DOG"));
    let zinc_num: u128 = Masher::from_base36(String::from("ZINC"));
    let walnut_num: u128 = Masher::from_base36(String::from("WALNUT"));
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
fn illegal_initialization() {
    let mash1 = Masher::new("ALLISNORMALBUT_UNDERSCORE");
    let mash2 = Masher::new("кирилица)");
    let mash3 = Masher::new("НАТЕ"); // actually cyrillic
    assert!(mash1.is_err());
    assert!(mash2.is_err());
    assert!(mash3.is_err());
}

#[test]
fn masher_from_unsigned() {
    let m1 = Masher::new(19u8).unwrap();
    let m2 = Masher::new(190u8).unwrap();
    let m3 = Masher::new(15789u16).unwrap();
    let m4 = Masher::new(1578990u32).unwrap();
    assert_eq!(m1.to_string(), "J");
    assert_eq!(m2.to_string(), "5A");
    assert_eq!(m3.to_string(), "C6L");
    assert_eq!(m4.to_string(), "XUCU");
}

#[test]
fn addition() {
    let cat_num: u128 = Masher::from_base36(String::from("CAT"));
    let crux_num: u128 = Masher::from_base36(String::from("CRUX"));
    assert_eq!(cat_num + crux_num, 611918);
    assert_eq!(Masher::to_base36(cat_num + crux_num), "D45Q");
}

#[test]
fn multiplication() {
    let boar_num: u128 = Masher::from_base36(String::from("BOAR"));
    let damned_num: u128 = Masher::from_base36(String::from("DAMNED"));
    assert_eq!(boar_num * damned_num, 437897150155935);
    assert_eq!(Masher::to_base36(boar_num * damned_num), "4B7Z6917DR");
}

#[test]
fn assign_add_mul() {
    let mut m_ass = Masher::new("PRIME").unwrap();
    let m_dummy = Masher::new("PRIME").unwrap();
    let m_movd = Masher::new("SECONDARY").unwrap();
    m_ass += m_movd.clone();
    assert_eq!(m_ass, m_dummy + m_movd);
}
