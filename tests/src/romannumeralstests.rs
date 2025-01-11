use learn_rust_lib::numbers::romannumerals as rnum;
use rnum::{
    MaxNumberExceededError, ParseRomanNumeralDigitsError, ParseRomanNumeralStringError,
    RomanDigit as RD, RomanNumeral as RN,
};
#[cfg(test)]
use std::str::FromStr;
use RD::{C, D, I, L, M, V, X};

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
pub fn test_roman_numeral_from_roman_digits() {
    // with try_from
    assert_eq!(
        *RN::try_from(vec![M, C, M, L, X, X, V, I, I])
            .unwrap()
            .get_content(),
        vec![M, C, M, L, X, X, V, I, I]
    );
    assert_eq!(
        *RN::try_from(vec![D, L, V]).unwrap().get_content(),
        vec![D, L, V]
    );
    assert_eq!(*RN::try_from(vec![X]).unwrap().get_content(), vec![X]);
    assert_eq!(
        RN::try_from(vec![I, V, M, C, D, L, C, X, M]),
        Err(ParseRomanNumeralDigitsError)
    );
    assert_eq!(
        RN::try_from(vec![M, C, M, L, X, X, V, I, V]),
        Err(ParseRomanNumeralDigitsError)
    );
    assert!(RN::try_from(Vec::new()).unwrap().empty());

    // with try_into
    let mut numeral: RN = vec![M, C, M, L, X, X, V, I, I].try_into().unwrap();
    assert_eq!(*numeral.get_content(), vec![M, C, M, L, X, X, V, I, I]);

    numeral = vec![D, L, V].try_into().unwrap();
    assert_eq!(*numeral.get_content(), vec![D, L, V]);

    numeral = vec![X].try_into().unwrap();
    assert_eq!(*numeral.get_content(), vec![X]);

    let mut numeral_error: Result<RN, ParseRomanNumeralDigitsError> =
        vec![I, V, M, C, D, L, C, X, M].try_into();
    assert_eq!(numeral_error, Err(ParseRomanNumeralDigitsError));

    numeral_error = vec![M, C, M, L, X, X, V, I, V].try_into();
    assert_eq!(numeral_error, Err(ParseRomanNumeralDigitsError));

    numeral = Vec::new().try_into().unwrap();
    assert!(numeral.empty());
}

#[test]
pub fn test_roman_numeral_to_roman_digits() {
    let mut content: Vec<RD> = RN::from_str("MCMLXXVII").unwrap().into();
    assert_eq!(content, vec![M, C, M, L, X, X, V, I, I]);

    content = RN::from_str("MmCcXxIi").unwrap().into();
    assert_eq!(content, vec![M, M, C, C, X, X, I, I]);

    content = RN::from_str("DlV").unwrap().into();
    assert_eq!(content, vec![D, L, V]);

    content = RN::from_str("dLV").unwrap().into();
    assert_eq!(content, vec![D, L, V]);

    content = RN::from_str("DLv").unwrap().into();
    assert_eq!(content, vec![D, L, V]);

    content = RN::from_str("X").unwrap().into();
    assert_eq!(content, vec![X]);

    content = RN::from_str("c").unwrap().into();
    assert_eq!(content, vec![C]);

    content = RN::from_str("").unwrap().into();
    assert_eq!(content, Vec::new());
}

#[test]
pub fn test_roman_numeral_from_str() {
    assert_eq!(
        *RN::from_str("MCMLXXVII").unwrap().get_content(),
        vec![M, C, M, L, X, X, V, I, I]
    );
    assert_eq!(
        *RN::from_str("MmCcXxIi").unwrap().get_content(),
        vec![M, M, C, C, X, X, I, I]
    );
    assert_eq!(*RN::from_str("DlV").unwrap().get_content(), vec![D, L, V]);
    assert_eq!(*RN::from_str("dLV").unwrap().get_content(), vec![D, L, V]);
    assert_eq!(*RN::from_str("DLv").unwrap().get_content(), vec![D, L, V]);
    assert_eq!(*RN::from_str("X").unwrap().get_content(), vec![X]);
    assert_eq!(*RN::from_str("c").unwrap().get_content(), vec![C]);
    assert_eq!(RN::from_str("IVMCDLCXM"), Err(ParseRomanNumeralStringError));
    assert_eq!(RN::from_str("MCMLXXVIV"), Err(ParseRomanNumeralStringError));
    assert_eq!(RN::from_str("MCNLXXVII"), Err(ParseRomanNumeralStringError));
    assert_eq!(RN::from_str("MC_LXXVII"), Err(ParseRomanNumeralStringError));
    assert_eq!(RN::from_str("MC LXXVII"), Err(ParseRomanNumeralStringError));
    assert_eq!(
        RN::from_str("MC\nLXXVII"),
        Err(ParseRomanNumeralStringError)
    );
    assert_eq!(RN::from_str("MC1LXXVII"), Err(ParseRomanNumeralStringError));
    assert_eq!(
        RN::from_str("MCMLXXVII "),
        Err(ParseRomanNumeralStringError)
    );
    assert_eq!(
        RN::from_str(" MCMLXXVII "),
        Err(ParseRomanNumeralStringError)
    );
    assert!(RN::from_str("").unwrap().empty());
}

#[test]
pub fn test_roman_numeral_to_string() {
    assert_eq!(
        RN::try_from(vec![M, C, M, L, X, X, V, I, I])
            .unwrap()
            .to_string(),
        "MCMLXXVII".to_string()
    );
    assert_eq!(
        RN::try_from(vec![D, L, V]).unwrap().to_string(),
        "DLV".to_string()
    );
    assert_eq!(RN::try_from(vec![X]).unwrap().to_string(), "X".to_string());
    assert!(RN::try_from(Vec::new()).unwrap().to_string().is_empty());
    assert!(RN::create().to_string().is_empty());
}

#[test]
pub fn test_roman_numeral_clone() {
    let numeral = RN::from_str("MCDXLIV").unwrap();
    let cloned_numeral = numeral.clone();

    assert_eq!(*numeral.get_content(), vec![M, C, D, X, L, I, V]);
    assert_eq!(*cloned_numeral.get_content(), vec![M, C, D, X, L, I, V]);

    let numeral = RN::create();
    let cloned_numeral = numeral.clone();

    assert!(numeral.empty());
    assert!(cloned_numeral.empty());
}

#[test]
pub fn test_roman_numeral_clear() {
    let mut numeral = RN::from_str("MCDXLIV").unwrap();
    numeral.clear();

    assert_eq!(numeral, RN::create());

    numeral = RN::create();
    numeral.clear();

    assert_eq!(numeral, RN::create());
}

#[test]
pub fn test_convert_number_to_roman_numeral() {
    let mut converter = rnum::NumberToRomanNumeralConverter::create();

    // building blocks
    assert_eq!(*converter.convert(1).unwrap().get_content(), vec![I]);
    assert_eq!(*converter.convert(4).unwrap().get_content(), vec![I, V]);
    assert_eq!(*converter.convert(5).unwrap().get_content(), vec![V]);
    assert_eq!(*converter.convert(9).unwrap().get_content(), vec![I, X]);
    assert_eq!(*converter.convert(10).unwrap().get_content(), vec![X]);
    assert_eq!(*converter.convert(40).unwrap().get_content(), vec![X, L]);
    assert_eq!(*converter.convert(50).unwrap().get_content(), vec![L]);
    assert_eq!(*converter.convert(90).unwrap().get_content(), vec![X, C]);
    assert_eq!(*converter.convert(100).unwrap().get_content(), vec![C]);
    assert_eq!(*converter.convert(400).unwrap().get_content(), vec![C, D]);
    assert_eq!(*converter.convert(500).unwrap().get_content(), vec![D]);
    assert_eq!(*converter.convert(900).unwrap().get_content(), vec![C, M]);
    assert_eq!(*converter.convert(1000).unwrap().get_content(), vec![M]);
    assert_eq!(
        *converter.convert(4000).unwrap().get_content(),
        vec![M, M, M, M]
    );

    // bounds and beyond bounds
    assert!(converter.convert(0).unwrap().empty());
    assert_eq!(converter.convert(5000), Err(MaxNumberExceededError));
    assert_eq!(converter.convert(5001), Err(MaxNumberExceededError));
    assert_eq!(converter.convert(9875), Err(MaxNumberExceededError));

    // random
    assert_eq!(*converter.convert(2).unwrap().get_content(), vec![I, I]);
    assert_eq!(
        *converter.convert(8).unwrap().get_content(),
        vec![V, I, I, I]
    );
    assert_eq!(*converter.convert(25).unwrap().get_content(), vec![X, X, V]);
    assert_eq!(
        *converter.convert(44).unwrap().get_content(),
        vec![X, L, I, V]
    );
    assert_eq!(
        *converter.convert(76).unwrap().get_content(),
        vec![L, X, X, V, I]
    );
    assert_eq!(
        *converter.convert(237).unwrap().get_content(),
        vec![C, C, X, X, X, V, I, I]
    );
    assert_eq!(
        *converter.convert(412).unwrap().get_content(),
        vec![C, D, X, I, I]
    );
    assert_eq!(
        *converter.convert(555).unwrap().get_content(),
        vec![D, L, V]
    );
    assert_eq!(
        *converter.convert(777).unwrap().get_content(),
        vec![D, C, C, L, X, X, V, I, I]
    );
    assert_eq!(
        *converter.convert(1111).unwrap().get_content(),
        vec![M, C, X, I]
    );
    assert_eq!(
        *converter.convert(1234).unwrap().get_content(),
        vec![M, C, C, X, X, X, I, V]
    );
    assert_eq!(
        *converter.convert(1453).unwrap().get_content(),
        vec![M, C, D, L, I, I, I]
    );
    assert_eq!(
        *converter.convert(1877).unwrap().get_content(),
        vec![M, D, C, C, C, L, X, X, V, I, I]
    );
    assert_eq!(
        *converter.convert(1918).unwrap().get_content(),
        vec![M, C, M, X, V, I, I, I]
    );
    assert_eq!(
        *converter.convert(2020).unwrap().get_content(),
        vec![M, M, X, X]
    );
    assert_eq!(
        *converter.convert(2222).unwrap().get_content(),
        vec![M, M, C, C, X, X, I, I]
    );
    assert_eq!(
        *converter.convert(2394).unwrap().get_content(),
        vec![M, M, C, C, C, X, C, I, V]
    );
    assert_eq!(
        *converter.convert(2695).unwrap().get_content(),
        vec![M, M, D, C, X, C, V]
    );
    assert_eq!(
        *converter.convert(2800).unwrap().get_content(),
        vec![M, M, D, C, C, C]
    );
    assert_eq!(
        *converter.convert(3000).unwrap().get_content(),
        vec![M, M, M]
    );
    assert_eq!(
        *converter.convert(3333).unwrap().get_content(),
        vec![M, M, M, C, C, C, X, X, X, I, I, I]
    );
    assert_eq!(
        *converter.convert(3456).unwrap().get_content(),
        vec![M, M, M, C, D, L, V, I]
    );
    assert_eq!(
        *converter.convert(3879).unwrap().get_content(),
        vec![M, M, M, D, C, C, C, L, X, X, I, X]
    );
    assert_eq!(
        *converter.convert(3987).unwrap().get_content(),
        vec![M, M, M, C, M, L, X, X, X, V, I, I]
    );
    assert_eq!(
        *converter.convert(4166).unwrap().get_content(),
        vec![M, M, M, M, C, L, X, V, I]
    );
    assert_eq!(
        *converter.convert(4444).unwrap().get_content(),
        vec![M, M, M, M, C, D, X, L, I, V]
    );
    assert_eq!(
        *converter.convert(4888).unwrap().get_content(),
        vec![M, M, M, M, D, C, C, C, L, X, X, X, V, I, I, I]
    );
    assert_eq!(
        *converter.convert(4987).unwrap().get_content(),
        vec![M, M, M, M, C, M, L, X, X, X, V, I, I]
    );
    assert_eq!(
        *converter.convert(4999).unwrap().get_content(),
        vec![M, M, M, M, C, M, X, C, I, X]
    );
}

#[test]
pub fn test_convert_number_to_roman_numeral_using_hash() {
    // building blocks
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(1)
            .unwrap()
            .get_content(),
        vec![I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(4)
            .unwrap()
            .get_content(),
        vec![I, V]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(5)
            .unwrap()
            .get_content(),
        vec![V]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(9)
            .unwrap()
            .get_content(),
        vec![I, X]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(10)
            .unwrap()
            .get_content(),
        vec![X]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(40)
            .unwrap()
            .get_content(),
        vec![X, L]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(50)
            .unwrap()
            .get_content(),
        vec![L]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(90)
            .unwrap()
            .get_content(),
        vec![X, C]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(100)
            .unwrap()
            .get_content(),
        vec![C]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(400)
            .unwrap()
            .get_content(),
        vec![C, D]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(500)
            .unwrap()
            .get_content(),
        vec![D]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(900)
            .unwrap()
            .get_content(),
        vec![C, M]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(1000)
            .unwrap()
            .get_content(),
        vec![M]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(4000)
            .unwrap()
            .get_content(),
        vec![M, M, M, M]
    );

    // bounds and beyond bounds
    assert!(rnum::convert_number_to_roman_numeral_using_hash(0)
        .unwrap()
        .empty());
    assert_eq!(
        rnum::convert_number_to_roman_numeral_using_hash(5000),
        Err(MaxNumberExceededError)
    );
    assert_eq!(
        rnum::convert_number_to_roman_numeral_using_hash(5001),
        Err(MaxNumberExceededError)
    );
    assert_eq!(
        rnum::convert_number_to_roman_numeral_using_hash(9875),
        Err(MaxNumberExceededError)
    );

    // random
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(2)
            .unwrap()
            .get_content(),
        vec![I, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(8)
            .unwrap()
            .get_content(),
        vec![V, I, I, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(25)
            .unwrap()
            .get_content(),
        vec![X, X, V]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(44)
            .unwrap()
            .get_content(),
        vec![X, L, I, V]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(76)
            .unwrap()
            .get_content(),
        vec![L, X, X, V, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(237)
            .unwrap()
            .get_content(),
        vec![C, C, X, X, X, V, I, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(412)
            .unwrap()
            .get_content(),
        vec![C, D, X, I, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(555)
            .unwrap()
            .get_content(),
        vec![D, L, V]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(777)
            .unwrap()
            .get_content(),
        vec![D, C, C, L, X, X, V, I, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(1111)
            .unwrap()
            .get_content(),
        vec![M, C, X, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(1234)
            .unwrap()
            .get_content(),
        vec![M, C, C, X, X, X, I, V]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(1453)
            .unwrap()
            .get_content(),
        vec![M, C, D, L, I, I, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(1877)
            .unwrap()
            .get_content(),
        vec![M, D, C, C, C, L, X, X, V, I, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(1918)
            .unwrap()
            .get_content(),
        vec![M, C, M, X, V, I, I, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(2020)
            .unwrap()
            .get_content(),
        vec![M, M, X, X]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(2222)
            .unwrap()
            .get_content(),
        vec![M, M, C, C, X, X, I, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(2394)
            .unwrap()
            .get_content(),
        vec![M, M, C, C, C, X, C, I, V]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(2695)
            .unwrap()
            .get_content(),
        vec![M, M, D, C, X, C, V]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(2800)
            .unwrap()
            .get_content(),
        vec![M, M, D, C, C, C]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(3000)
            .unwrap()
            .get_content(),
        vec![M, M, M]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(3333)
            .unwrap()
            .get_content(),
        vec![M, M, M, C, C, C, X, X, X, I, I, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(3456)
            .unwrap()
            .get_content(),
        vec![M, M, M, C, D, L, V, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(3879)
            .unwrap()
            .get_content(),
        vec![M, M, M, D, C, C, C, L, X, X, I, X]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(3987)
            .unwrap()
            .get_content(),
        vec![M, M, M, C, M, L, X, X, X, V, I, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(4166)
            .unwrap()
            .get_content(),
        vec![M, M, M, M, C, L, X, V, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(4444)
            .unwrap()
            .get_content(),
        vec![M, M, M, M, C, D, X, L, I, V]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(4888)
            .unwrap()
            .get_content(),
        vec![M, M, M, M, D, C, C, C, L, X, X, X, V, I, I, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(4987)
            .unwrap()
            .get_content(),
        vec![M, M, M, M, C, M, L, X, X, X, V, I, I]
    );
    assert_eq!(
        *rnum::convert_number_to_roman_numeral_using_hash(4999)
            .unwrap()
            .get_content(),
        vec![M, M, M, M, C, M, X, C, I, X]
    );
}

#[test]
pub fn test_roman_numeral_to_number_converter() {
    let mut converter = rnum::RomanNumeralToNumberConverter::create();

    // building blocks
    assert_eq!(converter.convert(&RN::from_str("I").unwrap()), 1);
    assert_eq!(converter.convert(&RN::from_str("IV").unwrap()), 4);
    assert_eq!(converter.convert(&RN::from_str("V").unwrap()), 5);
    assert_eq!(converter.convert(&RN::from_str("IX").unwrap()), 9);
    assert_eq!(converter.convert(&RN::from_str("X").unwrap()), 10);
    assert_eq!(converter.convert(&RN::from_str("XL").unwrap()), 40);
    assert_eq!(converter.convert(&RN::from_str("L").unwrap()), 50);
    assert_eq!(converter.convert(&RN::from_str("XC").unwrap()), 90);
    assert_eq!(converter.convert(&RN::from_str("C").unwrap()), 100);
    assert_eq!(converter.convert(&RN::from_str("CD").unwrap()), 400);
    assert_eq!(converter.convert(&RN::from_str("D").unwrap()), 500);
    assert_eq!(converter.convert(&RN::from_str("CM").unwrap()), 900);
    assert_eq!(converter.convert(&RN::from_str("M").unwrap()), 1000);
    assert_eq!(converter.convert(&RN::from_str("MMMM").unwrap()), 4000);

    // random
    assert_eq!(converter.convert(&RN::from_str("II").unwrap()), 2);
    assert_eq!(converter.convert(&RN::from_str("VIII").unwrap()), 8);
    assert_eq!(converter.convert(&RN::from_str("XXV").unwrap()), 25);
    assert_eq!(converter.convert(&RN::from_str("XLIV").unwrap()), 44);
    assert_eq!(converter.convert(&RN::from_str("LXXVI").unwrap()), 76);
    assert_eq!(converter.convert(&RN::from_str("CCXXXVII").unwrap()), 237);
    assert_eq!(converter.convert(&RN::from_str("CDXII").unwrap()), 412);
    assert_eq!(converter.convert(&RN::from_str("DLV").unwrap()), 555);
    assert_eq!(converter.convert(&RN::from_str("DCCLXXVII").unwrap()), 777);
    assert_eq!(converter.convert(&RN::from_str("MCXI").unwrap()), 1111);
    assert_eq!(converter.convert(&RN::from_str("MCCXXXIV").unwrap()), 1234);
    assert_eq!(converter.convert(&RN::from_str("MCDLIII").unwrap()), 1453);
    assert_eq!(
        converter.convert(&RN::from_str("MDCCCLXXVII").unwrap()),
        1877
    );
    assert_eq!(converter.convert(&RN::from_str("MCMXVIII").unwrap()), 1918);
    assert_eq!(converter.convert(&RN::from_str("MMXX").unwrap()), 2020);
    assert_eq!(converter.convert(&RN::from_str("MMCCXXII").unwrap()), 2222);
    assert_eq!(converter.convert(&RN::from_str("MMCCCXCIV").unwrap()), 2394);
    assert_eq!(converter.convert(&RN::from_str("MMDCXCV").unwrap()), 2695);
    assert_eq!(converter.convert(&RN::from_str("MMDCCC").unwrap()), 2800);
    assert_eq!(converter.convert(&RN::from_str("MMM").unwrap()), 3000);
    assert_eq!(
        converter.convert(&RN::from_str("MMMCCCXXXIII").unwrap()),
        3333
    );
    assert_eq!(converter.convert(&RN::from_str("MMMCDLVI").unwrap()), 3456);
    assert_eq!(
        converter.convert(&RN::from_str("MMMDCCCLXXIX").unwrap()),
        3879
    );
    assert_eq!(
        converter.convert(&RN::from_str("MMMCMLXXXVII").unwrap()),
        3987
    );
    assert_eq!(converter.convert(&RN::from_str("MMMMCLXVI").unwrap()), 4166);
    assert_eq!(
        converter.convert(&RN::from_str("MMMMCDXLIV").unwrap()),
        4444
    );
    assert_eq!(
        converter.convert(&RN::from_str("MMMMDCCCLXXXVIII").unwrap()),
        4888
    );
    assert_eq!(
        converter.convert(&RN::from_str("MMMMCMLXXXVII").unwrap()),
        4987
    );
    assert_eq!(
        converter.convert(&RN::from_str("MMMMCMXCIX").unwrap()),
        4999
    );

    // lower or mixed case numerals
    assert_eq!(
        converter.convert(&RN::from_str("MmDCclXxViI").unwrap()),
        2777
    );
    assert_eq!(converter.convert(&RN::from_str("dLv").unwrap()), 555);
    assert_eq!(converter.convert(&RN::from_str("mdclxvi").unwrap()), 1666);

    // empty numeral
    assert_eq!(converter.convert(&RN::create()), 0);
}
