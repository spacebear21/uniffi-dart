#[macro_use]
mod macros;
mod boolean;
mod duration;
mod string;

use crate::gen::render::{Renderable, TypeHelperRenderer};
use crate::gen::CodeType;
use genco::prelude::*;
use paste::paste;
use uniffi_bindgen::pipeline::general::nodes::{Literal, Radix, Type, TypeNode};

pub use boolean::BooleanCodeType;
pub use duration::DurationCodeType;
pub use string::StringCodeType;

fn render_literal(literal: &Literal) -> String {
    fn typed_number(type_node: &TypeNode, num_str: String) -> String {
        match &type_node.ty {
            Type::Int8
            | Type::UInt8
            | Type::Int16
            | Type::UInt16
            | Type::Int32
            | Type::UInt32
            | Type::UInt64
            | Type::Float32
            | Type::Float64
            | Type::Duration => num_str,
            _ => panic!("Unexpected literal: {num_str} is not a number"),
        }
    }

    match literal {
        Literal::Boolean(v) => format!("{v}"),
        Literal::String(s) => format!("'{s}'"),
        Literal::Int(i, radix, type_node) => typed_number(
            type_node,
            match radix {
                Radix::Octal => format!("{i:#x}"),
                Radix::Decimal => format!("{i}"),
                Radix::Hexadecimal => format!("{i:#x}"),
            },
        ),
        Literal::UInt(i, radix, type_node) => typed_number(
            type_node,
            match radix {
                Radix::Octal => format!("{i:#x}"),
                Radix::Decimal => format!("{i}"),
                Radix::Hexadecimal => format!("{i:#x}"),
            },
        ),
        Literal::Float(string, type_node) => typed_number(type_node, string.clone()),
        _ => unreachable!("Literal"),
    }
}

impl_code_type_for_primitive!(BytesCodeType, "Uint8List", "Uint8List");
impl_code_type_for_primitive!(Int8CodeType, "int", "Int8");
impl_code_type_for_primitive!(Int16CodeType, "int", "Int16");
impl_code_type_for_primitive!(Int32CodeType, "int", "Int32");
impl_code_type_for_primitive!(Int64CodeType, "int", "Int64");
impl_code_type_for_primitive!(UInt8CodeType, "int", "UInt8");
impl_code_type_for_primitive!(UInt16CodeType, "int", "UInt16");
impl_code_type_for_primitive!(UInt32CodeType, "int", "UInt32");
impl_code_type_for_primitive!(UInt64CodeType, "int", "UInt64");
impl_code_type_for_primitive!(Float32CodeType, "double", "Double32");
impl_code_type_for_primitive!(Float64CodeType, "double", "Double64");

impl_renderable_for_primitive!(BytesCodeType, "Uint8List", "Uint8List");
impl_renderable_for_primitive!(Int8CodeType, "int", "Int8", 1, -128, 127, "i8");
impl_renderable_for_primitive!(Int16CodeType, "int", "Int16", 2, -32768, 32767, "i16");
impl_renderable_for_primitive!(
    Int32CodeType,
    "int",
    "Int32",
    4,
    -2147483648,
    2147483647,
    "i32"
);
impl_renderable_for_primitive!(
    Int64CodeType,
    "int",
    "Int64",
    8,
    -9223372036854775808,
    9223372036854775807,
    "i64"
);
impl_renderable_for_primitive!(UInt8CodeType, "int", "UInt8", 1, 0, 255, "u8");
impl_renderable_for_primitive!(UInt16CodeType, "int", "UInt16", 2, 0, 65535, "u16");
impl_renderable_for_primitive!(UInt32CodeType, "int", "UInt32", 4, 0, 4294967295, "u32");
impl_renderable_for_primitive!(Float32CodeType, "double", "Double32", 4);
impl_renderable_for_primitive!(Float64CodeType, "double", "Double64", 8);

// Keep u64 on the legacy int path for now; full upper-bound validation lands with BigInt support.
impl Renderable for UInt64CodeType {
    fn render_type_helper(&self, _type_helper: &dyn TypeHelperRenderer) -> dart::Tokens {
        let cl_name = &self.ffi_converter_name();
        let type_signature = &self.type_label();

        quote! {
            class $cl_name {
                static $type_signature lift($type_signature value) => value;

                static LiftRetVal<$type_signature> read(Uint8List buf) {
                    return LiftRetVal(buf.buffer.asByteData(buf.offsetInBytes).getUint64(0), 8);
                }

                static $type_signature lower($type_signature value) {
                    if (value < 0) {
                        throw ArgumentError("Value out of range for u64: " + value.toString());
                    }
                    return value;
                }

                static int allocationSize([$type_signature value = 0]) {
                    return 8;
                }

                static int write($type_signature value, Uint8List buf) {
                    buf.buffer.asByteData(buf.offsetInBytes).setUint64(0, lower(value));
                    return 8;
                }
            }
        }
    }
}
