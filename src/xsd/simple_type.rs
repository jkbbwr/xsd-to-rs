use proc_macro2::Span;
use roxmltree::Node;

use crate::xsd::Restriction;

#[derive(Debug)]
pub enum SimpleType {
    Restriction { name: String, kind: Restriction },
}

impl<'a, 'input> From<Node<'a, 'input>> for SimpleType {
    fn from(node: Node<'a, 'input>) -> Self {
        let name = node.attribute("name").unwrap();
        Self::Restriction {
            name: name.into(),
            kind: node.into(),
        }
    }
}

impl From<SimpleType> for proc_macro::TokenStream {
    fn from(simple_type: SimpleType) -> Self {
        let SimpleType::Restriction { name, kind } = simple_type;
        let identifier = syn::Ident::new(name.as_str(), Span::call_site());
        let base = syn::Ident::new(kind.base.as_str(), Span::call_site());

        proc_macro::TokenStream::from(quote::quote! {
            type #identifier = #base;
        })
    }
}