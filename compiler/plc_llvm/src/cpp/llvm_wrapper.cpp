#include <llvm/Target/TargetLoweringObjectFile.h>
#include <llvm/Target/TargetOptions.h>
#include <llvm/Target/TargetMachine.h>
#include <llvm-c/TargetMachine.h>
#include <llvm/Support/CBindingWrapping.h>
#include <llvm/IR/DIBuilder.h>
#include <llvm/IR/DebugInfoMetadata.h>
#include <llvm-c/DebugInfo.h>

using namespace llvm;

DEFINE_SIMPLE_CONVERSION_FUNCTIONS(TargetMachine, LLVMTargetMachineRef)

// Expose a C interface to set the options on the target machine
extern "C" {

    // Set the init array option in TargetOptions
    void setUseInitArray(LLVMTargetMachineRef tm, LLVMBool useInitArray) {
        auto* targetMachine = unwrap(tm);
        TargetOptions* options = &targetMachine->Options;
        options->UseInitArray = useInitArray ? 1 : 0;
    }

    // Create a DICompileUnit via the C++ DIBuilder API, which exposes the
    // NameTableKind parameter that the LLVM C API does not.
    // When enablePubnames is false, sets NameTableKind::None to suppress
    // .debug_names emission (avoiding a GDB incompatibility with lld).
    //
    // The `lang` and `emissionKind` parameters use the LLVMDWARFSourceLanguage
    // and LLVMDWARFEmissionKind C enum values (0-based sequential), which need
    // to be mapped to their DWARF/C++ equivalents.
    LLVMMetadataRef createCompileUnit(
        LLVMDIBuilderRef builderRef,
        LLVMMetadataRef fileRef,
        unsigned lang,
        const char *producer, size_t producerLen,
        LLVMBool isOptimized,
        unsigned runtimeVer,
        unsigned emissionKind,
        uint64_t dwoId,
        LLVMBool splitDebugInlining,
        LLVMBool debugInfoForProfiling,
        LLVMBool enablePubnames,
        const char *sysRoot, size_t sysRootLen,
        const char *sdk, size_t sdkLen
    ) {
        auto *builder = unwrap(builderRef);
        auto *file = cast<DIFile>(unwrap(fileRef));
        auto nameTableKind = enablePubnames
            ? DICompileUnit::DebugNameTableKind::Default
            : DICompileUnit::DebugNameTableKind::None;

        // The LLVM C API uses a 0-based sequential enum for source languages
        // (LLVMDWARFSourceLanguageC89=0, C=1, ...) while the C++ API / DWARF
        // spec uses the actual DWARF values (DW_LANG_C89=0x0001, C=0x0002, ...).
        // Add 1 to convert from the C enum to the DWARF value.
        unsigned dwarfLang = lang + 1;

        // Similarly, LLVMDWARFEmissionKind: None=0, Full=1, LineTablesOnly=2
        // maps to DICompileUnit::DebugEmissionKind where NoDebug=0, FullDebug=1,
        // LineTablesOnly=2, DebugDirectivesOnly=3 — these happen to match directly.

        auto *cu = builder->createCompileUnit(
            dwarfLang, file,
            StringRef(producer, producerLen),
            isOptimized,
            /*Flags=*/"",
            runtimeVer,
            /*SplitName=*/"",
            static_cast<DICompileUnit::DebugEmissionKind>(emissionKind),
            dwoId,
            splitDebugInlining,
            debugInfoForProfiling,
            nameTableKind,
            /*RangesBaseAddress=*/false,
            StringRef(sysRoot, sysRootLen),
            StringRef(sdk, sdkLen)
        );
        return wrap(cu);
    }
}
