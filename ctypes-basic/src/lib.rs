pub mod error;
pub mod data;
mod _define;

pub use ctypes_endian::ByteOrder;
pub use _define::{
    Int8Type, Int16Type, Int32Type, Int64Type,
    UInt8Type, UInt16Type, UInt32Type, UInt64Type,
    Float32Type, Float64Type,
};
