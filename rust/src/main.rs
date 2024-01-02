/* --- --- Run length encoding --- --- */

// https://filestore.aqa.org.uk/resources/computing/AQA-8525-TG-RLE.PDF

// 97 97 97 97  __  98 98 98 98 98 98  __  99  __  100 100 100 100 100

// NOTE: may not need to have additional tuples
type CompressionResult = Vec<(u64, u8)>;

struct RLE;

impl RLE {
    fn fold(input: String) -> Option<CompressionResult> {
        if input.is_empty() {
            return None;
        }

        let bytes = input.as_bytes();

        let mut result: Vec<(u64, u8)> = vec![];

        let mut current_b = bytes[0];
        let mut counter = 0;

        for (index, &b) in bytes.iter().enumerate() {
            if index == bytes.len() - 1 {
                result.push((counter + 1, current_b));
                return Some(result);
            }

            if b != current_b {
                result.push((counter, current_b));
                current_b = b;
                counter = 1;
            } else {
                counter += 1;
            }
        }

        Some(result)
    }

    fn unfold(compressed_data: CompressionResult) -> Option<String> {
        todo!()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_empty_input() {
        let input = "".to_string();
        let result = RLE::fold(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_fold_single_character_input() {
        let input = "a".to_string();
        let result = RLE::fold(input);
        assert_eq!(result, Some(vec![(1, b'a')]));
    }

    #[test]
    fn test_fold_multiple_characters_input() {
        let input = "aaaabbbbbbcddddd".to_string();

        let result = RLE::fold(input);

        assert_eq!(
            result,
            Some(vec![(4, b'a'), (6, b'b'), (1, b'c'), (5, b'd')])
        );
    }
}
