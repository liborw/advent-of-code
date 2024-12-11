use itertools::Itertools;




// Helper function to validate Roman numeral syntax
fn is_valid_roman(roman: &str) -> bool {
    if roman.is_empty() { return false; };
    let valid_pattern = regex::Regex::new( r"^M{0,3}(CM|CD|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3})$").unwrap();
    valid_pattern.is_match(roman)
}

pub fn roman_to_int(roman: &str) -> Result<usize, String> {

    // Validate the Roman numeral syntax
    if !is_valid_roman(roman) {
        return Err("Invalid Roman numeral syntax.".to_string());
    }

    let mut values = roman.chars()
        .map(|c| {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                v   => panic!("{v} is not a roman number!")
            }
        });

    let mut result: isize = 0;
    let mut current = values.next();
    while let Some(a) = current {
        let next = values.next();
        if let Some(b) = next {
            result += if a < b {-a} else {a};
        } else {
            result += a
        }
        current = next;
    }

    Ok(result as usize)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_roman_numerals() {
        assert_eq!(roman_to_int("I").unwrap(), 1);
        assert_eq!(roman_to_int("IV").unwrap(), 4);
        assert_eq!(roman_to_int("IX").unwrap(), 9);
        assert_eq!(roman_to_int("XL").unwrap(), 40);
        assert_eq!(roman_to_int("XC").unwrap(), 90);
        assert_eq!(roman_to_int("CD").unwrap(), 400);
        assert_eq!(roman_to_int("CM").unwrap(), 900);
        assert_eq!(roman_to_int("MCMXCIV").unwrap(), 1994);
        assert_eq!(roman_to_int("MMXXIV").unwrap(), 2024);
        assert_eq!(roman_to_int("MMMCMXCIX").unwrap(), 3999); // Largest possible Roman numeral
    }

    #[test]
    fn test_invalid_roman_numerals() {
        assert!(roman_to_int("IIII").is_err()); // Invalid repetition
        assert!(roman_to_int("VV").is_err());   // Invalid repetition
        assert!(roman_to_int("IC").is_err());   // Invalid subtractive combination
        assert!(roman_to_int("VX").is_err());   // Invalid subtractive combination
        assert!(roman_to_int("ABC").is_err());  // Invalid characters
        assert!(roman_to_int("").is_err());     // Empty string
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(roman_to_int("MMM").unwrap(), 3000); // Only additive numerals
        assert_eq!(roman_to_int("D").unwrap(), 500);    // Single numeral
        assert!(roman_to_int("MMMM").is_err());         // Beyond valid Roman numeral range
    }
}
