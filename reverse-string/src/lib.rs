use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut chars = input.graphemes(true).collect::<Vec<&str>>();
    chars.reverse();
    chars.join("")
}
