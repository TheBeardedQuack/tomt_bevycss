use super::*;
use crate::{
    property::PropertyToken,
    selector::SelectorElement,
};

#[test]
fn parse_empty(
    // no args
) {
    assert!(
        StyleSheetParser::parse("").is_empty(),
        "Should return an empty list of rules"
    );
    assert!(
        StyleSheetParser::parse("{}").is_empty(),
        "\"{{}}\" Should return an empty list of rules"
    );
    assert!(
        StyleSheetParser::parse(" {}").is_empty(),
        "\" {{}}\" Should return an empty list of rules"
    );
    assert!(
        StyleSheetParser::parse("# {}").is_empty(),
        "\"# {{}}\" Should return an empty list of rules"
    );
    assert!(
        StyleSheetParser::parse("@@@ {}").is_empty(),
        "Should return an empty list of rules"
    );
    assert!(
        StyleSheetParser::parse("{}{}").is_empty(),
        "Should return an empty list of rules"
    );
}

#[test]
fn parse_single_name_selector_no_property(
    // no args
) {
    let rules = StyleSheetParser::parse("#id {}");
    assert_eq!(rules.len(), 1, "Should have a single rule");

    let rule = &rules[0];
    let tree = rule.selector.get_parent_tree();
    assert_eq!(tree.len(), 1, "Should have a single selector node");

    let node = &tree[0];
    assert_eq!(tree.len(), 1, "Should have a single selector");

    match node[0]
    {
        SelectorElement::Name(name) => assert_eq!(name, "id"),
        _ => panic!("Should have a name selector"),
    }

    assert!(rule.properties.is_empty(), "Should have no token");
}

#[test]
fn parse_single_class_selector_no_property(
    // no args
) {
    let rules = StyleSheetParser::parse(".class {}");
    assert_eq!(rules.len(), 1, "Should have a single rule");

    let rule = &rules[0];
    let tree = rule.selector.get_parent_tree();
    assert_eq!(tree.len(), 1, "Should have a single selector node");

    let node = &tree[0];
    assert_eq!(tree.len(), 1, "Should have a single selector");

    match node[0]
    {
        SelectorElement::Class(name) => assert_eq!(name, "class"),
        _ => panic!("Should have a class selector"),
    }

    assert!(rule.properties.is_empty(), "Should have no token");
}

#[test]
fn parse_single_component_selector_no_property(
    // no args
) {
    let rules = StyleSheetParser::parse("button {}");
    assert_eq!(rules.len(), 1, "Should have a single rule");

    let rule = &rules[0];
    let tree = rule.selector.get_parent_tree();
    assert_eq!(tree.len(), 1, "Should have a single selector node");

    let node = &tree[0];
    assert_eq!(tree.len(), 1, "Should have a single selector");

    match node[0]
    {
        SelectorElement::Component(name) => assert_eq!(name, "button"),
        _ => panic!("Should have a class selector"),
    }

    assert!(rule.properties.is_empty(), "Should have no token");
}

#[test]
fn parse_single_complex_class_selector_no_property(
    // no args
) {
    let rules = StyleSheetParser::parse(".a.b.c.d.e.f.g {}");
    assert_eq!(rules.len(), 1, "Should have a single rule");

    let rule = &rules[0];
    let tree = rule.selector.get_parent_tree();
    assert_eq!(tree.len(), 1, "Should have a single selector node");

    let node = &tree[0];
    assert_eq!(node.len(), 7, "Should have a 7 selector class");

    use SelectorElement::*;
    let expected: DynArray<SelectorElement> = smallvec![
        Class("a".to_string()),
        Class("b".to_string()),
        Class("c".to_string()),
        Class("d".to_string()),
        Class("e".to_string()),
        Class("f".to_string()),
        Class("g".to_string()),
    ];

    expected.into_iter()
        .zip(node)
        .for_each(|(expected, element)|
        {
            assert_eq!(expected, **element);
        });

    assert!(rule.properties.is_empty(), "Should have no token");
}

#[test]
fn parse_single_composed_selector_no_property(
    // no args
) {
    let rules = StyleSheetParser::parse("a.b#c.d {}");
    assert_eq!(rules.len(), 1, "Should have a single rule");

    let rule = &rules[0];
    let tree = rule.selector.get_parent_tree();
    assert_eq!(tree.len(), 1, "Should have a single selector node");

    let node = &tree[0];
    assert_eq!(node.len(), 4, "Should have a 4 selectors");

    use SelectorElement::*;
    let expected: DynArray<SelectorElement> = smallvec![
        Component("a".to_string()),
        Class("b".to_string()),
        Name("c".to_string()),
        Class("d".to_string()),
    ];

    expected.into_iter()
        .zip(node)
        .for_each(|(expected, element)|
        {
            assert_eq!(expected, **element);
        });

    assert!(rule.properties.is_empty(), "Should have no token");
}

#[test]
fn parse_multiple_composed_selector_no_property(
    // no args
) {
    let rules = StyleSheetParser::parse("a.b #c .d e#f .g.h i j.k#l {}");
    assert_eq!(rules.len(), 1, "Should have a single rule");

    let rule = &rules[0];
    let tree = rule.selector.get_parent_tree();
    assert_eq!(tree.len(), 7, "Should have a single selector node");

    use SelectorElement::*;
    let expected: DynArray<DynArray<SelectorElement>> = smallvec![
        smallvec![Component("a".to_string()), Class("b".to_string())],
        smallvec![Name("c".to_string())],
        smallvec![Class("d".to_string())],
        smallvec![Component("e".to_string()), Name("f".to_string())],
        smallvec![Class("g".to_string()), Class("h".to_string())],
        smallvec![Component("i".to_string())],
        smallvec![
            Component("j".to_string()),
            Class("k".to_string()),
            Name("l".to_string())
        ],
    ];

    expected.into_iter()
        .zip(tree)
        .for_each(|(node_expected, node)|
        {
            node_expected
                .into_iter()
                .zip(node)
                .for_each(|(expected, element)|
                {
                    assert_eq!(expected, *element);
                });
        });

    assert!(rule.properties.is_empty(), "Should have no properties");
}

#[test]
fn parse_single_token(
    // no args
) {
    let rules = StyleSheetParser::parse("a {b: c}");
    assert_eq!(rules.len(), 1, "Should have a single rule");

    let properties = &rules[0].properties;

    assert_eq!(properties.len(), 1, "Should have a single property");
    assert!(
        properties.contains_key(&"b".to_string()),
        "Should have a property named \"b\""
    );

    let values = properties.get(&"b".to_string()).unwrap();
    assert_eq!(values.len(), 1, "Should have a single property value");

    match &values[0]
    {
        PropertyToken::Identifier(ident) => assert_eq!(ident, "c"),
        _ => panic!("Should have a property value of type identifier token"),
    }
}

#[test]
fn parse_multiple_complex_properties(
    // no args
) {
    let rules = StyleSheetParser::parse(
        r#"a {
        b: c;
        d: 0px;
        e: #f; 
        g: h i j; 
        k-k: 100%;
        l: 15.3px 3%;
        m: 12.9;
        n: "str";
        o: p q #r #s "t" 1 45.67% 33px;
        }"#,
    );

    assert_eq!(rules.len(), 1, "Should have a single rule");

    let properties = &rules[0].properties;

    use PropertyToken::*;
    let expected = [
        ("b", vec![Identifier("c".to_string())]),
        ("d", vec![Dimension(0.0)]),
        ("e", vec![Hash("f".to_string())]),
        (
            "g",
            vec![
                Identifier("h".to_string()),
                Identifier("i".to_string()),
                Identifier("j".to_string()),
            ],
        ),
        ("k-k", vec![Percentage(100.0)]),
        ("l", vec![Dimension(15.3), Percentage(3.0)]),
        ("m", vec![Number(12.9)]),
        ("n", vec![String("str".to_string())]),
        (
            "o",
            vec![
                Identifier("p".to_string()),
                Identifier("q".to_string()),
                Hash("r".to_string()),
                Hash("s".to_string()),
                String("t".to_string()),
                Number(1.0),
                Percentage(45.67),
                Dimension(33.0),
            ],
        ),
    ];

    assert_eq!(properties.len(), expected.len(), "{:?}", properties);
    expected.into_iter()
        .for_each(|(name, values)|
        {
            assert!(properties.contains_key(name));
            values.iter()
                .zip(properties.get(name).unwrap().iter())
                .for_each(|(expected, token)|
                {
                    assert_eq!(token, expected);
                })
        });
}

#[test]
fn parse_multiple_rules(
    // no args
) {
    let rules = StyleSheetParser::parse(r#"a{a:a}a{a:a}a{a:a}a{a:a}"#);

    assert_eq!(rules.len(), 4, "Should have 4 rules");

    for rule in rules
    {
        match rule.selector.get_parent_tree()[0][0]
        {
            SelectorElement::Component(a) => assert_eq!(a, "a"),
            _ => panic!("Should have only a single component \"a\""),
        }

        match rule.properties.get(&"a".to_string())
            .expect("Should have a single property named \"a\"")
            .iter()
            .next()
            .expect("Should have a single property value")
        {
            PropertyToken::Identifier(a) => assert_eq!(a, "a"),
            _ => panic!("Should have only a single property value of type identifier"),
        }
    }
}
