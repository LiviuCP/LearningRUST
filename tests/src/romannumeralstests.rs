#[cfg(test)]
use learn_rust_lib::numbers::romannumerals as rnum;

#[test]
pub fn test_convert_number_to_roman_numeral() {
    let mut converter = rnum::NumberToRomanNumeralConverter::create();

    // building blocks
    assert_eq!(*(converter.convert(1)), vec!['I']);
    assert_eq!(*(converter.convert(4)), vec!['I', 'V']);
    assert_eq!(*(converter.convert(5)), vec!['V']);
    assert_eq!(*(converter.convert(9)), vec!['I', 'X']);
    assert_eq!(*(converter.convert(10)), vec!['X']);
    assert_eq!(*(converter.convert(40)), vec!['X', 'L']);
    assert_eq!(*(converter.convert(50)), vec!['L']);
    assert_eq!(*(converter.convert(90)), vec!['X', 'C']);
    assert_eq!(*(converter.convert(100)), vec!['C']);
    assert_eq!(*(converter.convert(400)), vec!['C', 'D']);
    assert_eq!(*(converter.convert(500)), vec!['D']);
    assert_eq!(*(converter.convert(900)), vec!['C', 'M']);
    assert_eq!(*(converter.convert(1000)), vec!['M']);
    assert_eq!(*(converter.convert(4000)), vec!['M', 'M', 'M', 'M']);

    // bounds and beyond bounds
    assert_eq!(*(converter.convert(0)), Vec::<char>::new());
    assert_eq!(*(converter.convert(5000)), Vec::<char>::new());
    assert_eq!(*(converter.convert(5001)), Vec::<char>::new());
    assert_eq!(*(converter.convert(9875)), Vec::<char>::new());

    // random
    assert_eq!(*(converter.convert(2)), vec!['I', 'I']);
    assert_eq!(*(converter.convert(8)), vec!['V', 'I', 'I', 'I']);
    assert_eq!(*(converter.convert(25)), vec!['X', 'X', 'V']);
    assert_eq!(*(converter.convert(44)), vec!['X', 'L', 'I', 'V']);
    assert_eq!(*(converter.convert(76)), vec!['L', 'X', 'X', 'V', 'I']);
    assert_eq!(*(converter.convert(237)), vec!['C', 'C', 'X', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(*(converter.convert(412)), vec!['C', 'D', 'X', 'I', 'I']);
    assert_eq!(*(converter.convert(555)), vec!['D', 'L', 'V']);
    assert_eq!(*(converter.convert(777)), vec!['D', 'C', 'C', 'L', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(*(converter.convert(1111)), vec!['M', 'C', 'X', 'I']);
    assert_eq!(*(converter.convert(1234)), vec!['M', 'C', 'C', 'X', 'X', 'X', 'I', 'V']);
    assert_eq!(*(converter.convert(1453)), vec!['M', 'C', 'D', 'L', 'I', 'I', 'I']);
    assert_eq!(*(converter.convert(1877)), vec!['M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(*(converter.convert(1918)), vec!['M', 'C', 'M', 'X', 'V', 'I', 'I', 'I']);
    assert_eq!(*(converter.convert(2020)), vec!['M', 'M', 'X', 'X']);
    assert_eq!(*(converter.convert(2222)), vec!['M', 'M', 'C', 'C', 'X', 'X', 'I', 'I']);
    assert_eq!(*(converter.convert(2394)), vec!['M', 'M', 'C', 'C', 'C', 'X', 'C', 'I', 'V']);
    assert_eq!(*(converter.convert(2695)), vec!['M', 'M', 'D', 'C', 'X', 'C', 'V']);
    assert_eq!(*(converter.convert(2800)), vec!['M', 'M', 'D', 'C', 'C', 'C']);
    assert_eq!(*(converter.convert(3000)), vec!['M', 'M', 'M']);
    assert_eq!(*(converter.convert(3333)), vec!['M', 'M', 'M', 'C', 'C', 'C', 'X', 'X', 'X', 'I', 'I', 'I']);
    assert_eq!(*(converter.convert(3456)), vec!['M', 'M', 'M', 'C', 'D', 'L', 'V', 'I']);
    assert_eq!(*(converter.convert(3879)), vec!['M', 'M', 'M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'I', 'X']);
    assert_eq!(*(converter.convert(3987)), vec!['M', 'M', 'M', 'C', 'M', 'L', 'X', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(*(converter.convert(4166)), vec!['M', 'M', 'M', 'M', 'C', 'L', 'X', 'V', 'I']);
    assert_eq!(*(converter.convert(4444)), vec!['M', 'M', 'M', 'M', 'C', 'D', 'X', 'L', 'I', 'V']);
    assert_eq!(*(converter.convert(4888)), vec!['M', 'M', 'M', 'M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'X', 'V', 'I', 'I', 'I']);
    assert_eq!(*(converter.convert(4987)), vec!['M', 'M', 'M', 'M', 'C', 'M', 'L', 'X', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(*(converter.convert(4999)), vec!['M', 'M', 'M', 'M', 'C', 'M', 'X', 'C', 'I', 'X']);
}

#[test]
pub fn test_convert_number_to_roman_numeral_using_hash() {
    // building blocks
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(1), vec!['I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(4), vec!['I', 'V']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(5), vec!['V']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(9), vec!['I', 'X']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(10), vec!['X']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(40), vec!['X', 'L']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(50), vec!['L']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(90), vec!['X', 'C']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(100), vec!['C']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(400), vec!['C', 'D']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(500), vec!['D']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(900), vec!['C', 'M']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(1000), vec!['M']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(4000), vec!['M', 'M', 'M', 'M']);

    // bounds and beyond bounds
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(0), Vec::<char>::new());
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(5000), Vec::<char>::new());
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(5001), Vec::<char>::new());
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(9875), Vec::<char>::new());

    // random
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(2), vec!['I', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(8), vec!['V', 'I', 'I', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(25), vec!['X', 'X', 'V']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(44), vec!['X', 'L', 'I', 'V']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(76), vec!['L', 'X', 'X', 'V', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(237), vec!['C', 'C', 'X', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(412), vec!['C', 'D', 'X', 'I', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(555), vec!['D', 'L', 'V']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(777), vec!['D', 'C', 'C', 'L', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(1111), vec!['M', 'C', 'X', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(1234), vec!['M', 'C', 'C', 'X', 'X', 'X', 'I', 'V']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(1453), vec!['M', 'C', 'D', 'L', 'I', 'I', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(1877), vec!['M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(1918), vec!['M', 'C', 'M', 'X', 'V', 'I', 'I', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(2020), vec!['M', 'M', 'X', 'X']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(2222), vec!['M', 'M', 'C', 'C', 'X', 'X', 'I', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(2394), vec!['M', 'M', 'C', 'C', 'C', 'X', 'C', 'I', 'V']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(2695), vec!['M', 'M', 'D', 'C', 'X', 'C', 'V']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(2800), vec!['M', 'M', 'D', 'C', 'C', 'C']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(3000), vec!['M', 'M', 'M']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(3333), vec!['M', 'M', 'M', 'C', 'C', 'C', 'X', 'X', 'X', 'I', 'I', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(3456), vec!['M', 'M', 'M', 'C', 'D', 'L', 'V', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(3879), vec!['M', 'M', 'M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'I', 'X']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(3987), vec!['M', 'M', 'M', 'C', 'M', 'L', 'X', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(4166), vec!['M', 'M', 'M', 'M', 'C', 'L', 'X', 'V', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(4444), vec!['M', 'M', 'M', 'M', 'C', 'D', 'X', 'L', 'I', 'V']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(4888), vec!['M', 'M', 'M', 'M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'X', 'V', 'I', 'I', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(4987), vec!['M', 'M', 'M', 'M', 'C', 'M', 'L', 'X', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(rnum::convert_number_to_roman_numeral_using_hash(4999), vec!['M', 'M', 'M', 'M', 'C', 'M', 'X', 'C', 'I', 'X']);
}

#[test]
pub fn test_roman_numeral_to_number_converter() {
    let mut converter = rnum::RomanNumeralToNumberConverter::create();

    // building blocks
    assert_eq!(converter.convert(&vec!['I']), 1);
    assert_eq!(converter.convert(&vec!['I', 'V']), 4);
    assert_eq!(converter.convert(&vec!['V']), 5);
    assert_eq!(converter.convert(&vec!['I', 'X']), 9);
    assert_eq!(converter.convert(&vec!['X']), 10);
    assert_eq!(converter.convert(&vec!['X', 'L']), 40);
    assert_eq!(converter.convert(&vec!['L']), 50);
    assert_eq!(converter.convert(&vec!['X', 'C']), 90);
    assert_eq!(converter.convert(&vec!['C']), 100);
    assert_eq!(converter.convert(&vec!['C', 'D']), 400);
    assert_eq!(converter.convert(&vec!['D']), 500);
    assert_eq!(converter.convert(&vec!['C', 'M']), 900);
    assert_eq!(converter.convert(&vec!['M']), 1000);
    assert_eq!(converter.convert(&vec!['M', 'M', 'M', 'M']), 4000);

    // random
    assert_eq!(converter.convert(&vec!['I', 'I']), 2);
    assert_eq!(converter.convert(&vec!['V', 'I', 'I', 'I']), 8);
    assert_eq!(converter.convert(&vec!['X', 'X', 'V']), 25);
    assert_eq!(converter.convert(&vec!['X', 'L', 'I', 'V']), 44);
    assert_eq!(converter.convert(&vec!['L', 'X', 'X', 'V', 'I']), 76);
    assert_eq!(converter.convert(&vec!['C', 'C', 'X', 'X', 'X', 'V', 'I', 'I']), 237);
    assert_eq!(converter.convert(&vec!['C', 'D', 'X', 'I', 'I']), 412);
    assert_eq!(converter.convert(&vec!['D', 'L', 'V']), 555);
    assert_eq!(converter.convert(&vec!['D', 'C', 'C', 'L', 'X', 'X', 'V', 'I', 'I']), 777);
    assert_eq!(converter.convert(&vec!['M', 'C', 'X', 'I']), 1111);
    assert_eq!(converter.convert(&vec!['M', 'C', 'C', 'X', 'X', 'X', 'I', 'V']), 1234);
    assert_eq!(converter.convert(&vec!['M', 'C', 'D', 'L', 'I', 'I', 'I']), 1453);
    assert_eq!(converter.convert(&vec!['M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'V', 'I', 'I']), 1877);
    assert_eq!(converter.convert(&vec!['M', 'C', 'M', 'X', 'V', 'I', 'I', 'I']), 1918);
    assert_eq!(converter.convert(&vec!['M', 'M', 'X', 'X']), 2020);
    assert_eq!(converter.convert(&vec!['M', 'M', 'C', 'C', 'X', 'X', 'I', 'I']), 2222);
    assert_eq!(converter.convert(&vec!['M', 'M', 'C', 'C', 'C', 'X', 'C', 'I', 'V']), 2394);
    assert_eq!(converter.convert(&vec!['M', 'M', 'D', 'C', 'X', 'C', 'V']), 2695);
    assert_eq!(converter.convert(&vec!['M', 'M', 'D', 'C', 'C', 'C']), 2800);
    assert_eq!(converter.convert(&vec!['M', 'M', 'M']), 3000);
    assert_eq!(converter.convert(&vec!['M', 'M', 'M', 'C', 'C', 'C', 'X', 'X', 'X', 'I', 'I', 'I']), 3333);
    assert_eq!(converter.convert(&vec!['M', 'M', 'M', 'C', 'D', 'L', 'V', 'I']), 3456);
    assert_eq!(converter.convert(&vec!['M', 'M', 'M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'I', 'X']), 3879);
    assert_eq!(converter.convert(&vec!['M', 'M', 'M', 'C', 'M', 'L', 'X', 'X', 'X', 'V', 'I', 'I']), 3987);
    assert_eq!(converter.convert(&vec!['M', 'M', 'M', 'M', 'C', 'L', 'X', 'V', 'I']), 4166);
    assert_eq!(converter.convert(&vec!['M', 'M', 'M', 'M', 'C', 'D', 'X', 'L', 'I', 'V']), 4444);
    assert_eq!(converter.convert(&vec!['M', 'M', 'M', 'M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'X', 'V', 'I', 'I', 'I']), 4888);
    assert_eq!(converter.convert(&vec!['M', 'M', 'M', 'M', 'C', 'M', 'L', 'X', 'X', 'X', 'V', 'I', 'I']), 4987);
    assert_eq!(converter.convert(&vec!['M', 'M', 'M', 'M', 'C', 'M', 'X', 'C', 'I', 'X']), 4999);

    // lower or mixed case numerals
    assert_eq!(converter.convert(&vec!['M', 'm', 'D', 'C', 'c', 'l', 'X', 'x', 'V', 'i', 'I']), 2777);
    assert_eq!(converter.convert(&vec!['d', 'L', 'v']), 555);
    assert_eq!(converter.convert(&vec!['m', 'd', 'c', 'l', 'x', 'v', 'i']), 1666);

    // error cases
    assert_eq!(converter.convert(&vec!['M', 'M', 'M', 'M', 'M', 'X', 'C', 'I', 'X']), 0);
    assert_eq!(converter.convert(&vec!['M', 'D', 'C', 'C', 'C', 'C', 'X', 'V', 'I', 'I', 'I']), 0);
    assert_eq!(converter.convert(&vec!['C', 'L', 'X', 'X', 'X', 'X']), 0);
    assert_eq!(converter.convert(&vec!['X', 'V', 'I', 'I', 'I', 'I']), 0);
    assert_eq!(converter.convert(&vec!['D', 'D', 'C', 'L']), 0);
    assert_eq!(converter.convert(&vec!['L', 'L', 'X']), 0);
    assert_eq!(converter.convert(&vec!['X', 'V', 'V', 'I']), 0);
    assert_eq!(converter.convert(&vec!['D', 'M']), 0);
    assert_eq!(converter.convert(&vec!['M', 'L', 'M', 'V']), 0);
    assert_eq!(converter.convert(&vec!['X', 'M', 'V']), 0);
    assert_eq!(converter.convert(&vec!['C', 'V', 'M', 'I', 'I']), 0);
    assert_eq!(converter.convert(&vec!['I', 'M', 'I', 'I']), 0);
    assert_eq!(converter.convert(&vec!['L', 'C', 'X', 'X']), 0);
    assert_eq!(converter.convert(&vec!['V', 'C', 'I']), 0);
    assert_eq!(converter.convert(&vec!['I', 'C', 'I', 'I']), 0);
    assert_eq!(converter.convert(&vec!['V', 'X', 'I', 'I']), 0);
    assert_eq!(converter.convert(&vec!['M', 'L', 'D', 'X', 'I', 'I']), 0);
    assert_eq!(converter.convert(&vec!['C', 'X', 'D', 'I', 'I']), 0);
    assert_eq!(converter.convert(&vec!['V', 'D', 'I', 'I']), 0);
    assert_eq!(converter.convert(&vec!['I', 'D', 'I']), 0);
    assert_eq!(converter.convert(&vec!['C', 'V', 'L', 'I', 'I']), 0);
    assert_eq!(converter.convert(&vec!['C', 'C', 'I', 'L', 'I']), 0);
    assert_eq!(converter.convert(&vec!['M', 'C', 'C', 'M']), 0);
    assert_eq!(converter.convert(&vec!['M', 'C', 'C', 'D', 'X', 'X']), 0);
    assert_eq!(converter.convert(&vec!['D', 'X', 'X', 'C', 'V']), 0);
    assert_eq!(converter.convert(&vec!['X', 'X', 'L', 'V']), 0);
    assert_eq!(converter.convert(&vec!['X', 'I', 'I', 'X']), 0);
    assert_eq!(converter.convert(&vec!['I', 'I', 'V']), 0);
    assert_eq!(converter.convert(&vec!['M', 'M', 'M', 'N', 'C', 'M', 'X', 'C', 'I', 'X']), 0);
    assert_eq!(converter.convert(&vec!['M', 'M', 'M', 'M', 'C', 'M', '_', 'C', 'I', 'X']), 0);
    assert_eq!(converter.convert(&vec!['M', 'M', 'M', 'M', 'C', 'M', 'X', 'C', '9', 'X']), 0);
    assert_eq!(converter.convert(&vec!['M', 'M', 'M', 'M', 'C', 'M', ' ', 'X', 'C', 'I', 'X']), 0);
    assert_eq!(converter.convert(&vec!['N']), 0);
    assert_eq!(converter.convert(&vec!['2']), 0);
    assert_eq!(converter.convert(&vec!['-']), 0);
    assert_eq!(converter.convert(&Vec::<char>::new()), 0);
}
