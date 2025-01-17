use std::fmt;

enum NestedEnum {
    Outer(OuterEnum),
}

enum OuterEnum {
    Inner(InnerEnum),
}

enum InnerEnum {
    VariantOne,
    VariantTwo(i32),
}

impl fmt::Display for NestedEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NestedEnum::Outer(outer) => write!(f, "{}", outer),
        }
    }
}

impl fmt::Display for OuterEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OuterEnum::Inner(inner) => write!(f, "{}", inner),
        }
    }
}

impl fmt::Display for InnerEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InnerEnum::VariantOne => write!(f, "VariantOne"),
            InnerEnum::VariantTwo(value) => write!(f, "VariantTwo with value {}", value),
        }
    }
}

fn main() {
    let nested = NestedEnum::Outer(OuterEnum::Inner(InnerEnum::VariantTwo(42)));

    match nested {
        NestedEnum::Outer(outer) => match outer {
            OuterEnum::Inner(inner) => match inner {
                InnerEnum::VariantOne => println!("Matched VariantOne"),
                InnerEnum::VariantTwo(value) => println!("Matched VariantTwo with value {}", value),
            },
        },
    }
}