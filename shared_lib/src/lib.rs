use sha2::{Digest, Sha256};

pub fn add(left: i32, right: i32) -> i32 {
    println!("[rust] add({}, {})", left, right);
    left + right
}


pub fn tokenize(input: String) ->  Vec<u64> {
    let tokenized_words = nomalize(input)
        .split_terminator(|c:char| !c.is_alphabetic())
        .filter(|s| !s.is_empty())
        .map(|word|{
            // let tokenized_word = Sha256::digest(work.as_bytes());
            // let mut tokenized_word = tokenized_word.as_slice().to_vec();
            // tokenized_word.truncate(8);
            // u64::from_be_bytes(tokenized_word.try_into().unwrap())

            Sha256::digest(word.as_bytes())
                .iter()
                .take(8)
                .copied()
                .collect::<Vec<_>>()
                .iter()
                .enumerate()
                .fold(0, |acc, (index, &byte)| {
                    acc + (byte as u64) * 256u64.pow(index as u32)
                })
        })
        .collect::<Vec<u64>>();
    tokenized_words
}
// #[uniffi::export]
pub fn index(count: u64, message_body: String) -> String {
    println!("[rust] index_pdu({}, {})", count, message_body);
    nomalize(message_body)
}

fn nomalize(input: String) -> String {
    // to lower case and remove diacritics
    let input =
        input
            .trim()
            .chars()
            .map(|x| match x {
                'À' | 'Á' | 'Ả' | 'Ạ' | 'Ã' | 'Â' | 'Ấ' | 'Ầ' | 'Ẩ' | 'Ẫ' | 'Ậ' | 'Ä' | 'Å'
                | 'Æ' | 'Ă' | 'Ắ' | 'Ằ' | 'Ẵ' | 'Ẳ' | 'Ặ' => 'a',
                'Þ' => 'b',
                'Ç' | 'Č' => 'c',
                'Ď' | 'Ð' => 'd',
                'Ě' | 'È' | 'É' | 'Ẽ' | 'Ẻ' | 'Ẹ' | 'Ê' | 'Ế' | 'Ề' | 'Ễ' | 'Ể' | 'Ệ' | 'Ë' => {
                    'e'
                }
                'Ƒ' => 'f',
                'Ì' | 'Í' | 'Ĩ' | 'Ỉ' | 'Ị' | 'Î' | 'Ï' => 'i',
                'Ň' | 'Ñ' => 'n',
                'Ò' | 'Ó' | 'Õ' | 'Ỏ' | 'Ọ' | 'Ô' | 'Ố' | 'Ồ' | 'Ỗ' | 'Ổ' | 'Ộ' | 'Ơ' | 'Ớ'
                | 'Ờ' | 'Ỡ' | 'Ở' | 'Ợ' | 'Ö' | 'Ø' => 'o',
                'Ř' => 'r',
                'Š' => 's',
                'Ť' => 't',
                'Ů' | 'Ù' | 'Ú' | 'Ũ' | 'Ủ' | 'Ụ' | 'Ư' | 'Ứ' | 'Ừ' | 'Ữ' | 'Ử' | 'Ự' | 'Û'
                | 'Ü' => 'u',
                'Ý' | 'Ỳ' | 'Ỹ' | 'Ỷ' | 'Ỵ' => 'y',
                'Ž' => 'z',

                'à' | 'á' | 'ã' | 'ả' | 'ạ' | 'â' | 'ấ' | 'ầ' | 'ẫ' | 'ẩ' | 'ậ' | 'ă' | 'ắ'
                | 'ằ' | 'ẵ' | 'ẳ' | 'ặ' | 'ä' | 'å' | 'æ' => 'a',
                'þ' => 'b',
                'ç' | 'č' => 'c',
                'ď' | 'ð' | 'đ' => 'd',
                'ě' | 'è' | 'é' | 'ẽ' | 'ẻ' | 'ẹ' | 'ê' | 'ế' | 'ề' | 'ễ' | 'ể' | 'ệ' | 'ë' => {
                    'e'
                }
                'ƒ' => 'f',
                'ì' | 'í' | 'ĩ' | 'ỉ' | 'ị' | 'î' | 'ï' => 'i',
                'ñ' | 'ň' => 'n',
                'ò' | 'ó' | 'õ' | 'ỏ' | 'ọ' | 'ô' | 'ố' | 'ồ' | 'ỗ' | 'ổ' | 'ộ' | 'ơ' | 'ớ'
                | 'ờ' | 'ỡ' | 'ở' | 'ợ' | 'ö' | 'ø' => 'o',
                'ř' => 'r',
                'š' => 's',
                'ť' => 't',
                'ů' | 'ù' | 'ú' | 'ũ' | 'ủ' | 'ụ' | 'ư' | 'ứ' | 'ừ' | 'ữ' | 'ử' | 'ự' | 'û'
                | 'ü' => 'u',
                'ý' | 'ỳ' | 'ỹ' | 'ỷ' | 'ỵ' | 'ÿ' => 'y',
                'ž' => 'z',
                'A'..='Z' => x.to_ascii_lowercase(),
                'a'..='z' => x,
                '0'..='9' => x,
                _ => ' ',
            })
            .collect::<String>();
    input
}

uniffi::include_scaffolding!("shared_lib");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
