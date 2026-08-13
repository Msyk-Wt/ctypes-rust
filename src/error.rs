
/// エラー
#[derive(Debug, derive_more::Display, derive_more::From)]
pub struct Error(pub(crate) ErrorBase);
impl Error {
    pub fn kind(&self) -> Option<ErrorKind> {
        match &self.0 {
            ErrorBase::Kind(k) |
            ErrorBase::KindWithMessage(k, _) => Some(*k),
            _ => None,
        }
    }
}
impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.0 {
            ErrorBase::Extern(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}
impl<T> Into<Result<T, Error>> for Error {
    fn into(self) -> Result<T, Error> {Err(self)}
}

/// エラー種別
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
pub enum ErrorKind {
    InvalidByteSize,
    DuplicateEnumeratorName,
    DuplicateEnumeratorValue,
}

#[derive(Debug, derive_more::Display, derive_more::From)]
pub(crate) enum ErrorBase {

    #[display("{}", _0)]
    Kind(ErrorKind),

    #[display("{}: {}", _0, _1)]
    KindWithMessage(ErrorKind, String),

    #[from()]
    Extern(Box<dyn core::error::Error>),
}

macro_rules! err {

    ($kind:ident) => {
        $crate::error::Error(
            $crate::error::ErrorBase::Kind(
                $crate::error::ErrorKind::$kind
            )
        ).into()
    };

    ($kind:ident; $($message:tt)*) => {
        $crate::error::Error(
            $crate::error::ErrorBase::KindWithMessage(
                $crate::error::ErrorKind::$kind,
                format!($($message)*)
            )
        ).into()
    };
}

pub(crate) use err;
