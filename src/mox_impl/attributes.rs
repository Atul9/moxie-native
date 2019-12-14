use crate::dom::attributes::*;

macro_rules! attribute {
    ($name:ident -> $class:ty) => {
        pub fn $name() -> $class {
            Default::default()
        }
    };
}

attribute!(style -> AttrStyle);
attribute!(title -> AttrTitle);
