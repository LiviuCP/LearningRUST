#[cfg(test)]
use learn_rust_lib::strings;

#[test]
pub fn test_convert_to_pig_latin() {
    assert_eq!(strings::convert_to_pig_latin(&"grapefruit".to_string()), "rapefruit-gay".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"banana".to_string()), "anana-bay".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"sCientist".to_string()), "cientist-say".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"abracadabra".to_string()), "abracadabra-hay".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"ananaS".to_string()), "ananas-hay".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"Avalon".to_string()), "avalon-hay".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"SUV".to_string()), "uv-say".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"bee".to_string()), "ee-bay".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"ea".to_string()), "ea-hay".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"bo".to_string()), "o-bay".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"uv".to_string()), "uv-hay".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"a".to_string()), "a-hay".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"b".to_string()), "".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"".to_string()), "".to_string());
    assert_eq!(strings::convert_to_pig_latin(&" ".to_string()), "".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"ananas ".to_string()), "".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"banana ".to_string()), "".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"ava lon".to_string()), "".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"word1".to_string()), "".to_string());
    assert_eq!(strings::convert_to_pig_latin(&"word-y".to_string()), "".to_string());
}