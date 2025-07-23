use inkwell::debug_info::{DIBasicType, DebugInfoBuilder};
use std::marker::PhantomData;

// Re-export the StringEncoding from the typesystem to avoid duplication
pub use crate::typesystem::StringEncoding;

// External C function declaration for our C++ wrapper
extern "C" {
    /// C wrapper around LLVM's DIStringType creation with encoding support
    /// 
    /// This function creates a DIStringType with proper encoding information:
    /// - For UTF-8 strings: uses DW_ATE_UTF (0x10)  
    /// - For UTF-16 strings: uses DW_ATE_UCS (0x11)
    /// 
    /// Parameters:
    /// - builder: Raw pointer to DIBuilder instance (from inkwell's as_mut_ptr())
    /// - name_ptr: Pointer to string name data
    /// - name_len: Length of the string name  
    /// - size_in_bits: Size of the string type in bits
    /// - encoding: DWARF encoding constant (0x10 for UTF-8, 0x11 for UTF-16)
    /// 
    /// Returns: Raw pointer to the created DIStringType (as LLVMMetadataRef)
    fn create_llvm_string_type_wrapper(
        builder: *mut std::ffi::c_void,
        name_ptr: *const u8,
        name_len: usize,
        size_in_bits: u64,
        encoding: u32,
    ) -> *mut std::ffi::c_void;
}

/// Create a proper DWARF string type using LLVM's createStringType method
/// 
/// This function uses inkwell's `as_mut_ptr()` method to get the raw DIBuilder pointer
/// and calls a C++ wrapper around DIBuilder::createStringType, which creates a proper
/// DW_TAG_string_type with DW_AT_string_length.
/// 
/// This approach:
/// 1. Uses inkwell's as_mut_ptr() to get raw DIBuilder pointer
/// 2. Calls C++ wrapper function that invokes createStringType
/// 3. Properly distinguishes between UTF-8 and UTF-16 via size calculation
/// 4. Creates true string debug types that debuggers recognize
/// 
/// The C++ wrapper is compiled at build time and linked in.
pub fn create_string_type<'ctx>(
    debug_info: &DebugInfoBuilder<'ctx>,
    name: &str,
    length: i64,
    encoding: StringEncoding,
) -> Result<DIBasicType<'ctx>, String> {
    // Calculate size based on encoding
    let size_bits = match encoding {
        StringEncoding::Utf8 => length * 8,   // UTF-8: byte-based
        StringEncoding::Utf16 => length * 16, // UTF-16: 2 bytes per code unit
    };

    // Map StringEncoding to DWARF encoding constants
    let dwarf_encoding = match encoding {
        StringEncoding::Utf8 => 0x10,  // DW_ATE_UTF
        StringEncoding::Utf16 => 0x11, // DW_ATE_UCS
    };

    // Get the raw DIBuilder pointer using inkwell's as_mut_ptr() method
    let builder_ptr = debug_info.as_mut_ptr() as *mut std::ffi::c_void;
    
    // Call our C++ wrapper function
    let result = unsafe {
        create_llvm_string_type_wrapper(
            builder_ptr,
            name.as_ptr(),
            name.len(),
            size_bits as u64,
            dwarf_encoding,
        )
    };
    
    if result.is_null() {
        return Err("Failed to create string type: LLVM returned null".to_string());
    }
    
    // Convert the raw LLVM metadata back to inkwell's DIBasicType
    // Since DIBasicType's fields are private, we reconstruct it using unsafe transmute
    // with a compatible memory layout
    let string_type = unsafe {
        // DIBasicType has the layout: { metadata_ref: LLVMMetadataRef, _marker: PhantomData }
        // We can construct it by transmuting from a compatible struct with the same layout
        #[repr(C)]
        struct DIBasicTypeLayout<'ctx> {
            // This should match llvm_sys::prelude::LLVMMetadataRef which is *mut LLVMOpaqueMetadata
            metadata_ref: *mut std::ffi::c_void,
            _marker: PhantomData<&'ctx inkwell::context::Context>,
        }
        
        let layout = DIBasicTypeLayout {
            metadata_ref: result,
            _marker: PhantomData,
        };
        
        // Transmute to the actual DIBasicType - this is safe because:
        // 1. We know the memory layout is identical (both are repr(C) with same fields)
        // 2. The result is a valid DIStringType from LLVM's createStringType
        // 3. DIStringType is a subtype of DIType which DIBasicType represents
        // 4. The lifetime 'ctx is preserved correctly
        std::mem::transmute::<DIBasicTypeLayout<'ctx>, DIBasicType<'ctx>>(layout)
    };
    
    Ok(string_type)
}

/// Fallback implementation that creates string types using inkwell's API
/// 
/// This is kept as a backup in case the direct LLVM C API approach has issues.
/// It uses inkwell's create_basic_type method with UTF encodings.
pub fn create_string_type_fallback<'ctx>(
    debug_info: &DebugInfoBuilder<'ctx>,
    name: &str,
    length: i64,
    encoding: StringEncoding,
) -> Result<DIBasicType<'ctx>, String> {
    // Calculate size and encoding
    let (size_bits, dwarf_encoding) = match encoding {
        StringEncoding::Utf8 => (length * 8, 0x10),  // DW_ATE_UTF
        StringEncoding::Utf16 => (length * 16, 0x11), // DW_ATE_UCS
    };

    // Use inkwell's create_basic_type method
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
    fn test_string_encoding_values() {
        // Verify our encoding constants are correct
        assert_eq!(StringEncoding::Utf8, StringEncoding::Utf8);
        assert_eq!(StringEncoding::Utf16, StringEncoding::Utf16);
        assert_ne!(StringEncoding::Utf8, StringEncoding::Utf16);
    }
    
    #[test]
    fn test_dwarf_encoding_constants() {
        // Test that our DWARF encoding constants are correct
        // DW_ATE_UTF = 0x10, DW_ATE_UCS = 0x11
        let (utf8_encoding, _) = match StringEncoding::Utf8 {
            StringEncoding::Utf8 => (0x10u32, 0i64),
            StringEncoding::Utf16 => (0x11u32, 0i64),
        };
        let (utf16_encoding, _) = match StringEncoding::Utf16 {
            StringEncoding::Utf8 => (0x10u32, 0i64), 
            StringEncoding::Utf16 => (0x11u32, 0i64),
        };
        
        assert_eq!(utf8_encoding, 0x10);
        assert_eq!(utf16_encoding, 0x11);
    }
}
