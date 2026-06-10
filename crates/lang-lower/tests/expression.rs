#[test]
fn test_argument_debug() {
    use crate::ir;
    let arg1 = Argument {
        id: Some("id1".into()),
        expression: ir::ConstantExpression::QualifiedName("my::name1".into()),
        src_ref: SrcRef::none(),
    };

    let arg2 = Argument {
        id: None,
        expression: ir::ConstantExpression::QualifiedName("my::name2".into()),
        src_ref: SrcRef::none(),
    };

    let arg3 = Argument {
        id: Some(Identifier::none()),
        expression: ir::ConstantExpression::QualifiedName("my::name2".into()),
        src_ref: SrcRef::none(),
    };

    let mut args = ir::ArgumentList::<ir::ConstantExpression>::default();

    args.try_push(arg1).expect("test error");
    args.try_push(arg2).expect("test error");
    args.try_push(arg3).expect("test error");

    log::info!("{args}");
    log::info!("{args:?}");
}
