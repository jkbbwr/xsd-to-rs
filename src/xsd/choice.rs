use roxmltree::Node;

use crate::xsd::Element;

#[derive(Debug)]
pub struct Choice {
    choices: Vec<Element>,
}

impl<'a, 'input> From<Node<'a, 'input>> for Choice {
    fn from(node: Node<'a, 'input>) -> Self {
        Self {
            choices: node
                .children()
                .filter_map(|n| match !n.is_text() {
                    true => Some(Element::from(n)),
                    false => None,
                })
                .collect::<Vec<_>>(),
        }
    }
}
