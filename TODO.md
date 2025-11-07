# Initializer Implementation Status

## Overview

The goal is for each datatype/POU to get a constructor (`<TypeName>_ctor`), and then a global constructor is created with calls to all constructors in the unit.

## Current Implementation

### What's Already Created (in `plc_lowering/src/initializer.rs`)

#### 1. Per-Type Constructors (`<TypeName>_ctor`)
- [x] Collects constructor body statements for each POU and struct
- [x] Tracks `Body::Internal`, `Body::External`, or `Body::None` per type
- [x] Generates assignment statements (`self.field := value`) for variables with initializers
- [x] Handles nested struct initializers
- [x] Generates call to user-defined `FB_INIT` method if present
- [x] Creates the constructor POU and Implementation AST nodes via `new_constructor()`
- [x] Adds constructors to the CompilationUnit

#### 2. Stack Constructors (for temp/local variables in stateless POUs)
- [x] Tracks per-POU (`stack_constructor` map)
- [x] Applied to function bodies

#### 3. Global Constructor Statements
- [x] Collects statements for global variables into `global_constructor` Vec
- [x] Generated statements for global initializers

---

## What's Missing

### 4. Features from Old Implementation Not Yet Ported

Comparing old `InitVisitor` (in `src/lowering/init_visitor.rs` + `src/lowering/initializers.rs`) vs new `Initializer`:

| Feature | Old (`InitVisitor`) | New (`Initializer`) |
|---------|---------------------|---------------------|
| Type constructors (`__init_<type>`) | Done | Done (`<type>_ctor`) |
| User init functions (`__user_init_<type>`) | Done | Missing |
| Global wrapper (`__init___<project>`) | Done | Missing |
| VAR_CONFIG init | Done | Missing |
| VTable init in constructor | Done | Missing |
| Stack var init in functions | Done | Collected but not applied |
| External linkage handling | Done | Done |

---

## Implementation Plan

### Step 1: Generate Global Constructor

In `apply_initialization()`, after adding all type constructors:

```rust
// Create __global_ctor function
if !self.global_constructor.is_empty() {
    let mut global_ctor_body = vec![];

    // Add calls to constructors for global struct instances
    for (var_name, var_type) in global_struct_instances {
        if self.constructors.contains_key(&var_type) {
            let call = create_call_statement(
                &format!("{}_ctor", var_type),
                var_name,
                None,
                self.id_provider.clone(),
                &SourceLocation::internal(),
            );
            global_ctor_body.push(call);
        }
    }

    // Add collected assignment statements
    global_ctor_body.extend(self.global_constructor);

    // Create the POU and implementation
    let (pou, impl) = new_global_constructor("__global_ctor", global_ctor_body, ...);
    unit.pous.push(pou);
    unit.implementations.push(impl);
}
```

### Step 2: Apply Stack Constructors

Modify function implementations to include stack initialization:

```rust
// For each implementation in unit.implementations
for impl in &mut unit.implementations {
    if let Some(Body::Internal(stmts)) = self.stack_constructor.get(&impl.name) {
        // Prepend stack constructor statements to the implementation body
        let mut new_body = stmts.clone();
        new_body.extend(impl.statements.drain(..));
        impl.statements = new_body;
    }
}
```

### Step 3: Ensure Constructor Call Chain

In `visit_variable()`, verify that for struct-typed variables:
1. First, call the struct's constructor: `<StructType>_ctor(self.var_name)`
2. Then, apply any field overrides from the initializer

### Step 4: Port Remaining Features

1. **VTable initialization:** Add `self.__vtable := ADR(__vtable_<type>_instance)` to constructor body for classes/FBs
2. **User init functions:** Generate `__user_init_<type>` that calls `FB_INIT` if present
3. **VAR_CONFIG init:** Handle VAR_CONFIG initialization in global constructor

---

## Files to Modify

- `compiler/plc_lowering/src/initializer.rs` - Main implementation
- `src/lowering/helper.rs` - May need additional helper functions
- `compiler/plc_driver/src/pipelines/participant.rs` - InitParticipant integration

## Testing

Existing tests in `compiler/plc_lowering/src/initializer.rs` cover:
- Struct constructors
- Nested structs
- Pointer initializers
- Enum defaults
- Global constructor collection
- Function/program constructors
- External types
- FB_INIT calls
- Inheritance chains

Additional tests needed:
- [ ] Global constructor function generation
- [ ] Stack constructor application to function bodies
- [ ] Constructor call chain for nested structs
- [ ] VTable initialization
- [ ] Integration with codegen (actual LLVM output)
