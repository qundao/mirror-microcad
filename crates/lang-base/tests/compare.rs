use microcad_lang_base::{ComputedHash, Hashed, LineCol, SrcRef, TextEdit};

fn source(text: &str) -> microcad_lang_base::Source {
    microcad_lang_base::Source {
        url: microcad_lang_base::Url::parse("file:///test.mu").unwrap(),
        line_offset: 0,
        code: Hashed::new(text.to_string()),
    }
}

#[test]
fn test_identical_strings_yield_no_edits() {
    let edits =
        source("hello world\nthis is a test").compare(&source("hello world\nthis is a test"));

    assert!(edits.is_empty(), "Expected no edits for identical strings");
}

#[test]
fn test_pure_insertion() {
    let old = source("hello world");
    let new = source("hello beautiful world");
    let edits = old.compare(&new);

    assert_eq!(edits.len(), 1);
    assert_eq!(
        edits[0],
        TextEdit {
            src_ref: SrcRef::new(
                &(6..6),
                LineCol { line: 1, col: 6 },
                old.code.computed_hash()
            ),
            new_text: "beautiful ".to_string(),
        }
    );
}

#[test]
fn test_pure_deletion() {
    let old = source("hello beautiful world");
    let new = source("hello world");
    let edits = old.compare(&new);

    assert_eq!(edits.len(), 1);
    assert_eq!(
        edits[0],
        TextEdit {
            // Should delete "beautiful " spanning from char 6 to 16
            src_ref: SrcRef::new(
                &(6..16),
                LineCol { line: 1, col: 6 },
                old.code.computed_hash()
            ),
            new_text: String::new(),
        }
    );
}

#[test]
fn test_multiline_replacement() {
    let old = source("line one\nline two\nline three");
    let new = source("line one\nline changed\nline three");
    let edits = old.compare(&new);

    // Depending on how `dissimilar` slices it, this might show up as
    // a deletion of "two" and an insertion of "changed".
    assert_eq!(edits.len(), 2);

    // 1. Deletion of "two"
    assert_eq!(
        edits[0],
        TextEdit {
            // Should delete "beautiful " spanning from char 6 to 16
            src_ref: SrcRef::new(
                &(14..17),
                LineCol { line: 2, col: 5 },
                old.code.computed_hash()
            ),
            new_text: String::new(),
        }
    );

    // 2. Insertion of "changed"
    assert_eq!(
        edits[1],
        TextEdit {
            // Should delete "beautiful " spanning from char 6 to 16
            src_ref: SrcRef::new(
                &(17..17),
                LineCol { line: 1, col: 8 },
                old.code.computed_hash()
            ),
            new_text: "changed".to_string()
        }
    );
}

#[test]
fn test_complex_mixed_changes() {
    let old = source("fn main() {\n    println!(\"Hello\");\n}");
    let new = source("fn main() {\n    // Greet\n    println!(\"Hello World\");\n}");
    let edits = old.compare(&new);

    // We expect edits that insert the comment and modify the string literal
    assert!(!edits.is_empty());

    // A robust way to verify complex edits is to ensure they don't panic
    // and target reasonable line boundaries.
    for edit in &edits {
        // Ensure no inverted ranges
        assert!(edit.src_ref.start <= edit.src_ref.end);

        // Check that bounds stay within the original file lines (0 to 2)
        assert!(edit.src_ref.at.line <= 2);
    }
}
