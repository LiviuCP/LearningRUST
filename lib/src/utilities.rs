use std::io;

pub fn clear_screen() {
    std::process::Command::new("clear").status().unwrap();
}

pub fn read_char(ch: &mut char) -> bool {
    let mut buffer = String::new();

    io::stdin()
	.read_line(&mut buffer)
	.expect("Failed reading the line!");

    let mut is_buffer_empty = buffer.len() == 0;

    if !is_buffer_empty {
	let first_char = buffer.chars().nth(0).unwrap();

	if first_char != '\n' {
            *ch = first_char;
	}
	else {
            is_buffer_empty = true;
	}
    }

    !is_buffer_empty
}

pub fn convert_char_to_lowercase(input_char: &mut char) {
    for ch in input_char.to_lowercase() {
	*input_char = ch;
	break;
    }
}
