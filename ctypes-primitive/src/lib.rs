
pub use ctypes_endian::Endian;

macro_rules! map {
    (I8) => {i8};
    (I16) => {i16};
    (I32) => {i32};
    (I64) => {i64};
    (U8) => {u8};
    (U16) => {u16};
    (U32) => {u32};
    (U64) => {u64};
    (F32) => {f32};
    (F64) => {f64};
}

macro_rules! def_primitive_type {

    (   $(#[$type_meta:meta])*
        $type_name:ident
    ) => {

        $(#[$type_meta])*
        #[derive(Debug, Clone, Copy,
            derive_more::Display, derive_more::From,
            derive_more::Deref, derive_more::DerefMut,
            derive_more::LowerExp, derive_more::UpperExp)]
        pub struct $type_name(map!($type_name));
        impl $type_name {

            pub const NAME: &'static str = stringify!(map!($type_name));

            pub const SIZE: usize = core::mem::size_of::<map!($type_name)>();

            pub fn from_bytes(bytes: &[u8; Self::SIZE], endian: Endian) -> Self {
                Self(endian.value_from_bytes(bytes))
            }

            pub fn to_bytes(&self, endian: Endian) -> [u8; Self::SIZE] {
                endian.bytes_from_value(&self.0)
            }
        }
        impl num_traits::FromBytes for $type_name {
            type Bytes = [u8; Self::SIZE];
            fn from_le_bytes(bytes: &Self::Bytes) -> Self {Self(<map!($type_name)>::from_le_bytes(*bytes))}
            fn from_be_bytes(bytes: &Self::Bytes) -> Self {Self(<map!($type_name)>::from_be_bytes(*bytes))}
        }
        impl num_traits::ToBytes for $type_name {
            type Bytes = [u8; Self::SIZE];
            fn to_le_bytes(&self) -> Self::Bytes {self.0.to_le_bytes()}
            fn to_be_bytes(&self) -> Self::Bytes {self.0.to_be_bytes()}
        }
    };
}

macro_rules! def_integer_type {

    (   $(#[$type_meta:meta])*
        $type_name:ident
    ) => {

        def_primitive_type!(
            $(#[$type_meta])*
            #[derive(Hash, derive_more::LowerHex, derive_more::UpperHex)]
            $type_name
        );
    };
}

def_integer_type!(
    /// 符号付き8ビット整数
    I8
);
def_integer_type!(
    /// 符号付き16ビット整数
    I16
);
def_integer_type!(
    /// 符号付き32ビット整数
    I32
);
def_integer_type!(
    /// 符号付き64ビット整数
    I64
);
def_integer_type!(
    /// 符号無し8ビット整数
    U8
);
def_integer_type!(
    /// 符号無し16ビット整数
    U16
);
def_integer_type!(
    /// 符号無し32ビット整数
    U32
);
def_integer_type!(
    /// 符号無し64ビット整数
    U64
);
def_primitive_type!(
    /// 浮動小数点32ビット実数
    F32
);
def_primitive_type!(
    /// 浮動小数点64ビット実数
    F64
);
