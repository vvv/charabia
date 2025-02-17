use unicode_segmentation::UnicodeSegmentation;

use super::Segmenter;

/// Arabic specialized [`Segmenter`].
///
/// This Segmenter uses [`UnicodeSegmentation`] internally to segment the provided text.
/// Arabic text is segmented by word boundaries and by punctuation.
/// We need a workaround to segment the Arabic text that starts with `ال` (the) because it is not segmented by word boundaries.
/// One possible solution is to segment any word that starts with `ال` into two words. The `ال` and the rest of the word.
/// with this solution, we will have `الشجرة` (the tree) segmented into `ال` (the) and `شجرة` (tree). and if we search for `شجرة` (tree) or `الشجرة` (thetree) we will find results.
/// Some Arabic text starts with `ال` and not meant to be (the) like `البانيا` (Albania). In this case, we will have `ال` and `بانيا` segmented. and if we search for `البانيا` we will find results.

pub struct ArabicSegmenter;

// All specialized segmenters only need to implement the method `segment_str` of the `Segmenter` trait.
impl Segmenter for ArabicSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        // Create the iterator that will segment the provided text.
        let segment_iterator = to_segment
            // Split the text by word boundaries.
            .split_word_bounds()
            // Check if the lemma starts with `ال` and if so, split it into two lemmas.
            .flat_map(|lemma| {
                // check if lemma starts with 'ال', 'أل', 'إل', 'آل' or 'ٱل'
                if lemma.len() > 2
                    && (lemma.starts_with("ال")
                        || lemma.starts_with("أل")
                        || lemma.starts_with("إل")
                        || lemma.starts_with("آل")
                        || lemma.starts_with("ٱل"))
                {
                    vec![&lemma[..4], &lemma[4..]]
                } else {
                    vec![lemma]
                }
            });

        // Return the created iterator wrapping it in a Box.
        Box::new(segment_iterator)
    }
}

// Test the segmenter:
#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    // Original version of the text.
    const TEXT: &str = "السلام عليكم، كيف حالكم؟ (أتمنى أن تكونوا بأفضل ٱلأحوال)";

    // Segmented version of the text.
    const SEGMENTED: &[&str] = &[
        "ال",
        "سلام",
        " ",
        "عليكم",
        "،",
        " ",
        "كيف",
        " ",
        "حالكم",
        "؟",
        " ",
        "(",
        "أتمنى",
        " ",
        "أن",
        " ",
        "تكونوا",
        " ",
        "بأفضل",
        " ",
        "ٱل",
        "أحوال",
        ")",
    ];

    // Segmented and normalized version of the text.
    const TOKENIZED: &[&str] = &[
        "ال",
        "سلام",
        " ",
        "عليكم",
        "،",
        " ",
        "كيف",
        " ",
        "حالكم",
        "؟",
        " ",
        "(",
        "اتمني",
        " ",
        "ان",
        " ",
        "تكونوا",
        " ",
        "بافضل",
        " ",
        "ال",
        "احوال",
        ")",
    ];

    // Macro that run several tests on the Segmenter.
    test_segmenter!(ArabicSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Arabic, Language::Ara);
}
