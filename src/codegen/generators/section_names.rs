use crate::index::Index;
use crate::typesystem::{self, DataTypeInformation, StringEncoding, TypeSize};
use plc_diagnostics::diagnostics::Diagnostic;
use section_mangler::{StringEncoding as SectionStringEncoding, Type};

pub fn mangle_type(index: &Index, ty: &typesystem::DataType) -> Result<section_mangler::Type, Diagnostic> {
    // TODO: This is a bit ugly because we keep dereferencing references to Copy types like
    // bool, u32, etc, because `DataTypeInformation::Pointer` keeps a `String` which is not
    // Copy. the alternative is for section_mangle::Type to keep references everywhere, and
    // have a lifetime generic parameter, e.g. `section_mangler::Type<'a>` - which is also
    // annoying.
    let mangled = match ty.get_type_information() {
        DataTypeInformation::Void => Type::Void,
        DataTypeInformation::Integer { signed, size, semantic_size, .. } => {
            Type::Integer { signed: *signed, size: *size, semantic_size: *semantic_size }
        }
        DataTypeInformation::Float { size, .. } => Type::Float { size: *size },
        DataTypeInformation::String { size: TypeSize::LiteralInteger(size), encoding } => {
            let encoding = match encoding {
                StringEncoding::Utf8 => SectionStringEncoding::Utf8,
                StringEncoding::Utf16 => SectionStringEncoding::Utf16,
            };

            Type::String { size: *size as usize, encoding }
        }
        DataTypeInformation::Pointer { inner_type_name, .. } => Type::Pointer {
            inner: Box::new(mangle_type(index, index.get_effective_type_by_name(inner_type_name)?)?),
        },
        // FIXME: For now, encode all unknown types as "void" since this is not required for
        // execution. Not doing so (and doing an `unreachable!()` for example) obviously causes
        // failures, because complex types are already implemented in the compiler.
        _ => Type::Void,
    };

    Ok(mangled)
}
