//! OID Names Database
//!
//! The contents of this database are generated from the official IANA
//! [Object Identifier Descriptors] Registry CSV file and from [RFC 5280].
//! If we are missing values you care about, please contribute a patch to
//! `oiddbgen` (a subcrate in the source code) to generate the values from
//! the relevant standard.
//!
//! [RFC 5280]: https://datatracker.ietf.org/doc/html/rfc5280
//! [Object Identifier Descriptors]: https://www.iana.org/assignments/ldap-parameters/ldap-parameters.xhtml#ldap-parameters-3

#![allow(missing_docs)]

mod gen;

pub use gen::*;

use crate::{Error, NamedOid, ObjectIdentifier};

/// A const implementation of byte equals.
const fn eq(lhs: &[u8], rhs: &[u8]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }

    let mut i = 0usize;
    while i < lhs.len() {
        if lhs[i] != rhs[i] {
            return false;
        }

        i += 1;
    }

    true
}

/// A query interface for OIDs/Names.
pub struct Database<'a>(&'a [&'a NamedOid<'a>]);

impl<'a> Database<'a> {
    /// Looks up a name for an OID.
    ///
    /// Errors if the input is not a valid OID.
    /// Returns the input if no name is found.
    pub fn resolve<'b>(&self, oid: &'b str) -> Result<&'b str, Error>
    where
        'a: 'b,
    {
        let oi = oid.parse()?;
        Ok(self.by_oid(&oi).map(|n| n.name).unwrap_or(oid))
    }

    /// Finds a named oid by its associated OID.
    pub const fn by_oid(&self, oid: &ObjectIdentifier) -> Option<&'a NamedOid<'a>> {
        let mut i = 0;

        while i < self.0.len() {
            let lhs = self.0[i].oid;
            if lhs.length == oid.length && eq(&lhs.bytes, &oid.bytes) {
                return Some(self.0[i]);
            }

            i += 1;
        }

        None
    }

    /// Finds a named oid by its associated name.
    pub const fn by_name(&self, name: &str) -> Option<&'a NamedOid<'a>> {
        let mut i = 0;

        while i < self.0.len() {
            let lhs = self.0[i].name;
            if eq(lhs.as_bytes(), name.as_bytes()) {
                return Some(self.0[i]);
            }

            i += 1;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::attr::rfc4519::CN;

    #[test]
    fn by_oid() {
        let cn = super::DB.by_oid(&CN.oid).expect("cn not found");
        assert_eq!(&CN, cn);

        let cn = super::attr::DB.by_oid(&CN.oid).expect("cn not found");
        assert_eq!(&CN, cn);

        assert_eq!(None, super::obj::DB.by_oid(&CN.oid));
    }

    #[test]
    fn by_name() {
        let cn = super::DB.by_name(CN.name).expect("cn not found");
        assert_eq!(&CN, cn);

        let cn = super::attr::DB.by_name(CN.name).expect("cn not found");
        assert_eq!(&CN, cn);

        assert_eq!(None, super::obj::DB.by_name(CN.name));
    }
}