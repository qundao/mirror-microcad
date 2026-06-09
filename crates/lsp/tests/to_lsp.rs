use microcad_lsp::to_lsp::compare_strs_to_lsp_edits;
use tower_lsp::lsp_types::{Position, Range, TextEdit};

#[test]
fn test_identical_strings_yield_no_edits() {
    let old = "hello world\nthis is a test";
    let new = "hello world\nthis is a test";
    let edits = compare_strs_to_lsp_edits(old, new);

    assert!(edits.is_empty(), "Expected no edits for identical strings");
}

#[test]
fn test_pure_insertion() {
    let old = "hello world";
    let new = "hello beautiful world";
    let edits = compare_strs_to_lsp_edits(old, new);

    assert_eq!(edits.len(), 1);
    assert_eq!(
        edits[0],
        TextEdit {
            // Should insert right after "hello " (line 0, char 6)
            range: Range::new(Position::new(0, 6), Position::new(0, 6)),
            new_text: "beautiful ".to_string(),
        }
    );
}

#[test]
fn test_pure_deletion() {
    let old = "hello beautiful world";
    let new = "hello world";
    let edits = compare_strs_to_lsp_edits(old, new);

    assert_eq!(edits.len(), 1);
    assert_eq!(
        edits[0],
        TextEdit {
            // Should delete "beautiful " spanning from char 6 to 16
            range: Range::new(Position::new(0, 6), Position::new(0, 16)),
            new_text: String::new(),
        }
    );
}

#[test]
fn test_multiline_replacement() {
    let old = "line one\nline two\nline three";
    let new = "line one\nline changed\nline three";
    let edits = compare_strs_to_lsp_edits(old, new);

    // Depending on how `dissimilar` slices it, this might show up as
    // a deletion of "two" and an insertion of "changed".
    assert_eq!(edits.len(), 2);

    // 1. Deletion of "two"
    assert_eq!(
        edits[0],
        TextEdit {
            range: Range::new(Position::new(1, 5), Position::new(1, 8)),
            new_text: String::new(),
        }
    );

    // 2. Insertion of "changed"
    assert_eq!(
        edits[1],
        TextEdit {
            range: Range::new(Position::new(1, 8), Position::new(1, 8)),
            new_text: "changed".to_string(),
        }
    );
}

#[test]
fn test_complex_mixed_changes() {
    let old = "fn main() {\n    println!(\"Hello\");\n}";
    let new = "fn main() {\n    // Greet\n    println!(\"Hello World\");\n}";

    let edits = compare_strs_to_lsp_edits(old, new);

    // We expect edits that insert the comment and modify the string literal
    assert!(!edits.is_empty());

    // A robust way to verify complex edits is to ensure they don't panic
    // and target reasonable line boundaries.
    for edit in &edits {
        // Ensure no inverted ranges
        assert!(edit.range.start <= edit.range.end);

        // Check that bounds stay within the original file lines (0 to 2)
        assert!(edit.range.end.line <= 2);
    }
}
