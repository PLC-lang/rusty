window.SIDEBAR_ITEMS = {"enum":["LLVMJITSymbolGenericFlags","LLVMOrcJITDylibLookupFlags","LLVMOrcLookupKind","LLVMOrcOpaqueDefinitionGenerator","LLVMOrcOpaqueDumpObjects","LLVMOrcOpaqueExecutionSession","LLVMOrcOpaqueIRTransformLayer","LLVMOrcOpaqueIndirectStubsManager","LLVMOrcOpaqueJITDylib","LLVMOrcOpaqueJITTargetMachineBuilder","LLVMOrcOpaqueLazyCallThroughManager","LLVMOrcOpaqueLookupState","LLVMOrcOpaqueMaterializationResponsibility","LLVMOrcOpaqueMaterializationUnit","LLVMOrcOpaqueObjectLayer","LLVMOrcOpaqueObjectLinkingLayer","LLVMOrcOpaqueObjectTransformLayer","LLVMOrcOpaqueResourceTracker","LLVMOrcOpaqueSymbolStringPool","LLVMOrcOpaqueSymbolStringPoolEntry","LLVMOrcOpaqueThreadSafeContext","LLVMOrcOpaqueThreadSafeModule","LLVMOrcSymbolLookupFlags"],"fn":["LLVMOrcAbsoluteSymbols","LLVMOrcCreateCustomCAPIDefinitionGenerator","LLVMOrcCreateCustomMaterializationUnit","LLVMOrcCreateDumpObjects","LLVMOrcCreateDynamicLibrarySearchGeneratorForPath","LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess","LLVMOrcCreateLocalIndirectStubsManager","LLVMOrcCreateLocalLazyCallThroughManager","LLVMOrcCreateNewThreadSafeContext","LLVMOrcCreateNewThreadSafeModule","LLVMOrcCreateStaticLibrarySearchGeneratorForPath","LLVMOrcDisposeCSymbolFlagsMap","LLVMOrcDisposeDefinitionGenerator","LLVMOrcDisposeDumpObjects","LLVMOrcDisposeIndirectStubsManager","LLVMOrcDisposeJITTargetMachineBuilder","LLVMOrcDisposeLazyCallThroughManager","LLVMOrcDisposeMaterializationResponsibility","LLVMOrcDisposeMaterializationUnit","LLVMOrcDisposeObjectLayer","LLVMOrcDisposeSymbols","LLVMOrcDisposeThreadSafeContext","LLVMOrcDisposeThreadSafeModule","LLVMOrcDumpObjects_CallOperator","LLVMOrcExecutionSessionCreateBareJITDylib","LLVMOrcExecutionSessionCreateJITDylib","LLVMOrcExecutionSessionGetJITDylibByName","LLVMOrcExecutionSessionGetSymbolStringPool","LLVMOrcExecutionSessionIntern","LLVMOrcExecutionSessionSetErrorReporter","LLVMOrcIRTransformLayerEmit","LLVMOrcIRTransformLayerSetTransform","LLVMOrcJITDylibAddGenerator","LLVMOrcJITDylibClear","LLVMOrcJITDylibCreateResourceTracker","LLVMOrcJITDylibDefine","LLVMOrcJITDylibGetDefaultResourceTracker","LLVMOrcJITTargetMachineBuilderCreateFromTargetMachine","LLVMOrcJITTargetMachineBuilderDetectHost","LLVMOrcJITTargetMachineBuilderGetTargetTriple","LLVMOrcJITTargetMachineBuilderSetTargetTriple","LLVMOrcLazyReexports","LLVMOrcMaterializationResponsibilityAddDependencies","LLVMOrcMaterializationResponsibilityAddDependenciesForAll","LLVMOrcMaterializationResponsibilityDefineMaterializing","LLVMOrcMaterializationResponsibilityDelegate","LLVMOrcMaterializationResponsibilityFailMaterialization","LLVMOrcMaterializationResponsibilityGetExecutionSession","LLVMOrcMaterializationResponsibilityGetInitializerSymbol","LLVMOrcMaterializationResponsibilityGetRequestedSymbols","LLVMOrcMaterializationResponsibilityGetSymbols","LLVMOrcMaterializationResponsibilityGetTargetDylib","LLVMOrcMaterializationResponsibilityNotifyEmitted","LLVMOrcMaterializationResponsibilityNotifyResolved","LLVMOrcMaterializationResponsibilityReplace","LLVMOrcObjectLayerAddObjectFile","LLVMOrcObjectLayerAddObjectFileWithRT","LLVMOrcObjectLayerEmit","LLVMOrcObjectTransformLayerSetTransform","LLVMOrcReleaseResourceTracker","LLVMOrcReleaseSymbolStringPoolEntry","LLVMOrcResourceTrackerRemove","LLVMOrcResourceTrackerTransferTo","LLVMOrcRetainSymbolStringPoolEntry","LLVMOrcSymbolStringPoolClearDeadEntries","LLVMOrcSymbolStringPoolEntryStr","LLVMOrcThreadSafeContextGetContext","LLVMOrcThreadSafeModuleWithModuleDo"],"mod":["ee","lljit"],"struct":["LLVMJITCSymbolMapPair","LLVMJITEvaluatedSymbol","LLVMJITSymbolFlags","LLVMOrcCDependenceMapPair","LLVMOrcCLookupSetElement","LLVMOrcCSymbolAliasMapEntry","LLVMOrcCSymbolAliasMapPair","LLVMOrcCSymbolFlagsMapPair","LLVMOrcCSymbolsList"],"type":["LLVMJITSymbolTargetFlags","LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction","LLVMOrcCDependenceMapPairs","LLVMOrcCLookupSet","LLVMOrcCSymbolAliasMapPairs","LLVMOrcCSymbolFlagsMapPairs","LLVMOrcCSymbolMapPairs","LLVMOrcDefinitionGeneratorRef","LLVMOrcDumpObjectsRef","LLVMOrcErrorReporterFunction","LLVMOrcExecutionSessionRef","LLVMOrcExecutorAddress","LLVMOrcGenericIRModuleOperationFunction","LLVMOrcIRTransformLayerRef","LLVMOrcIRTransformLayerTransformFunction","LLVMOrcIndirectStubsManagerRef","LLVMOrcJITDylibRef","LLVMOrcJITTargetAddress","LLVMOrcJITTargetMachineBuilderRef","LLVMOrcLazyCallThroughManagerRef","LLVMOrcLookupStateRef","LLVMOrcMaterializationResponsibilityRef","LLVMOrcMaterializationUnitDestroyFunction","LLVMOrcMaterializationUnitDiscardFunction","LLVMOrcMaterializationUnitMaterializeFunction","LLVMOrcMaterializationUnitRef","LLVMOrcObjectLayerRef","LLVMOrcObjectLinkingLayerRef","LLVMOrcObjectTransformLayerRef","LLVMOrcObjectTransformLayerTransformFunction","LLVMOrcResourceTrackerRef","LLVMOrcSymbolPredicate","LLVMOrcSymbolStringPoolEntryRef","LLVMOrcSymbolStringPoolRef","LLVMOrcThreadSafeContextRef","LLVMOrcThreadSafeModuleRef"]};