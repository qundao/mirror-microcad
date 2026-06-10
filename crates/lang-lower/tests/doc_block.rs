#[test]
fn doc_block_merge() {
    let doc_a = DocBlock(Refer::none(vec!["/// line 1".to_string()]));
    let doc_b = DocBlock(Refer::none(vec!["/// line 2".to_string()]));
    let empty = DocBlock::default();

    // Test: Merge with empty blocks
    assert!(DocBlock::merge(&empty, &empty).is_empty());

    let merge_with_empty_left = DocBlock::merge(&empty, &doc_a);
    assert_eq!(merge_with_empty_left.0.value, vec!["/// line 1"]);

    let merge_with_empty_right = DocBlock::merge(&doc_a, &empty);
    assert_eq!(merge_with_empty_right.0.value, vec!["/// line 1"]);

    // Test: Merge two populated blocks
    let merged = DocBlock::merge(&doc_a, &doc_b);

    // The implementation adds an empty line (String::default()) between blocks
    let expected = vec![
        "/// line 1".to_string(),
        "".to_string(),
        "/// line 2".to_string(),
    ];

    assert_eq!(merged.0.value, expected);
}

#[test]
fn doc_block_fetch_text() {
    // Test 1: Standard space-separated doc comments
    let doc1 = DocBlock(Refer::none(vec![
        "/// Line one".to_string(),
        "/// Line two ".to_string(), // Note the trailing space
    ]));
    assert_eq!(doc1.fetch_lines().join("\n"), "Line one\nLine two");

    // Test 2: Mixed prefixes (with and without space)
    let doc2 = DocBlock(Refer::none(vec![
        "///Space".to_string(),
        "///No space".to_string(),
    ]));
    assert_eq!(doc2.fetch_lines().join("\n"), "Space\nNo space");

    // Test 3: Lines that don't start with '///' should be ignored
    let doc3 = DocBlock(Refer::none(vec![
        "/// Valid".to_string(),
        "Invalid line".to_string(),
        "/// Also valid".to_string(),
    ]));
    assert_eq!(doc3.fetch_lines().join("\n"), "Valid\nAlso valid");

    // Test 4: Empty DocBlock
    let doc_empty = DocBlock::default();
    assert_eq!(doc_empty.fetch_lines().join("\n"), "");
}
