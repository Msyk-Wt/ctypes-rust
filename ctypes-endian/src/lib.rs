
use num_traits::{ToBytes, FromBytes};

/// データエンディアン
pub enum Endian {
    /// リトルエンディアン
    Little,
    /// ビッグエンディアン
    Big,
}

impl Endian {

    pub fn value_from_bytes<V>(&self, bytes: &V::Bytes) -> V
    where V: FromBytes {
        self.get_converter_value_from_bytes()(bytes)
    }

    pub fn bytes_from_value<V>(&self, value: &V) -> V::Bytes
    where V: ToBytes {
        self.get_converter_bytes_from_value()(value)
    }

    pub const fn get_converter_value_from_bytes<V>(&self) -> impl Fn(&V::Bytes) -> V
    where V: FromBytes {
        match self {
            Endian::Little => V::from_le_bytes,
            Endian::Big => V::from_be_bytes,
        }
    }

    pub const fn get_converter_bytes_from_value<V>(&self) -> impl Fn(&V) -> V::Bytes
    where V: ToBytes {
        match self {
            Endian::Little => V::to_le_bytes,
            Endian::Big => V::to_be_bytes,
        }
    }
}
