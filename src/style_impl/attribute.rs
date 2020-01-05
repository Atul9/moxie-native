use super::{keyword, types::Edges, Attribute, AttributeHasValue, Border, Length};
use crate::layout::{LogicalLength, LogicalSideOffsets};
use crate::style::{ComputedValues, Direction, DisplayType};
use crate::Color;

macro_rules! define_attribute {
    (
        $name:ident($class:ident) {
            $(
                $value:ty => |$id_values:ident, $id_value:ident $(, $id_parent:ident)*| $expr:block
            )+
        }
    ) => {
        pub struct $class;

        impl Attribute for $class {}

        $(
            impl AttributeHasValue<$value> for $class {
                $(#[illicit::from_env($id_parent: &ComputedValues)])*
                fn set(&self, $id_values: &mut ComputedValues, $id_value: $value) {
                    $expr;
                }
            }
        )+

        pub fn $name() -> $class {
            $class
        }
    }
}

define_attribute! {
    text_size(TextSize) {
        Length => |values, value| {
            values.text_size = value.into()
        }
    }
}

define_attribute! {
    text_color(TextColor) {
        Color => |values, value| {
            values.text_color = value
        }
    }
}

define_attribute! {
    background_color(BackgroundColor) {
        Color => |values, value| {
            values.background_color = value
        }
    }
}

define_attribute! {
    width(Width) {
        Length => |values, value| {
            if let DisplayType::Block(ref mut block) = values.display {
                block.width = Some(value.into());
            }
        }
    }
}

define_attribute! {
    height(Height) {
        Length => |values, value| {
            if let DisplayType::Block(ref mut block) = values.display {
                block.height = Some(value.into());
            }
        }
    }
}

define_attribute! {
    display(DisplayAttr) {
        keyword::Block => |values, _value| {
            values.display = DisplayType::Block(Default::default())
        }
    }
}

define_attribute! {
    direction(DirectionAttr) {
        keyword::Horizontal => |values, _value| {
            if let DisplayType::Block(ref mut block) = values.display {
                block.direction = Direction::Horizontal;
            }
        }
        keyword::Vertical => |values, _value| {
            if let DisplayType::Block(ref mut block) = values.display {
                block.direction = Direction::Vertical;
            }
        }
    }
}

define_attribute! {
    padding(Padding) {
        Length => |values, value| {
            if let DisplayType::Block(ref mut block) = values.display {
                block.padding = LogicalSideOffsets::from_length_all_same(value.into());
            }
        }
        Edges<Length> => |values, value, parent| {
            if let DisplayType::Block(ref mut block) = values.display {
                let parent_values = if let DisplayType::Block(ref parent_block) = parent.display {
                    parent_block.padding
                } else {
                    LogicalSideOffsets::default()
                };
                block.padding = LogicalSideOffsets::from_lengths(
                    value.top.map(Into::into).unwrap_or(LogicalLength::new(parent_values.left)),
                    value.right.map(Into::into).unwrap_or(LogicalLength::new(parent_values.right)),
                    value.bottom.map(Into::into).unwrap_or(LogicalLength::new(parent_values.bottom)),
                    value.left.map(Into::into).unwrap_or(LogicalLength::new(parent_values.left)),
                );
            }
        }
    }
}

define_attribute! {
    margin(Margin) {
        Length => |values, value| {
            if let DisplayType::Block(ref mut block) = values.display {
                block.margin = LogicalSideOffsets::from_length_all_same(value.into());
            }
        }
        Edges<Length> => |values, value, parent| {
            if let DisplayType::Block(ref mut block) = values.display {
                let parent_values = if let DisplayType::Block(ref parent_block) = parent.display {
                    parent_block.margin
                } else {
                    LogicalSideOffsets::default()
                };
                block.margin = LogicalSideOffsets::from_lengths(
                    value.top.map(Into::into).unwrap_or(LogicalLength::new(parent_values.left)),
                    value.right.map(Into::into).unwrap_or(LogicalLength::new(parent_values.right)),
                    value.bottom.map(Into::into).unwrap_or(LogicalLength::new(parent_values.bottom)),
                    value.left.map(Into::into).unwrap_or(LogicalLength::new(parent_values.left)),
                );
            }
        }
    }
}

define_attribute! {
    border(BorderAttr) {
        Border => |values, value| {
            values.border_color = value.color;
            values.border_thickness = LogicalSideOffsets::from_length_all_same(value.width.into());
            let _ = value.style;
        }
        Edges<Border> => |values, value, parent| {
            values.border_color = value.left.map(|x| x.color).unwrap_or(parent.border_color);
            values.border_thickness = LogicalSideOffsets::from_lengths(
                value.top.map(|b| b.width.into()).unwrap_or(LogicalLength::new(parent.border_thickness.top)),
                value.right.map(|b| b.width.into()).unwrap_or(LogicalLength::new(parent.border_thickness.right)),
                value.bottom.map(|b| b.width.into()).unwrap_or(LogicalLength::new(parent.border_thickness.bottom)),
                value.left.map(|b| b.width.into()).unwrap_or(LogicalLength::new(parent.border_thickness.left))
            );
        }
    }
}

/*
display
direction
text_size
text_color
font_family
font_weight
background_color
border_radius
border_thickness
border_color
padding
margin
width
height
*/
