#include "llvm/IR/PassManager.h"
#include "llvm/Transforms/Instrumentation/InstrProfiling.h"
#include "llvm/Transforms/Instrumentation.h"

#include "llvm/Analysis/AliasAnalysis.h"
#include "llvm/Analysis/TargetLibraryInfo.h"
#include "llvm/Analysis/TargetTransformInfo.h"
#include "llvm/CodeGen/TargetSubtargetInfo.h"
#include "llvm/InitializePasses.h"
#include "llvm/IR/AutoUpgrade.h"
#include "llvm/IR/AssemblyAnnotationWriter.h"
#include "llvm/IR/IntrinsicInst.h"
#include "llvm/IR/Verifier.h"
#include "llvm/Object/ObjectFile.h"
#include "llvm/Object/IRObjectFile.h"
#include "llvm/Passes/PassBuilder.h"
#include "llvm/Passes/PassPlugin.h"
#include "llvm/Passes/StandardInstrumentations.h"
#include "llvm/Support/CBindingWrapping.h"
#include "llvm/Support/FileSystem.h"
#include "llvm/Support/Host.h"
#include "llvm/MC/TargetRegistry.h"
#include "llvm/Target/TargetMachine.h"
#include "llvm/Transforms/IPO/PassManagerBuilder.h"
#include "llvm/Transforms/IPO/AlwaysInliner.h"
#include "llvm/Transforms/IPO/FunctionImport.h"
#include "llvm/Transforms/Utils/AddDiscriminators.h"
#include "llvm/Transforms/Utils/FunctionImportUtils.h"
#include "llvm/LTO/LTO.h"
#include "llvm/Bitcode/BitcodeWriterPass.h"
#include "llvm-c/Transforms/PassManagerBuilder.h"

#include "llvm/Transforms/Instrumentation.h"
#include "llvm/Transforms/Instrumentation/AddressSanitizer.h"
#include "llvm/Support/TimeProfiler.h"
#include "llvm/Transforms/Instrumentation/GCOVProfiler.h"
#include "llvm/Transforms/Instrumentation/InstrProfiling.h"
#include "llvm/Transforms/Instrumentation/ThreadSanitizer.h"
#include "llvm/Transforms/Instrumentation/MemorySanitizer.h"
#include "llvm/Transforms/Instrumentation/HWAddressSanitizer.h"
#include "llvm/Transforms/Utils/CanonicalizeAliases.h"
#include "llvm/Transforms/Utils/NameAnonGlobals.h"
#include "llvm/Transforms/Utils.h"

using namespace llvm;

ModulePass *createInstrumentationPass() {
  // Options - https://github.com/llvm-mirror/llvm/blob/2c4ca6832fa6b306ee6a7010bfb80a3f2596f824/include/llvm/Transforms/Instrumentation.h#L129C1-L148C1
  InstrProfOptions Options;

  Options.InstrProfileOutput = "rust.profraw";
  // Options.Atomic = true;

  // No longer legacy once we updated to newer passes
  // return new InstrProfilingLegacyPass(Options, false);
  return createInstrProfilingLegacyPass(Options, false);
}

extern "C" void LLVMRustAddInstrumentationPass(LLVMPassManagerRef PM) {

  unwrap(PM)->add(createInstrumentationPass());
  // unwrap(PM)->addPass(InstrProfiling(Options, false));
}

// https://github.com/rust-lang/rust/blob/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/compiler/rustc_llvm/llvm-wrapper/PassWrapper.cpp#L923-L930
extern "C" void LLVMRustRunInstrumentationPass(LLVMModuleRef M) {
  // Options
  InstrProfOptions Options;
  Options.InstrProfileOutput = "rust.profraw";
  Options.Atomic = true;

  // Create the analysis managers.
  // These must be declared in this order so that they are destroyed in the
  // correct order due to inter-analysis-manager references.
  LoopAnalysisManager LAM;
  FunctionAnalysisManager FAM;
  CGSCCAnalysisManager CGAM;
  ModuleAnalysisManager MAM;

  // Create the new pass manager builder.
  // Take a look at the PassBuilder constructor parameters for more
  // customization, e.g. specifying a TargetMachine or various debugging
  // options.
  PassBuilder PB;

  // Register all the basic analyses with the managers.
  PB.registerModuleAnalyses(MAM);
  PB.registerCGSCCAnalyses(CGAM);
  PB.registerFunctionAnalyses(FAM);
  PB.registerLoopAnalyses(LAM);
  PB.crossRegisterProxies(LAM, FAM, CGAM, MAM);

  // Create the pass manager.
  // This one corresponds to a typical -O2 optimization pipeline.
  // ModulePassManager MPM = PB.buildPerModuleDefaultPipeline(OptimizationLevel::O0);
  ModulePassManager MPM = PB.buildO0DefaultPipeline(OptimizationLevel::O0, /* PreLinkLTO */ false);

  // TODO - this needed to be updated in LLVM >=18
  MPM.addPass(InstrProfiling(Options, false));

  // Optimize the IR!
  // MPM.run(MyModule, MAM);
  MPM.run(*unwrap(M), MAM);


}