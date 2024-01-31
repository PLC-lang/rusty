#![allow(dead_code, unused_variables)]

// These data structures provide definitions for the wrapped C LLVM interface.
//
// These type definitions are taken from:
// - [`rustc_codegen_ssa/src/coverageinfo/ffi.rs`](https://github.com/rust-lang/rust/blob/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/compiler/rustc_codegen_ssa/src/coverageinfo/ffi.rs#L4-L5)
// - [`rustc_codegen_llvm/src/coverageinfo/ffi.rs`](https://github.com/rust-lang/rust/blob/56278a6e2824acc96b222e5816bf2d74e85dab93/compiler/rustc_codegen_llvm/src/coverageinfo/ffi.rs#L4)
// - [`rustc_middle/src/mir/coverage.rs`](https://github.com/rust-lang/rust/blob/56278a6e2824acc96b222e5816bf2d74e85dab93/compiler/rustc_middle/src/mir/coverage.rs#L9)
//

use std::cell::RefCell;

#[repr(C)]
pub struct RustString {
    pub bytes: RefCell<Vec<u8>>,
}

impl RustString {
    pub fn new() -> Self {
        Self { bytes: RefCell::new(Vec::new()) }
    }

    pub fn len(&self) -> usize {
        self.bytes.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.borrow().is_empty()
    }
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct CounterId(u32);

impl CounterId {
    pub const START: Self = Self(0);

    pub fn new(value: u32) -> Self {
        CounterId(value)
    }

    pub fn from_u32(value: u32) -> Self {
        CounterId::new(value)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ExpressionId(u32);

impl ExpressionId {
    pub const START: Self = Self(0);

    pub fn new(value: u32) -> Self {
        ExpressionId(value)
    }

    pub fn from_u32(value: u32) -> Self {
        ExpressionId::new(value)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

/// Corresponds to enum `llvm::coverage::CounterMappingRegion::RegionKind`.
///
/// Must match the layout of `LLVMRustCounterMappingRegionKind`.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum RegionKind {
    /// A CodeRegion associates some code with a counter
    CodeRegion = 0,

    /// An ExpansionRegion represents a file expansion region that associates
    /// a source range with the expansion of a virtual source file, such as
    /// for a macro instantiation or #include file.
    ExpansionRegion = 1,

    /// A SkippedRegion represents a source range with code that was skipped
    /// by a preprocessor or similar means.
    SkippedRegion = 2,

    /// A GapRegion is like a CodeRegion, but its count is only set as the
    /// line execution count when its the only region in the line.
    GapRegion = 3,

    /// A BranchRegion represents leaf-level boolean expressions and is
    /// associated with two counters, each representing the number of times the
    /// expression evaluates to true or false.
    BranchRegion = 4,
}

/// This struct provides LLVM's representation of a "CoverageMappingRegion", encoded into the
/// coverage map, in accordance with the
/// [LLVM Code Coverage Mapping Format](https://github.com/rust-lang/llvm-project/blob/rustc/13.0-2021-09-30/llvm/docs/CoverageMappingFormat.rst#llvm-code-coverage-mapping-format).
/// The struct composes fields representing the `Counter` type and value(s) (injected counter
/// ID, or expression type and operands), the source file (an indirect index into a "filenames
/// array", encoded separately), and source location (start and end positions of the represented
/// code region).
///
/// Corresponds to struct `llvm::coverage::CounterMappingRegion`.
///
/// Must match the layout of `LLVMRustCounterMappingRegion`.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct CounterMappingRegion {
    /// The counter type and type-dependent counter data, if any.
    pub counter: Counter,

    /// If the `RegionKind` is a `BranchRegion`, this represents the counter
    /// for the false branch of the region.
    pub false_counter: Counter,

    /// An indirect reference to the source filename. In the LLVM Coverage Mapping Format, the
    /// file_id is an index into a function-specific `virtual_file_mapping` array of indexes
    /// that, in turn, are used to look up the filename for this region.
    file_id: u32,

    /// If the `RegionKind` is an `ExpansionRegion`, the `expanded_file_id` can be used to find
    /// the mapping regions created as a result of macro expansion, by checking if their file id
    /// matches the expanded file id.
    expanded_file_id: u32,

    /// 1-based starting line of the mapping region.
    start_line: u32,

    /// 1-based starting column of the mapping region.
    start_col: u32,

    /// 1-based ending line of the mapping region.
    end_line: u32,

    /// 1-based ending column of the mapping region. If the high bit is set, the current
    /// mapping region is a gap area.
    end_col: u32,

    pub kind: RegionKind,
}

impl CounterMappingRegion {
    pub fn code_region(
        counter: Counter,
        file_id: u32,
        start_line: u32,
        start_col: u32,
        end_line: u32,
        end_col: u32,
    ) -> Self {
        Self {
            counter,
            false_counter: Counter::ZERO,
            file_id,
            expanded_file_id: 0,
            start_line,
            start_col,
            end_line,
            end_col,
            kind: RegionKind::CodeRegion,
        }
    }

    // This function might be used in the future; the LLVM API is still evolving, as is coverage
    // support.
    // #[allow(dead_code)]
    pub fn branch_region(
        counter: Counter,
        false_counter: Counter,
        file_id: u32,
        start_line: u32,
        start_col: u32,
        end_line: u32,
        end_col: u32,
    ) -> Self {
        Self {
            counter,
            false_counter,
            file_id,
            expanded_file_id: 0,
            start_line,
            start_col,
            end_line,
            end_col,
            kind: RegionKind::BranchRegion,
        }
    }

    // This function might be used in the future; the LLVM API is still evolving, as is coverage
    // support.
    #[allow(dead_code)]
    pub(crate) fn expansion_region(
        file_id: u32,
        expanded_file_id: u32,
        start_line: u32,
        start_col: u32,
        end_line: u32,
        end_col: u32,
    ) -> Self {
        Self {
            counter: Counter::ZERO,
            false_counter: Counter::ZERO,
            file_id,
            expanded_file_id,
            start_line,
            start_col,
            end_line,
            end_col,
            kind: RegionKind::ExpansionRegion,
        }
    }

    // This function might be used in the future; the LLVM API is still evolving, as is coverage
    // support.
    #[allow(dead_code)]
    pub(crate) fn skipped_region(
        file_id: u32,
        start_line: u32,
        start_col: u32,
        end_line: u32,
        end_col: u32,
    ) -> Self {
        Self {
            counter: Counter::ZERO,
            false_counter: Counter::ZERO,
            file_id,
            expanded_file_id: 0,
            start_line,
            start_col,
            end_line,
            end_col,
            kind: RegionKind::SkippedRegion,
        }
    }

    // This function might be used in the future; the LLVM API is still evolving, as is coverage
    // support.
    #[allow(dead_code)]
    pub(crate) fn gap_region(
        counter: Counter,
        file_id: u32,
        start_line: u32,
        start_col: u32,
        end_line: u32,
        end_col: u32,
    ) -> Self {
        Self {
            counter,
            false_counter: Counter::ZERO,
            file_id,
            expanded_file_id: 0,
            start_line,
            start_col,
            end_line,
            end_col: (1_u32 << 31) | end_col,
            kind: RegionKind::GapRegion,
        }
    }
}

/// Aligns with [llvm::coverage::Counter::CounterKind](https://github.com/rust-lang/llvm-project/blob/rustc/13.0-2021-09-30/llvm/include/llvm/ProfileData/Coverage/CoverageMapping.h#L95)
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum CounterKind {
    Zero = 0,
    CounterValueReference = 1,
    Expression = 2,
}

/// A reference to an instance of an abstract "counter" that will yield a value in a coverage
/// report. Note that `id` has different interpretations, depending on the `kind`:
///   * For `CounterKind::Zero`, `id` is assumed to be `0`
///   * For `CounterKind::CounterValueReference`,  `id` matches the `counter_id` of the injected
///     instrumentation counter (the `index` argument to the LLVM intrinsic
///     `instrprof.increment()`)
///   * For `CounterKind::Expression`, `id` is the index into the coverage map's array of
///     counter expressions.
/// Aligns with [llvm::coverage::Counter](https://github.com/rust-lang/llvm-project/blob/rustc/13.0-2021-09-30/llvm/include/llvm/ProfileData/Coverage/CoverageMapping.h#L102-L103)
/// Important: The Rust struct layout (order and types of fields) must match its C++ counterpart.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Counter {
    // Important: The layout (order and types of fields) must match its C++ counterpart.
    pub kind: CounterKind,
    pub id: u32,
}

impl Counter {
    /// A `Counter` of kind `Zero`. For this counter kind, the `id` is not used.
    pub const ZERO: Self = Self { kind: CounterKind::Zero, id: 0 };

    /// Constructs a new `Counter` of kind `CounterValueReference`.
    pub fn counter_value_reference(counter_id: CounterId) -> Self {
        Self { kind: CounterKind::CounterValueReference, id: counter_id.as_u32() }
    }

    /// Constructs a new `Counter` of kind `Expression`.
    pub fn expression(expression_id: ExpressionId) -> Self {
        Self { kind: CounterKind::Expression, id: expression_id.as_u32() }
    }
}

/// Aligns with [llvm::coverage::CounterExpression::ExprKind](https://github.com/rust-lang/llvm-project/blob/rustc/13.0-2021-09-30/llvm/include/llvm/ProfileData/Coverage/CoverageMapping.h#L150)
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum ExprKind {
    Subtract = 0,
    Add = 1,
}

/// Aligns with [llvm::coverage::CounterExpression](https://github.com/rust-lang/llvm-project/blob/rustc/13.0-2021-09-30/llvm/include/llvm/ProfileData/Coverage/CoverageMapping.h#L151-L152)
/// Important: The Rust struct layout (order and types of fields) must match its C++
/// counterpart.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct CounterExpression {
    pub kind: ExprKind,
    pub lhs: Counter,
    pub rhs: Counter,
}

impl CounterExpression {
    pub fn new(lhs: Counter, kind: ExprKind, rhs: Counter) -> Self {
        Self { kind, lhs, rhs }
    }
}
