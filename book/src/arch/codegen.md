# Code-Generation
The codegen module contains all code that turns the parsed and verified code represented as an AST into [llvm-ir](https://llvm.org/docs/LangRef.html) code. To generate the *IR* we use a crate that wraps the native llvm [C-API](https://github.com/TheDan64/inkwell).

The code-generator is basically a transformation from the ST-AST into an IR-Tree representation. Therefore the AST is traversed in a visitor-like way and transformed simultaneously. The code generation is split into specialized sub-generators for different tasks:

| Generator          | Responsibilities |
|--------------------|------------------|
| pou_generator      | The pou-generator takes care of generating the programming organization units (Programs, FunctionBlocks, Functions) including their signature and body. More specialized tasks are delegated to other generators.  |
| data_type_generator  | Generates complex datatypes like Structs, Arrays, Enums, Strings, etc. |
| variable_generator   | Generates global variables and their initialization | 
| statement_generator  | Generates everything of the body of a POU except expressions. Non-expressions include: IFs, Loops, Assignments, etc. | 
| expression_generator | Generates expressions (everything that *possibly* resolves to a value) including: call-statements, references, array-access, etc. | 

## Generating POUs
Generating POUs (Programs, Function-Blocks, Functions) must generate the POU's body itself, as well as the POU's interface (or state) variables. In this segment we focus on generating the interface for a POU. Further information about generating a POU's body can be found [here].
### Programs
A program is *static* POU with some code attached. This means that there is exactly one instance. So wherever from it is called, every caller uses the exact same instance which means that you may see the residuals of the laster caller in the program's variables when you call it yourself. 

```iecst
PROGRAM prg
    VAR
        x : DINT;
        y : DINT;
    END_VAR

END_PROGRAM
```
 
The program's interface is persistent across calls, so we store it in a global variable. Therefore the code-generator creates a dedicated struct-type called `prg_interface`. A global variable called `prg_instance` is generated to store the program's state across calls. This global instance variable is passed as a `this` pointer to calls to the `prg` function. 

```llvm
%prg_interface = type { i32, i32 }

@prg_instance = global %prg_interface zeroinitializer

define void @prg(%prg_interface* %this) {
entry:
  ret void
}
```

### FunctionBlocks
A FunctionBlock is an POU that is instantiated in a declaration. So in contrast to Programs, a FunctionBlock can have multiple instances. Nevertheless the code-generator uses a very similar strategy. A struct-type for the FunctionBlock's interface is created but no global instance-variable is allocated. Instead the function block can be used as a DataType to declare instances like in the following example:

```iecst
FUNCTION_BLOCK foo
  VAR_INPUT
    x, y : INT;
  END_VAR
END_FUNCTION_BLOCK

PROGRAM prg
  VAR
    f : foo;
  END_VAR
END_PROGRAM
```

So for the given example, we see the code-generator creating a type for the FunctionBlock's state (`foo_interface`). The declared instance of foo, in `prg's` interface is seen in the program's generated interface struct-type (`prg_interface`).

```llvm
; ModuleID = 'main'
source_filename = "main"

%prg_interface = type { %foo_interface }
%foo_interface = type { i16, i16 }

@prg_instance = global %prg_interface zeroinitializer

define void @foo(%foo_interface* %0) {
entry:
  ret void
}

define void @prg(%prg_interface* %0) {
entry:
  ret void
}
```

### Functions
Functions generate very similar to program's and function_block's. The main difference is, that no instance-global is allocated and the function's interface-type cannot be used as a datatype to declare your own instances. Instances of the program's interface-type are allocated whenever the function is called for the lifetime of a single call. Otherwise the code generated for functions is comparable to the code presented above for programs and function-blocks.

## Generating Data Types
IEC61131-3 languages offer a wide range of data types. Next to the built-in intrinsic data types, we support following user defined data types:
### Range Types
For range types we don't generate special code. Internally the new data type just becomes an alias for the derived type.
### Pointer Types
For pointer types we don't generate special code. Internally the new data type just becomes an alias for the pointer-type.
### Struct Types
Struct types translate direclty to llvm struct datatypes. We generate a new datatype with the user-type's name for the struct.
```iecst
TYPE MyStruct: 
  STRUCT
    a: DINT;
    b: INT;
  END_STRUCT
END_TYPE
```
This struct simply generates a llvm struct type:
```llvm
%MyStruct = type { i32, i16 }
```
### Enum Types
Enumerations are represented as `DINT`. 
```iecst
TYPE MyEnum: (red, yellow, green);
END_TYPE
```
For every enum's element we generate a global variable with the element's value.
```llvm
@red = global i32 0
@yellow = global i32 1
@green = global i32 2
```

### Array Types
Array types are generated as fixed sized llvm vector types - note that Array types must be fixed sized in *ST*:
```iecst
TYPE MyArray: ARRAY[0..9] OF INT; 
END_TYPE

VAR_GLOBAL
  x : MyArray;
  y : ARRAY[0..5] OF REAL;
END_VAR
```
Custom array data types are not reflected as dedicated types on the llvm-level.
```llvm
@x = global [10 x i16] zeroinitializer
@y = global [6 x float] zeroinitializer
```

#### Multi dimensional arrays

Arrays can be declared as multi-dimensional:
```iecst
VAR_GLOBAL
  x : ARRAY[0..5, 2..5, 0..1] OF INT;
END_VAR
```

The compiler will flatten these type of arrays to a single-dimension. To accomplish that, it calculates the total
length by mulitplying the sizes of all dimensions:
```ignore
    0..5 x 2..5 x 0..1
      6  x   4  x   2  = 64
```
So the array `x : ARRAY[0..5, 2..5, 0..1] OF INT;` will be generated as:
```llvm
@x = global [64 x i16] zeroinitializer
```
This means that such a multidimensional array must be initialized like a single-dimensional array:
- *wrong*
```iecst
VAR_GLOBAL
  wrong_array : ARRAY[1..2, 0..3] OF INT := [ [10, 11, 12], 
                                              [20, 21, 22], 
                                              [30, 31, 32]]; 
END_VAR
```
- *correct*
```iecst
VAR_GLOBAL
  correct_array : ARRAY[1..2, 0..3] OF INT := [ 10, 11, 12, 
                                                20, 21, 22, 
                                                30, 31, 32]; 
END_VAR
```
> *Nested Arrays*
>
> Note that arrays declared as `x : ARRAY[0..2] OF ARRAY[0..2] OF INT` are different from mutli-dimensional 
> arrays discussed in this section. Nested arrays are represented as multi-dimensional arrays on the LLVM-IR 
> level and must also be initialized using nested array-literals!

### String Types
String types are generated as fixed sized vector types.
```iecst
VAR_GLOBAL
    str  : STRING[20];
    wstr : WSTRING[20];
END_VAR
```
Strings can be represented in two different encodings: *UTF-8 (STRING)* or *UTF-16 (WSTRING)*. 
```llvm
@str = global [21 x i8] zeroinitializer
@wstr = global [21 x i16] zeroinitializer
```

