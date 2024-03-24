use sis_core::{rgbau8_to_rgbaf32, RGBAf32};

use crate::token::Spanned;

pub enum Statement {
    Profile { name: String, body: Vec<Self> },
    Func { name: String, body: Vec<Self> },
    Call{ name: Spanned<String>, args: Vec<Spanned<Value>> },
    Macro { name: String, body: Vec<Self> },
    ColorAnimation { name: String, body: Vec<Spanned<Keyframe>> },
}

pub(crate) struct Keyframe {
    pub(crate) timestamp: Spanned<f32>,
    pub(crate) color: Spanned<Color>
}

impl Into<sis_core::Keyframe> for Keyframe {
    fn into(self) -> sis_core::Keyframe {
        let Keyframe {
            timestamp: (timestamp, _),
            color: (color, _),
        } = self;

        sis_core::Keyframe {
            timestamp,
            color: color.into()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Color(pub Spanned<i32>, pub Spanned<i32>, pub Spanned<i32>, pub Spanned<i32>);

impl Into<RGBAf32> for Color {
    fn into(self) -> RGBAf32 {
        rgbau8_to_rgbaf32(
            (
                self.0.0 as u8,
                self.1.0 as u8,
                self.2.0 as u8,
                self.3.0 as u8
            )
        )
    }
}

pub enum Value {
    Variable{ name: String },
    EnumVariant{ enum_name: String, variant: String },
    Float(f32),
    Integer(i32),
    Color(Color),
    Bool(bool)
}
