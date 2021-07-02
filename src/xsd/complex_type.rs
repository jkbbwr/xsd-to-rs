use proc_macro2::Span;
use quote::quote;
use roxmltree::Node;

use crate::xsd::{Choice, Sequence, SimpleContent};

#[derive(Debug)]
pub enum ComplexTypeKind {
    SimpleContent(SimpleContent),
    Sequence(Sequence),
    Choice(Choice),
}

#[derive(Debug)]
pub struct ComplexType {
    name: String,
    kind: ComplexTypeKind,
}

impl<'a, 'input> From<Node<'a, 'input>> for ComplexType {
    fn from(node: Node<'a, 'input>) -> Self {
        let name = node.attribute("name").unwrap();
        let child = node
            .descendants()
            .find(|n| {
                n.has_tag_name("choice")
                    || n.has_tag_name("simpleContent")
                    || n.has_tag_name("sequence")
            })
            .expect("Unexpected complex type.");

        let complex_type_name = child.tag_name().name();
        let kind = match complex_type_name {
            "choice" => ComplexTypeKind::Choice(child.into()),
            "simpleContent" => ComplexTypeKind::SimpleContent(child.into()),
            "sequence" => ComplexTypeKind::Sequence(child.into()),
            _ => {
                todo!("unreachable?");
            }
        };

        Self {
            name: name.into(),
            kind,
        }
    }
}

impl From<ComplexType> for proc_macro::TokenStream {
    fn from(complex_type: ComplexType) -> Self {
        let identifier = syn::Ident::new(complex_type.name.as_str(), Span::call_site());

        proc_macro::TokenStream::from(quote! {
            struct #identifier;
        })
    }
}
