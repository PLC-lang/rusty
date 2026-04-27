//! This crate provides Rust bindings for LLVM Target Machine functionalities
//! and extensions to inkwell's debug info API.

use std::ffi::c_void;
use std::marker::PhantomData;

use inkwell::debug_info::{DICompileUnit, DIFile, DWARFEmissionKind, DWARFSourceLanguage, DebugInfoBuilder};
use inkwell::llvm_sys::debuginfo::{LLVMDWARFEmissionKind, LLVMDWARFSourceLanguage};
use inkwell::llvm_sys::prelude::LLVMBool;
use inkwell::module::Module;
use inkwell::targets::TargetMachine;

/// Opaque pointer matching LLVMDIBuilderRef (= *mut LLVMOpaqueDIBuilder).
/// We define our own alias to avoid depending on the private re-export path.
type LLVMDIBuilderRef = *mut c_void;

mod ffi {
    use std::ffi::c_void;

    use inkwell::llvm_sys::prelude::{LLVMBool, LLVMMetadataRef};
    use inkwell::llvm_sys::target_machine::LLVMTargetMachineRef;

    /// Opaque pointer matching LLVMDIBuilderRef.
    type LLVMDIBuilderRef = *mut c_void;

    #[link(name = "llvm_wrapper")]
    unsafe extern "C" {
        pub fn setUseInitArray(tm: LLVMTargetMachineRef, use_init_array: LLVMBool);

        pub fn createCompileUnit(
            builder: LLVMDIBuilderRef,
            file: LLVMMetadataRef,
            lang: std::ffi::c_uint,
            producer: *const std::ffi::c_char,
            producer_len: usize,
            is_optimized: LLVMBool,
            runtime_ver: std::ffi::c_uint,
            emission_kind: std::ffi::c_uint,
            dwo_id: u64,
            split_debug_inlining: LLVMBool,
            debug_info_for_profiling: LLVMBool,
            enable_pubnames: LLVMBool,
            sys_root: *const std::ffi::c_char,
            sys_root_len: usize,
            sdk: *const std::ffi::c_char,
            sdk_len: usize,
        ) -> LLVMMetadataRef;
    }
}

pub trait TargetMachineExt {
    fn use_init_array(&mut self, use_init_array: bool);
}

impl TargetMachineExt for TargetMachine {
    /// Set whether to use `.init_array` and `.fini_array` sections for global
    /// constructors and destructors instead of the legacy `.ctors` and `.dtors`
    /// sections.
    ///
    /// By default, LLVM uses `.ctors` and `.dtors` sections. This method allows
    /// changing that behavior.
    fn use_init_array(&mut self, use_init_array: bool) {
        let tm = self.as_mut_ptr();
        unsafe {
            ffi::setUseInitArray(tm, if use_init_array { 1 } else { 0 } as LLVMBool);
        }
    }
}

/// Creates a [`DebugInfoBuilder`] and [`DICompileUnit`] with control over `.debug_names` emission.
///
/// This wraps LLVM's C++ `DIBuilder::createCompileUnit` which exposes the `NameTableKind`
/// parameter that the LLVM C API does not. When `generate_pubnames` is `false`, sets
/// `NameTableKind::None` to suppress `.debug_names` emission, avoiding a GDB incompatibility
/// when multiple compilation units are linked with `lld`.
#[allow(clippy::too_many_arguments)]
pub fn create_debug_info<'ctx>(
    module: &Module<'ctx>,
    allow_unresolved: bool,
    language: DWARFSourceLanguage,
    filename: &str,
    directory: &str,
    producer: &str,
    is_optimized: bool,
    runtime_ver: std::ffi::c_uint,
    emission_kind: DWARFEmissionKind,
    generate_pubnames: bool,
) -> (DebugInfoBuilder<'ctx>, DICompileUnit<'ctx>) {
    use inkwell::llvm_sys::debuginfo::{
        LLVMCreateDIBuilder, LLVMCreateDIBuilderDisallowUnresolved, LLVMDIBuilderCreateFile,
    };

    unsafe {
        let raw_module = module.as_mut_ptr();
        let builder_ref: LLVMDIBuilderRef = if allow_unresolved {
            LLVMCreateDIBuilder(raw_module).cast()
        } else {
            LLVMCreateDIBuilderDisallowUnresolved(raw_module).cast()
        };

        let file_ref = LLVMDIBuilderCreateFile(
            builder_ref.cast(),
            filename.as_ptr().cast(),
            filename.len(),
            directory.as_ptr().cast(),
            directory.len(),
        );

        let cu_ref = ffi::createCompileUnit(
            builder_ref,
            file_ref,
            {
                let lang: LLVMDWARFSourceLanguage = language.into();
                lang as std::ffi::c_uint
            },
            producer.as_ptr().cast(),
            producer.len(),
            is_optimized as LLVMBool,
            runtime_ver,
            {
                let kind: LLVMDWARFEmissionKind = emission_kind.into();
                kind as std::ffi::c_uint
            },
            0, // dwo_id
            0, // split_debug_inlining = false
            0, // debug_info_for_profiling = false
            generate_pubnames as LLVMBool,
            c"".as_ptr(),
            0,
            c"".as_ptr(),
            0,
        );

        // Construct inkwell wrapper types from raw LLVM pointers.
        // These types are thin wrappers with predictable layout:
        //   DebugInfoBuilder { builder: LLVMDIBuilderRef, _marker: PhantomData }
        //   DIFile           { metadata_ref: LLVMMetadataRef, _marker: PhantomData }
        //   DICompileUnit    { file: DIFile, metadata_ref: LLVMMetadataRef, _marker: PhantomData }
        let debug_info: DebugInfoBuilder<'ctx> = std::mem::transmute(builder_ref);
        let compile_unit: DICompileUnit<'ctx> =
            std::mem::transmute((file_ref, cu_ref, PhantomData::<&'ctx ()>));

        // Compile-time layout verification
        const _: () = {
            assert!(size_of::<DebugInfoBuilder>() == size_of::<usize>() + size_of::<PhantomData<&()>>());
            assert!(
                size_of::<DICompileUnit>()
                    == size_of::<DIFile>() + size_of::<usize>() + size_of::<PhantomData<&()>>()
            );
        };

        // Runtime verification in debug builds
        debug_assert_eq!(debug_info.as_mut_ptr().cast::<c_void>(), builder_ref);
        debug_assert_eq!(compile_unit.as_mut_ptr(), cu_ref);
        debug_assert_eq!(compile_unit.get_file().as_mut_ptr(), file_ref);

        (debug_info, compile_unit)
    }
}
