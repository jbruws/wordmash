use crate::*;
use masher::Masher;
use rug::Integer;

#[test]
fn conversion_to_decimal() {
    let dog_num = Masher::from_mashed(String::from("DOG"));
    let zinc_num = Masher::from_mashed(String::from("ZINC"));
    let walnut_num = Masher::from_mashed(String::from("WALNUT"));
    assert_eq!(dog_num, 3950);
    assert_eq!(zinc_num, 992292);
    assert_eq!(walnut_num, 1000027399);
}

#[test]
fn illegal_initialization() {
    let mash1 = Masher::new("lowerc");
    let mash2 = Masher::new("кирилица)");
    let mash3 = Masher::new("НАТЕ"); // actually cyrillic
    assert!(mash1.is_err());
    assert!(mash2.is_err());
    assert!(mash3.is_err());
}

#[test]
fn addition() {
    let cat_num = Masher::from_mashed(String::from("CAT"));
    let crux_num = Masher::from_mashed(String::from("CRUX"));
    assert_eq!(cat_num.clone() + crux_num.clone(), 101294);
    assert_eq!(Masher::to_mashed(Integer::from(cat_num + crux_num)), "CTVI");
}

#[test]
fn multiplication() {
    let boar_num = Masher::from_mashed(String::from("BOAR"));
    let damned_num = Masher::from_mashed(String::from("DAMNED"));
    assert_eq!(
        boar_num.clone() + damned_num.clone(),
        Integer::from(136848592)
    );
}

#[test]
fn assign_add_mul() {
    let mut m_ass = Masher::new("PRIME").unwrap();
    let m_dummy = Masher::new("PRIME").unwrap();
    let m_movd = Masher::new("SECONDARY").unwrap();
    m_ass += m_movd.clone();
    assert_eq!(m_ass, m_dummy + m_movd);
}

#[test]
fn u32_to_masher() {
    let m = Masher::new(33u32).unwrap();
    assert_eq!(m.to_string(), " ");
}

#[test]
fn conversion_to_uppercase() {
    let m = Masher::new("qetqe".to_string().to_uppercase());
    assert!(m.is_ok());
    assert_eq!(Masher::from_mashed(m.unwrap().to_string()), 21561104)
}

#[test]
fn long_multiplication() {
    let m = Masher::new("CALIPH").unwrap();
    let m2 = Masher::new("AVERYLONGINSTRUCTIONWORD").unwrap();
    let mres = m * m2;
}
