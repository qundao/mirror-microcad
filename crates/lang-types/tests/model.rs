// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::BuiltinId;
use microcad_lang_types::{
    Arguments, Identifier, Model, ModelOutputType, ModelTree, Value, arguments,
    model::{
        BooleanOp, Element, ModelNodeExt, Properties,
        element::{self, BuiltinWorkbenchKind},
    },
};

// Simple helper to create a base model for testing
fn create_test_model(name: &str) -> Model {
    Model {
        name: Some(Identifier::from(name)),
        element: Element::BuiltinWorkpiece(element::BuiltinWorkbenchKind::Primitive2D),
        ..Default::default()
    }
}

#[test]
fn test_model_tree_creation_and_root() {
    let root_model = create_test_model("root");
    let tree = ModelTree::new(root_model);

    let root_ref = tree.root();

    assert_eq!(root_ref.name(), Some(&Identifier::from("root")));
}

#[test]
fn test_tree_hierarchy_and_children() {
    let root_model = create_test_model("parent");
    let mut tree = ModelTree::new(root_model);

    // Add children using the arena directly
    let child1 = tree.arena.new_node(create_test_model("child1"));
    let child2 = tree.arena.new_node(create_test_model("child2"));

    tree.root.append(child1, &mut tree.arena);
    tree.root.append(child2, &mut tree.arena);

    // Verify traversal via root reference
    let root_ref = tree.root();
    let child_names: Vec<String> = root_ref
        .children()
        .filter_map(|node| node.name().map(|id| id.to_string()))
        .collect();

    assert_eq!(child_names, vec!["child1", "child2"]);
}

#[test]
fn test_deduce_output_type_fallback() {
    // Create a root model with an undetermined output type (e.g., a generic Group)
    let mut group_model = create_test_model("group");
    group_model.element = Element::Group; // Assuming Group produces NotDetermined initially

    let mut tree = ModelTree::new(group_model);

    // Child model has a concrete output type
    let child_model =
        Model::primitive2d(BuiltinId::from("__mu::geo2d::Circle"), Arguments::default());
    let child_id = tree.arena.new_node(child_model);
    tree.root.append(child_id, &mut tree.arena);

    // The root should fall back to its child's type
    let root_ref = tree.root();
    let deduced_type = root_ref.deduce_output_type();

    assert_ne!(deduced_type, ModelOutputType::NotDetermined);
}

#[test]
fn test_into_group_child() {
    let group_model = Model {
        element: Element::Group,
        ..create_test_model("parent_group")
    };
    let mut tree = ModelTree::new(group_model);

    let inner_group = Model {
        element: Element::Group,
        ..create_test_model("inner_group")
    };
    let inner_id = tree.arena.new_node(inner_group);
    tree.root.append(inner_id, &mut tree.arena);

    let root_ref = tree.root();
    let group_child = root_ref.into_group_child();

    println!("{root_ref}");

    // Should successfully unwrap the single nested group child
    assert!(group_child.is_some());
    assert_eq!(
        group_child.unwrap().name(),
        Some(&Identifier::from("inner_group"))
    );
}

/// Helper to construct a simple leaf ModelTree with a specific name
fn make_leaf(name: &str) -> ModelTree {
    ModelTree::new(Model {
        name: Some(Identifier::from(name)),
        element: Element::BuiltinWorkpiece(element::BuiltinWorkbenchKind::Primitive2D),
        ..Model::default()
    })
}

/// Helper to extract child names under the inner group
fn get_group_child_names(tree: &ModelTree) -> Vec<String> {
    let root = tree.root();
    let group_node = root
        .into_group_child()
        .expect("Root should contain a single Group child");

    group_node
        .children()
        .filter_map(|child| child.name().map(|id| id.to_string()))
        .collect()
}

#[test]
fn test_basic_boolean_op_structure() {
    let a = make_leaf("a");
    let b = make_leaf("b");

    // a - b (Difference)
    let diff_tree = a - b;

    let root = diff_tree.root();

    // 1. Check root is BooleanOp::Difference
    assert_eq!(
        root.element,
        Element::BuiltinWorkpiece(BuiltinWorkbenchKind::BooleanOp(BooleanOp::Subtract))
    );

    // 2. Check group children are "a" and "b"
    let child_names = get_group_child_names(&diff_tree);
    assert_eq!(child_names, vec!["a", "b"]);
}

#[test]
fn test_flattening_same_boolean_op() {
    let a = make_leaf("a");
    let b = make_leaf("b");
    let c = make_leaf("c");

    // Chained Union: (a | b) | c
    let union_tree = a | b | c;

    // Verify root is single BooleanOp::Union
    assert_eq!(
        union_tree.root().element,
        Element::BuiltinWorkpiece(BuiltinWorkbenchKind::BooleanOp(BooleanOp::Union))
    );

    // Verify all 3 leaves are flattened under the SAME inner group
    let child_names = get_group_child_names(&union_tree);
    assert_eq!(child_names, vec!["a", "b", "c"]);
}

#[test]
fn test_nested_different_boolean_ops() {
    let a = make_leaf("a");
    let b = make_leaf("b");
    let c = make_leaf("c");

    // Mixed operations: (a | b) & c -> Union inside Intersection
    let mixed_tree = (a | b) & c;

    // Root should be Intersection
    assert_eq!(
        mixed_tree.root().element,
        Element::BuiltinWorkpiece(BuiltinWorkbenchKind::BooleanOp(BooleanOp::Intersect))
    );

    // Inner group of the root should have 2 children: [UnionNode, Leaf("c")]
    let root_group = mixed_tree
        .root()
        .into_group_child()
        .expect("Root must have group child");

    let children: Vec<_> = root_group.children().collect();
    assert_eq!(children.len(), 2);

    // First child is the nested Union operation
    assert_eq!(
        children[0].element,
        Element::BuiltinWorkpiece(BuiltinWorkbenchKind::BooleanOp(BooleanOp::Union))
    );

    // Second child is leaf "c"
    assert_eq!(children[1].name(), Some(&Identifier::from("c")));
}

#[test]
fn test_subtree_adoption_preserves_hierarchy() {
    let mut parent_tree = make_leaf("parent");
    let child1 = make_leaf("child1");
    let child2 = make_leaf("child2");

    // Construct a deep subtree for LHS
    let child1_id = parent_tree.adopt_tree(child1.root, &child1.arena);
    let child2_id = parent_tree.adopt_tree(child2.root, &child2.arena);
    parent_tree.root.append(child1_id, &mut parent_tree.arena);
    parent_tree.root.append(child2_id, &mut parent_tree.arena);

    let rhs = make_leaf("rhs");

    // Combine deep parent_tree with simple rhs
    let combined = parent_tree | rhs;

    let root_group = combined.root().into_group_child().unwrap();
    let group_children: Vec<_> = root_group.children().collect();

    // First item should be "parent", which still contains its 2 sub-children
    let adopted_parent = group_children[0];
    assert_eq!(adopted_parent.name(), Some(&Identifier::from("parent")));

    let adopted_subchildren: Vec<_> = adopted_parent
        .children()
        .filter_map(|c| c.name().map(|id| id.to_string()))
        .collect();

    assert_eq!(adopted_subchildren, vec!["child1", "child2"]);
}

/// Helper to build a `__mu::ops::translate(x, y, z)` transform node
fn make_translation_node(x_mm: f64, y_mm: f64, z_mm: f64) -> Model {
    let args = arguments!(x = x_mm, y = y_mm, z = z_mm);
    Model {
        name: Some(Identifier::from("translate")),
        properties: Properties::from(args),
        attr: Default::default(),
        element: Element::BuiltinWorkpiece(BuiltinWorkbenchKind::Transform),
        creator: None,
    }
}

/// Helper to build a `__mu::geo2d::Circle(radius = 5.0)` input tree
fn make_circle_input_tree(radius_mm: f64) -> ModelTree {
    let args = arguments!(radius = radius_mm);
    ModelTree::new(Model {
        name: Some(Identifier::from("Circle")),
        properties: Properties::from(args),
        attr: Default::default(),
        element: Element::BuiltinWorkpiece(BuiltinWorkbenchKind::Primitive2D),
        creator: None,
    })
}

#[test]
fn test_replace_input_placeholders_multiplicity() {
    // 1. Build Multiplicity template tree:
    // Multiplicity
    //   ├── translate(0, 0, 0)
    //   │     └── InputPlaceholder
    //   ├── translate(0, 10, 0)
    //   │     └── InputPlaceholder
    //   ├── translate(10, 0, 0)
    //   │     └── InputPlaceholder
    //   └── translate(10, 10, 0)
    //         └── InputPlaceholder
    let mut template_tree = ModelTree::new(Model {
        name: Some(Identifier::from("Multiplicity")),
        properties: Properties::new(),
        attr: Default::default(),
        element: Element::Multiplicity,
        creator: None,
    });

    let translations = [
        (0.0, 0.0, 0.0),
        (0.0, 10.0, 0.0),
        (10.0, 0.0, 0.0),
        (10.0, 10.0, 0.0),
    ];

    for (x, y, z) in translations {
        let trans_model = make_translation_node(x, y, z);
        let trans_id = template_tree.arena.new_node(trans_model);

        let placeholder_model = Model {
            name: None,
            properties: Properties::new(),
            attr: Default::default(),
            element: Element::InputPlaceholder,
            creator: None,
        };
        let placeholder_id = template_tree.arena.new_node(placeholder_model);

        // Attach InputPlaceholder under translation, and translation under Multiplicity
        trans_id.append(placeholder_id, &mut template_tree.arena);
        template_tree
            .root
            .append(trans_id, &mut template_tree.arena);
    }

    // 2. Build input tree to replace placeholders (__mu::geo2d::Circle(radius = 5.0))
    let circle_tree = make_circle_input_tree(5.0);

    // 3. Perform replacement
    let result_tree = template_tree.replace_input_placeholders(&circle_tree);

    // 4. Assertions
    let root = result_tree.root();
    assert_eq!(root.element, Element::Multiplicity);

    let branches: Vec<_> = root.children().collect();
    assert_eq!(branches.len(), 4, "Should have 4 translation branches");

    for (idx, branch) in branches.iter().enumerate() {
        // Verify branch is still translate transform
        assert_eq!(branch.name(), Some(&Identifier::from("translate")));
        assert!(matches!(
            branch.element,
            Element::BuiltinWorkpiece(BuiltinWorkbenchKind::Transform)
        ));

        // Verify children under translate
        let branch_children: Vec<_> = branch.children().collect();
        assert_eq!(
            branch_children.len(),
            1,
            "Branch {idx} should have exactly 1 child"
        );

        let replaced_child = branch_children[0];

        // Ensure InputPlaceholder is replaced by the Circle model
        assert_ne!(
            replaced_child.element,
            Element::InputPlaceholder,
            "Placeholder in branch {idx} was not replaced"
        );
        assert_eq!(replaced_child.name(), Some(&Identifier::from("Circle")));
        assert_eq!(
            replaced_child
                .properties
                .get_property("radius")
                .map(|p| &p.value),
            Some(&Value::from(5.0))
        );
    }

    println!("{result_tree}")
}
