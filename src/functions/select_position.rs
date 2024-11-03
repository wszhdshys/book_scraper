use pinyin::ToPinyin;

pub fn select_position(position: &str) -> String {
    let mut address = vec![];
    for pinyin in position.to_pinyin() {
        if let Some(pinyin) = pinyin {
            address.push(pinyin.plain());
        } else {
            print!("(无法转换的字符) ");
        }
    }
    let eturn = address.join("");
    // println!("{}", Return);
    eturn
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_position() {
        let input = "中国";
        let expected = "zhongguo";
        let result = select_position(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_select_position2() {
        let input = "内蒙古";
        let expected = "neimenggu"; // 假设 "!" 是无法转换的字符
        let result = select_position(input);
        assert_eq!(result, expected);
    }
}
