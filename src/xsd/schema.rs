use roxmltree::Node;

use crate::xsd::{Element, ComplexType, SimpleType};

#[derive(Debug)]
pub struct Schema {
    pub element: Element,
    pub complex_types: Vec<ComplexType>,
    pub simple_types: Vec<SimpleType>,
}

impl<'a, 'input> From<Node<'a, 'input>> for Schema {
    fn from(node: Node<'a, 'input>) -> Self {
        let complex_types = node
            .descendants()
            .filter_map(|n| match n.has_tag_name("complexType") {
                true => Some(n.into()),
                false => None,
            })
            .collect::<Vec<_>>();

        let simple_types = node
            .descendants()
            .filter_map(|n| match n.has_tag_name("simpleType") {
                true => Some(n.into()),
                false => None,
            })
            .collect::<Vec<_>>();

        let element = node
            .descendants()
            .filter_map(|n| match n.has_tag_name("element") {
                true => Some(n.into()),
                false => None,
            })
            .take(1)
            .next()
            .unwrap();

        Self {
            element,
            complex_types,
            simple_types,
        }
    }
}