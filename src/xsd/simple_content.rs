use roxmltree::Node;

use crate::xsd::Attribute;

#[derive(Debug)]
pub enum SimpleContent {
    Extension { base: String, attribute: Attribute },
}

impl<'a, 'input> From<Node<'a, 'input>> for SimpleContent {
    fn from(node: Node<'a, 'input>) -> Self {
        let extension = node
            .children()
            .find(|n| n.has_tag_name("extension"))
            .expect("Expected extension");

        let base = extension.attribute("base").unwrap();
        let attribute = extension
            .children()
            .find(|n| n.has_tag_name("attribute"))
            .expect("Expected attribute");

        let name = attribute.attribute("name").unwrap();
        let r#type = attribute.attribute("type").unwrap();
        let r#use = attribute.attribute("use").unwrap();

        Self::Extension {
            base: base.into(),
            attribute: Attribute {
                name: name.into(),
                r#type: r#type.into(),
                r#use: r#use.into(),
            },
        }
    }
}
