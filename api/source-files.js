var sourcesIndex = JSON.parse('{\
"aho_corasick":["",[["packed",[["teddy",[],["compile.rs","mod.rs","runtime.rs"]]],["api.rs","mod.rs","pattern.rs","rabinkarp.rs","vector.rs"]]],["ahocorasick.rs","automaton.rs","buffer.rs","byte_frequencies.rs","classes.rs","dfa.rs","error.rs","lib.rs","nfa.rs","prefilter.rs","state_id.rs"]],\
"atty":["",[],["lib.rs"]],\
"beef":["",[],["generic.rs","lean.rs","lib.rs","traits.rs","wide.rs"]],\
"bitflags":["",[],["lib.rs"]],\
"cfg_if":["",[],["lib.rs"]],\
"chrono":["",[["datetime",[],["mod.rs"]],["format",[],["mod.rs","parse.rs","parsed.rs","scan.rs","strftime.rs"]],["naive",[["datetime",[],["mod.rs"]],["time",[],["mod.rs"]]],["date.rs","internals.rs","isoweek.rs","mod.rs"]],["offset",[],["fixed.rs","mod.rs","utc.rs"]]],["date.rs","lib.rs","month.rs","oldtime.rs","round.rs","traits.rs","weekday.rs"]],\
"clap":["",[["builder",[],["action.rs","app_settings.rs","arg.rs","arg_group.rs","arg_predicate.rs","arg_settings.rs","command.rs","debug_asserts.rs","macros.rs","mod.rs","possible_value.rs","usage_parser.rs","value_hint.rs","value_parser.rs"]],["error",[],["context.rs","kind.rs","mod.rs"]],["output",[],["fmt.rs","help.rs","mod.rs","usage.rs"]],["parser",[["features",[],["mod.rs","suggestions.rs"]],["matches",[],["any_value.rs","arg_matches.rs","matched_arg.rs","mod.rs","value_source.rs"]]],["arg_matcher.rs","error.rs","mod.rs","parser.rs","validator.rs"]],["util",[],["color.rs","fnv.rs","graph.rs","id.rs","mod.rs","str_to_bool.rs"]]],["derive.rs","lib.rs","macros.rs","mkeymap.rs"]],\
"clap_derive":["",[["derives",[],["args.rs","into_app.rs","mod.rs","parser.rs","subcommand.rs","value_enum.rs"]],["utils",[],["doc_comments.rs","mod.rs","spanned.rs","ty.rs"]]],["attrs.rs","dummies.rs","lib.rs","parse.rs"]],\
"clap_lex":["",[],["lib.rs"]],\
"codespan_reporting":["",[["term",[],["config.rs","renderer.rs","views.rs"]]],["diagnostic.rs","files.rs","lib.rs","term.rs"]],\
"either":["",[],["lib.rs"]],\
"encoding_rs":["",[],["ascii.rs","big5.rs","data.rs","euc_jp.rs","euc_kr.rs","gb18030.rs","handles.rs","iso_2022_jp.rs","lib.rs","macros.rs","mem.rs","replacement.rs","shift_jis.rs","single_byte.rs","utf_16.rs","utf_8.rs","variant.rs","x_user_defined.rs"]],\
"encoding_rs_io":["",[],["lib.rs","util.rs"]],\
"fastrand":["",[],["lib.rs"]],\
"fnv":["",[],["lib.rs"]],\
"generational_arena":["",[],["lib.rs"]],\
"glob":["",[],["lib.rs"]],\
"hashbrown":["",[["external_trait_impls",[],["mod.rs"]],["raw",[],["alloc.rs","bitmask.rs","mod.rs","sse2.rs"]]],["lib.rs","macros.rs","map.rs","scopeguard.rs","set.rs"]],\
"heck":["",[],["kebab.rs","lib.rs","lower_camel.rs","shouty_kebab.rs","shouty_snake.rs","snake.rs","title.rs","train.rs","upper_camel.rs"]],\
"indexmap":["",[["map",[["core",[],["raw.rs"]]],["core.rs"]]],["arbitrary.rs","equivalent.rs","lib.rs","macros.rs","map.rs","mutable_keys.rs","set.rs","util.rs"]],\
"inkwell":["",[["support",[],["error_handling.rs","mod.rs"]],["types",[],["array_type.rs","enums.rs","float_type.rs","fn_type.rs","int_type.rs","metadata_type.rs","mod.rs","ptr_type.rs","struct_type.rs","traits.rs","vec_type.rs","void_type.rs"]],["values",[],["array_value.rs","basic_value_use.rs","call_site_value.rs","callable_value.rs","enums.rs","float_value.rs","fn_value.rs","generic_value.rs","global_value.rs","instruction_value.rs","int_value.rs","metadata_value.rs","mod.rs","phi_value.rs","ptr_value.rs","struct_value.rs","traits.rs","vec_value.rs"]]],["attributes.rs","basic_block.rs","builder.rs","comdat.rs","context.rs","data_layout.rs","debug_info.rs","execution_engine.rs","intrinsics.rs","lib.rs","memory_buffer.rs","module.rs","object_file.rs","passes.rs","targets.rs"]],\
"inkwell_internals":["",[],["lib.rs"]],\
"io_lifetimes":["",[],["example_ffi.rs","lib.rs","portability.rs","raw.rs","traits.rs","views.rs"]],\
"itertools":["",[["adaptors",[],["coalesce.rs","map.rs","mod.rs","multi_product.rs"]]],["combinations.rs","combinations_with_replacement.rs","concat_impl.rs","cons_tuples_impl.rs","diff.rs","duplicates_impl.rs","either_or_both.rs","exactly_one_err.rs","extrema_set.rs","flatten_ok.rs","format.rs","free.rs","group_map.rs","groupbylazy.rs","grouping_map.rs","impl_macros.rs","intersperse.rs","k_smallest.rs","kmerge_impl.rs","lazy_buffer.rs","lib.rs","merge_join.rs","minmax.rs","multipeek_impl.rs","pad_tail.rs","peek_nth.rs","peeking_take_while.rs","permutations.rs","powerset.rs","process_results_impl.rs","put_back_n_impl.rs","rciter_impl.rs","repeatn.rs","size_hint.rs","sources.rs","tee.rs","tuple_impl.rs","unique_impl.rs","unziptuple.rs","with_position.rs","zip_eq_impl.rs","zip_longest.rs","ziptuple.rs"]],\
"itoa":["",[],["lib.rs","udiv128.rs"]],\
"lazy_static":["",[],["inline_lazy.rs","lib.rs"]],\
"libc":["",[["unix",[["linux_like",[["linux",[["arch",[["generic",[],["mod.rs"]]],["mod.rs"]],["gnu",[["b64",[["x86_64",[],["align.rs","mod.rs","not_x32.rs"]]],["mod.rs"]]],["align.rs","mod.rs"]]],["align.rs","mod.rs","non_exhaustive.rs"]]],["mod.rs"]]],["align.rs","mod.rs"]]],["fixed_width_ints.rs","lib.rs","macros.rs"]],\
"linux_raw_sys":["",[["x86_64",[],["errno.rs","general.rs","ioctl.rs"]]],["lib.rs"]],\
"lld_rs":["",[],["lib.rs"]],\
"llvm_sys":["",[["orc2",[],["ee.rs","lljit.rs","mod.rs"]],["transforms",[],["aggressive_instcombine.rs","coroutines.rs","instcombine.rs","ipo.rs","pass_builder.rs","pass_manager_builder.rs","scalar.rs","util.rs","vectorize.rs"]]],["analysis.rs","bit_reader.rs","bit_writer.rs","comdat.rs","core.rs","debuginfo.rs","disassembler.rs","error.rs","error_handling.rs","execution_engine.rs","initialization.rs","ir_reader.rs","lib.rs","linker.rs","lto.rs","object.rs","remarks.rs","support.rs","target.rs","target_machine.rs"]],\
"lock_api":["",[],["lib.rs","mutex.rs","remutex.rs","rwlock.rs"]],\
"logos":["",[],["internal.rs","lexer.rs","lib.rs","source.rs"]],\
"logos_derive":["",[["generator",[],["context.rs","fork.rs","leaf.rs","mod.rs","rope.rs","tables.rs"]],["graph",[],["fork.rs","impls.rs","meta.rs","mod.rs","range.rs","regex.rs","rope.rs"]],["parser",[],["definition.rs","ignore_flags.rs","mod.rs","nested.rs","subpattern.rs","type_params.rs"]]],["error.rs","leaf.rs","lib.rs","mir.rs","util.rs"]],\
"memchr":["",[["memchr",[["x86",[],["avx.rs","mod.rs","sse2.rs"]]],["fallback.rs","iter.rs","mod.rs","naive.rs"]],["memmem",[["prefilter",[["x86",[],["avx.rs","mod.rs","sse.rs"]]],["fallback.rs","genericsimd.rs","mod.rs"]],["x86",[],["avx.rs","mod.rs","sse.rs"]]],["byte_frequencies.rs","genericsimd.rs","mod.rs","rabinkarp.rs","rarebytes.rs","twoway.rs","util.rs","vector.rs"]]],["cow.rs","lib.rs"]],\
"num_integer":["",[],["average.rs","lib.rs","roots.rs"]],\
"num_traits":["",[["ops",[],["checked.rs","euclid.rs","inv.rs","mod.rs","mul_add.rs","overflowing.rs","saturating.rs","wrapping.rs"]]],["bounds.rs","cast.rs","float.rs","identities.rs","int.rs","lib.rs","macros.rs","pow.rs","sign.rs"]],\
"once_cell":["",[],["imp_std.rs","lib.rs","race.rs"]],\
"os_str_bytes":["",[["common",[],["mod.rs","raw.rs"]]],["iter.rs","lib.rs","pattern.rs","raw_str.rs"]],\
"parking_lot":["",[],["condvar.rs","deadlock.rs","elision.rs","fair_mutex.rs","lib.rs","mutex.rs","once.rs","raw_fair_mutex.rs","raw_mutex.rs","raw_rwlock.rs","remutex.rs","rwlock.rs","util.rs"]],\
"parking_lot_core":["",[["thread_parker",[],["linux.rs","mod.rs"]]],["lib.rs","parking_lot.rs","spinwait.rs","util.rs","word_lock.rs"]],\
"proc_macro2":["",[],["detection.rs","fallback.rs","lib.rs","marker.rs","parse.rs","rcvec.rs","wrapper.rs"]],\
"proc_macro_error":["",[["imp",[],["fallback.rs"]]],["diagnostic.rs","dummy.rs","lib.rs","macros.rs","sealed.rs"]],\
"proc_macro_error_attr":["",[],["lib.rs","parse.rs","settings.rs"]],\
"quote":["",[],["ext.rs","format.rs","ident_fragment.rs","lib.rs","runtime.rs","spanned.rs","to_tokens.rs"]],\
"regex":["",[["literal",[],["imp.rs","mod.rs"]]],["backtrack.rs","compile.rs","dfa.rs","error.rs","exec.rs","expand.rs","find_byte.rs","input.rs","lib.rs","pikevm.rs","pool.rs","prog.rs","re_builder.rs","re_bytes.rs","re_set.rs","re_trait.rs","re_unicode.rs","sparse.rs","utf8.rs"]],\
"regex_syntax":["",[["ast",[],["mod.rs","parse.rs","print.rs","visitor.rs"]],["hir",[["literal",[],["mod.rs"]]],["interval.rs","mod.rs","print.rs","translate.rs","visitor.rs"]],["unicode_tables",[],["age.rs","case_folding_simple.rs","general_category.rs","grapheme_cluster_break.rs","mod.rs","perl_word.rs","property_bool.rs","property_names.rs","property_values.rs","script.rs","script_extension.rs","sentence_break.rs","word_break.rs"]]],["either.rs","error.rs","lib.rs","parser.rs","unicode.rs","utf8.rs"]],\
"rustix":["",[["backend",[["linux_raw",[["arch",[["inline",[],["mod.rs","x86_64.rs"]]],["mod.rs"]],["fs",[],["dir.rs","makedev.rs","mod.rs","syscalls.rs","types.rs"]],["io",[],["epoll.rs","errno.rs","mod.rs","poll_fd.rs","syscalls.rs","types.rs"]],["process",[],["cpu_set.rs","mod.rs","syscalls.rs","types.rs","wait.rs"]],["time",[],["mod.rs","types.rs"]]],["c.rs","conv.rs","elf.rs","mod.rs","reg.rs","weak.rs"]]]],["ffi",[],["mod.rs"]],["fs",[],["abs.rs","at.rs","constants.rs","copy_file_range.rs","cwd.rs","dir.rs","fadvise.rs","fcntl.rs","fd.rs","file_type.rs","makedev.rs","memfd_create.rs","mod.rs","mount.rs","openat2.rs","raw_dir.rs","sendfile.rs","statx.rs"]],["io",[],["close.rs","context.rs","dup.rs","errno.rs","eventfd.rs","fcntl.rs","ioctl.rs","is_read_write.rs","mod.rs","pipe.rs","poll.rs","read_write.rs","stdio.rs"]],["path",[],["arg.rs","mod.rs"]],["process",[],["chdir.rs","exit.rs","id.rs","kill.rs","membarrier.rs","mod.rs","prctl.rs","priority.rs","rlimit.rs","sched.rs","sched_yield.rs","uname.rs","wait.rs"]]],["const_assert.rs","cstr.rs","lib.rs","utils.rs"]],\
"rusty":["",[["ast",[],["pre_processor.rs"]],["codegen",[["generators",[],["data_type_generator.rs","date_time_util.rs","expression_generator.rs","llvm.rs","pou_generator.rs","statement_generator.rs","variable_generator.rs"]]],["debug.rs","generators.rs","llvm_index.rs","llvm_typesystem.rs"]],["index",[],["const_expressions.rs","instance_iterator.rs","symbol.rs","visitor.rs"]],["lexer",[],["tokens.rs"]],["parser",[],["control_parser.rs","expressions_parser.rs"]],["resolver",[],["const_evaluator.rs","generics.rs"]],["validation",[],["global.rs","pou.rs","recursive.rs","statement.rs","types.rs","variable.rs"]]],["ast.rs","build.rs","builtins.rs","cli.rs","codegen.rs","datalayout.rs","diagnostics.rs","expression_path.rs","hardware_binding.rs","index.rs","lexer.rs","lib.rs","linker.rs","parser.rs","resolver.rs","runner.rs","test_utils.rs","typesystem.rs","validation.rs"]],\
"rustyc":["",[],["main.rs"]],\
"ryu":["",[["buffer",[],["mod.rs"]],["pretty",[],["exponent.rs","mantissa.rs","mod.rs"]]],["common.rs","d2s.rs","d2s_full_table.rs","d2s_intrinsics.rs","digit_table.rs","f2s.rs","f2s_intrinsics.rs","lib.rs"]],\
"scopeguard":["",[],["lib.rs"]],\
"serde":["",[["de",[],["format.rs","ignored_any.rs","impls.rs","mod.rs","seed.rs","utf8.rs","value.rs"]],["private",[],["de.rs","doc.rs","mod.rs","ser.rs","size_hint.rs"]],["ser",[],["fmt.rs","impls.rs","impossible.rs","mod.rs"]]],["integer128.rs","lib.rs","macros.rs"]],\
"serde_derive":["",[["internals",[],["ast.rs","attr.rs","case.rs","check.rs","ctxt.rs","mod.rs","receiver.rs","respan.rs","symbol.rs"]]],["bound.rs","de.rs","dummy.rs","fragment.rs","lib.rs","pretend.rs","ser.rs","this.rs","try.rs"]],\
"serde_json":["",[["features_check",[],["mod.rs"]],["io",[],["mod.rs"]],["value",[],["de.rs","from.rs","index.rs","mod.rs","partial_eq.rs","ser.rs"]]],["de.rs","error.rs","iter.rs","lib.rs","macros.rs","map.rs","number.rs","read.rs","ser.rs"]],\
"shell_words":["",[],["lib.rs"]],\
"smallvec":["",[],["lib.rs"]],\
"strsim":["",[],["lib.rs"]],\
"syn":["",[["gen",[],["clone.rs","fold.rs","gen_helper.rs"]]],["attr.rs","await.rs","bigint.rs","buffer.rs","custom_keyword.rs","custom_punctuation.rs","data.rs","derive.rs","discouraged.rs","drops.rs","error.rs","export.rs","expr.rs","ext.rs","file.rs","generics.rs","group.rs","ident.rs","item.rs","lib.rs","lifetime.rs","lit.rs","lookahead.rs","mac.rs","macros.rs","op.rs","parse.rs","parse_macro_input.rs","parse_quote.rs","pat.rs","path.rs","print.rs","punctuated.rs","reserved.rs","sealed.rs","span.rs","spanned.rs","stmt.rs","thread.rs","token.rs","ty.rs","verbatim.rs","whitespace.rs"]],\
"tempfile":["",[["file",[["imp",[],["mod.rs","unix.rs"]]],["mod.rs"]]],["dir.rs","error.rs","lib.rs","spooled.rs","util.rs"]],\
"termcolor":["",[],["lib.rs"]],\
"textwrap":["",[],["core.rs","indentation.rs","lib.rs","line_ending.rs","word_separators.rs","word_splitters.rs","wrap_algorithms.rs"]],\
"thiserror":["",[],["aserror.rs","display.rs","lib.rs"]],\
"thiserror_impl":["",[],["ast.rs","attr.rs","expand.rs","fmt.rs","generics.rs","lib.rs","prop.rs","valid.rs"]],\
"toml":["",[],["datetime.rs","de.rs","lib.rs","macros.rs","map.rs","ser.rs","spanned.rs","tokens.rs","value.rs"]],\
"unicode_ident":["",[],["lib.rs","tables.rs"]],\
"unicode_width":["",[],["lib.rs","tables.rs"]],\
"which":["",[],["checker.rs","error.rs","finder.rs","lib.rs"]]\
}');
createSourceSidebar();