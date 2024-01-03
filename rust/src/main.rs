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
        let mut counter = 0;

        for i in 0..bytes.len() {
            if i != 0 && bytes[i] != bytes[i - 1] {
                result.push((counter, bytes[i - 1]));
                counter = 1;
            } else {
                counter += 1;
            }
        }

        result.push((counter, bytes[bytes.len() - 1]));

        Some(result)
    }

    fn unfold(compressed_data: CompressionResult) -> Option<String> {
        if compressed_data.is_empty() {
            return None;
        }

        // let mut result_string = String::new();

        // for (counter, value) in compressed_data {
        //     let ascii_char = value as char;

        //     for _ in 0..counter {
        //         result_string.push_str(ascii_char.to_string().as_str())
        //     }
        // }

        let result_string: String = compressed_data
            .iter()
            .flat_map(|(counter, value)| {
                std::iter::repeat((*value as char).to_string()).take(*counter as usize)
            })
            .collect();

        Some(result_string)
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

    #[test]
    fn test_unfold_empty_input() {
        let compressed_data: CompressionResult = vec![];
        let result = RLE::unfold(compressed_data);
        assert_eq!(result, None);
    }

    #[test]
    fn test_unfold_single_character_input() {
        let compressed_data: CompressionResult = vec![(1, b'a')];
        let result = RLE::unfold(compressed_data);
        assert_eq!(result, Some("a".to_string()));
    }

    #[test]
    fn test_unfold_multiple_characters_input() {
        let input = "aaaabbbbbbcddddd".to_string();
        let compressed = RLE::fold(input.clone()).unwrap();
        let uncompressed = RLE::unfold(compressed).unwrap();

        assert_eq!(input, uncompressed);
    }
}
