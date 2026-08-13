
use crate::{environment::*, table::*};
use super::{Type, TypeEntryRef};

/// ポインタ型
#[derive(Debug, Clone)]
pub struct PointerType {
    pub(crate) key: usize,
    pub(crate) env: Environment,
}
impl PointerType {

    /// バイトサイズを取得する
    pub fn byte_size(&self) -> usize {
        let env = self.env.as_ref();
        self.get_entry(&env).byte_size(&env)
    }

    /// 参照先のデータ型を取得する
    pub fn referenced_type(&self) -> Type {
        let env = self.env.as_ref();
        self.get_entry(&env).referenced_type_key().make_type(self.env.clone())
    }

    pub(crate) fn new(key: usize, env: Environment) -> Self {
        Self { key, env }
    }

    fn get_entry<'a>(&self, env: &'a EnvBase) -> &'a PointerTypeEntry {
        &env.type_table.pointer[self.key]
    }
}
impl core::fmt::Display for PointerType {

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let env = self.env.as_ref();
        self.get_entry(&env).display(f, &env)
    }
}

#[derive(Debug, Clone)]
pub struct PointerTypeBuilder {
    referenced_type_key: Key,
    endian: crate::Endian,
    env: Environment,
}
impl PointerTypeBuilder {

    pub fn endian(mut self, endian: crate::Endian) -> Self {
        self.endian = endian;
        self
    }

    pub fn build(self) -> PointerType {
        let entry = PointerTypeEntry {
            referenced_type_key: self.referenced_type_key,
            endian: self.endian,
        };
        let mut env = self.env.as_mut();
        let entries = &mut env.type_table.pointer;
        for (i, e) in entries.iter().enumerate() {
            if entry == *e {
                return PointerType::new(i, self.env.clone());
            }
        }
        let key = entries.len();
        entries.push(entry);
        PointerType::new(key, self.env.clone())
    }

    pub(crate) fn new(referenced_type_key: Key, env: Environment) -> Self {
        Self {
            referenced_type_key,
            endian: env.default_endian(),
            env
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PointerTypeEntry {
    pub referenced_type_key: Key,
    pub endian: crate::Endian,
}
impl PointerTypeEntry {

    pub fn byte_size(&self, env: &EnvBase) -> usize {
        env.address_type.byte_size()
    }

    pub fn referenced_type_key(&self) -> Key {
        self.referenced_type_key
    }

    pub fn referenced_type_entry<'a>(&self, env: &'a EnvBase) -> TypeEntryRef<'a> {
        self.referenced_type_key.get_entry_ref(&env.type_table)
    }

    pub fn display(&self, f: &mut core::fmt::Formatter<'_>, env: &EnvBase) -> core::fmt::Result {
        write!(f, "*")?;
        self.referenced_type_entry(env).display(f, env)
    }
}
