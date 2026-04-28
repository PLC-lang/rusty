#include <llvm/Target/TargetLoweringObjectFile.h>
#include <llvm/Target/TargetOptions.h>
#include <llvm/Target/TargetMachine.h>
#include <llvm-c/TargetMachine.h>
#include <llvm/Support/CBindingWrapping.h>

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
}
