use roxmltree::{Document, Node};
use std::fs;
use quote::{quote, format_ident};
use proc_macro2::{Ident, Span};

#[derive(Debug)]
pub enum RestrictionKind {
    FractionDigits(i64),
    TotalDigits(i64),
    MinInclusive(i64),
    MinLength(i64),
    MaxLength(i64),
    Pattern(String),
    Enumeration(String),
}

#[derive(Debug)]
pub struct Restriction {
    pub base: String,
    pub restrictions: Vec<RestrictionKind>,
}

impl<'a, 'input> From<Node<'a, 'input>> for RestrictionKind {
    fn from(node: Node<'a, 'input>) -> Self {
        match node.tag_name().name() {
            "fractionDigits" => {
                Self::FractionDigits(node.attribute("value").unwrap().parse::<i64>().unwrap())
            }
            "totalDigits" => {
                Self::TotalDigits(node.attribute("value").unwrap().parse::<i64>().unwrap())
            }
            "minInclusive" => {
                Self::MinInclusive(node.attribute("value").unwrap().parse::<i64>().unwrap())
            }
            "pattern" => {
                Self::Pattern(node.attribute("value").unwrap().into())
            }
            "enumeration" => {
                Self::Enumeration(node.attribute("value").unwrap().into())
            }
            "minLength" => {
                Self::MinLength(node.attribute("value").unwrap().parse::<i64>().unwrap())
            }
            "maxLength" => {
                Self::MaxLength(node.attribute("value").unwrap().parse::<i64>().unwrap())
            }
            _ => todo!("Not implemented: {}", node.tag_name().name()),
        }
    }
}

impl<'a, 'input> From<Node<'a, 'input>> for Restriction {
    fn from(node: Node<'a, 'input>) -> Self {
        let restriction = node
            .descendants()
            .find(|n| n.has_tag_name("restriction"))
            .expect("Expected restriction.");
        let base = restriction.attribute("base").unwrap();

        let rtype = match base {
            "xs:decimal" => "Decimal",
            "xs:string" => "String",
            "xs:boolean" => "bool",
            "xs:date" => "String", // TODO: do dates properly
            "xs:dateTime" => "String",
            "xs:gYear" => "String",
            "xs:base64Binary" => "String",
            _ => todo!("Unsupported type. {}", base)
        };

        let restrictions = restriction
            .children()
            .filter(|n| !n.is_text())
            .map(RestrictionKind::from)
            .collect::<Vec<_>>();
        Self {
            base: rtype.into(),
            restrictions,
        }
    }
}

#[derive(Debug)]
pub enum SimpleType {
    Restriction {
        name: String,
        kind: Restriction,
    },
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
        proc_macro::TokenStream::from(quote! {
            type #identifier = #base;
        })
    }
}

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

#[derive(Debug)]
pub struct Choice {
    choices: Vec<Element>,
}

impl<'a, 'input> From<Node<'a, 'input>> for Choice {
    fn from(node: Node<'a, 'input>) -> Self {
        Self {
            choices: node
                .children()
                .filter(|n| !n.is_text())
                .map(Element::from)
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Debug)]
pub struct Sequence {
    elements: Vec<Element>,
}

impl<'a, 'input> From<Node<'a, 'input>> for Sequence {
    fn from(node: Node<'a, 'input>) -> Self {
        Self {
            elements: node
                .children()
                .filter(|n| !n.is_text())
                .map(Element::from)
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Debug)]
pub struct Attribute {
    name: String,
    r#type: String,
    r#use: String,
}

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


#[derive(Debug)]
pub struct Schema {
    pub element: Element,
    pub complex_types: Vec<ComplexType>,
    pub simple_types: Vec<SimpleType>,
}

impl<'a, 'input> From<Node<'a, 'input>> for Schema {
    fn from(node: Node<'a, 'input>) -> Self {
        // Find the complex types.
        let complex_types = node
            .descendants()
            .filter(|n| n.has_tag_name("complexType"))
            .map(|n| n.into())
            .collect::<Vec<ComplexType>>();
        let simple_types = node
            .descendants()
            .filter(|n| n.has_tag_name("simpleType"))
            .map(|n| n.into())
            .collect::<Vec<SimpleType>>();
        let element = node
            .descendants()
            .filter(|n| n.has_tag_name("element"))
            .map(Element::from)
            .take(1)
            .next()
            .unwrap();

        Self { element, complex_types, simple_types }
    }
}

pub fn parse_xsd(path: &str) -> Schema {
    let data = fs::read_to_string(path).expect("Can't read path.");
    let document = Document::parse(&data).unwrap();
    Schema::from(document.root())
}