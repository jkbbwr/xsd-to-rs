// grab the modules
mod attribute;
mod choice;
mod complex_type;
mod element;
mod restriction;
mod schema;
mod sequence;
mod simple_content;
mod simple_type;

// expose the components
pub use attribute::Attribute;
pub use choice::Choice;
pub use complex_type::ComplexType;
pub use complex_type::ComplexTypeKind;
pub use element::Element;
pub use restriction::Restriction;
pub use restriction::RestrictionKind;
pub use schema::Schema;
pub use sequence::Sequence;
pub use simple_content::SimpleContent;
pub use simple_type::SimpleType;

pub fn parse_xsd(path: &str) -> Schema {
    let data = std::fs::read_to_string(path).expect("Can't read path.");
    let document = roxmltree::Document::parse(&data).unwrap();

    Schema::from(document.root())
}
