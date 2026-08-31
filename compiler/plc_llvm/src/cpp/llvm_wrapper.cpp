#include <llvm/Target/TargetLoweringObjectFile.h>
#include <llvm/Target/TargetOptions.h>
#include <llvm/Target/TargetMachine.h>
#include <llvm-c/TargetMachine.h>
#include <llvm/Support/CBindingWrapping.h>
#include <llvm/Support/CommandLine.h>

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

    // Set a registered LLVM command-line option by name. Some backend knobs are plain
    // `cl::opt`s with no `TargetOptions` field and no C-API entry point, and this is the
    // only way to reach them. `addOccurrence` is used rather than a cast to a concrete
    // `cl::opt<T>` so the value type does not have to be known here.
    // Returns 1 on success, 0 if LLVM does not know the option or rejects the value.
    LLVMBool setLLVMOption(const char* name, const char* value) {
        auto& registered = cl::getRegisteredOptions();
        auto entry = registered.find(name);
        if (entry == registered.end()) {
            return 0;
        }
        // addOccurrence reports a parse error by returning true
        return entry->second->addOccurrence(0, name, value) ? 0 : 1;
    }
}
