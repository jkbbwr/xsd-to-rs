use roxmltree::Node;

use crate::xsd::Element;

#[derive(Debug)]
pub struct Sequence {
    elements: Vec<Element>,
}

impl<'a, 'input> From<Node<'a, 'input>> for Sequence {
    fn from(node: Node<'a, 'input>) -> Self {
        Self {
            elements: node
                .children()
                .filter_map(|n| match !n.is_text() {
                    true => Some(n.into()),
                    false => None,
                })
                .collect::<Vec<_>>(),
        }
    }
}