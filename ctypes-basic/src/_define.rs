
use num_traits::ToPrimitive;
use super::{ByteOrder, error::*};

macro_rules! define_primitive_type {
    (
        $Base:ty, $to_base:ident;
        #[$TypeDoc:meta] $Type:ident;
        #[$DataBufDoc:meta] $DataBuf:ident;
        #[$DataMutDoc:meta] $DataMut:ident;
        #[$DataRefDoc:meta] $DataRef:ident;
    ) => {
        
        #[$TypeDoc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $Type;
        impl $Type {

            /// 型名
            #[inline]
            pub const fn name() -> &'static str {
                stringify!($Base)
            }

            /// バイトサイズ
            #[inline]
            pub const fn byte_size() -> usize {
                std::mem::size_of::<$Base>()
            }

            /// 数値からデータ(実体)を生成
            /// 
            /// # 引数
            /// - numeric: 数値
            /// - endian: バイトオーダー
            /// 
            /// # 戻り値
            /// - 生成結果
            ///   - Ok: データ(実体)
            ///   - Err: エラー
            pub fn data_buf_from_numeric<N: ToPrimitive>(numeric: N, endian: ByteOrder) -> Result<$DataBuf, ArithmeticOverflow> {
                numeric.$to_base()
                    .map(|value| $DataBuf::_new(endian.bytes_from_value(&value), endian))
                    .ok_or(ArithmeticOverflow)
            }

            /// バイト列からデータ(実体)を生成
            /// 
            /// # 引数
            /// - bytes: バイト列
            /// - endian: バイトオーダー
            /// 
            /// # 戻り値
            /// - 生成結果
            ///   - Ok: データ(実体)
            ///   - Err: エラー
            pub fn data_buf_from_bytes(bytes: &[u8], endian: ByteOrder) -> Result<$DataBuf, InvalidByteSize> {
                bytes.try_into()
                    .map(|bytes| $DataBuf::_new(bytes, endian))
                    .map_err(|_| InvalidByteSize)
            }

            /// バイト列からデータ(可変参照)を生成
            /// 
            /// # 引数
            /// - bytes: バイト列
            /// - endian: バイトオーダー
            /// 
            /// # 戻り値
            /// - 生成結果
            ///   - Ok: データ(可変参照)
            ///   - Err: エラー
            pub fn data_mut_from_bytes<'b>(bytes: &'b mut [u8], endian: ByteOrder) -> Result<$DataMut<'b>, InvalidByteSize> {
                bytes.try_into()
                    .map(|bytes| $DataMut::_new(bytes, endian))
                    .map_err(|_| InvalidByteSize)
            }

            /// バイト列からデータ(不変参照)を生成
            /// 
            /// # 引数
            /// - bytes: バイト列
            /// - endian: バイトオーダー
            /// 
            /// # 戻り値
            /// - 生成結果
            ///   - Ok: データ(不変参照)
            ///   - Err: エラー
            pub fn data_ref_from_bytes<'b>(bytes: &'b [u8], endian: ByteOrder) -> Result<$DataRef<'b>, InvalidByteSize> {
                bytes.try_into()
                    .map(|bytes| $DataRef::_new(bytes, endian))
                    .map_err(|_| InvalidByteSize)
            }
        }
        impl core::fmt::Display for $Type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                Self::name().fmt(f)
            }
        }

        #[$DataBufDoc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $DataBuf {
            bytes: [u8; $Type::byte_size()],
            endian: ByteOrder,
        }
        impl $DataBuf {

            /// データ(可変参照)に変換
            #[inline]
            pub fn as_data_mut(&mut self) -> $DataMut<'_> {
                $DataMut::_new(&mut self.bytes, self.endian)
            }

            /// データ(不変参照)に変換
            #[inline]
            pub fn as_data_ref(&self) -> $DataRef<'_> {
                $DataRef::_new(&self.bytes, self.endian)
            }

            /// 数値に変換
            #[inline]
            pub fn to_numeric(&self) -> $Base {
                self.endian.value_from_bytes(&self.bytes)
            }

            /// 数値で書き換え
            /// 
            /// # 引数
            /// - numeric: 数値
            /// 
            /// # 戻り値
            /// - 書き換え結果
            ///   - Ok: 成功
            ///   - Err: エラー
            #[inline]
            pub fn rewrite_numeric<N: ToPrimitive>(&mut self, numeric: N) -> Result<(), ArithmeticOverflow> {
                self.as_data_mut().rewrite_numeric(numeric)
            }

            #[inline]
            pub(crate) fn _new(bytes: [u8; $Type::byte_size()], endian: ByteOrder) -> Self {
                Self {bytes, endian}
            }
        }
        impl core::fmt::Display for $DataBuf {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.to_numeric().fmt(f)
            }
        }
        impl ToPrimitive for $DataBuf {
            fn to_i64(&self) -> Option<i64> {self.to_numeric().to_i64()}
            fn to_u64(&self) -> Option<u64> {self.to_numeric().to_u64()}
            fn to_f64(&self) -> Option<f64> {self.to_numeric().to_f64()}
        }

        #[$DataMutDoc]
        #[derive(Debug, PartialEq, Eq)]
        pub struct $DataMut<'b> {
            bytes: &'b mut [u8; $Type::byte_size()],
            endian: ByteOrder,
        }
        impl<'b> $DataMut<'b> {

            /// データ(実体)に変換
            #[inline]
            pub fn to_data_buf(&self) -> $DataBuf {
                $DataBuf::_new(*self.bytes, self.endian)
            }

            /// データ(不変参照)に変換
            #[inline]
            pub fn as_data_ref(&self) -> $DataRef<'_> {
                $DataRef::_new(self.bytes, self.endian)
            }

            /// 数値に変換
            #[inline]
            pub fn to_numeric(&self) -> $Base {
                self.endian.value_from_bytes(self.bytes)
            }

            /// 数値で書き換え
            /// 
            /// # 引数
            /// - numeric: 数値
            /// 
            /// # 戻り値
            /// - 書き換え結果
            ///   - Ok: 成功
            ///   - Err: エラー
            pub fn rewrite_numeric<N: ToPrimitive>(&mut self, numeric: N) -> Result<(), ArithmeticOverflow> {
                if let Some(value) = numeric.$to_base() {
                    *self.bytes = self.endian.bytes_from_value(&value);
                    Ok(())
                } else {
                    Err(ArithmeticOverflow)
                }
            }

            #[inline]
            pub(crate) fn _new(bytes: &'b mut [u8; $Type::byte_size()], endian: ByteOrder) -> Self {
                Self {bytes, endian}
            }
        }
        impl<'b> core::fmt::Display for $DataMut<'b> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.to_numeric().fmt(f)
            }
        }
        impl<'b> ToPrimitive for $DataMut<'b> {
            fn to_i64(&self) -> Option<i64> {self.to_numeric().to_i64()}
            fn to_u64(&self) -> Option<u64> {self.to_numeric().to_u64()}
            fn to_f64(&self) -> Option<f64> {self.to_numeric().to_f64()}
        }

        #[$DataRefDoc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $DataRef<'b> {
            bytes: &'b [u8; $Type::byte_size()],
            endian: ByteOrder,
        }
        impl<'b> $DataRef<'b> {

            /// データ(実体)に変換
            #[inline]
            pub fn to_data_buf(&self) -> $DataBuf {
                $DataBuf::_new(*self.bytes, self.endian)
            }

            /// 数値に変換
            #[inline]
            pub fn to_numeric(&self) -> $Base {
                self.endian.value_from_bytes(self.bytes)
            }

            #[inline]
            pub(crate) fn _new(bytes: &'b [u8; $Type::byte_size()], endian: ByteOrder) -> Self {
                Self {bytes, endian}
            }
        }
        impl<'b> core::fmt::Display for $DataRef<'b> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.to_numeric().fmt(f)
            }
        }
        impl<'b> ToPrimitive for $DataRef<'b> {
            fn to_i64(&self) -> Option<i64> {self.to_numeric().to_i64()}
            fn to_u64(&self) -> Option<u64> {self.to_numeric().to_u64()}
            fn to_f64(&self) -> Option<f64> {self.to_numeric().to_f64()}
        }
    };
}

macro_rules! define_integer_type {
    (
        $Base:ty, $to_base:ident;
        #[$TypeDoc:meta] $Type:ident;
        #[$DataBufDoc:meta] $DataBuf:ident;
        #[$DataMutDoc:meta] $DataMut:ident;
        #[$DataRefDoc:meta] $DataRef:ident;
    ) => {

        define_primitive_type!(
            $Base, $to_base;
            #[$TypeDoc] $Type;
            #[$DataBufDoc] $DataBuf;
            #[$DataMutDoc] $DataMut;
            #[$DataRefDoc] $DataRef;
        );

        impl core::fmt::UpperHex for $DataBuf {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.to_numeric().fmt(f)
            }
        }
        impl<'b> core::fmt::UpperHex for $DataMut<'b> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.to_numeric().fmt(f)
            }
        }
        impl<'b> core::fmt::UpperHex for $DataRef<'b> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.to_numeric().fmt(f)
            }
        }
        impl core::fmt::LowerHex for $DataBuf {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.to_numeric().fmt(f)
            }
        }
        impl<'b> core::fmt::LowerHex for $DataMut<'b> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.to_numeric().fmt(f)
            }
        }
        impl<'b> core::fmt::LowerHex for $DataRef<'b> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.to_numeric().fmt(f)
            }
        }
    };
}

macro_rules! define_float_type {
    (
        $Base:ty, $to_base:ident;
        #[$TypeDoc:meta] $Type:ident;
        #[$DataBufDoc:meta] $DataBuf:ident;
        #[$DataMutDoc:meta] $DataMut:ident;
        #[$DataRefDoc:meta] $DataRef:ident;
    ) => {

        define_primitive_type!(
            $Base, $to_base;
            #[$TypeDoc] $Type;
            #[$DataBufDoc] $DataBuf;
            #[$DataMutDoc] $DataMut;
            #[$DataRefDoc] $DataRef;
        );

        impl core::fmt::UpperExp for $DataBuf {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.to_numeric().fmt(f)
            }
        }
        impl<'b> core::fmt::UpperExp for $DataMut<'b> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.to_numeric().fmt(f)
            }
        }
        impl<'b> core::fmt::UpperExp for $DataRef<'b> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.to_numeric().fmt(f)
            }
        }
        impl core::fmt::LowerExp for $DataBuf {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.to_numeric().fmt(f)
            }
        }
        impl<'b> core::fmt::LowerExp for $DataMut<'b> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.to_numeric().fmt(f)
            }
        }
        impl<'b> core::fmt::LowerExp for $DataRef<'b> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.to_numeric().fmt(f)
            }
        }
    };
}


define_integer_type!(
    i8, to_i8;
    /// 符号付き8ビット整数型
    Int8Type;
    /// 符号付き8ビット整数データ(実体)
    Int8DataBuf;
    /// 符号付き8ビット整数データ(可変参照)
    Int8DataMut;
    /// 符号付き8ビット整数データ(不変参照)
    Int8DataRef;
);
define_integer_type!(
    i16, to_i16;
    /// 符号付き16ビット整数型
    Int16Type;
    /// 符号付き16ビット整数データ(実体)
    Int16DataBuf;
    /// 符号付き16ビット整数データ(可変参照)
    Int16DataMut;
    /// 符号付き16ビット整数データ(不変参照)
    Int16DataRef;
);
define_integer_type!(
    i32, to_i32;
    /// 符号付き32ビット整数型
    Int32Type;
    /// 符号付き32ビット整数データ(実体)
    Int32DataBuf;
    /// 符号付き32ビット整数データ(可変参照)
    Int32DataMut;
    /// 符号付き32ビット整数データ(不変参照)
    Int32DataRef;
);
define_integer_type!(
    i64, to_i64;
    /// 符号付き64ビット整数型
    Int64Type;
    /// 符号付き64ビット整数データ(実体)
    Int64DataBuf;
    /// 符号付き64ビット整数データ(可変参照)
    Int64DataMut;
    /// 符号付き64ビット整数データ(不変参照)
    Int64DataRef;
);

define_integer_type!(
    u8, to_u8;
    /// 符号無し8ビット整数型
    UInt8Type;
    /// 符号無し8ビット整数データ(実体)
    UInt8DataBuf;
    /// 符号無し8ビット整数データ(可変参照)
    UInt8DataMut;
    /// 符号無し8ビット整数データ(不変参照)
    UInt8DataRef;
);
define_integer_type!(
    u16, to_u16;
    /// 符号無し16ビット整数型
    UInt16Type;
    /// 符号無し16ビット整数データ(実体)
    UInt16DataBuf;
    /// 符号無し16ビット整数データ(可変参照)
    UInt16DataMut;
    /// 符号無し16ビット整数データ(不変参照)
    UInt16DataRef;
);
define_integer_type!(
    u32, to_u32;
    /// 符号無し32ビット整数型
    UInt32Type;
    /// 符号無し32ビット整数データ(実体)
    UInt32DataBuf;
    /// 符号無し32ビット整数データ(可変参照)
    UInt32DataMut;
    /// 符号無し32ビット整数データ(不変参照)
    UInt32DataRef;
);
define_integer_type!(
    u64, to_u64;
    /// 符号無し64ビット整数型
    UInt64Type;
    /// 符号無し64ビット整数データ(実体)
    UInt64DataBuf;
    /// 符号無し64ビット整数データ(可変参照)
    UInt64DataMut;
    /// 符号無し64ビット整数データ(不変参照)
    UInt64DataRef;
);

define_float_type!(
    f32, to_f32;
    /// 浮動小数点32ビット実数型
    Float32Type;
    /// 浮動小数点32ビット実数データ(実体)
    Float32DataBuf;
    /// 浮動小数点32ビット実数データ(可変参照)
    Float32DataMut;
    /// 浮動小数点32ビット実数データ(不変参照)
    Float32DataRef;
);
define_float_type!(
    f64, to_f64;
    /// 浮動小数点64ビット実数型
    Float64Type;
    /// 浮動小数点64ビット実数データ(実体)
    Float64DataBuf;
    /// 浮動小数点64ビット実数データ(可変参照)
    Float64DataMut;
    /// 浮動小数点64ビット実数データ(不変参照)
    Float64DataRef;
);
