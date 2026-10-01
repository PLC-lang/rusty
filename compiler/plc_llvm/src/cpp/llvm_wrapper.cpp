#include <llvm/Target/TargetLoweringObjectFile.h>
#include <llvm/Target/TargetOptions.h>
#include <llvm/Target/TargetMachine.h>
#include <llvm-c/DebugInfo.h>
#include <llvm-c/TargetMachine.h>
#include <llvm/IR/DIBuilder.h>
#include <llvm/IR/DebugInfoMetadata.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/Instruction.h>
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

    // Replaces the function's subprogram by a copy that places `is_stmt` by Key Instructions; the
    // flag is fixed at creation. Call it before anything refers to the subprogram.
    void enableKeyInstructions(LLVMDIBuilderRef builder, LLVMValueRef function) {
        auto* func = unwrap<Function>(function);
        DISubprogram* old = func->getSubprogram();
        if (!old || old->getKeyInstructionsEnabled()) {
            return;
        }

        func->setSubprogram(unwrap(builder)->createFunction(
            old->getScope(), old->getName(), old->getLinkageName(), old->getFile(), old->getLine(),
            old->getType(), old->getScopeLine(), old->getFlags(), old->getSPFlags(),
            old->getTemplateParams(), old->getDeclaration(), old->getThrownTypes(),
            old->getAnnotations(), old->getTargetFuncName(), /* UseKeyInstructions */ true));
    }

    // Moves the instruction's debug location into the given Key Instructions atom
    void setKeyInstruction(LLVMValueRef instruction, uint64_t group, uint8_t rank) {
        auto* inst = unwrap<Instruction>(instruction);
        const DILocation* location = inst->getDebugLoc().get();
        if (!location) {
            return;
        }

        inst->setDebugLoc(DILocation::get(location->getContext(), location->getLine(),
                                          location->getColumn(), location->getScope(),
                                          location->getInlinedAt(), location->isImplicitCode(),
                                          group, rank));
    }
}
