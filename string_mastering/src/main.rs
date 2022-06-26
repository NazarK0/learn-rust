use std::mem;

fn main() {
    let mut s = String::from("Привіт💖 and hello!❤");

    // delete_duplicates(&mut s);
    let s_bytes = s.as_bytes();

    println!("{:#?}, ",&s_bytes[0..4]);


    print!("char sizes 1: ");
    for ch in s.chars() {
        print!("{:#?}, ", char_length(ch));
    }
    print!("\n");
}

fn char_length(ch: char) -> usize {
    ch.to_string().as_bytes().iter().count()
}

fn find_duplicate_char_ids(src: &String) -> Vec<usize> {
    let alphabet_length = src.chars().count();
    let source_chars = src
        .chars()
        // skip last character
        .take(alphabet_length - 1)
        .enumerate();

    let mut duplicate_indexes = Vec::<usize>::new();

    for source_data in source_chars {
        let source_idx = source_data.0;
        let source_char = source_data.1;

        let inspected_string = src.chars().skip(source_idx + 1).enumerate();

        for insp_data in inspected_string {
            let inspected_idx = insp_data.0;
            let inspected_char = insp_data.1;

            if source_char == inspected_char {
                duplicate_indexes.push(inspected_idx + 1 + source_idx);
            }
        }
    }

    duplicate_indexes
}

fn delete_duplicates(src: &mut String) {
    let duplicate_indexes = find_duplicate_char_ids(src);

    for data in duplicate_indexes.iter().enumerate() {
        let idx = data.0;
        let duplicate_idx = data.1;

        // let substring_left = &src[0..duplicate_idx -1];
        // let start = duplicate_idx;
        // let end = duplicate_idx + 2;
        // let substring_right = &src[start..end];

        src.remove(duplicate_idx - idx);
    }
}

fn byte_idx(source: &str, char_idx: usize) -> usize {
    
}
