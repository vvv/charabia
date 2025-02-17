use finl_unicode::categories::CharacterCategories;
use slice_group_by::StrGroupBy;

/// Returns an iterator over substrings of `str` separated on camelCase boundaries.
/// For instance, "camelCase" is split into ["camel", "Case"].
/// A camelCase boundary constitutes a lowercase letter directly followed by an uppercase letter
/// optionally with any number of non-spacing marks in between.
pub(crate) fn split_camel_case_bounds(str: &str) -> impl Iterator<Item = &str> {
    let mut last_char_was_lowercase = match str.chars().next() {
        None => false,
        Some(c) => c.is_letter_lowercase(),
    };

    str.linear_group_by(move |_, char| {
        if char.is_mark_nonspacing() {
            return true;
        }

        if last_char_was_lowercase && char.is_letter_uppercase() {
            return false;
        }

        last_char_was_lowercase = char.is_letter_lowercase();
        true
    })
}

#[cfg(test)]
mod test {
    use super::split_camel_case_bounds;

    macro_rules! test_segmentation {
        ($text:expr, $segmented:expr, $name:ident) => {
            #[test]
            fn $name() {
                let segmented_text: Vec<_> = split_camel_case_bounds($text).collect();
                assert_eq!(segmented_text, $segmented);
            }
        };
    }

    test_segmentation!("a", ["a"], one_letter_is_preserved);
    test_segmentation!("aB", ["a", "B"], two_letter_boundary_is_split);
    test_segmentation!("camelCase", ["camel", "Case"], camel_case_is_split);
    test_segmentation!("SCREAMING", ["SCREAMING"], all_caps_is_not_split);
    test_segmentation!("resuméWriter", ["resumé", "Writer"], non_ascii_boundary_on_left);
    test_segmentation!("KarelČapek", ["Karel", "Čapek"], non_ascii_boundary_on_right);
    test_segmentation!(
        "resume\u{0301}Writer",
        ["resume\u{0301}", "Writer"],
        non_spacing_marks_are_respected
    );
    test_segmentation!("a\u{0301}B", ["a\u{0301}", "B"], non_spacing_mark_after_first_letter);
}
