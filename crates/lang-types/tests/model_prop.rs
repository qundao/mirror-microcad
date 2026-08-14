use microcad_lang_base::{Identifier, SrcRef};
use microcad_lang_types::{
    Value, arguments,
    model::{Properties, Property, PropertyType},
};

#[test]
fn test_properties_basic_crud() {
    let mut props = Properties::new();

    assert!(props.is_empty());
    assert_eq!(props.len(), 0);
    assert!(!props.contains("radius"));

    // 1. Insert property
    let inserted = props.set_property(
        "radius",
        Value::from(10.5),
        PropertyType::Input,
        SrcRef::none(),
    );

    assert_eq!(inserted.name, Identifier::from("radius"));
    assert_eq!(inserted.value, Value::from(10.5));
    assert_eq!(inserted.ty, PropertyType::Input);

    // 2. Query property
    assert_eq!(props.len(), 1);
    assert!(!props.is_empty());
    assert!(props.contains("radius"));

    let retrieved = props.get_property("radius").expect("Property should exist");
    assert_eq!(retrieved.value, Value::from(10.5));

    // 3. Update existing property
    props.set_property("radius", 20.0, PropertyType::Input, SrcRef::none());

    assert_eq!(props.len(), 1); // Length should remain 1
    let updated = props.get_property("radius").unwrap();
    assert_eq!(updated.value, Value::from(20.0));
    assert_eq!(updated.ty, PropertyType::Input);
}

#[test]
fn test_from_arguments() {
    let args = arguments!(width = 100.0, height = 50.0);

    // Convert Arguments -> Properties
    let props = Properties::from(args);

    assert_eq!(props.len(), 2);
    assert!(props.contains("width"));
    assert!(props.contains("height"));

    let width_prop = props.get_property("width").unwrap();
    assert_eq!(width_prop.value, Value::from(100.0));
    assert_eq!(width_prop.ty, PropertyType::Input);
    assert_eq!(width_prop.src_ref, SrcRef::default());
}

#[test]
fn test_deref_and_iterators() {
    let mut props = Properties::new();
    props.set_property("x", 1.0, PropertyType::Input, SrcRef::none());
    props.set_property("y", 2.0, PropertyType::Input, SrcRef::none());

    // Test Deref to BTreeMap methods (e.g. .keys(), .values())
    let keys: Vec<_> = props.keys().map(|id| id.to_string()).collect();
    assert_eq!(keys, vec!["x", "y"]);

    // Test IntoIterator for &Properties
    let mut count = 0;
    for (id, prop) in &props {
        assert_eq!(id, &prop.name);
        count += 1;
    }
    assert_eq!(count, 2);

    // Test IntoIterator for owned Properties
    let owned_items: Vec<(Identifier, Property)> = props.into_iter().collect();
    assert_eq!(owned_items.len(), 2);
}
