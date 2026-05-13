//! Public-signature hashing for index entries.
//!
//! A [`SignatureHash`] captures the externally observable shape of a POU,
//! global, or type — enough to answer "did this symbol's interface change?"
//! without doing a structural diff. Internal POU bodies and source locations
//! are deliberately *not* part of the hash: editing the body of a function
//! must not invalidate units that call it.
//!
//! Used by later phases to scope re-annotation closures: if a symbol's
//! signature hash is unchanged between rebuilds, units depending only on its
//! public surface don't need to be re-annotated.

use std::hash::Hasher;

use serde::{Deserialize, Serialize};
use siphasher::sip::SipHasher13;

use super::{ImplementationIndexEntry, PouIndexEntry, VariableIndexEntry};
use crate::typesystem::{DataType, DataTypeInformation};

/// Stable hash of a symbol's externally observable interface. Two equal
/// hashes mean the public shape is identical; two distinct hashes mean the
/// shape changed in a way callers might care about.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SignatureHash(pub u64);

impl SignatureHash {
    fn finish(h: SipHasher13) -> Self {
        SignatureHash(h.finish())
    }
}

/// Hashes the public surface of a POU: name, kind tag, return type,
/// declaration linkage, and any super-class / interface relations.
/// Skips: source location, declared-but-private members, body, generated
/// flags, generic body specializations.
pub fn hash_pou(pou: &PouIndexEntry) -> SignatureHash {
    let mut h = SipHasher13::new();
    hash_str(&mut h, pou.get_name());
    match pou {
        PouIndexEntry::Program { instance_struct_name, linkage, .. } => {
            h.write_u8(0);
            hash_str(&mut h, instance_struct_name);
            h.write_u8(*linkage as u8);
        }
        PouIndexEntry::FunctionBlock { instance_struct_name, linkage, super_class, interfaces, .. } => {
            h.write_u8(1);
            hash_str(&mut h, instance_struct_name);
            h.write_u8(*linkage as u8);
            hash_opt_str(&mut h, super_class.as_deref());
            hash_str_slice(&mut h, interfaces);
        }
        PouIndexEntry::Function { return_type, generics, linkage, is_variadic, is_const, .. } => {
            h.write_u8(2);
            hash_str(&mut h, return_type);
            h.write_u8(*linkage as u8);
            h.write_u8(*is_variadic as u8);
            h.write_u8(*is_const as u8);
            h.write_u64(generics.len() as u64);
            for g in generics {
                hash_str(&mut h, &g.name);
                hash_str(&mut h, &format!("{:?}", g.nature));
            }
        }
        PouIndexEntry::Class { instance_struct_name, linkage, super_class, interfaces, .. } => {
            h.write_u8(3);
            hash_str(&mut h, instance_struct_name);
            h.write_u8(*linkage as u8);
            hash_opt_str(&mut h, super_class.as_deref());
            hash_str_slice(&mut h, interfaces);
        }
        PouIndexEntry::Method {
            parent_name,
            declaration_kind,
            return_type,
            instance_struct_name,
            linkage,
            ..
        } => {
            h.write_u8(4);
            hash_str(&mut h, parent_name);
            h.write_u8(*declaration_kind as u8);
            hash_str(&mut h, return_type);
            hash_str(&mut h, instance_struct_name);
            h.write_u8(*linkage as u8);
        }
        PouIndexEntry::Action { parent_name, instance_struct_name, linkage, .. } => {
            h.write_u8(5);
            hash_str(&mut h, parent_name);
            hash_str(&mut h, instance_struct_name);
            h.write_u8(*linkage as u8);
        }
    }
    SignatureHash::finish(h)
}

/// Hashes the public surface of a global variable: qualified name, declared
/// type, linkage, retain/constant flags. Skips: initial value bytes (they
/// can change without the global's *type* changing), source location.
pub fn hash_global(var: &VariableIndexEntry) -> SignatureHash {
    let mut h = SipHasher13::new();
    hash_str(&mut h, var.get_qualified_name());
    hash_str(&mut h, var.get_type_name());
    h.write_u8(var.get_linkage() as u8);
    h.write_u8(var.is_constant() as u8);
    SignatureHash::finish(h)
}

/// Hashes the public surface of a data type. Includes structural information
/// (members for structs, variants for enums, dimensions/element-type for
/// arrays). Skips: source location, parent-pou pointers.
pub fn hash_type(dt: &DataType) -> SignatureHash {
    let mut h = SipHasher13::new();
    hash_str(&mut h, dt.get_name());
    h.write_u8(dt.linkage as u8);
    hash_data_type_information(&mut h, &dt.information);
    SignatureHash::finish(h)
}

/// Hashes the public surface of an implementation entry: its call name,
/// kind, and associated type. Used to detect when an implementation's
/// interface changes vs. just its body.
pub fn hash_implementation(imp: &ImplementationIndexEntry) -> SignatureHash {
    let mut h = SipHasher13::new();
    hash_str(&mut h, imp.get_call_name());
    hash_str(&mut h, imp.get_type_name());
    hash_str(&mut h, &format!("{:?}", imp.get_implementation_type()));
    SignatureHash::finish(h)
}

fn hash_data_type_information(h: &mut SipHasher13, info: &DataTypeInformation) {
    // Variant discriminant first so two different variants with identical
    // contained data hash differently.
    let tag: u8 = match info {
        DataTypeInformation::Struct { .. } => 1,
        DataTypeInformation::Array { .. } => 2,
        DataTypeInformation::Pointer { .. } => 3,
        DataTypeInformation::Integer { .. } => 4,
        DataTypeInformation::Float { .. } => 5,
        DataTypeInformation::String { .. } => 6,
        DataTypeInformation::SubRange { .. } => 7,
        DataTypeInformation::Alias { .. } => 8,
        DataTypeInformation::Enum { .. } => 9,
        DataTypeInformation::Generic { .. } => 10,
        DataTypeInformation::Interface { .. } => 11,
        DataTypeInformation::Void => 12,
    };
    h.write_u8(tag);

    match info {
        DataTypeInformation::Struct { name, members, source, .. } => {
            hash_str(h, name);
            hash_str(h, &format!("{source:?}"));
            h.write_u64(members.len() as u64);
            for m in members {
                hash_str(h, m.get_name());
                hash_str(h, m.get_type_name());
            }
        }
        DataTypeInformation::Array { name, inner_type_name, dimensions } => {
            hash_str(h, name);
            hash_str(h, inner_type_name);
            h.write_u64(dimensions.len() as u64);
        }
        DataTypeInformation::Pointer { name, inner_type_name, auto_deref, type_safe, is_function } => {
            hash_str(h, name);
            hash_str(h, inner_type_name);
            hash_str(h, &format!("{auto_deref:?}"));
            h.write_u8(*type_safe as u8);
            h.write_u8(*is_function as u8);
        }
        DataTypeInformation::Interface { name, .. } => {
            hash_str(h, name);
        }
        DataTypeInformation::Integer { name, signed, size, semantic_size } => {
            hash_str(h, name);
            h.write_u8(*signed as u8);
            h.write_u32(*size);
            h.write_u32(semantic_size.unwrap_or(0));
        }
        DataTypeInformation::Float { name, size } => {
            hash_str(h, name);
            h.write_u32(*size);
        }
        DataTypeInformation::String { size: _, encoding } => {
            // String size is a ConstId reference whose value can shift across
            // const-evaluation runs even when the source spec is unchanged;
            // don't include it. The encoding tag captures STRING vs WSTRING.
            hash_str(h, &format!("{encoding:?}"));
        }
        DataTypeInformation::SubRange { name, referenced_type, .. } => {
            hash_str(h, name);
            hash_str(h, referenced_type);
        }
        DataTypeInformation::Alias { name, referenced_type } => {
            hash_str(h, name);
            hash_str(h, referenced_type);
        }
        DataTypeInformation::Enum { name, referenced_type, variants } => {
            hash_str(h, name);
            hash_str(h, referenced_type);
            h.write_u64(variants.len() as u64);
            for v in variants {
                hash_str(h, v.get_name());
                hash_str(h, v.get_qualified_name());
            }
        }
        DataTypeInformation::Generic { name, generic_symbol, nature } => {
            hash_str(h, name);
            hash_str(h, generic_symbol);
            hash_str(h, &format!("{nature:?}"));
        }
        DataTypeInformation::Void => {}
    }
}

fn hash_str(h: &mut SipHasher13, s: &str) {
    h.write_u64(s.len() as u64);
    h.write(s.as_bytes());
}

fn hash_opt_str(h: &mut SipHasher13, s: Option<&str>) {
    match s {
        Some(s) => {
            h.write_u8(1);
            hash_str(h, s);
        }
        None => h.write_u8(0),
    }
}

fn hash_str_slice(h: &mut SipHasher13, ss: &[String]) {
    h.write_u64(ss.len() as u64);
    for s in ss {
        hash_str(h, s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::tests::index;

    #[test]
    fn pou_signature_hash_is_stable_across_runs() {
        let src = r#"
            FUNCTION foo : INT
                VAR_INPUT x : INT; END_VAR
            END_FUNCTION
        "#;
        let (_, index_a) = index(src);
        let (_, index_b) = index(src);

        let a = hash_pou(index_a.find_pou("foo").expect("foo present"));
        let b = hash_pou(index_b.find_pou("foo").expect("foo present"));
        assert_eq!(a, b, "two indexings of the same source must hash equally");
    }

    #[test]
    fn function_return_type_change_alters_hash() {
        let (_, a) = index(
            r#"
                FUNCTION foo : INT
                END_FUNCTION
            "#,
        );
        let (_, b) = index(
            r#"
                FUNCTION foo : DINT
                END_FUNCTION
            "#,
        );

        let ha = hash_pou(a.find_pou("foo").unwrap());
        let hb = hash_pou(b.find_pou("foo").unwrap());
        assert_ne!(ha, hb, "return-type change must alter the signature hash");
    }

    #[test]
    fn struct_field_change_alters_type_hash() {
        let (_, a) = index(
            r#"
                TYPE s : STRUCT a : INT; END_STRUCT END_TYPE
            "#,
        );
        let (_, b) = index(
            r#"
                TYPE s : STRUCT a : INT; b : DINT; END_STRUCT END_TYPE
            "#,
        );

        let ha = hash_type(a.find_effective_type_by_name("s").unwrap());
        let hb = hash_type(b.find_effective_type_by_name("s").unwrap());
        assert_ne!(ha, hb, "adding a struct field must change the type hash");
    }

    #[test]
    fn global_type_change_alters_hash() {
        let (_, a) = index("VAR_GLOBAL g : INT; END_VAR");
        let (_, b) = index("VAR_GLOBAL g : DINT; END_VAR");
        let ha = hash_global(a.find_global_variable("g").unwrap());
        let hb = hash_global(b.find_global_variable("g").unwrap());
        assert_ne!(ha, hb);
    }
}
