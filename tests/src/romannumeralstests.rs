#[cfg(test)]
use learn_rust_lib::numbers::romannumerals as rnum;
use learn_rust_lib::numbers::romannumerals::RomanDigit as RD;
use learn_rust_lib::numbers::romannumerals::RomanNumeral as RN;
use RD::{I, V, X, L, C, D, M};

#[test]
pub fn test_roman_digit_as_char() {
    assert_eq!(I.as_char(), 'I');
    assert_eq!(V.as_char(), 'V');
    assert_eq!(X.as_char(), 'X');
    assert_eq!(L.as_char(), 'L');
    assert_eq!(C.as_char(), 'C');
    assert_eq!(D.as_char(), 'D');
    assert_eq!(M.as_char(), 'M');
}

#[test]
pub fn test_roman_digit_from_char() {
    assert_eq!(RD::from_char('i'), Some(I));
    assert_eq!(RD::from_char('I'), Some(I));
    assert_eq!(RD::from_char('v'), Some(V));
    assert_eq!(RD::from_char('V'), Some(V));
    assert_eq!(RD::from_char('x'), Some(X));
    assert_eq!(RD::from_char('X'), Some(X));
    assert_eq!(RD::from_char('l'), Some(L));
    assert_eq!(RD::from_char('L'), Some(L));
    assert_eq!(RD::from_char('c'), Some(C));
    assert_eq!(RD::from_char('C'), Some(C));
    assert_eq!(RD::from_char('d'), Some(D));
    assert_eq!(RD::from_char('D'), Some(D));
    assert_eq!(RD::from_char('m'), Some(M));
    assert_eq!(RD::from_char('M'), Some(M));
    assert_eq!(RD::from_char('a'), None);
    assert_eq!(RD::from_char('N'), None);
    assert_eq!(RD::from_char('2'), None);
    assert_eq!(RD::from_char(' '), None);
    assert_eq!(RD::from_char('_'), None);
    assert_eq!(RD::from_char('\n'), None);
}

#[test]
pub fn test_is_valid_roman_numeral_string() {
    // building blocks
    assert!(RN::is_valid_roman_numeral_string("I"));
    assert!(RN::is_valid_roman_numeral_string("IV"));
    assert!(RN::is_valid_roman_numeral_string("V"));
    assert!(RN::is_valid_roman_numeral_string("IX"));
    assert!(RN::is_valid_roman_numeral_string("X"));
    assert!(RN::is_valid_roman_numeral_string("XL"));
    assert!(RN::is_valid_roman_numeral_string("L"));
    assert!(RN::is_valid_roman_numeral_string("XC"));
    assert!(RN::is_valid_roman_numeral_string("C"));
    assert!(RN::is_valid_roman_numeral_string("CD"));
    assert!(RN::is_valid_roman_numeral_string("D"));
    assert!(RN::is_valid_roman_numeral_string("CM"));
    assert!(RN::is_valid_roman_numeral_string("M"));
    assert!(RN::is_valid_roman_numeral_string("MMMM"));

    // random
    assert!(RN::is_valid_roman_numeral_string("II"));
    assert!(RN::is_valid_roman_numeral_string("VIII"));
    assert!(RN::is_valid_roman_numeral_string("XXV"));
    assert!(RN::is_valid_roman_numeral_string("XLIV"));
    assert!(RN::is_valid_roman_numeral_string("LXXVI"));
    assert!(RN::is_valid_roman_numeral_string("CCXXXVII"));
    assert!(RN::is_valid_roman_numeral_string("CDXII"));
    assert!(RN::is_valid_roman_numeral_string("DLV"));
    assert!(RN::is_valid_roman_numeral_string("DCCLXXVII"));
    assert!(RN::is_valid_roman_numeral_string("MCXI"));
    assert!(RN::is_valid_roman_numeral_string("MCCXXXIV"));
    assert!(RN::is_valid_roman_numeral_string("MCDLIII"));
    assert!(RN::is_valid_roman_numeral_string("MDCCCLXXVII"));
    assert!(RN::is_valid_roman_numeral_string("MCMXVIII"));
    assert!(RN::is_valid_roman_numeral_string("MMXX"));
    assert!(RN::is_valid_roman_numeral_string("MMCCXXII"));
    assert!(RN::is_valid_roman_numeral_string("MMCCCXCIV"));
    assert!(RN::is_valid_roman_numeral_string("MMDCXCV"));
    assert!(RN::is_valid_roman_numeral_string("MMDCCC"));
    assert!(RN::is_valid_roman_numeral_string("MMM"));
    assert!(RN::is_valid_roman_numeral_string("MMMCCCXXXIII"));
    assert!(RN::is_valid_roman_numeral_string("MMMCDLVI"));
    assert!(RN::is_valid_roman_numeral_string("MMMDCCCLXXIX"));
    assert!(RN::is_valid_roman_numeral_string("MMMCMLXXXVII"));
    assert!(RN::is_valid_roman_numeral_string("MMMMCLXVI"));
    assert!(RN::is_valid_roman_numeral_string("MMMMCDXLIV"));
    assert!(RN::is_valid_roman_numeral_string("MMMMDCCCLXXXVIII"));
    assert!(RN::is_valid_roman_numeral_string("MMMMCMLXXXVII"));
    assert!(RN::is_valid_roman_numeral_string("MMMMCMXCIX"));

    // lower-case
    assert!(RN::is_valid_roman_numeral_string("mmdcclxiv"));
    assert!(RN::is_valid_roman_numeral_string("x"));

    // mixed-case
    assert!(RN::is_valid_roman_numeral_string("MmCcXxIi"));
    assert!(RN::is_valid_roman_numeral_string("DlV"));
    assert!(RN::is_valid_roman_numeral_string("dLV"));
    assert!(RN::is_valid_roman_numeral_string("DLv"));

    // invalid strings
    assert!(!RN::is_valid_roman_numeral_string("MCMCDXXXLVII"));
    assert!(!RN::is_valid_roman_numeral_string("IVMCDLCXM"));
    assert!(!RN::is_valid_roman_numeral_string("mcdlxxxivi"));
    assert!(!RN::is_valid_roman_numeral_string("MCNLXXVII"));
    assert!(!RN::is_valid_roman_numeral_string("MC_LXXVII"));
    assert!(!RN::is_valid_roman_numeral_string("MC LXXVII"));
    assert!(!RN::is_valid_roman_numeral_string("MC\nLXXVII"));
    assert!(!RN::is_valid_roman_numeral_string("MC1LXXVII"));
    assert!(!RN::is_valid_roman_numeral_string("MCMLXXVII "));
    assert!(!RN::is_valid_roman_numeral_string(" MCMLXXVII"));
    assert!(!RN::is_valid_roman_numeral_string(" MCMLXXVII "));
    assert!(!RN::is_valid_roman_numeral_string(" "));
    assert!(!RN::is_valid_roman_numeral_string(""));
}

#[test]
pub fn test_roman_numeral_from_string() {
    assert_eq!(*RN::from_string("MCMLXXVII").get_content(), vec![M, C, M, L, X, X, V, I, I]);
    assert_eq!(*RN::from_string("MmCcXxIi").get_content(), vec![M, M, C, C, X, X, I, I]);
    assert_eq!(*RN::from_string("DlV").get_content(), vec![D, L, V]);
    assert_eq!(*RN::from_string("dLV").get_content(), vec![D, L, V]);
    assert_eq!(*RN::from_string("DLv").get_content(), vec![D, L, V]);
    assert_eq!(*RN::from_string("X").get_content(), vec![X]);
    assert_eq!(*RN::from_string("c").get_content(), vec![C]);
    assert!(RN::from_string("IVMCDLCXM").empty());
    assert!(RN::from_string("MCNLXXVII").empty());
    assert!(RN::from_string("MC_LXXVII").empty());
    assert!(RN::from_string("MC LXXVII").empty());
    assert!(RN::from_string("MC\nLXXVII").empty());
    assert!(RN::from_string("MC1LXXVII").empty());
    assert!(RN::from_string("MCMLXXVII ").empty());
    assert!(RN::from_string(" MCMLXXVII ").empty());
    assert!(RN::from_string("").empty());
}

#[test]
pub fn test_roman_numeral_to_string() {
    assert_eq!(RN::from_string("MCMLXXVII").to_string(), "MCMLXXVII".to_string());
    assert_eq!(RN::from_string("MmCcXxIi").to_string(), "MMCCXXII".to_string());
    assert_eq!(RN::from_string("DlV").to_string(), "DLV".to_string());
    assert_eq!(RN::from_string("dLV").to_string(), "DLV".to_string());
    assert_eq!(RN::from_string("DLv").to_string(), "DLV".to_string());
    assert_eq!(RN::from_string("X").to_string(), "X".to_string());
    assert_eq!(RN::from_string("c").to_string(), "C".to_string());
    assert_eq!(RN::from_string("IVMCDLCXM").to_string(), "".to_string());
    assert_eq!(RN::from_string("MCNLXXVII").to_string(), "".to_string());
    assert_eq!(RN::from_string("MC_LXXVII").to_string(), "".to_string());
    assert_eq!(RN::from_string("MC LXXVII").to_string(), "".to_string());
    assert_eq!(RN::from_string("MC\nLXXVII").to_string(), "".to_string());
    assert_eq!(RN::from_string("MC1LXXVII").to_string(), "".to_string());
    assert_eq!(RN::from_string("MCMLXXVII ").to_string(), "".to_string());
    assert_eq!(RN::from_string(" MCMLXXVII ").to_string(), "".to_string());
    assert_eq!(RN::from_string("").to_string(), "".to_string());
}

#[test]
pub fn test_roman_numeral_from_roman_digits() {
    assert_eq!(*RN::from_roman_digits(&vec![M, C, M, L, X, X, V, I, I]).get_content(), vec![M, C, M, L, X, X, V, I, I]);
    assert_eq!(*RN::from_roman_digits(&vec![D, L, V]).get_content(), vec![D, L, V]);
    assert_eq!(*RN::from_roman_digits(&vec![X]).get_content(), vec![X]);
    assert!(RN::from_roman_digits(&vec![I, V, M, C, D, L, C, X, M]).empty());
    assert!(RN::from_roman_digits(&vec![M, C, M, L, X, X, V, I, V]).empty());
    assert!(RN::from_roman_digits(&Vec::new()).empty());
}

#[test]
pub fn test_roman_numeral_clone() {
    let numeral = RN::from_string("MCDXLIV");
    let cloned_numeral = numeral.clone();

    assert_eq!(*numeral.get_content(), vec![M, C, D, X, L, I, V]);
    assert_eq!(*cloned_numeral.get_content(), vec![M, C, D, X, L, I, V]);

    let numeral = RN::from_string("");
    let cloned_numeral = numeral.clone();

    assert!(numeral.empty());
    assert!(cloned_numeral.empty());
}

#[test]
pub fn test_convert_number_to_roman_numeral() {
    let mut converter = rnum::NumberToRomanNumeralConverter::create();

    // building blocks
    assert_eq!(*converter.convert(1).get_content(), vec![I]);
    assert_eq!(*converter.convert(4).get_content(), vec![I, V]);
    assert_eq!(*converter.convert(5).get_content(), vec![V]);
    assert_eq!(*converter.convert(9).get_content(), vec![I, X]);
    assert_eq!(*converter.convert(10).get_content(), vec![X]);
    assert_eq!(*converter.convert(40).get_content(), vec![X, L]);
    assert_eq!(*converter.convert(50).get_content(), vec![L]);
    assert_eq!(*converter.convert(90).get_content(), vec![X, C]);
    assert_eq!(*converter.convert(100).get_content(), vec![C]);
    assert_eq!(*converter.convert(400).get_content(), vec![C, D]);
    assert_eq!(*converter.convert(500).get_content(), vec![D]);
    assert_eq!(*converter.convert(900).get_content(), vec![C, M]);
    assert_eq!(*converter.convert(1000).get_content(), vec![M]);
    assert_eq!(*converter.convert(4000).get_content(), vec![M, M, M, M]);

    // bounds and beyond bounds
    assert_eq!(*converter.convert(0).get_content(), Vec::<RD>::new());
    assert_eq!(*converter.convert(5000).get_content(), Vec::<RD>::new());
    assert_eq!(*converter.convert(5001).get_content(), Vec::<RD>::new());
    assert_eq!(*converter.convert(9875).get_content(), Vec::<RD>::new());

    // random
    assert_eq!(*converter.convert(2).get_content(), vec![I, I]);
    assert_eq!(*converter.convert(8).get_content(), vec![V, I, I, I]);
    assert_eq!(*converter.convert(25).get_content(), vec![X, X, V]);
    assert_eq!(*converter.convert(44).get_content(), vec![X, L, I, V]);
    assert_eq!(*converter.convert(76).get_content(), vec![L, X, X, V, I]);
    assert_eq!(*converter.convert(237).get_content(), vec![C, C, X, X, X, V, I, I]);
    assert_eq!(*converter.convert(412).get_content(), vec![C, D, X, I, I]);
    assert_eq!(*converter.convert(555).get_content(), vec![D, L, V]);
    assert_eq!(*converter.convert(777).get_content(), vec![D, C, C, L, X, X, V, I, I]);
    assert_eq!(*converter.convert(1111).get_content(), vec![M, C, X, I]);
    assert_eq!(*converter.convert(1234).get_content(), vec![M, C, C, X, X, X, I, V]);
    assert_eq!(*converter.convert(1453).get_content(), vec![M, C, D, L, I, I, I]);
    assert_eq!(*converter.convert(1877).get_content(), vec![M, D, C, C, C, L, X, X, V, I, I]);
    assert_eq!(*converter.convert(1918).get_content(), vec![M, C, M, X, V, I, I, I]);
    assert_eq!(*converter.convert(2020).get_content(), vec![M, M, X, X]);
    assert_eq!(*converter.convert(2222).get_content(), vec![M, M, C, C, X, X, I, I]);
    assert_eq!(*converter.convert(2394).get_content(), vec![M, M, C, C, C, X, C, I, V]);
    assert_eq!(*converter.convert(2695).get_content(), vec![M, M, D, C, X, C, V]);
    assert_eq!(*converter.convert(2800).get_content(), vec![M, M, D, C, C, C]);
    assert_eq!(*converter.convert(3000).get_content(), vec![M, M, M]);
    assert_eq!(*converter.convert(3333).get_content(), vec![M, M, M, C, C, C, X, X, X, I, I, I]);
    assert_eq!(*converter.convert(3456).get_content(), vec![M, M, M, C, D, L, V, I]);
    assert_eq!(*converter.convert(3879).get_content(), vec![M, M, M, D, C, C, C, L, X, X, I, X]);
    assert_eq!(*converter.convert(3987).get_content(), vec![M, M, M, C, M, L, X, X, X, V, I, I]);
    assert_eq!(*converter.convert(4166).get_content(), vec![M, M, M, M, C, L, X, V, I]);
    assert_eq!(*converter.convert(4444).get_content(), vec![M, M, M, M, C, D, X, L, I, V]);
    assert_eq!(*converter.convert(4888).get_content(), vec![M, M, M, M, D, C, C, C, L, X, X, X, V, I, I, I]);
    assert_eq!(*converter.convert(4987).get_content(), vec![M, M, M, M, C, M, L, X, X, X, V, I, I]);
    assert_eq!(*converter.convert(4999).get_content(), vec![M, M, M, M, C, M, X, C, I, X]);
}

#[test]
pub fn test_convert_number_to_roman_numeral_using_hash() {
    // building blocks
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(1).get_content(), vec![I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(4).get_content(), vec![I, V]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(5).get_content(), vec![V]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(9).get_content(), vec![I, X]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(10).get_content(), vec![X]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(40).get_content(), vec![X, L]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(50).get_content(), vec![L]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(90).get_content(), vec![X, C]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(100).get_content(), vec![C]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(400).get_content(), vec![C, D]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(500).get_content(), vec![D]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(900).get_content(), vec![C, M]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(1000).get_content(), vec![M]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(4000).get_content(), vec![M, M, M, M]);

    // bounds and beyond bounds
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(0).get_content(), Vec::<RD>::new());
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(5000).get_content(), Vec::<RD>::new());
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(5001).get_content(), Vec::<RD>::new());
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(9875).get_content(), Vec::<RD>::new());

    // random
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(2).get_content(), vec![I, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(8).get_content(), vec![V, I, I, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(25).get_content(), vec![X, X, V]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(44).get_content(), vec![X, L, I, V]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(76).get_content(), vec![L, X, X, V, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(237).get_content(), vec![C, C, X, X, X, V, I, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(412).get_content(), vec![C, D, X, I, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(555).get_content(), vec![D, L, V]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(777).get_content(), vec![D, C, C, L, X, X, V, I, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(1111).get_content(), vec![M, C, X, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(1234).get_content(), vec![M, C, C, X, X, X, I, V]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(1453).get_content(), vec![M, C, D, L, I, I, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(1877).get_content(), vec![M, D, C, C, C, L, X, X, V, I, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(1918).get_content(), vec![M, C, M, X, V, I, I, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(2020).get_content(), vec![M, M, X, X]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(2222).get_content(), vec![M, M, C, C, X, X, I, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(2394).get_content(), vec![M, M, C, C, C, X, C, I, V]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(2695).get_content(), vec![M, M, D, C, X, C, V]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(2800).get_content(), vec![M, M, D, C, C, C]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(3000).get_content(), vec![M, M, M]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(3333).get_content(), vec![M, M, M, C, C, C, X, X, X, I, I, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(3456).get_content(), vec![M, M, M, C, D, L, V, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(3879).get_content(), vec![M, M, M, D, C, C, C, L, X, X, I, X]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(3987).get_content(), vec![M, M, M, C, M, L, X, X, X, V, I, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(4166).get_content(), vec![M, M, M, M, C, L, X, V, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(4444).get_content(), vec![M, M, M, M, C, D, X, L, I, V]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(4888).get_content(), vec![M, M, M, M, D, C, C, C, L, X, X, X, V, I, I, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(4987).get_content(), vec![M, M, M, M, C, M, L, X, X, X, V, I, I]);
    assert_eq!(*rnum::convert_number_to_roman_numeral_using_hash(4999).get_content(), vec![M, M, M, M, C, M, X, C, I, X]);
}

#[test]
pub fn test_roman_numeral_to_number_converter() {
    let mut converter = rnum::RomanNumeralToNumberConverter::create();

    // building blocks
    assert_eq!(converter.convert(&RN::from_string("I")), 1);
    assert_eq!(converter.convert(&RN::from_string("IV")), 4);
    assert_eq!(converter.convert(&RN::from_string("V")), 5);
    assert_eq!(converter.convert(&RN::from_string("IX")), 9);
    assert_eq!(converter.convert(&RN::from_string("X")), 10);
    assert_eq!(converter.convert(&RN::from_string("XL")), 40);
    assert_eq!(converter.convert(&RN::from_string("L")), 50);
    assert_eq!(converter.convert(&RN::from_string("XC")), 90);
    assert_eq!(converter.convert(&RN::from_string("C")), 100);
    assert_eq!(converter.convert(&RN::from_string("CD")), 400);
    assert_eq!(converter.convert(&RN::from_string("D")), 500);
    assert_eq!(converter.convert(&RN::from_string("CM")), 900);
    assert_eq!(converter.convert(&RN::from_string("M")), 1000);
    assert_eq!(converter.convert(&RN::from_string("MMMM")), 4000);

    // random
    assert_eq!(converter.convert(&RN::from_string("II")), 2);
    assert_eq!(converter.convert(&RN::from_string("VIII")), 8);
    assert_eq!(converter.convert(&RN::from_string("XXV")), 25);
    assert_eq!(converter.convert(&RN::from_string("XLIV")), 44);
    assert_eq!(converter.convert(&RN::from_string("LXXVI")), 76);
    assert_eq!(converter.convert(&RN::from_string("CCXXXVII")), 237);
    assert_eq!(converter.convert(&RN::from_string("CDXII")), 412);
    assert_eq!(converter.convert(&RN::from_string("DLV")), 555);
    assert_eq!(converter.convert(&RN::from_string("DCCLXXVII")), 777);
    assert_eq!(converter.convert(&RN::from_string("MCXI")), 1111);
    assert_eq!(converter.convert(&RN::from_string("MCCXXXIV")), 1234);
    assert_eq!(converter.convert(&RN::from_string("MCDLIII")), 1453);
    assert_eq!(converter.convert(&RN::from_string("MDCCCLXXVII")), 1877);
    assert_eq!(converter.convert(&RN::from_string("MCMXVIII")), 1918);
    assert_eq!(converter.convert(&RN::from_string("MMXX")), 2020);
    assert_eq!(converter.convert(&RN::from_string("MMCCXXII")), 2222);
    assert_eq!(converter.convert(&RN::from_string("MMCCCXCIV")), 2394);
    assert_eq!(converter.convert(&RN::from_string("MMDCXCV")), 2695);
    assert_eq!(converter.convert(&RN::from_string("MMDCCC")), 2800);
    assert_eq!(converter.convert(&RN::from_string("MMM")), 3000);
    assert_eq!(converter.convert(&RN::from_string("MMMCCCXXXIII")), 3333);
    assert_eq!(converter.convert(&RN::from_string("MMMCDLVI")), 3456);
    assert_eq!(converter.convert(&RN::from_string("MMMDCCCLXXIX")), 3879);
    assert_eq!(converter.convert(&RN::from_string("MMMCMLXXXVII")), 3987);
    assert_eq!(converter.convert(&RN::from_string("MMMMCLXVI")), 4166);
    assert_eq!(converter.convert(&RN::from_string("MMMMCDXLIV")), 4444);
    assert_eq!(converter.convert(&RN::from_string("MMMMDCCCLXXXVIII")), 4888);
    assert_eq!(converter.convert(&RN::from_string("MMMMCMLXXXVII")), 4987);
    assert_eq!(converter.convert(&RN::from_string("MMMMCMXCIX")), 4999);

    // lower or mixed case numerals
    assert_eq!(converter.convert(&RN::from_string("MmDCclXxViI")), 2777);
    assert_eq!(converter.convert(&RN::from_string("dLv")), 555);
    assert_eq!(converter.convert(&RN::from_string("mdclxvi")), 1666);

    // error cases
    assert_eq!(converter.convert(&RN::from_string("MMMMMXCIX")), 0);
    assert_eq!(converter.convert(&RN::from_string("MDCCCCXVIII")), 0);
    assert_eq!(converter.convert(&RN::from_string("CLXXXX")), 0);
    assert_eq!(converter.convert(&RN::from_string("XVIIII")), 0);
    assert_eq!(converter.convert(&RN::from_string("DDCL")), 0);
    assert_eq!(converter.convert(&RN::from_string("LLX")), 0);
    assert_eq!(converter.convert(&RN::from_string("XVVI")), 0);
    assert_eq!(converter.convert(&RN::from_string("DM")), 0);
    assert_eq!(converter.convert(&RN::from_string("MLMV")), 0);
    assert_eq!(converter.convert(&RN::from_string("XMV")), 0);
    assert_eq!(converter.convert(&RN::from_string("CVMII")), 0);
    assert_eq!(converter.convert(&RN::from_string("IMII")), 0);
    assert_eq!(converter.convert(&RN::from_string("LCXX")), 0);
    assert_eq!(converter.convert(&RN::from_string("VCI")), 0);
    assert_eq!(converter.convert(&RN::from_string("ICII")), 0);
    assert_eq!(converter.convert(&RN::from_string("VXII")), 0);
    assert_eq!(converter.convert(&RN::from_string("MLDXII")), 0);
    assert_eq!(converter.convert(&RN::from_string("CXDII")), 0);
    assert_eq!(converter.convert(&RN::from_string("VDII")), 0);
    assert_eq!(converter.convert(&RN::from_string("IDI")), 0);
    assert_eq!(converter.convert(&RN::from_string("CVLII")), 0);
    assert_eq!(converter.convert(&RN::from_string("CCILI")), 0);
    assert_eq!(converter.convert(&RN::from_string("MCCM")), 0);
    assert_eq!(converter.convert(&RN::from_string("MCCDXX")), 0);
    assert_eq!(converter.convert(&RN::from_string("DXXCV")), 0);
    assert_eq!(converter.convert(&RN::from_string("XXLV")), 0);
    assert_eq!(converter.convert(&RN::from_string("XIIX")), 0);
    assert_eq!(converter.convert(&RN::from_string("IIV")), 0);
    assert_eq!(converter.convert(&RN::from_string("MMMNCMXCIX")), 0);
    assert_eq!(converter.convert(&RN::from_string("MMMMCM_CIX")), 0);
    assert_eq!(converter.convert(&RN::from_string("MMMMCMXC9X")), 0);
    assert_eq!(converter.convert(&RN::from_string("MMMMCM XCIX")), 0);
    assert_eq!(converter.convert(&RN::from_string("N")), 0);
    assert_eq!(converter.convert(&RN::from_string("2")), 0);
    assert_eq!(converter.convert(&RN::from_string("-")), 0);
    assert_eq!(converter.convert(&RN::from_string("")), 0);
}
