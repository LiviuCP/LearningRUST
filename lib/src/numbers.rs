pub fn reverse_int(input:(u64, u8)) -> (u64, u8) {
    let mut input_number = input.0;
    let mut output_number = 0;
    let mut output_zeroes_count:u8 = 0;
    let mut non_zero_digits_found = false;

    while input_number > 0 {
        let digit = input_number % 10;
        output_number = output_number * 10 + digit;
	input_number = input_number / 10;

        if digit == 0 {
            if !non_zero_digits_found {
	        output_zeroes_count += 1;
	    }
	}
	else {
	    if !non_zero_digits_found {
	       non_zero_digits_found = true;
	    }
	}
    }

    if output_number == 0 {
       output_zeroes_count = input.1;
    }
    else {
       output_number *= u64::pow(10, input.1 as u32);
    }

    return (output_number, output_zeroes_count);
}
