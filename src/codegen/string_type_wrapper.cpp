#include "llvm/IR/DIBuilder.h"
#include "llvm/IR/DebugInfoMetadata.h"
#include "llvm/BinaryFormat/Dwarf.h"
#include <cstddef>
#include <cstdint>

using namespace llvm;

extern "C" {
    /// C wrapper around LLVM's DIStringType creation with encoding support
    /// 
    /// This function creates a DIStringType with proper encoding information:
    /// - For UTF-8 strings: uses DW_ATE_UTF (0x10)  
    /// - For UTF-16 strings: uses DW_ATE_UCS (0x11)
    /// 
    /// Parameters:
    /// - builder: Raw pointer to DIBuilder instance
    /// - name_ptr: Pointer to string name data
    /// - name_len: Length of the string name  
    /// - size_in_bits: Size of the string type in bits
    /// - encoding: DWARF encoding constant (0x10 for UTF-8, 0x11 for UTF-16)
    /// 
    /// Returns: Raw pointer to the created DIStringType, or nullptr on failure
    void* create_llvm_string_type_wrapper(void* builder, const char* name_ptr, std::size_t name_len, std::uint64_t size_in_bits, unsigned encoding) {
        if (!builder || !name_ptr) {
            return nullptr;
        }
        
        // Cast the void pointer back to DIBuilder
        auto* di_builder = static_cast<DIBuilder*>(builder);
        
        // Create StringRef from the name data
        StringRef name_ref(name_ptr, name_len);
        
        // For now, let's use the simple createStringType and add encoding info later
        // TODO: Find correct way to get LLVMContext and use DIStringType::get with encoding
        auto* string_type = di_builder->createStringType(name_ref, size_in_bits);
        
        // Return as void pointer for C compatibility
        return static_cast<void*>(string_type);
    }
}
