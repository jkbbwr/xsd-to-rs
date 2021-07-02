use roxmltree::Node;

#[derive(Debug)]
pub enum Element {
    Element { name: String, value: String },
    Any,
}

impl<'a, 'input> From<Node<'a, 'input>> for Element {
    fn from(node: Node<'a, 'input>) -> Self {
        if node.has_tag_name("any") {
            Element::Any
        } else {
            let name = node.attribute("name").unwrap();
            let value = node.attribute("type").unwrap();
            Self::Element {
                name: name.into(),
                value: value.into(),
            }
        }
    }
}
