searchState.loadedDescShard("rusty", 0, "A St&amp;ructured Text LLVM Frontent\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nResolves (partial) expressions &amp; statements and annotates …\nReturns the argument unchanged.\nReturns the requested function from the builtin index or …\nCalls <code>U::from(self)</code>.\nthe codegen struct carries all dependencies required to …\nA wrapper around the LLVM context to allow passing it …\nthe debugging module creates debug information at …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\ngenerates all TYPEs, GLOBAL-sections and POUs of the given …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nthe module represents a llvm compilation unit\nconstructs a new code-generator that generates …\nPersists the module into the disk based on output and …\nPersists a given LLVM module to a static object and saves …\nPersists the given LLVM module into a bitcode file\nPersits the given LLVM module into LLVM IR and saves it to …\nPersists the given LLVM module to a dynamic non PIC object …\nPersists a given LLVM module to a shared postiion …\nPersists the given module to a string\nPrints the content of the module to the stderr\nRuns the function given by <code>name</code> inside the compiled module.\nRuns the function given by <code>name</code> inside the compiled module.\nExpands the given name to reference all underlying …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nRetrieves hardware bindings from all defined instances in …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nType alias for an IndexMap using the <code>fx</code> hashing algorithm, …\nType alias for a IndexSet using the <code>fx</code> hashing algorithm, …\nThe global index of the rusty-compiler\nA label represents a possible jump point in the source. It …\nthe TypeIndex carries all types. it is extracted into its …\nThe datatype (size) of the binding\nAdds a label definition for the POU\nthe type of variable\ncreates a new Action-PouIndexEntry\ncreates a new Class-PouIndexEntry\ncreates a new FunctionBlock-PouIndexEntry\ncreates a new Function-PouIndexEntry\ncreates a new Function-PouIndexEntry generated by the …\ncreates a new Method-PouIndexEntry\ncreates a new Program-PouIndexEntry\nthe variable’s datatype\nSpecifies if the binding is an In/Out or Memory binding\nA list of entries that form this binding\nCreates an iterator over all instances in the index The …\nRetrieves the “Effective” type behind this datatype An …\nreturns the effective DataType of the given type if it …\nreturns the effective DataType of the type with the given …\nreturns the effective DataTypeInformation of the type with …\nReturns the index entry of the enum variant or <code>None</code> if it …\nReturns the index entry of the enum variant by its …\nTries to return an enum variant defined within a POU\nreturn the <code>VariableIndexEntry</code> associated with the given …\nreturns the <code>VariableIndexEntry</code> of the global initializer …\nreturns the <code>VariableIndexEntry</code> of the global variable with …\nreturns the ImplementationIndexEntry associated with this …\nreturns <code>Some(DataType)</code> associated with this pou or <code>None</code> if …\nCreates an iterator over all instances in the index\nreturns the intrinsic (built-in) type represented by the …\nreturn the <code>VariableIndexEntry</code> with the qualified name: …\nSearches for variable name in the given container, if not …\nSearches for method names in the given container, if not …\nreturns the <code>VariableIndexEntry</code> of the global variable with …\nreturns the implementation of the sub-range-check-function …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nIf the provided name is a builtin function, returns it …\nreturns all registered ConstExpressions\nreturns the name of the POUs container it has no container …\nreturns all member variables of the given container (e.g. …\nreturns some if the current index is a VAR_INPUT, …\nreturns the effective DataType of the type with the given …\nreturns the effective type of the type with the with the …\nReturns all enum variants of the given variable.\nReturns all enum variants defined in the given POU\nReturns the map of globals, should not be used to search …\nReturns the initioal value registered for the given …\nReturns a default initialization name for a variable or …\nreturns the name of the struct-type used to store the POUs …\nreturns the struct-datatype associated with this pou or …\nreturns the intrinsic type of the type with the given name …\nreturns the linkage type of this pou\nreturns the mutable reference to all registered …\nreturns the fully qualified name of this pou\nreturns all member variables of the given POU (e.g. …\nReturns the map of pou_types, should not be used to search …\nReturns the map of pous, should not be used to search for …\nreturns the super class of this pou if supported\nexpect a built-in type This only returns types, not POUs …\nReturns the map of types, should not be used to search for …\nreturns the void-type the NULL-statement has a type of …\nimports all entries from the given index into the current …\nan optional initial value of this variable\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreates a new VariableIndexEntry from the current entry …\nreturns true if this pou is a function with generic …\nreturns <code>true</code> for <code>VAR_INPUT {ref}</code> and <code>VAR_IN_OUT</code>\nreturns true if this pou has a variadic (last) parameter\nThe location in the original source-file\ncreates a member-variable of a container to be accessed in …\nthe location in the original source-file\nIndicates that the const expression is not resolvable …\nconstant expressions registered here are wrapped behind …\nInitializers which rely on code-execution/allocated memory …\nIndicates that the const expression was not resolvable for …\nIndicates that the const expression was not resolvable …\nadds the given constant expression to the constants arena …\nadds the const expression <code>statement</code>\nclones the expression in the ConstExpressions and returns …\nsimilar to <code>find_expression</code> but it does not return the …\nreturns the expression associated with the given <code>id</code> …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nquery the constants arena for an expression that can be …\nquery the constants arena for an expression associated …\nreturns an optional qualifier that should be used as a …\nquery the constants arena for a resolved expression …\nreturns the const-expression represented as an AST-element\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nmarks the const-expression represented by the given <code>id</code> as …\nmarks the const-expression represented by the given <code>id</code> as …\nconvinience-method to add the constant exression if there …\nconvinience-method to query for an optional constant …\nthe name of the variable this expression is assigned to, …\noptional qualifier used when evaluating this expression …\nA multi-map implementation with a stable order of …\nreturn <code>true</code> if an equivalent to key exists in the map.\nremoves and returns all elements in the SymbolMap\nreturns an iterator over all elements key-value tuples in …\nreturns an iterator over all entries of this map as pairs …\nextends the map with the contents of an iterator.\nReturns the argument unchanged.\nreturns the first element associated with the given key or …\nreturns all elements associated with the given key or None …\nassociates the given value with the give key. Existing …\ninserts all given elements and associates them with the …\nCalls <code>U::from(self)</code>.\nreturns an iterator over the keys in the map, in their …\nreturns an iterator over the values in the map The order …\nreturns true if the given token closes an open region\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nthe range of the <code>last_token</code>\nthe token parsed before the current one stored in <code>token</code>\nTries to consume the given token, returning false if it …\nError emitted by the linker\nError in path conversion\nInvalid target\nAdd a library path to look in for libraries\nAdd a library seaBoxh path to look in for libraries\nAdd an object file or static library to linker input\nAdd path to system root\nSet the output file and run the linker to generate an …\nSet the output file and run the linker to generate a …\nSet the output file and run the linker to generate a …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nIndicates that the compile result will be LLVM Bitcode\nIndicates that the compile result will be LLVM IR\nIndicates that the compiled object will be a DynamicNoPIC …\nIndicates that the result will be an object file (e.g. No …\nIndicates that the linked object will be Position …\nIndicates that the compiled object will be relocatable …\nIndicates that the linked object will be shared and …\nIndicates that the output format will be linked statically …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nparse_expression(): returns expression as Statement. if a …\nParses a literal integer without considering Signs or the …\na reference to a function\na reference to a label in a POU\na reference to a program call or reference (e.g. <code>PLC_PRG</code>)\na reference to a type (e.g. <code>INT</code>)\nan expression that resolves to a certain type (e.g. <code>a + b</code> …\na reference that resolves to a declared variable (e.g. <code>a</code> …\nContext object passed by the visitor Denotes the current …\nAnnotates the ast statement with its original generic …\nannotates the given statement (using it’s <code>get_id()</code>) with …\nannotates the given statement s with the call-statement f …\nscopes that can be used for general references. Will …\nDerives the correct type for the generic call from the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nreturns the name of the callable that is refered by the …\nreturns the function call previously annoted on s via …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\ncreates a new empty AnnotationMap\ndouplicates the given generic_function under the <code>new_name</code> …\nupdates the expected types of statements on the right side …\nConstructs a new <code>StatementAnnotation::Value</code> with the given …\nannotates the given AST elements with the type-name …\ndenotes the variable type of this variable, hence whether …\ndenotes wheter this variable has the auto-deref trait and …\nThe call name of the function iff it defers from the …\ndenotes whether this variable is declared as a constant\nthe fully qualified name of this variable (e.g. <code>&quot;MyFB.a&quot;</code>)\nThe defined qualified name of the function\nthe name of the variable’s type (e.g. <code>&quot;INT&quot;</code>)\nThe defined return type of the function\na wrapper for an unresolvable const-expression with the …\nreturns the resolved constants index and a Vec of …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nReturns the argument unchanged.\nBuilds the correct generic name from the given information\nCalls <code>U::from(self)</code>.\nThis method returns the qualified name, but has the same …\nused internally for forced casts to u1\nindicates where this Struct origins from.\nEnum for ranges and aggregate type sizes.\nreturns the const expression represented by this TypeSize …\ntries to compile-time evaluate the size-expression to an …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the array length if <code>DataTypeInformation</code> is of …\nReturns the bigger of the two provided types\nreturns the compare-function name for the given type and …\nRecursively retrieves all type names for nested arrays.\nIdentical to [<code>get_range</code>] except for adding 1 to the end of …\nreturns the number of bits of this type, as understood by …\nreturns the signed version of the given data_type if its a …\nreturns the number of bits used to store this type\nReturns the String encoding’s alignment (character)\nthe initial value defined on the TYPE-declaration\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nreturns true if this type is an array, struct or string\nReturns true if the variable was declared as <code>REFERENCE TO</code>, …\nreturns true if this type is an array\nreturns true if this type is an enum\nreturns true if this type is an internal, auto-generated …\nReturns true if the variable was declared as <code>REFERENCE TO</code>, …\nReturns true if provided types have the same type nature …\nthe numer of bits represented by this type (may differ …\nthe number of bit stored in memory\nThis trait should be implemented by any validator used by …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.")