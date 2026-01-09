
use num_traits::{FromBytes, ToBytes};

/// バイトオーダー
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteOrder {
    /// リトルエンディアン
    Little,
    /// ビッグエンディアン
    Big,
}
impl ByteOrder {

    /// バイト列から値を生成
    /// 
    /// # 引数
    /// - bytes: バイト列
    /// 
    /// # 戻り値
    /// - 値
    /// 
    /// # 例
    /// ```
    /// let byte_order = ctypes_endian::ByteOrder::Little;
    /// let bytes = [0x78, 0x56, 0x34, 0x12];
    /// let value = byte_order.from_bytes::<u32>(&bytes);
    /// assert_eq!(value, 0x12345678);
    /// ```
    pub fn from_bytes<V: FromBytes>(self, bytes: &V::Bytes) -> V {
        match self {
            Self::Little => V::from_le_bytes(bytes),
            Self::Big => V::from_be_bytes(bytes),
        }
    }

    /// 値からバイト列を生成
    /// 
    /// # 引数
    /// - value: 値
    /// 
    /// # 戻り値
    /// - バイト列
    /// 
    /// # 例
    /// ```
    /// let byte_order = ctypes_endian::ByteOrder::Big;
    /// let value: u32 = 0x12345678;
    /// let bytes = byte_order.to_bytes(&value);
    /// assert_eq!(bytes, [0x12, 0x34, 0x56, 0x78]);
    /// ```
    pub fn to_bytes<V: ToBytes>(self, value: &V) -> V::Bytes {
        match self {
            Self::Little => V::to_le_bytes(value),
            Self::Big => V::to_be_bytes(value),
        }
    }
}
