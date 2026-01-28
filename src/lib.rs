use std::{cmp, collections::HashMap};
use unicode_segmentation::UnicodeSegmentation;


pub fn count_char_diff_ascii(a: &String, b: &String) -> i32 {
    let mut a_counts: Vec<i32> = vec![0; 256];
    for a_char in a.as_bytes() {
        a_counts[usize::from(*a_char)] += 1;
    }

    for b_char in b.as_bytes() {
        a_counts[usize::from(*b_char)] -= 1;
    }

    let mut diff = 0;
    for c in a_counts {
        diff += c.abs();
    }

    return diff;
}

pub fn count_char_diff_utf(a: &String, b: &String) -> i32 {
    let mut a_counts: HashMap<&str, i32> = HashMap::with_capacity(a.len());
    for a_grapheme in a.graphemes(true) {
        a_counts.insert(a_grapheme, a_counts.get(a_grapheme).unwrap_or(&0) + 1);
    }

    for b_grapheme in b.graphemes(true) {
        a_counts.insert(b_grapheme, a_counts.get(b_grapheme).unwrap_or(&0) - 1);
    }

    let mut diff = 0;
    for c in a_counts.values() {
        diff += c.abs()
    }

    return diff;
}

pub fn is_ascii(s: &String) -> bool {
    for c in s.as_bytes() {
        if *c as u8 > 127 {
            return false;
        }
    }
    return true;
}

pub fn levenshtein_ascii(a: &String, b: &String) -> i32 {
    // len + 1 is an extra row/col to avoid an extra loop after the main one
    // len + 2 is an extra row/col to avoid branching
    let row_size = b.len() + 2;
    let mut states: Vec<i32> = vec![
        cmp::max(a.len(), b.len()).try_into().unwrap();
        (a.len() + 2) * row_size
    ];
    states[0] = 0;

    let a_bytes = &a.as_bytes();
    let b_bytes = &b.as_bytes();

    for a_pos in 0..a.len() + 1 {
        let row_offset = a_pos * row_size;
        let next_row_offset = row_offset + row_size;

        let mut a_char: u8 = 0;
        if a_pos < a.len() {
            a_char = a_bytes[a_pos];
        };

        for b_pos in 0..b.len() + 1 {
            let col_offset = row_offset + b_pos;
            let next_row_col_offset = next_row_offset + b_pos;
            let d = states[col_offset];

            let mut b_char: u8 = 0;
            if b_pos < b.len() {
                b_char = b_bytes[b_pos];
            };

            // Update all related elements
            // Can hit an existing value here. It means we got into the
            // same position by a different path. Just use the best option found.

            // remove from b (~add to a)
            states[col_offset + 1] = cmp::min(d + 1, states[col_offset + 1]);

            // remove from a
            states[next_row_col_offset] = cmp::min(d + 1, states[next_row_col_offset]);

            // replace or keep
            // guaranteed to be visited the first time, so no min()
            states[next_row_col_offset + 1] = d + ((a_char != b_char) as i32);
        }
    }

    return states[a.len() * row_size + b.len()];
}

pub fn levenshtein_utf(a: &String, b: &String) -> i32 {
    // len + 1 is an extra row/col to avoid an extra loop after the main one
    // len + 2 is an extra row/col to avoid branching
    let row_size = b.len() + 2;
    let mut states: Vec<i32> = vec![
        cmp::max(a.len(), b.len()).try_into().unwrap();
        (a.len() + 2) * row_size
    ];
    states[0] = 0;

    let nullstr = "\u{0}";
    let mut b_len = 0; // grapheme count
    let mut b_graphemes = Vec::<&str>::with_capacity(b.len() + 1);
    for c in b.graphemes(true) {
        b_graphemes.push(&c);
        b_len += 1;
    }
    b_graphemes.push(&nullstr);

    let mut a_len = 0; // grapheme count
    let mut a_grapheme_iter = a.graphemes(true);
    for a_pos in 0..a.len() + 1 {
        let row_offset = a_pos * row_size;
        let next_row_offset = row_offset + row_size;

        let a_char: &str = a_grapheme_iter.next().unwrap_or(&nullstr);
        a_len += 1;

        for b_pos in 0..b_len + 1{
            let col_offset = row_offset + b_pos;
            let next_row_col_offset = next_row_offset + b_pos;
            let d = states[col_offset];

            let b_char: &str = b_graphemes[b_pos];

            // Update all related elements
            // Can hit an existing value here. It means we got into the
            // same position by a different path. Just use the best option found.

            // remove from b (~add to a)
            states[col_offset + 1] = cmp::min(d + 1, states[col_offset + 1]);

            // remove from a
            states[next_row_col_offset] = cmp::min(d + 1, states[next_row_col_offset]);

            // replace or keep
            // guaranteed to be visited the first time, so no min()
            states[next_row_col_offset + 1] = d + ((a_char != b_char) as i32);

        }

        if a_char == nullstr {
            break;
        }
    }

    a_len -= 1;

    return states[a_len * row_size + b_len];
}


pub fn count_char_diff(a: &String, b: &String) -> i32 {
    if is_ascii(&a) && is_ascii(&b) {
        return count_char_diff_ascii(&a, &b);
    } else {
        return count_char_diff_utf(&a, &b);
    }
}

pub fn levenshtein(a: &String, b: &String) -> i32 {
    if is_ascii(&a) && is_ascii(&b) {
        return levenshtein_ascii(&a, &b);
    } else {
        return levenshtein_utf(&a, &b);
    }
}
