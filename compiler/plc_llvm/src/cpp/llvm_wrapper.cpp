// #include <llvm/Target/TargetOptions.h>
#include <llvm-14/llvm/Target/TargetLoweringObjectFile.h>
#include <llvm-14/llvm/Target/TargetOptions.h>
#include <llvm-14/llvm/Target/TargetMachine.h>

using namespace llvm;

// Expose a C interface to set the options on the target machine
extern "C" {

    // Set the init array option in TargetOptions
    void setUseInitArray(TargetMachine* tm, bool useInitArray) {
        TargetOptions* options = &tm->Options;
        options->UseInitArray = useInitArray ? 1 : 0;
    }
}
