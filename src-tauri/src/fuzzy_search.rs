use pinyin::ToPinyin;

/// 将中文转换为拼音首字母
pub fn to_pinyin_initials(text: &str) -> String {
    text.chars()
        .filter_map(|c| {
            c.to_pinyin().map(|py| {
                py.plain()
                    .chars()
                    .next()
                    .unwrap_or(c)
            })
        })
        .collect()
}

/// 将中文转换为完整拼音
pub fn to_pinyin_full(text: &str) -> String {
    text.chars()
        .filter_map(|c| {
            c.to_pinyin().map(|py| py.plain().to_string())
        })
        .collect::<Vec<_>>()
        .join("")
}

/// 模糊搜索匹配
pub fn fuzzy_match(query: &str, text: &str) -> bool {
    let query_lower = query.to_lowercase();
    let text_lower = text.to_lowercase();
    
    // 1. 精确匹配
    if text_lower.contains(&query_lower) {
        return true;
    }
    
    // 2. 拼音首字母匹配
    let pinyin_initials = to_pinyin_initials(&text_lower);
    if pinyin_initials.contains(&query_lower) {
        return true;
    }
    
    // 3. 完整拼音匹配
    let pinyin_full = to_pinyin_full(&text_lower);
    if pinyin_full.contains(&query_lower) {
        return true;
    }
    
    // 4. 容错匹配（允许跳过字符）
    fuzzy_match_chars(&query_lower, &text_lower)
}

/// 字符级容错匹配
fn fuzzy_match_chars(query: &str, text: &str) -> bool {
    let mut query_chars = query.chars().peekable();
    let mut text_chars = text.chars().peekable();
    
    while let Some(&q_char) = query_chars.peek() {
        let mut found = false;
        while let Some(&t_char) = text_chars.peek() {
            if q_char == t_char {
                query_chars.next();
                text_chars.next();
                found = true;
                break;
            }
            text_chars.next();
        }
        if !found {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match() {
        assert!(fuzzy_match("gx", "工作"));
        assert!(fuzzy_match("gongzuo", "工作"));
        assert!(fuzzy_match("work", "工作"));
    }
}
