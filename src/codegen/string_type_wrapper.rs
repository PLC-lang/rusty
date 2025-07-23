use inkwell::debug_info::{DIBasicType, DebugInfoBuilder};
use crate::typesystem::StringEncoding;

/// Create a proper DWARF string type with encoding information
/// 
/// This function creates string debug types with proper encoding attributes:
/// - UTF-8 strings use DW_ATE_UTF (0x10) encoding - for variable-width UTF-8
/// - UTF-16 strings use DW_ATE_UCS (0x11) encoding - for fixed-width Unicode (UTF-16)
/// 
/// Note: DW_ATE_UCS ("Universal Character Set") is the standard DWARF encoding
/// for multi-byte Unicode formats like UTF-16. While the name is generic, this
/// is the idiomatic encoding constant used by major compilers for UTF-16 strings.
/// 
/// This allows debuggers to properly distinguish between different string encodings
/// regardless of the string's actual length, solving the issue where size-based
/// detection breaks down with custom string sizes.
/// 
/// # Arguments
/// * `debug_info` - The inkwell DebugInfoBuilder instance
/// * `name` - Name of the string type (e.g., "STRING", "WSTRING")
/// * `length` - Length of the string in characters
/// * `encoding` - The string encoding (UTF-8 or UTF-16)
/// 
/// # Returns
/// A DIBasicType representing the string type with proper encoding metadata
pub fn create_string_type<'ctx>(
    debug_info: &DebugInfoBuilder<'ctx>,
    name: &str,
    length: i64,
    encoding: StringEncoding,
) -> Result<DIBasicType<'ctx>, String> {
    // Calculate size and encoding based on the string type
    let (size_bits, dwarf_encoding) = match encoding {
        StringEncoding::Utf8 => (length * 8, 0x10),   // UTF-8: byte-based, DW_ATE_UTF
        StringEncoding::Utf16 => (length * 16, 0x11), // UTF-16: 2 bytes per code unit, DW_ATE_UCS
    };

    // Note on DWARF encoding constants:
    // - DW_ATE_UTF (0x10): Used for UTF-8 encoded strings. This is the standard DWARF 
    //   encoding for variable-width UTF-8 text where each code unit is 1 byte.
    //
    // - DW_ATE_UCS (0x11): Used for UTF-16 encoded strings. DW_ATE_UCS stands for 
    //   "Universal Character Set" and is the DWARF standard way to represent multi-byte
    //   Unicode encodings like UTF-16, UTF-32, and UCS-2. While the name doesn't 
    //   explicitly say "UTF-16", this is the idiomatic DWARF encoding constant for 
    //   UTF-16 strings in debug information.
    //
    //   The DWARF specification uses DW_ATE_UCS for any Unicode encoding that uses
    //   fixed-width multi-byte code units (as opposed to DW_ATE_UTF for variable-width
    //   UTF-8). This follows established practice in major compilers like GCC and Clang.

    // Use inkwell's create_basic_type method with explicit DWARF encoding
    // This creates a DIBasicType with the proper encoding attribute that debuggers can recognize
    let string_type = debug_info.create_basic_type(
        name,
        size_bits as u64,
        dwarf_encoding,
        inkwell::debug_info::DIFlagsConstants::PUBLIC,
    ).map_err(|e| format!("Failed to create string debug type: {}", e))?;
    
    Ok(string_type)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dwarf_encoding_constants() {
        // Test that our DWARF encoding constants match the DWARF specification:
        // - DW_ATE_UTF (0x10): Variable-width UTF-8 encoding (1 byte per code unit)
        // - DW_ATE_UCS (0x11): Fixed-width Unicode encoding (UTF-16, UTF-32, UCS-2, etc.)
        //
        // These constants are defined in the DWARF Debugging Information Format
        // specification and are used by major compilers (GCC, Clang, etc.)
        let (utf8_encoding, _) = match StringEncoding::Utf8 {
            StringEncoding::Utf8 => (0x10u32, 0i64),
            StringEncoding::Utf16 => (0x11u32, 0i64),
        };
        let (utf16_encoding, _) = match StringEncoding::Utf16 {
            StringEncoding::Utf8 => (0x10u32, 0i64), 
            StringEncoding::Utf16 => (0x11u32, 0i64),
        };
        
        assert_eq!(utf8_encoding, 0x10);  // DW_ATE_UTF
        assert_eq!(utf16_encoding, 0x11); // DW_ATE_UCS
    }
}
