use microcad_lang_base::Identifier;
use microcad_lang_types::ty::*;

#[test]
fn test_tuple_type_eq() {
    assert_eq!(TupleType::new_color(), TupleType::new_color());
}

#[test]
fn test_tuple_type_match() {
    let args = TupleType {
        named: [
            (Identifier::no_ref("x"), Type::Integer),
            (
                Identifier::no_ref("y"),
                Type::Array(Box::new(Type::Integer)),
            ),
        ]
        .into_iter()
        .collect(),
        unnamed: Default::default(),
    };
    let params = TupleType {
        named: [
            (Identifier::no_ref("x"), Type::Integer),
            (Identifier::no_ref("y"), Type::Integer),
        ]
        .into_iter()
        .collect(),
        unnamed: Default::default(),
    };
    assert!(args.is_matching(&params));
}

#[test]
fn test_common_type() {
    let list = TypeList::new(vec![Type::Integer, Type::Integer]);
    assert_eq!(Some(Type::Integer), list.common_type());

    let list = TypeList::new(vec![Type::Integer, Type::Quantity(QuantityType::Scalar)]);
    assert_eq!(None, list.common_type());

    let list = TypeList::new(Vec::new());
    assert_eq!(None, list.common_type());
}

#[test]
fn type_matching() {
    assert!(Type::scalar().is_matching(&Type::scalar()));
    assert!(!Type::scalar().is_matching(&Type::Integer));
    assert!(Type::Integer.is_matching(&Type::scalar()));
    assert!(!Type::scalar().is_matching(&Type::String));
    assert!(!Type::String.is_matching(&Type::scalar()));
}
