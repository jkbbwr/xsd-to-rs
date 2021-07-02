use roxmltree::Node;

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
            "pattern" => Self::Pattern(node.attribute("value").unwrap().into()),
            "enumeration" => Self::Enumeration(node.attribute("value").unwrap().into()),
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
            _ => todo!("Unsupported type. {}", base),
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
