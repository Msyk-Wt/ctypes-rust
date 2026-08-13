
use ctypes_primitive::*;
use crate::{Endian, error::*};


macro_rules! def_primitive_type {

    (   $(#[$type_meta:meta])*
        $type_name:ident<$base_type:ty>
    ) => {

        $(#[$type_meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $type_name {
            endian: Endian,
        }

        impl $type_name {

            pub const fn name(&self) -> &'static str {
                <$base_type>::NAME
            }

            pub const fn byte_size(&self) -> usize {
                <$base_type>::SIZE
            }

            pub const fn to_le(&self) -> Self {
                Self::new(Endian::Little)
            }

            pub const fn to_be(&self) -> Self {
                Self::new(Endian::Big)
            }

            pub(crate) const fn new(endian: Endian) -> Self {
                Self { endian }
            }
        }

        impl core::fmt::Display for $type_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Display::fmt(self.name(), f)
            }
        }
    };

    {$( $(#[$type_meta:meta])*
        $type_name:ident<$base_type:ty>;
    )*} => {$(
        def_primitive_type!(
            $(#[$type_meta])*
            $type_name<$base_type>
        );
    )*}
}

def_primitive_type! {
    /// 符号付き8ビット整数型
    Int8Type<I8>;
    /// 符号付き16ビット整数
    Int16Type<I16>;
    /// 符号付き32ビット整数
    Int32Type<I32>;
    /// 符号付き64ビット整数
    Int64Type<I64>;
    /// 符号無し8ビット整数
    Uint8Type<U8>;
    /// 符号無し16ビット整数
    Uint16Type<U16>;
    /// 符号無し32ビット整数
    Uint32Type<U32>;
    /// 符号無し64ビット整数
    Uint64Type<U64>;
    /// 浮動小数点32ビット実数
    Float32Type<F32>;
    /// 浮動小数点64ビット実数
    Float64Type<F64>;
}


macro_rules! def_basic_type {

    (   $(#[$type_meta:meta])*
        $type_name:ident {$(
            $(#[$variant_meta:meta])*
            $variant_name:ident($variant_type:ty),
        )*}
    ) => {

        $(#[$type_meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display, derive_more::From)]
        pub(crate) enum $type_name {
            $(
                $(#[$variant_meta])*
                $variant_name($variant_type),
            )*
        }

        impl $type_name {

            pub const fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant_name(t) => t.name(),)*
                }
            }

            pub const fn byte_size(&self) -> usize {
                match self {
                    $(Self::$variant_name(t) => t.byte_size(),)*
                }
            }

            pub fn to_le(&self) -> Self {
                match self {
                    $(Self::$variant_name(t) => Self::$variant_name(t.to_le()),)*
                }
            }

            pub fn to_be(&self) -> Self {
                match self {
                    $(Self::$variant_name(t) => Self::$variant_name(t.to_be()),)*
                }
            }
        }
    };

    {$( $(#[$type_meta:meta])*
        $type_name:ident {$(
            $(#[$variant_meta:meta])*
            $variant_name:ident($variant_type:ty),
        )*};
    )*} => {$(
        def_basic_type!(
            $(#[$type_meta])*
            $type_name {$(
                $(#[$variant_meta])*
                $variant_name($variant_type),
            )*}
        );
    )*}
}

def_basic_type! {

    /// 符号付き整数型
    SignedType {
        /// 符号付き8ビット整数型
        Bit8(Int8Type),
        /// 符号付き16ビット整数
        Bit16(Int16Type),
        /// 符号付き32ビット整数
        Bit32(Int32Type),
        /// 符号付き64ビット整数
        Bit64(Int64Type),
    };

    /// 符号無し整数型
    UnsignedType {
        /// 符号無し8ビット整数
        Bit8(Uint8Type),
        /// 符号無し16ビット整数
        Bit16(Uint16Type),
        /// 符号無し32ビット整数
        Bit32(Uint32Type),
        /// 符号無し64ビット整数
        Bit64(Uint64Type),
    };

    /// 浮動小数点実数型
    FloatType {
        /// 浮動小数点32ビット実数
        Bit32(Float32Type),
        /// 浮動小数点64ビット実数
        Bit64(Float64Type),
    };

    /// 整数型
    IntegerType {
        /// 符号付き整数型
        #[from(SignedType, Int8Type, Int16Type, Int32Type, Int64Type)]
        Signed(SignedType),
        /// 符号無し整数型
        #[from(UnsignedType, Uint8Type, Uint16Type, Uint32Type, Uint64Type)]
        Unsigned(UnsignedType),
    };

    /// 実数型
    RealType {
        /// 浮動小数点実数型
        #[from(FloatType, Float32Type, Float64Type)]
        Float(FloatType),
    };

    /// 基本型
    BasicType {
        /// 整数型
        #[from(IntegerType, SignedType, UnsignedType, Int8Type, Int16Type, Int32Type, Int64Type, Uint8Type, Uint16Type, Uint32Type, Uint64Type)]
        Integer(IntegerType),
        /// 実数型
        #[from(RealType, FloatType, Float32Type, Float64Type)]
        Real(RealType),
    };
}

impl SignedType {
    pub fn from_byte_size(byte_size: usize, endian: Endian) -> Result<Self, Error> {
        match byte_size {
            1 => Ok(Self::Bit8(Int8Type{endian})),
            2 => Ok(Self::Bit16(Int16Type{endian})),
            4 => Ok(Self::Bit32(Int32Type{endian})),
            8 => Ok(Self::Bit64(Int64Type{endian})),
            _ => err!(InvalidByteSize; "Invalid byte size for SignedType: {}", byte_size),
        }
    }
}

impl UnsignedType {
    pub fn from_byte_size(byte_size: usize, endian: Endian) -> Result<Self, Error> {
        match byte_size {
            1 => Ok(Self::Bit8(Uint8Type{endian})),
            2 => Ok(Self::Bit16(Uint16Type{endian})),
            4 => Ok(Self::Bit32(Uint32Type{endian})),
            8 => Ok(Self::Bit64(Uint64Type{endian})),
            _ => err!(InvalidByteSize; "Invalid byte size for UnsignedType: {}", byte_size),
        }
    }
}
