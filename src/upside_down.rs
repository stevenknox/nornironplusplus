macro_rules! upside_down {
	($($rightside:literal, $upside:literal),*) => {
		pub fn rightside_up(ch: char) -> char {
			match ch {
				$(
                    $upside => $rightside,
				)*
                _ => ch
			}
		}
		pub fn upside_down(ch: char) -> char {
			match ch {
				$(
                    $rightside => $upside,
				)*
                _ => ch
			}
		}
	};
}

upside_down!(
    'a', 'ɐ', 'b', 'q', 'c', 'ɔ', 'd', 'p', 'e', 'ǝ', 'f', 'ɟ', 'g', 'ƃ', 'h', 'ɥ', 'i', 'ᴉ', 'j',
    'ɾ', 'k', 'ʞ', 'l', 'l', 'm', 'ɯ', 'n', 'u', 'p', 'd', 'q', 'b', 'r', 'ɹ', 's', 's', 't', 'ʇ',
    'u', 'n', 'v', 'ʌ', 'w', 'ʍ', 'x', 'x', 'y', 'ʎ', 'z', 'z', 'A', '∀', 'B', 'q', 'C', 'Ɔ', 'D',
    'p', 'E', 'Ǝ', 'F', 'Ⅎ', 'G', 'פ', 'I', 'I', 'F', 'ſ', 'K', 'ʞ', 'L', '˥', 'M', 'W', 'N', 'N',
    'O', 'O', 'P', 'Ԁ', 'Q', 'Q', 'R', 'ɹ', 'S', 'S', 'T', '┴', 'U', '∩', 'V', 'Λ', 'W', 'M', 'X',
    'X', 'Y', '⅄', 'Z', 'Z', '0', '0', '1', 'Ɩ', '2', 'ᄅ', '3', 'Ɛ', '4', 'ㄣ', '5', 'ϛ', '6',
    '9', '7', 'ㄥ', '8', '8', '9', '6', '(', ')', ')', '(', '<', '>', '>', '<'
);
