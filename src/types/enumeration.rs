
use crate::{Endian, error::*, environment::*, types::SignedType};


/// 列挙型
#[derive(Debug, Clone)]
pub struct EnumerationType {
    pub(crate) key: usize,
    pub(crate) env: Environment,
}
impl EnumerationType {

    /// バイトサイズを取得する
    pub fn byte_size(&self) -> usize {
        let env = self.env.as_ref();
        self.get_entry(&env).byte_size()
    }

    pub(crate) fn new(key: usize, env: Environment) -> Self {
        Self { key, env }
    }

    fn get_entry<'a>(&self, env: &'a EnvBase) -> &'a EnumerationTypeEntry {
        &env.type_table.enumeration[self.key]
    }
}
impl core::fmt::Display for EnumerationType {

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let env = self.env.as_ref();
        self.get_entry(&env).display(f)
    }
}


/// 列挙型ビルダー
#[derive(Debug, Clone)]
pub struct EnumerationTypeBuilder {
    pub(crate) name: Option<String>,
    pub(crate) value_type: SignedType,
    pub(crate) enumerators: Vec<Enumerator>,
    pub(crate) env: Environment,
}
impl EnumerationTypeBuilder {

    /// 名称を設定する
    pub fn name<S: core::fmt::Display>(mut self, name: S) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// 列挙子を追加する
    pub fn enumerator<S: core::fmt::Display>(mut self, name: S, value: i64) -> Result<Self, Error> {
        if self.enumerators.iter().any(|e| e.name == name.to_string()) {
            return err!(DuplicateEnumeratorName; "列挙子名`{}`が重複しています", name.to_string());
        }
        if self.enumerators.iter().any(|e| e.value == value) {
            return err!(DuplicateEnumeratorValue; "列挙子の値`{}`が重複しています", value);
        }
        self.enumerators.push(Enumerator {
            name: name.to_string(),
            value,
        });
        Ok(self)
    }

    /// エンディアンを設定する
    pub fn endian(mut self, endian: Endian) -> Self {
        self.value_type = match endian {
            Endian::Little => self.value_type.to_le(),
            Endian::Big => self.value_type.to_be(),
        };
        self
    }

    /// 列挙型を構築する
    pub fn build(self) -> Result<EnumerationType, Error> {
        let entry = EnumerationTypeEntry {
            name: self.name,
            value_type: self.value_type,
            enumerators: self.enumerators,
        };
        let mut env = self.env.as_mut();
        let entries = &mut env.type_table.enumeration;
        for (i, e) in entries.iter().enumerate() {
            if entry == *e {
                return Ok(EnumerationType::new(i, self.env.clone()));
            }
        }
        let key = entries.len();
        entries.push(entry);
        Ok(EnumerationType::new(key, self.env.clone()))
    }

    pub(crate) fn new(byte_size: usize, env: Environment) -> Result<Self, Error> {
        Ok(Self {
            name: None,
            value_type: SignedType::from_byte_size(byte_size, env.default_endian())?,
            enumerators: Vec::new(),
            env
        })
    }
}


/// 列挙子
#[derive(Debug, Clone, PartialEq)]
pub struct Enumerator {
    pub(crate) name: String,
    pub(crate) value: i64,
}


#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EnumerationTypeEntry {
    pub name: Option<String>,
    pub value_type: SignedType,
    pub enumerators: Vec<Enumerator>,
}
impl EnumerationTypeEntry {

    pub fn byte_size(&self) -> usize {
        self.value_type.byte_size()
    }

    pub fn display(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "enum {} {{ ", name)?;
        } else {
            write!(f, "enum {{ ")?;
        };
        for (i, enumerator) in self.enumerators.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{} = {}", enumerator.name, enumerator.value)?;
        }
        write!(f, " }}")?;
        Ok(())
    }
}
