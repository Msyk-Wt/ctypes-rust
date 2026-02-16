//! エラー


/// 算術オーバーフロー
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArithmeticOverflow;
impl core::fmt::Display for ArithmeticOverflow {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "算術オーバーフロー")
    }
}
impl std::error::Error for ArithmeticOverflow {}


/// 不正なバイトサイズ
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidByteSize;
impl core::fmt::Display for InvalidByteSize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "不正なバイトサイズ")
    }
}
impl core::error::Error for InvalidByteSize {}
