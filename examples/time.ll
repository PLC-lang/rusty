; ModuleID = '.\time.cpp'
source_filename = ".\\time.cpp"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc19.29.30133"

%"class.std::basic_ostream" = type { i32*, [4 x i8], i32, %"class.std::basic_ios" }
%"class.std::basic_ios" = type { %"class.std::ios_base", %"class.std::basic_streambuf"*, %"class.std::basic_ostream"*, i8 }
%"class.std::ios_base" = type { i32 (...)**, i64, i32, i32, i32, i64, i64, %"struct.std::ios_base::_Iosarray"*, %"struct.std::ios_base::_Fnarray"*, %"class.std::locale"* }
%"struct.std::ios_base::_Iosarray" = type { %"struct.std::ios_base::_Iosarray"*, i32, i32, i8* }
%"struct.std::ios_base::_Fnarray" = type { %"struct.std::ios_base::_Fnarray"*, i32, void (i32, %"class.std::ios_base"*, i32)* }
%"class.std::locale" = type { [8 x i8], %"class.std::locale::_Locimp"* }
%"class.std::locale::_Locimp" = type { %"class.std::locale::facet", %"class.std::locale::facet"**, i64, i32, i8, %"class.std::_Yarn" }
%"class.std::locale::facet" = type { %"class.std::_Facet_base", i32 }
%"class.std::_Facet_base" = type { i32 (...)** }
%"class.std::_Yarn" = type { i8*, i8 }
%"class.std::basic_streambuf" = type { i32 (...)**, i8*, i8*, i8**, i8**, i8*, i8*, i8**, i8**, i32, i32, i32*, i32*, %"class.std::locale"* }
%"class.std::locale::id" = type { i64 }
%rtti.TypeDescriptor26 = type { i8**, i8*, [27 x i8] }
%eh.CatchableType = type { i32, i32, i32, i32, i32, i32, i32 }
%rtti.TypeDescriptor22 = type { i8**, i8*, [23 x i8] }
%rtti.TypeDescriptor23 = type { i8**, i8*, [24 x i8] }
%rtti.TypeDescriptor19 = type { i8**, i8*, [20 x i8] }
%eh.CatchableTypeArray.5 = type { i32, [5 x i32] }
%eh.ThrowInfo = type { i32, i32, i32, i32 }
%rtti.CompleteObjectLocator = type { i32, i32, i32, i32, i32, i32 }
%rtti.TypeDescriptor35 = type { i8**, i8*, [36 x i8] }
%rtti.ClassHierarchyDescriptor = type { i32, i32, i32, i32 }
%rtti.BaseClassDescriptor = type { i32, i32, i32, i32, i32, i32, i32 }
%rtti.TypeDescriptor24 = type { i8**, i8*, [25 x i8] }
%"struct.std::_Fake_allocator" = type { i8 }
%rtti.TypeDescriptor30 = type { i8**, i8*, [31 x i8] }
%eh.CatchableTypeArray.3 = type { i32, [3 x i32] }
%rtti.TypeDescriptor73 = type { i8**, i8*, [74 x i8] }
%rtti.TypeDescriptor21 = type { i8**, i8*, [22 x i8] }
%rtti.TypeDescriptor25 = type { i8**, i8*, [26 x i8] }
%rtti.TypeDescriptor20 = type { i8**, i8*, [21 x i8] }
%rtti.TypeDescriptor18 = type { i8**, i8*, [19 x i8] }
%eh.CatchableTypeArray.2 = type { i32, [2 x i32] }
%"class.std::basic_ostream<char, std::char_traits<char>>::sentry" = type { %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base", i8 }
%"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base" = type { %"class.std::basic_ostream"* }
%"class.std::num_put" = type { %"class.std::locale::facet" }
%"class.std::ostreambuf_iterator" = type { i8, %"class.std::basic_streambuf"* }
%"class.std::ios_base::failure" = type { %"class.std::system_error" }
%"class.std::system_error" = type { %"class.std::_System_error" }
%"class.std::_System_error" = type { %"class.std::runtime_error", %"class.std::error_code" }
%"class.std::runtime_error" = type { %"class.std::exception" }
%"class.std::exception" = type { i32 (...)**, %struct.__std_exception_data }
%struct.__std_exception_data = type { i8*, i8 }
%"class.std::error_code" = type { i32, %"class.std::error_category"* }
%"class.std::error_category" = type { i32 (...)**, i64 }
%"class.std::_Iostream_error_category2" = type { %"class.std::error_category" }
%"class.std::basic_string" = type { %"class.std::_Compressed_pair" }
%"class.std::_Compressed_pair" = type { %"class.std::_String_val" }
%"class.std::_String_val" = type { %"union.std::_String_val<std::_Simple_types<char>>::_Bxty", i64, i64 }
%"union.std::_String_val<std::_Simple_types<char>>::_Bxty" = type { i8*, [8 x i8] }
%"class.std::error_condition" = type { i32, %"class.std::error_category"* }
%"struct.std::_Zero_then_variadic_args_t" = type { i8 }
%"struct.std::_Fake_proxy_ptr_impl" = type { i8 }
%"struct.std::_Container_base0" = type { i8 }
%"class.std::allocator" = type { i8 }
%class.anon = type { i8 }
%"class.std::bad_array_new_length" = type { %"class.std::bad_alloc" }
%"class.std::bad_alloc" = type { %"class.std::exception" }
%"struct.std::_One_then_variadic_args_t" = type { i8 }
%class.anon.0 = type { i8 }
%"class.std::_Lockit" = type { i32 }
%"class.std::unique_ptr" = type { %"class.std::_Compressed_pair.2" }
%"class.std::_Compressed_pair.2" = type { %"class.std::_Facet_base"* }
%"class.std::_Locinfo" = type { %"class.std::_Lockit", %"class.std::_Yarn", %"class.std::_Yarn", %"class.std::_Yarn.3", %"class.std::_Yarn.3", %"class.std::_Yarn", %"class.std::_Yarn" }
%"class.std::_Yarn.3" = type { i16*, i16 }
%"class.std::bad_cast" = type { %"class.std::exception" }
%"struct.std::default_delete" = type { i8 }
%"struct.std::_Crt_new_delete" = type { i8 }
%"class.std::numpunct" = type { %"class.std::locale::facet", i8*, i8, i8, i8*, i8* }
%"class.std::ctype" = type { %"struct.std::ctype_base", %struct._Ctypevec }
%"struct.std::ctype_base" = type { %"class.std::locale::facet" }
%struct._Ctypevec = type { i32, i16*, i32, i16* }
%struct.__crt_locale_pointers = type { %struct.__crt_locale_data*, %struct.__crt_multibyte_data* }
%struct.__crt_locale_data = type opaque
%struct.__crt_multibyte_data = type opaque
%class.anon.6 = type { i8 }
%class.anon.4 = type { i8 }
%struct.lconv = type { i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8*, i8, i8, i8, i8, i8, i8, i8, i8, i16*, i16*, i16*, i16*, i16*, i16*, i16*, i16* }
%struct._Cvtvec = type { i32, i32, i32, [32 x i8] }
%"struct.std::_Tidy_guard" = type { %"class.std::numpunct"* }
%class.anon.8 = type { i8 }
%"struct.std::_Equal_allocators" = type { i8 }
%"class.std::_Locbase" = type { i8 }

$"??$?6U?$char_traits@D@std@@@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@0@AEAV10@D@Z" = comdat any

$"??$?6U?$char_traits@D@std@@@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@0@AEAV10@PEBD@Z" = comdat any

$"??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@_K@Z" = comdat any

$"??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@J@Z" = comdat any

$"??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@_J@Z" = comdat any

$"??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@O@Z" = comdat any

$"??__E?id@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@2V0locale@2@A@@YAXXZ" = comdat any

$"??0id@locale@std@@QEAA@_K@Z" = comdat any

$"??__E?id@?$numpunct@D@std@@2V0locale@2@A@@YAXXZ" = comdat any

$"?length@?$_Narrow_char_traits@DH@std@@SA_KQEBD@Z" = comdat any

$"?width@ios_base@std@@QEBA_JXZ" = comdat any

$"??0sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@AEAV12@@Z" = comdat any

$"??Bsentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEBA_NXZ" = comdat any

$"?flags@ios_base@std@@QEBAHXZ" = comdat any

$"?eq_int_type@?$_Narrow_char_traits@DH@std@@SA_NAEBH0@Z" = comdat any

$"?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ" = comdat any

$"?sputc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHD@Z" = comdat any

$"?fill@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBADXZ" = comdat any

$"?eof@?$_Narrow_char_traits@DH@std@@SAHXZ" = comdat any

$"?sputn@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAA_JPEBD_J@Z" = comdat any

$"?width@ios_base@std@@QEAA_J_J@Z" = comdat any

$"?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z" = comdat any

$"??1sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ" = comdat any

$"??0_Sentry_base@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@AEAV12@@Z" = comdat any

$"?good@ios_base@std@@QEBA_NXZ" = comdat any

$"?tie@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_ostream@DU?$char_traits@D@std@@@2@XZ" = comdat any

$"?flush@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@XZ" = comdat any

$"??1_Sentry_base@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ" = comdat any

$"?rdstate@ios_base@std@@QEBAHXZ" = comdat any

$"?pubsync@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHXZ" = comdat any

$"?_Pnavail@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBA_JXZ" = comdat any

$"?to_int_type@?$_Narrow_char_traits@DH@std@@SAHAEBD@Z" = comdat any

$"?_Pninc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAPEADXZ" = comdat any

$"?clear@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z" = comdat any

$"?clear@ios_base@std@@QEAAXH_N@Z" = comdat any

$"?make_error_code@std@@YA?AVerror_code@1@W4io_errc@1@@Z" = comdat any

$"??0failure@ios_base@std@@QEAA@PEBDAEBVerror_code@2@@Z" = comdat any

$"??0failure@ios_base@std@@QEAA@AEBV012@@Z" = comdat any

$"??0system_error@std@@QEAA@AEBV01@@Z" = comdat any

$"??0_System_error@std@@QEAA@AEBV01@@Z" = comdat any

$"??0runtime_error@std@@QEAA@AEBV01@@Z" = comdat any

$"??0exception@std@@QEAA@AEBV01@@Z" = comdat any

$"??1failure@ios_base@std@@UEAA@XZ" = comdat any

$"?iostream_category@std@@YAAEBVerror_category@1@XZ" = comdat any

$"??0error_code@std@@QEAA@HAEBVerror_category@1@@Z" = comdat any

$"??$_Immortalize_memcpy_image@V_Iostream_error_category2@std@@@std@@YAAEBV_Iostream_error_category2@0@XZ" = comdat any

$"??1_Iostream_error_category2@std@@UEAA@XZ" = comdat any

$"??_G_Iostream_error_category2@std@@UEAAPEAXI@Z" = comdat any

$"?name@_Iostream_error_category2@std@@UEBAPEBDXZ" = comdat any

$"?message@_Iostream_error_category2@std@@UEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@H@Z" = comdat any

$"?default_error_condition@error_category@std@@UEBA?AVerror_condition@2@H@Z" = comdat any

$"?equivalent@error_category@std@@UEBA_NAEBVerror_code@2@H@Z" = comdat any

$"?equivalent@error_category@std@@UEBA_NHAEBVerror_condition@2@@Z" = comdat any

$"??1error_category@std@@UEAA@XZ" = comdat any

$"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@QEBD_K@Z" = comdat any

$"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@QEBD@Z" = comdat any

$"??$?0$$V@?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@U_Zero_then_variadic_args_t@1@@Z" = comdat any

$"??0_Fake_proxy_ptr_impl@std@@QEAA@AEBU_Fake_allocator@1@AEBU_Container_base0@1@@Z" = comdat any

$"?_Tidy_init@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXXZ" = comdat any

$"?assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@QEBD_K@Z" = comdat any

$"?_Release@_Fake_proxy_ptr_impl@std@@QEAAXXZ" = comdat any

$"??1?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@XZ" = comdat any

$"??0?$allocator@D@std@@QEAA@XZ" = comdat any

$"??0?$_String_val@U?$_Simple_types@D@std@@@std@@QEAA@XZ" = comdat any

$"??0_Bxty@?$_String_val@U?$_Simple_types@D@std@@@std@@QEAA@XZ" = comdat any

$"?assign@?$_Narrow_char_traits@DH@std@@SAXAEADAEBD@Z" = comdat any

$"?_Myptr@?$_String_val@U?$_Simple_types@D@std@@@std@@QEAAPEADXZ" = comdat any

$"?move@?$_Char_traits@DH@std@@SAPEADQEADQEBD_K@Z" = comdat any

$"??$_Reallocate_for@V<lambda_1>@?0??assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV34@QEBD_K@Z@PEBD@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV01@_KV<lambda_1>@?0??assign@01@QEAAAEAV01@QEBD0@Z@PEBD@Z" = comdat any

$"?_Large_string_engaged@?$_String_val@U?$_Simple_types@D@std@@@std@@QEBA_NXZ" = comdat any

$"??$_Unfancy@D@std@@YAPEADPEAD@Z" = comdat any

$"?max_size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ" = comdat any

$"?_Xlen_string@std@@YAXXZ" = comdat any

$"?_Calculate_growth@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEBA_K_K@Z" = comdat any

$"?_Getal@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV?$allocator@D@2@XZ" = comdat any

$"?allocate@?$allocator@D@std@@QEAAPEAD_K@Z" = comdat any

$"?_Orphan_all@_Container_base0@std@@QEAAXXZ" = comdat any

$"??R<lambda_1>@?0??assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@QEBD_K@Z@QEBA?A?<auto>@@QEAD10@Z" = comdat any

$"?deallocate@?$allocator@D@std@@QEAAXQEAD_K@Z" = comdat any

$"??$_Construct_in_place@PEADAEBQEAD@std@@YAXAEAPEADAEBQEAD@Z" = comdat any

$"?max_size@?$_Default_allocator_traits@V?$allocator@D@std@@@std@@SA_KAEBV?$allocator@D@2@@Z" = comdat any

$"?_Getal@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEBAAEBV?$allocator@D@2@XZ" = comdat any

$"??$max@_K@std@@YAAEB_KAEB_K0@Z" = comdat any

$"??$min@_K@std@@YAAEB_KAEB_K0@Z" = comdat any

$"?max@?$numeric_limits@_J@std@@SA_JXZ" = comdat any

$"?_Get_first@?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEBAAEBV?$allocator@D@2@XZ" = comdat any

$"?_Calculate_growth@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@CA_K_K00@Z" = comdat any

$"?_Get_first@?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAAAEAV?$allocator@D@2@XZ" = comdat any

$"??$_Allocate@$0BA@U_Default_allocate_traits@std@@$0A@@std@@YAPEAX_K@Z" = comdat any

$"??$_Get_size_of_n@$00@std@@YA_K_K@Z" = comdat any

$"??$_Allocate_manually_vector_aligned@U_Default_allocate_traits@std@@@std@@YAPEAX_K@Z" = comdat any

$"?_Allocate@_Default_allocate_traits@std@@SAPEAX_K@Z" = comdat any

$"?_Throw_bad_array_new_length@std@@YAXXZ" = comdat any

$"??0bad_array_new_length@std@@QEAA@XZ" = comdat any

$"??0bad_array_new_length@std@@QEAA@AEBV01@@Z" = comdat any

$"??0bad_alloc@std@@QEAA@AEBV01@@Z" = comdat any

$"??1bad_array_new_length@std@@UEAA@XZ" = comdat any

$"??0bad_alloc@std@@AEAA@QEBD@Z" = comdat any

$"??_Gbad_array_new_length@std@@UEAAPEAXI@Z" = comdat any

$"?what@exception@std@@UEBAPEBDXZ" = comdat any

$"??0exception@std@@QEAA@QEBDH@Z" = comdat any

$"??_Gbad_alloc@std@@UEAAPEAXI@Z" = comdat any

$"??_Gexception@std@@UEAAPEAXI@Z" = comdat any

$"??1exception@std@@UEAA@XZ" = comdat any

$"??1bad_alloc@std@@UEAA@XZ" = comdat any

$"?copy@?$_Char_traits@DH@std@@SAPEADQEADQEBD_K@Z" = comdat any

$"??$_Deallocate@$0BA@$0A@@std@@YAXPEAX_K@Z" = comdat any

$"?_Adjust_manually_vector_aligned@std@@YAXAEAPEAXAEA_K@Z" = comdat any

$"??$_Voidify_iter@PEAPEAD@std@@YAPEAXPEAPEAD@Z" = comdat any

$"??$addressof@PEAD@std@@YAPEAPEADAEAPEAD@Z" = comdat any

$"??$forward@AEBQEAD@std@@YAAEBQEADAEBQEAD@Z" = comdat any

$"??1?$_String_val@U?$_Simple_types@D@std@@@std@@QEAA@XZ" = comdat any

$"??1_Bxty@?$_String_val@U?$_Simple_types@D@std@@@std@@QEAA@XZ" = comdat any

$"?assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@QEBD@Z" = comdat any

$"??$_Convert_size@_K@std@@YA_K_K@Z" = comdat any

$"??0error_condition@std@@QEAA@HAEBVerror_category@1@@Z" = comdat any

$"??8error_category@std@@QEBA_NAEBV01@@Z" = comdat any

$"?category@error_code@std@@QEBAAEBVerror_category@2@XZ" = comdat any

$"?value@error_code@std@@QEBAHXZ" = comdat any

$"??8std@@YA_NAEBVerror_condition@0@0@Z" = comdat any

$"?category@error_condition@std@@QEBAAEBVerror_category@2@XZ" = comdat any

$"?value@error_condition@std@@QEBAHXZ" = comdat any

$"??0system_error@std@@QEAA@Verror_code@1@PEBD@Z" = comdat any

$"??_Gfailure@ios_base@std@@UEAAPEAXI@Z" = comdat any

$"??0_System_error@std@@IEAA@Verror_code@1@AEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@1@@Z" = comdat any

$"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ" = comdat any

$"??_Gsystem_error@std@@UEAAPEAXI@Z" = comdat any

$"?_Makestr@_System_error@std@@CA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@Verror_code@2@V32@@Z" = comdat any

$"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@AEBV01@@Z" = comdat any

$"??0runtime_error@std@@QEAA@AEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@1@@Z" = comdat any

$"??_G_System_error@std@@UEAAPEAXI@Z" = comdat any

$"?empty@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_NXZ" = comdat any

$"?append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@QEBD@Z" = comdat any

$"?append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@AEBV12@@Z" = comdat any

$"?message@error_code@std@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ" = comdat any

$"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@$$QEAV01@@Z" = comdat any

$"?size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ" = comdat any

$"?append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@QEBD_K@Z" = comdat any

$"??$_Reallocate_grow_by@V<lambda_1>@?0??append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV34@QEBD_K@Z@PEBD_K@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV01@_KV<lambda_1>@?0??append@01@QEAAAEAV01@QEBD0@Z@PEBD_K@Z" = comdat any

$"??R<lambda_1>@?0??append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@QEBD_K@Z@QEBA?A?<auto>@@QEAD0101@Z" = comdat any

$"?_Myptr@?$_String_val@U?$_Simple_types@D@std@@@std@@QEBAPEBDXZ" = comdat any

$"??$move@AEAV?$allocator@D@std@@@std@@YA$$QEAV?$allocator@D@0@AEAV10@@Z" = comdat any

$"??$?0V?$allocator@D@std@@$$V@?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@U_One_then_variadic_args_t@1@$$QEAV?$allocator@D@1@@Z" = comdat any

$"?_Alloc_proxy@_Container_base0@std@@QEAAXAEBU_Fake_allocator@2@@Z" = comdat any

$"?_Take_contents@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXAEAV12@@Z" = comdat any

$"??$forward@V?$allocator@D@std@@@std@@YA$$QEAV?$allocator@D@0@AEAV10@@Z" = comdat any

$"?_Memcpy_val_from@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXAEBV12@@Z" = comdat any

$"??$addressof@V?$_String_val@U?$_Simple_types@D@std@@@std@@@std@@YAPEAV?$_String_val@U?$_Simple_types@D@std@@@0@AEAV10@@Z" = comdat any

$"??$addressof@$$CBV?$_String_val@U?$_Simple_types@D@std@@@std@@@std@@YAPEBV?$_String_val@U?$_Simple_types@D@std@@@0@AEBV10@@Z" = comdat any

$"?select_on_container_copy_construction@?$_Default_allocator_traits@V?$allocator@D@std@@@std@@SA?AV?$allocator@D@2@AEBV32@@Z" = comdat any

$"?_Construct_lv_contents@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXAEBV12@@Z" = comdat any

$"?c_str@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBAPEBDXZ" = comdat any

$"??0exception@std@@QEAA@QEBD@Z" = comdat any

$"??_Gruntime_error@std@@UEAAPEAXI@Z" = comdat any

$"??1runtime_error@std@@UEAA@XZ" = comdat any

$"??1_System_error@std@@UEAA@XZ" = comdat any

$"?_Tidy_deallocate@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXXZ" = comdat any

$"??$_Destroy_in_place@PEAD@std@@YAXAEAPEAD@Z" = comdat any

$"??1system_error@std@@UEAA@XZ" = comdat any

$"?_Osfx@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAXXZ" = comdat any

$"??$use_facet@V?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@@std@@YAAEBV?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@0@AEBVlocale@0@@Z" = comdat any

$"?getloc@ios_base@std@@QEBA?AVlocale@2@XZ" = comdat any

$"??1locale@std@@QEAA@XZ" = comdat any

$"?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@D_K@Z" = comdat any

$"??0?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@@Z" = comdat any

$"?failed@?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEBA_NXZ" = comdat any

$"??Bid@locale@std@@QEAA_KXZ" = comdat any

$"?_Getfacet@locale@std@@QEBAPEBVfacet@12@_K@Z" = comdat any

$"?_Getcat@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z" = comdat any

$"?_Throw_bad_cast@std@@YAXXZ" = comdat any

$"??$?0U?$default_delete@V_Facet_base@std@@@std@@$0A@@?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAA@PEAV_Facet_base@1@@Z" = comdat any

$"?release@?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAAPEAV_Facet_base@2@XZ" = comdat any

$"??1?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAA@XZ" = comdat any

$"?c_str@locale@std@@QEBAPEBDXZ" = comdat any

$"??0_Locinfo@std@@QEAA@PEBD@Z" = comdat any

$"??0?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z" = comdat any

$"??1_Locinfo@std@@QEAA@XZ" = comdat any

$"?c_str@?$_Yarn@D@std@@QEBAPEBDXZ" = comdat any

$"??0?$_Yarn@D@std@@QEAA@XZ" = comdat any

$"??0?$_Yarn@_W@std@@QEAA@XZ" = comdat any

$"??1?$_Yarn@D@std@@QEAA@XZ" = comdat any

$"??1?$_Yarn@_W@std@@QEAA@XZ" = comdat any

$"?_Tidy@?$_Yarn@D@std@@AEAAXXZ" = comdat any

$"?_Tidy@?$_Yarn@_W@std@@AEAAXXZ" = comdat any

$"??0facet@locale@std@@IEAA@_K@Z" = comdat any

$"?_Init@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@IEAAXAEBV_Locinfo@2@@Z" = comdat any

$"??1facet@locale@std@@MEAA@XZ" = comdat any

$"??_G?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEAAPEAXI@Z" = comdat any

$"?_Incref@facet@locale@std@@UEAAXXZ" = comdat any

$"?_Decref@facet@locale@std@@UEAAPEAV_Facet_base@3@XZ" = comdat any

$"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEBX@Z" = comdat any

$"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DO@Z" = comdat any

$"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DN@Z" = comdat any

$"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@D_K@Z" = comdat any

$"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@D_J@Z" = comdat any

$"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DK@Z" = comdat any

$"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DJ@Z" = comdat any

$"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@D_N@Z" = comdat any

$"??0_Facet_base@std@@QEAA@XZ" = comdat any

$"??_Gfacet@locale@std@@MEAAPEAXI@Z" = comdat any

$"??_G_Facet_base@std@@UEAAPEAXI@Z" = comdat any

$"??1_Facet_base@std@@UEAA@XZ" = comdat any

$"??1?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEAA@XZ" = comdat any

$"?_Iput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEAD_K@Z" = comdat any

$sprintf_s = comdat any

$"??$use_facet@V?$ctype@D@std@@@std@@YAAEBV?$ctype@D@0@AEBVlocale@0@@Z" = comdat any

$"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@_KD@Z" = comdat any

$"?widen@?$ctype@D@std@@QEBAPEBDPEBD0PEAD@Z" = comdat any

$"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAD_K@Z" = comdat any

$"??$use_facet@V?$numpunct@D@std@@@std@@YAAEBV?$numpunct@D@0@AEBVlocale@0@@Z" = comdat any

$"?grouping@?$numpunct@D@std@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ" = comdat any

$"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBAAEBD_K@Z" = comdat any

$"?thousands_sep@?$numpunct@D@std@@QEBADXZ" = comdat any

$"?insert@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@_K0D@Z" = comdat any

$"?_Rep@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@D_K@Z" = comdat any

$"?_Put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@PEBD_K@Z" = comdat any

$"?_Getcat@?$ctype@D@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z" = comdat any

$"??0?$ctype@D@std@@QEAA@AEBV_Locinfo@1@_K@Z" = comdat any

$"??0ctype_base@std@@QEAA@_K@Z" = comdat any

$"?_Init@?$ctype@D@std@@IEAAXAEBV_Locinfo@2@@Z" = comdat any

$"??1ctype_base@std@@UEAA@XZ" = comdat any

$"??_G?$ctype@D@std@@MEAAPEAXI@Z" = comdat any

$"?do_tolower@?$ctype@D@std@@MEBAPEBDPEADPEBD@Z" = comdat any

$"?do_tolower@?$ctype@D@std@@MEBADD@Z" = comdat any

$"?do_toupper@?$ctype@D@std@@MEBAPEBDPEADPEBD@Z" = comdat any

$"?do_toupper@?$ctype@D@std@@MEBADD@Z" = comdat any

$"?do_widen@?$ctype@D@std@@MEBAPEBDPEBD0PEAD@Z" = comdat any

$"?do_widen@?$ctype@D@std@@MEBADD@Z" = comdat any

$"?do_narrow@?$ctype@D@std@@MEBAPEBDPEBD0DPEAD@Z" = comdat any

$"?do_narrow@?$ctype@D@std@@MEBADDD@Z" = comdat any

$"??_Gctype_base@std@@UEAAPEAXI@Z" = comdat any

$"?_Getctype@_Locinfo@std@@QEBA?AU_Ctypevec@@XZ" = comdat any

$"??1?$ctype@D@std@@MEAA@XZ" = comdat any

$"?_Tidy@?$ctype@D@std@@IEAAXXZ" = comdat any

$"??$_Adl_verify_range@PEADPEBD@std@@YAXAEBQEADAEBQEBD@Z" = comdat any

$"??$_Adl_verify_range@PEBDPEBD@std@@YAXAEBQEBD0@Z" = comdat any

$"?assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@_KD@Z" = comdat any

$"?assign@?$_Narrow_char_traits@DH@std@@SAPEADQEAD_KD@Z" = comdat any

$"??$_Reallocate_for@V<lambda_1>@?0??assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV34@_KD@Z@D@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV01@_KV<lambda_1>@?0??assign@01@QEAAAEAV01@0D@Z@D@Z" = comdat any

$"??R<lambda_1>@?0??assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@_KD@Z@QEBA?A?<auto>@@QEAD0D@Z" = comdat any

$"?_Getcat@?$numpunct@D@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z" = comdat any

$"??0?$numpunct@D@std@@QEAA@AEBV_Locinfo@1@_K_N@Z" = comdat any

$"?_Init@?$numpunct@D@std@@IEAAXAEBV_Locinfo@2@_N@Z" = comdat any

$"??_G?$numpunct@D@std@@MEAAPEAXI@Z" = comdat any

$"?do_decimal_point@?$numpunct@D@std@@MEBADXZ" = comdat any

$"?do_thousands_sep@?$numpunct@D@std@@MEBADXZ" = comdat any

$"?do_grouping@?$numpunct@D@std@@MEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ" = comdat any

$"?do_falsename@?$numpunct@D@std@@MEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ" = comdat any

$"?do_truename@?$numpunct@D@std@@MEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ" = comdat any

$"?_Getlconv@_Locinfo@std@@QEBAPEBUlconv@@XZ" = comdat any

$"?_Getcvt@_Locinfo@std@@QEBA?AU_Cvtvec@@XZ" = comdat any

$"??$_Maklocstr@D@std@@YAPEADPEBDPEADAEBU_Cvtvec@@@Z" = comdat any

$"?_Getfalse@_Locinfo@std@@QEBAPEBDXZ" = comdat any

$"?_Gettrue@_Locinfo@std@@QEBAPEBDXZ" = comdat any

$"??$_Maklocchr@D@std@@YADDPEADAEBU_Cvtvec@@@Z" = comdat any

$"??$_Getvals@D@?$numpunct@D@std@@IEAAXDPEBUlconv@@U_Cvtvec@@@Z" = comdat any

$"??1?$_Tidy_guard@V?$numpunct@D@std@@@std@@QEAA@XZ" = comdat any

$"?_Tidy@?$numpunct@D@std@@AEAAXXZ" = comdat any

$"??1?$numpunct@D@std@@MEAA@XZ" = comdat any

$"?_Check_offset@?$_String_val@U?$_Simple_types@D@std@@@std@@QEBAX_K@Z" = comdat any

$"??$_Reallocate_grow_by@V<lambda_1>@?0??insert@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV34@_K0D@Z@_K_KD@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV01@_KV<lambda_1>@?0??insert@01@QEAAAEAV01@00D@Z@_K2D@Z" = comdat any

$"?_Xran@?$_String_val@U?$_Simple_types@D@std@@@std@@SAXXZ" = comdat any

$"??R<lambda_1>@?0??insert@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@_K0D@Z@QEBA?A?<auto>@@QEADQEBD000D@Z" = comdat any

$"??D?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAAAEAV01@XZ" = comdat any

$"??4?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAAAEAV01@D@Z" = comdat any

$"??E?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAAAEAV01@XZ" = comdat any

$_vsprintf_s_l = comdat any

$__local_stdio_printf_options = comdat any

$"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ" = comdat any

$"?precision@ios_base@std@@QEBA_JXZ" = comdat any

$"??$_Float_put_desired_precision@O@std@@YAH_JH@Z" = comdat any

$fabsl = comdat any

$frexpl = comdat any

$"?resize@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAX_KD@Z" = comdat any

$"?_Ffmt@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAPEADPEADDH@Z" = comdat any

$"?_Fput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEBD_K@Z" = comdat any

$"?_Eos@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAX_K@Z" = comdat any

$"?append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@_KD@Z" = comdat any

$"??$_Reallocate_grow_by@V<lambda_1>@?0??append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV34@_KD@Z@_KD@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV01@_KV<lambda_1>@?0??append@01@QEAAAEAV01@0D@Z@_KD@Z" = comdat any

$"??R<lambda_1>@?0??append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@_KD@Z@QEBA?A?<auto>@@QEADQEBD00D@Z" = comdat any

$"?decimal_point@?$numpunct@D@std@@QEBADXZ" = comdat any

$"??$_Float_put_desired_precision@N@std@@YAH_JH@Z" = comdat any

$"?_Ifmt@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAPEADPEADPEBDH@Z" = comdat any

$"?assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@$$QEAV12@@Z" = comdat any

$"?truename@?$numpunct@D@std@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ" = comdat any

$"?falsename@?$numpunct@D@std@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ" = comdat any

$"??$move@AEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@YA$$QEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@0@AEAV10@@Z" = comdat any

$"??4?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV01@$$QEAV01@@Z" = comdat any

$"??$addressof@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@YAPEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@0@AEAV10@@Z" = comdat any

$"?_Move_assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXAEAV12@U_Equal_allocators@2@@Z" = comdat any

$"??$_Pocma@V?$allocator@D@std@@@std@@YAXAEAV?$allocator@D@0@0@Z" = comdat any

$"??0bad_cast@std@@QEAA@XZ" = comdat any

$"??0bad_cast@std@@QEAA@AEBV01@@Z" = comdat any

$"??1bad_cast@std@@UEAA@XZ" = comdat any

$"??_Gbad_cast@std@@UEAAPEAXI@Z" = comdat any

$"??$?0AEAPEAV_Facet_base@std@@@?$_Compressed_pair@U?$default_delete@V_Facet_base@std@@@std@@PEAV_Facet_base@2@$00@std@@QEAA@U_Zero_then_variadic_args_t@1@AEAPEAV_Facet_base@1@@Z" = comdat any

$"??$forward@AEAPEAV_Facet_base@std@@@std@@YAAEAPEAV_Facet_base@0@AEAPEAV10@@Z" = comdat any

$"??$exchange@PEAV_Facet_base@std@@$$T@std@@YAPEAV_Facet_base@0@AEAPEAV10@$$QEA$$T@Z" = comdat any

$"?_Get_first@?$_Compressed_pair@U?$default_delete@V_Facet_base@std@@@std@@PEAV_Facet_base@2@$00@std@@QEAAAEAU?$default_delete@V_Facet_base@std@@@2@XZ" = comdat any

$"??R?$default_delete@V_Facet_base@std@@@std@@QEBAXPEAV_Facet_base@1@@Z" = comdat any

$"??0locale@std@@QEAA@AEBV01@@Z" = comdat any

$"?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DJ@Z" = comdat any

$"?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@D_J@Z" = comdat any

$"?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DO@Z" = comdat any

$"??_C@_06FMLHDGIC@Size?3?5?$AA@" = comdat any

$"??_C@_0M@NKLJEELK@Max?5Long?5?3?5?$AA@" = comdat any

$"??_C@_0BB@CCIDIEPP@Max?5Long?5Long?5?3?5?$AA@" = comdat any

$"??_C@_06GGONACPB@Result?$AA@" = comdat any

$"?id@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@2V0locale@2@A" = comdat any

$"?id@?$numpunct@D@std@@2V0locale@2@A" = comdat any

$"??_C@_0BF@PHHKMMFD@ios_base?3?3badbit?5set?$AA@" = comdat any

$"??_C@_0BG@FMKFHCIL@ios_base?3?3failbit?5set?$AA@" = comdat any

$"??_C@_0BF@OOHOMBOF@ios_base?3?3eofbit?5set?$AA@" = comdat any

$"??_R0?AVfailure@ios_base@std@@@8" = comdat any

$"_CT??_R0?AVfailure@ios_base@std@@@8??0failure@ios_base@std@@QEAA@AEBV012@@Z40" = comdat any

$"??_R0?AVsystem_error@std@@@8" = comdat any

$"_CT??_R0?AVsystem_error@std@@@8??0system_error@std@@QEAA@AEBV01@@Z40" = comdat any

$"??_R0?AV_System_error@std@@@8" = comdat any

$"_CT??_R0?AV_System_error@std@@@8??0_System_error@std@@QEAA@AEBV01@@Z40" = comdat any

$"??_R0?AVruntime_error@std@@@8" = comdat any

$"_CT??_R0?AVruntime_error@std@@@8??0runtime_error@std@@QEAA@AEBV01@@Z24" = comdat any

$"??_R0?AVexception@std@@@8" = comdat any

$"_CT??_R0?AVexception@std@@@8??0exception@std@@QEAA@AEBV01@@Z24" = comdat any

$"_CTA5?AVfailure@ios_base@std@@" = comdat any

$"_TI5?AVfailure@ios_base@std@@" = comdat any

$"?_Static@?1???$_Immortalize_memcpy_image@V_Iostream_error_category2@std@@@std@@YAAEBV_Iostream_error_category2@1@XZ@4V21@A" = comdat any

$"??_7_Iostream_error_category2@std@@6B@" = comdat largest

$"?$TSS0@?1???$_Immortalize_memcpy_image@V_Iostream_error_category2@std@@@std@@YAAEBV_Iostream_error_category2@0@XZ@4HA" = comdat any

$"??_R4_Iostream_error_category2@std@@6B@" = comdat any

$"??_R0?AV_Iostream_error_category2@std@@@8" = comdat any

$"??_R3_Iostream_error_category2@std@@8" = comdat any

$"??_R2_Iostream_error_category2@std@@8" = comdat any

$"??_R1A@?0A@EA@_Iostream_error_category2@std@@8" = comdat any

$"??_R1A@?0A@EA@error_category@std@@8" = comdat any

$"??_R0?AVerror_category@std@@@8" = comdat any

$"??_R3error_category@std@@8" = comdat any

$"??_R2error_category@std@@8" = comdat any

$"??_C@_08LLGCOLLL@iostream?$AA@" = comdat any

$"?_Iostream_error@?4??message@_Iostream_error_category2@std@@UEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@3@H@Z@4QBDB" = comdat any

$"??_C@_0BA@JFNIOLAK@string?5too?5long?$AA@" = comdat any

$"??_R0?AVbad_array_new_length@std@@@8" = comdat any

$"_CT??_R0?AVbad_array_new_length@std@@@8??0bad_array_new_length@std@@QEAA@AEBV01@@Z24" = comdat any

$"??_R0?AVbad_alloc@std@@@8" = comdat any

$"_CT??_R0?AVbad_alloc@std@@@8??0bad_alloc@std@@QEAA@AEBV01@@Z24" = comdat any

$"_CTA3?AVbad_array_new_length@std@@" = comdat any

$"_TI3?AVbad_array_new_length@std@@" = comdat any

$"??_C@_0BF@KINCDENJ@bad?5array?5new?5length?$AA@" = comdat any

$"??_7bad_array_new_length@std@@6B@" = comdat largest

$"??_R4bad_array_new_length@std@@6B@" = comdat any

$"??_R3bad_array_new_length@std@@8" = comdat any

$"??_R2bad_array_new_length@std@@8" = comdat any

$"??_R1A@?0A@EA@bad_array_new_length@std@@8" = comdat any

$"??_R1A@?0A@EA@bad_alloc@std@@8" = comdat any

$"??_R3bad_alloc@std@@8" = comdat any

$"??_R2bad_alloc@std@@8" = comdat any

$"??_R1A@?0A@EA@exception@std@@8" = comdat any

$"??_R3exception@std@@8" = comdat any

$"??_R2exception@std@@8" = comdat any

$"??_7bad_alloc@std@@6B@" = comdat largest

$"??_R4bad_alloc@std@@6B@" = comdat any

$"??_7exception@std@@6B@" = comdat largest

$"??_R4exception@std@@6B@" = comdat any

$"??_C@_0BC@EOODALEL@Unknown?5exception?$AA@" = comdat any

$"??_7failure@ios_base@std@@6B@" = comdat largest

$"??_R4failure@ios_base@std@@6B@" = comdat any

$"??_R3failure@ios_base@std@@8" = comdat any

$"??_R2failure@ios_base@std@@8" = comdat any

$"??_R1A@?0A@EA@failure@ios_base@std@@8" = comdat any

$"??_R1A@?0A@EA@system_error@std@@8" = comdat any

$"??_R3system_error@std@@8" = comdat any

$"??_R2system_error@std@@8" = comdat any

$"??_R1A@?0A@EA@_System_error@std@@8" = comdat any

$"??_R3_System_error@std@@8" = comdat any

$"??_R2_System_error@std@@8" = comdat any

$"??_R1A@?0A@EA@runtime_error@std@@8" = comdat any

$"??_R3runtime_error@std@@8" = comdat any

$"??_R2runtime_error@std@@8" = comdat any

$"??_7system_error@std@@6B@" = comdat largest

$"??_R4system_error@std@@6B@" = comdat any

$"??_7_System_error@std@@6B@" = comdat largest

$"??_R4_System_error@std@@6B@" = comdat any

$"??_C@_02LMMGGCAJ@?3?5?$AA@" = comdat any

$"??_7runtime_error@std@@6B@" = comdat largest

$"??_R4runtime_error@std@@6B@" = comdat any

$"?_Psave@?$_Facetptr@V?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@@std@@2PEBVfacet@locale@2@EB" = comdat any

$"??_C@_00CNPNBAHC@?$AA@" = comdat any

$"??_C@_0BA@ELKIONDK@bad?5locale?5name?$AA@" = comdat any

$"??_7?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@6B@" = comdat largest

$"??_R4?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@6B@" = comdat any

$"??_R0?AV?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@@8" = comdat any

$"??_R3?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@8" = comdat any

$"??_R2?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@8" = comdat any

$"??_R1A@?0A@EA@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@8" = comdat any

$"??_R1A@?0A@EA@facet@locale@std@@8" = comdat any

$"??_R0?AVfacet@locale@std@@@8" = comdat any

$"??_R3facet@locale@std@@8" = comdat any

$"??_R2facet@locale@std@@8" = comdat any

$"??_R1A@?0A@EA@_Facet_base@std@@8" = comdat any

$"??_R0?AV_Facet_base@std@@@8" = comdat any

$"??_R3_Facet_base@std@@8" = comdat any

$"??_R2_Facet_base@std@@8" = comdat any

$"??_R17?0A@EA@_Crt_new_delete@std@@8" = comdat any

$"??_R0?AU_Crt_new_delete@std@@@8" = comdat any

$"??_R3_Crt_new_delete@std@@8" = comdat any

$"??_R2_Crt_new_delete@std@@8" = comdat any

$"??_R1A@?0A@EA@_Crt_new_delete@std@@8" = comdat any

$"??_7facet@locale@std@@6B@" = comdat largest

$"??_R4facet@locale@std@@6B@" = comdat any

$"??_7_Facet_base@std@@6B@" = comdat largest

$"??_R4_Facet_base@std@@6B@" = comdat any

$"??_C@_02BBAHNLBA@?$CFp?$AA@" = comdat any

$"?_Psave@?$_Facetptr@V?$ctype@D@std@@@std@@2PEBVfacet@locale@2@EB" = comdat any

$"??_7?$ctype@D@std@@6B@" = comdat largest

$"??_R4?$ctype@D@std@@6B@" = comdat any

$"??_R0?AV?$ctype@D@std@@@8" = comdat any

$"??_R3?$ctype@D@std@@8" = comdat any

$"??_R2?$ctype@D@std@@8" = comdat any

$"??_R1A@?0A@EA@?$ctype@D@std@@8" = comdat any

$"??_R1A@?0A@EA@ctype_base@std@@8" = comdat any

$"??_R0?AUctype_base@std@@@8" = comdat any

$"??_R3ctype_base@std@@8" = comdat any

$"??_R2ctype_base@std@@8" = comdat any

$"??_7ctype_base@std@@6B@" = comdat largest

$"??_R4ctype_base@std@@6B@" = comdat any

$"?_Psave@?$_Facetptr@V?$numpunct@D@std@@@std@@2PEBVfacet@locale@2@EB" = comdat any

$"??_7?$numpunct@D@std@@6B@" = comdat largest

$"??_R4?$numpunct@D@std@@6B@" = comdat any

$"??_R0?AV?$numpunct@D@std@@@8" = comdat any

$"??_R3?$numpunct@D@std@@8" = comdat any

$"??_R2?$numpunct@D@std@@8" = comdat any

$"??_R1A@?0A@EA@?$numpunct@D@std@@8" = comdat any

$"??_C@_05LAPONLG@false?$AA@" = comdat any

$"??_C@_04LOAJBDKD@true?$AA@" = comdat any

$"??_C@_0BI@CFPLBAOH@invalid?5string?5position?$AA@" = comdat any

$"?_OptionsStorage@?1??__local_stdio_printf_options@@9@4_KA" = comdat any

$"??_C@_02MDKMJEGG@eE?$AA@" = comdat any

$"??_C@_02OOPEBDOJ@pP?$AA@" = comdat any

$"??_C@_02CLHGNPPK@Lu?$AA@" = comdat any

$"??_C@_02HIKPPMOK@Ld?$AA@" = comdat any

$"??_C@_02BDDLJJBK@lu?$AA@" = comdat any

$"??_C@_02EAOCLKAK@ld?$AA@" = comdat any

$"??_R0?AVbad_cast@std@@@8" = comdat any

$"_CT??_R0?AVbad_cast@std@@@8??0bad_cast@std@@QEAA@AEBV01@@Z24" = comdat any

$"_CTA2?AVbad_cast@std@@" = comdat any

$"_TI2?AVbad_cast@std@@" = comdat any

$"??_C@_08EPJLHIJG@bad?5cast?$AA@" = comdat any

$"??_7bad_cast@std@@6B@" = comdat largest

$"??_R4bad_cast@std@@6B@" = comdat any

$"??_R3bad_cast@std@@8" = comdat any

$"??_R2bad_cast@std@@8" = comdat any

$"??_R1A@?0A@EA@bad_cast@std@@8" = comdat any

@"?cout@std@@3V?$basic_ostream@DU?$char_traits@D@std@@@1@A" = external dso_local global %"class.std::basic_ostream", align 8
@"??_C@_06FMLHDGIC@Size?3?5?$AA@" = linkonce_odr dso_local unnamed_addr constant [7 x i8] c"Size: \00", comdat, align 1
@"??_C@_0M@NKLJEELK@Max?5Long?5?3?5?$AA@" = linkonce_odr dso_local unnamed_addr constant [12 x i8] c"Max Long : \00", comdat, align 1
@"??_C@_0BB@CCIDIEPP@Max?5Long?5Long?5?3?5?$AA@" = linkonce_odr dso_local unnamed_addr constant [17 x i8] c"Max Long Long : \00", comdat, align 1
@"??_C@_06GGONACPB@Result?$AA@" = linkonce_odr dso_local unnamed_addr constant [7 x i8] c"Result\00", comdat, align 1
@"?id@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@2V0locale@2@A" = linkonce_odr dso_local global %"class.std::locale::id" zeroinitializer, comdat, align 8
@"?id@?$numpunct@D@std@@2V0locale@2@A" = linkonce_odr dso_local global %"class.std::locale::id" zeroinitializer, comdat, align 8
@"??_C@_0BF@PHHKMMFD@ios_base?3?3badbit?5set?$AA@" = linkonce_odr dso_local unnamed_addr constant [21 x i8] c"ios_base::badbit set\00", comdat, align 1
@"??_C@_0BG@FMKFHCIL@ios_base?3?3failbit?5set?$AA@" = linkonce_odr dso_local unnamed_addr constant [22 x i8] c"ios_base::failbit set\00", comdat, align 1
@"??_C@_0BF@OOHOMBOF@ios_base?3?3eofbit?5set?$AA@" = linkonce_odr dso_local unnamed_addr constant [21 x i8] c"ios_base::eofbit set\00", comdat, align 1
@"??_7type_info@@6B@" = external constant i8*
@"??_R0?AVfailure@ios_base@std@@@8" = linkonce_odr global %rtti.TypeDescriptor26 { i8** @"??_7type_info@@6B@", i8* null, [27 x i8] c".?AVfailure@ios_base@std@@\00" }, comdat
@__ImageBase = external dso_local constant i8
@"_CT??_R0?AVfailure@ios_base@std@@@8??0failure@ios_base@std@@QEAA@AEBV012@@Z40" = linkonce_odr unnamed_addr constant %eh.CatchableType { i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor26* @"??_R0?AVfailure@ios_base@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 -1, i32 0, i32 40, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%"class.std::ios_base::failure"* (%"class.std::ios_base::failure"*, %"class.std::ios_base::failure"*)* @"??0failure@ios_base@std@@QEAA@AEBV012@@Z" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, section ".xdata", comdat
@"??_R0?AVsystem_error@std@@@8" = linkonce_odr global %rtti.TypeDescriptor22 { i8** @"??_7type_info@@6B@", i8* null, [23 x i8] c".?AVsystem_error@std@@\00" }, comdat
@"_CT??_R0?AVsystem_error@std@@@8??0system_error@std@@QEAA@AEBV01@@Z40" = linkonce_odr unnamed_addr constant %eh.CatchableType { i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor22* @"??_R0?AVsystem_error@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 -1, i32 0, i32 40, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%"class.std::system_error"* (%"class.std::system_error"*, %"class.std::system_error"*)* @"??0system_error@std@@QEAA@AEBV01@@Z" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, section ".xdata", comdat
@"??_R0?AV_System_error@std@@@8" = linkonce_odr global %rtti.TypeDescriptor23 { i8** @"??_7type_info@@6B@", i8* null, [24 x i8] c".?AV_System_error@std@@\00" }, comdat
@"_CT??_R0?AV_System_error@std@@@8??0_System_error@std@@QEAA@AEBV01@@Z40" = linkonce_odr unnamed_addr constant %eh.CatchableType { i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor23* @"??_R0?AV_System_error@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 -1, i32 0, i32 40, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%"class.std::_System_error"* (%"class.std::_System_error"*, %"class.std::_System_error"*)* @"??0_System_error@std@@QEAA@AEBV01@@Z" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, section ".xdata", comdat
@"??_R0?AVruntime_error@std@@@8" = linkonce_odr global %rtti.TypeDescriptor23 { i8** @"??_7type_info@@6B@", i8* null, [24 x i8] c".?AVruntime_error@std@@\00" }, comdat
@"_CT??_R0?AVruntime_error@std@@@8??0runtime_error@std@@QEAA@AEBV01@@Z24" = linkonce_odr unnamed_addr constant %eh.CatchableType { i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor23* @"??_R0?AVruntime_error@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 -1, i32 0, i32 24, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%"class.std::runtime_error"* (%"class.std::runtime_error"*, %"class.std::runtime_error"*)* @"??0runtime_error@std@@QEAA@AEBV01@@Z" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, section ".xdata", comdat
@"??_R0?AVexception@std@@@8" = linkonce_odr global %rtti.TypeDescriptor19 { i8** @"??_7type_info@@6B@", i8* null, [20 x i8] c".?AVexception@std@@\00" }, comdat
@"_CT??_R0?AVexception@std@@@8??0exception@std@@QEAA@AEBV01@@Z24" = linkonce_odr unnamed_addr constant %eh.CatchableType { i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor19* @"??_R0?AVexception@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 -1, i32 0, i32 24, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%"class.std::exception"* (%"class.std::exception"*, %"class.std::exception"*)* @"??0exception@std@@QEAA@AEBV01@@Z" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, section ".xdata", comdat
@"_CTA5?AVfailure@ios_base@std@@" = linkonce_odr unnamed_addr constant %eh.CatchableTypeArray.5 { i32 5, [5 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%eh.CatchableType* @"_CT??_R0?AVfailure@ios_base@std@@@8??0failure@ios_base@std@@QEAA@AEBV012@@Z40" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%eh.CatchableType* @"_CT??_R0?AVsystem_error@std@@@8??0system_error@std@@QEAA@AEBV01@@Z40" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%eh.CatchableType* @"_CT??_R0?AV_System_error@std@@@8??0_System_error@std@@QEAA@AEBV01@@Z40" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%eh.CatchableType* @"_CT??_R0?AVruntime_error@std@@@8??0runtime_error@std@@QEAA@AEBV01@@Z24" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%eh.CatchableType* @"_CT??_R0?AVexception@std@@@8??0exception@std@@QEAA@AEBV01@@Z24" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32)] }, section ".xdata", comdat
@"_TI5?AVfailure@ios_base@std@@" = linkonce_odr unnamed_addr constant %eh.ThrowInfo { i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (void (%"class.std::ios_base::failure"*)* @"??1failure@ios_base@std@@UEAA@XZ" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%eh.CatchableTypeArray.5* @"_CTA5?AVfailure@ios_base@std@@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, section ".xdata", comdat
@"?_Static@?1???$_Immortalize_memcpy_image@V_Iostream_error_category2@std@@@std@@YAAEBV_Iostream_error_category2@1@XZ@4V21@A" = linkonce_odr dso_local global { i8**, i64 } { i8** @"??_7_Iostream_error_category2@std@@6B@", i64 5 }, comdat, align 8
@0 = private unnamed_addr constant { [7 x i8*] } { [7 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4_Iostream_error_category2@std@@6B@" to i8*), i8* bitcast (i8* (%"class.std::_Iostream_error_category2"*, i32)* @"??_G_Iostream_error_category2@std@@UEAAPEAXI@Z" to i8*), i8* bitcast (i8* (%"class.std::_Iostream_error_category2"*)* @"?name@_Iostream_error_category2@std@@UEBAPEBDXZ" to i8*), i8* bitcast (void (%"class.std::_Iostream_error_category2"*, %"class.std::basic_string"*, i32)* @"?message@_Iostream_error_category2@std@@UEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@H@Z" to i8*), i8* bitcast (void (%"class.std::error_category"*, %"class.std::error_condition"*, i32)* @"?default_error_condition@error_category@std@@UEBA?AVerror_condition@2@H@Z" to i8*), i8* bitcast (i1 (%"class.std::error_category"*, %"class.std::error_code"*, i32)* @"?equivalent@error_category@std@@UEBA_NAEBVerror_code@2@H@Z" to i8*), i8* bitcast (i1 (%"class.std::error_category"*, i32, %"class.std::error_condition"*)* @"?equivalent@error_category@std@@UEBA_NHAEBVerror_condition@2@@Z" to i8*)] }, comdat($"??_7_Iostream_error_category2@std@@6B@")
@"?$TSS0@?1???$_Immortalize_memcpy_image@V_Iostream_error_category2@std@@@std@@YAAEBV_Iostream_error_category2@0@XZ@4HA" = linkonce_odr global i32 0, comdat, align 4
@_Init_thread_epoch = external thread_local global i32, align 4
@"??_R4_Iostream_error_category2@std@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor35* @"??_R0?AV_Iostream_error_category2@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3_Iostream_error_category2@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4_Iostream_error_category2@std@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R0?AV_Iostream_error_category2@std@@@8" = linkonce_odr global %rtti.TypeDescriptor35 { i8** @"??_7type_info@@6B@", i8* null, [36 x i8] c".?AV_Iostream_error_category2@std@@\00" }, comdat
@"??_R3_Iostream_error_category2@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 0, i32 2, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([3 x i32]* @"??_R2_Iostream_error_category2@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2_Iostream_error_category2@std@@8" = linkonce_odr constant [3 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@_Iostream_error_category2@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@error_category@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@"??_R1A@?0A@EA@_Iostream_error_category2@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor35* @"??_R0?AV_Iostream_error_category2@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 1, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3_Iostream_error_category2@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R1A@?0A@EA@error_category@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor24* @"??_R0?AVerror_category@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3error_category@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R0?AVerror_category@std@@@8" = linkonce_odr global %rtti.TypeDescriptor24 { i8** @"??_7type_info@@6B@", i8* null, [25 x i8] c".?AVerror_category@std@@\00" }, comdat
@"??_R3error_category@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 0, i32 1, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([2 x i32]* @"??_R2error_category@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2error_category@std@@8" = linkonce_odr constant [2 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@error_category@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@"??_C@_08LLGCOLLL@iostream?$AA@" = linkonce_odr dso_local unnamed_addr constant [9 x i8] c"iostream\00", comdat, align 1
@"?_Iostream_error@?4??message@_Iostream_error_category2@std@@UEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@3@H@Z@4QBDB" = linkonce_odr dso_local constant [22 x i8] c"iostream stream error\00", comdat, align 16
@"?_Fake_alloc@std@@3U_Fake_allocator@1@B" = internal constant %"struct.std::_Fake_allocator" undef, align 1
@"??_C@_0BA@JFNIOLAK@string?5too?5long?$AA@" = linkonce_odr dso_local unnamed_addr constant [16 x i8] c"string too long\00", comdat, align 1
@"??_R0?AVbad_array_new_length@std@@@8" = linkonce_odr global %rtti.TypeDescriptor30 { i8** @"??_7type_info@@6B@", i8* null, [31 x i8] c".?AVbad_array_new_length@std@@\00" }, comdat
@"_CT??_R0?AVbad_array_new_length@std@@@8??0bad_array_new_length@std@@QEAA@AEBV01@@Z24" = linkonce_odr unnamed_addr constant %eh.CatchableType { i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor30* @"??_R0?AVbad_array_new_length@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 -1, i32 0, i32 24, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%"class.std::bad_array_new_length"* (%"class.std::bad_array_new_length"*, %"class.std::bad_array_new_length"*)* @"??0bad_array_new_length@std@@QEAA@AEBV01@@Z" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, section ".xdata", comdat
@"??_R0?AVbad_alloc@std@@@8" = linkonce_odr global %rtti.TypeDescriptor19 { i8** @"??_7type_info@@6B@", i8* null, [20 x i8] c".?AVbad_alloc@std@@\00" }, comdat
@"_CT??_R0?AVbad_alloc@std@@@8??0bad_alloc@std@@QEAA@AEBV01@@Z24" = linkonce_odr unnamed_addr constant %eh.CatchableType { i32 16, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor19* @"??_R0?AVbad_alloc@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 -1, i32 0, i32 24, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%"class.std::bad_alloc"* (%"class.std::bad_alloc"*, %"class.std::bad_alloc"*)* @"??0bad_alloc@std@@QEAA@AEBV01@@Z" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, section ".xdata", comdat
@"_CTA3?AVbad_array_new_length@std@@" = linkonce_odr unnamed_addr constant %eh.CatchableTypeArray.3 { i32 3, [3 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%eh.CatchableType* @"_CT??_R0?AVbad_array_new_length@std@@@8??0bad_array_new_length@std@@QEAA@AEBV01@@Z24" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%eh.CatchableType* @"_CT??_R0?AVbad_alloc@std@@@8??0bad_alloc@std@@QEAA@AEBV01@@Z24" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%eh.CatchableType* @"_CT??_R0?AVexception@std@@@8??0exception@std@@QEAA@AEBV01@@Z24" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32)] }, section ".xdata", comdat
@"_TI3?AVbad_array_new_length@std@@" = linkonce_odr unnamed_addr constant %eh.ThrowInfo { i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (void (%"class.std::bad_array_new_length"*)* @"??1bad_array_new_length@std@@UEAA@XZ" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%eh.CatchableTypeArray.3* @"_CTA3?AVbad_array_new_length@std@@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, section ".xdata", comdat
@"??_C@_0BF@KINCDENJ@bad?5array?5new?5length?$AA@" = linkonce_odr dso_local unnamed_addr constant [21 x i8] c"bad array new length\00", comdat, align 1
@1 = private unnamed_addr constant { [3 x i8*] } { [3 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4bad_array_new_length@std@@6B@" to i8*), i8* bitcast (i8* (%"class.std::bad_array_new_length"*, i32)* @"??_Gbad_array_new_length@std@@UEAAPEAXI@Z" to i8*), i8* bitcast (i8* (%"class.std::exception"*)* @"?what@exception@std@@UEBAPEBDXZ" to i8*)] }, comdat($"??_7bad_array_new_length@std@@6B@")
@"??_R4bad_array_new_length@std@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor30* @"??_R0?AVbad_array_new_length@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3bad_array_new_length@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4bad_array_new_length@std@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R3bad_array_new_length@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 0, i32 3, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([4 x i32]* @"??_R2bad_array_new_length@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2bad_array_new_length@std@@8" = linkonce_odr constant [4 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@bad_array_new_length@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@bad_alloc@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@exception@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@"??_R1A@?0A@EA@bad_array_new_length@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor30* @"??_R0?AVbad_array_new_length@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 2, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3bad_array_new_length@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R1A@?0A@EA@bad_alloc@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor19* @"??_R0?AVbad_alloc@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 1, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3bad_alloc@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R3bad_alloc@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 0, i32 2, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([3 x i32]* @"??_R2bad_alloc@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2bad_alloc@std@@8" = linkonce_odr constant [3 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@bad_alloc@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@exception@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@"??_R1A@?0A@EA@exception@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor19* @"??_R0?AVexception@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3exception@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R3exception@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 0, i32 1, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([2 x i32]* @"??_R2exception@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2exception@std@@8" = linkonce_odr constant [2 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@exception@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@2 = private unnamed_addr constant { [3 x i8*] } { [3 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4bad_alloc@std@@6B@" to i8*), i8* bitcast (i8* (%"class.std::bad_alloc"*, i32)* @"??_Gbad_alloc@std@@UEAAPEAXI@Z" to i8*), i8* bitcast (i8* (%"class.std::exception"*)* @"?what@exception@std@@UEBAPEBDXZ" to i8*)] }, comdat($"??_7bad_alloc@std@@6B@")
@"??_R4bad_alloc@std@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor19* @"??_R0?AVbad_alloc@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3bad_alloc@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4bad_alloc@std@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@3 = private unnamed_addr constant { [3 x i8*] } { [3 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4exception@std@@6B@" to i8*), i8* bitcast (i8* (%"class.std::exception"*, i32)* @"??_Gexception@std@@UEAAPEAXI@Z" to i8*), i8* bitcast (i8* (%"class.std::exception"*)* @"?what@exception@std@@UEBAPEBDXZ" to i8*)] }, comdat($"??_7exception@std@@6B@")
@"??_R4exception@std@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor19* @"??_R0?AVexception@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3exception@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4exception@std@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_C@_0BC@EOODALEL@Unknown?5exception?$AA@" = linkonce_odr dso_local unnamed_addr constant [18 x i8] c"Unknown exception\00", comdat, align 1
@4 = private unnamed_addr constant { [3 x i8*] } { [3 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4failure@ios_base@std@@6B@" to i8*), i8* bitcast (i8* (%"class.std::ios_base::failure"*, i32)* @"??_Gfailure@ios_base@std@@UEAAPEAXI@Z" to i8*), i8* bitcast (i8* (%"class.std::exception"*)* @"?what@exception@std@@UEBAPEBDXZ" to i8*)] }, comdat($"??_7failure@ios_base@std@@6B@")
@"??_R4failure@ios_base@std@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor26* @"??_R0?AVfailure@ios_base@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3failure@ios_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4failure@ios_base@std@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R3failure@ios_base@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 0, i32 5, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([6 x i32]* @"??_R2failure@ios_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2failure@ios_base@std@@8" = linkonce_odr constant [6 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@failure@ios_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@system_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@_System_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@runtime_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@exception@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@"??_R1A@?0A@EA@failure@ios_base@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor26* @"??_R0?AVfailure@ios_base@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 4, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3failure@ios_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R1A@?0A@EA@system_error@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor22* @"??_R0?AVsystem_error@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 3, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3system_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R3system_error@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 0, i32 4, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([5 x i32]* @"??_R2system_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2system_error@std@@8" = linkonce_odr constant [5 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@system_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@_System_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@runtime_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@exception@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@"??_R1A@?0A@EA@_System_error@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor23* @"??_R0?AV_System_error@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 2, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3_System_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R3_System_error@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 0, i32 3, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([4 x i32]* @"??_R2_System_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2_System_error@std@@8" = linkonce_odr constant [4 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@_System_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@runtime_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@exception@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@"??_R1A@?0A@EA@runtime_error@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor23* @"??_R0?AVruntime_error@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 1, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3runtime_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R3runtime_error@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 0, i32 2, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([3 x i32]* @"??_R2runtime_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2runtime_error@std@@8" = linkonce_odr constant [3 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@runtime_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@exception@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@5 = private unnamed_addr constant { [3 x i8*] } { [3 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4system_error@std@@6B@" to i8*), i8* bitcast (i8* (%"class.std::system_error"*, i32)* @"??_Gsystem_error@std@@UEAAPEAXI@Z" to i8*), i8* bitcast (i8* (%"class.std::exception"*)* @"?what@exception@std@@UEBAPEBDXZ" to i8*)] }, comdat($"??_7system_error@std@@6B@")
@"??_R4system_error@std@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor22* @"??_R0?AVsystem_error@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3system_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4system_error@std@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@6 = private unnamed_addr constant { [3 x i8*] } { [3 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4_System_error@std@@6B@" to i8*), i8* bitcast (i8* (%"class.std::_System_error"*, i32)* @"??_G_System_error@std@@UEAAPEAXI@Z" to i8*), i8* bitcast (i8* (%"class.std::exception"*)* @"?what@exception@std@@UEBAPEBDXZ" to i8*)] }, comdat($"??_7_System_error@std@@6B@")
@"??_R4_System_error@std@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor23* @"??_R0?AV_System_error@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3_System_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4_System_error@std@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_C@_02LMMGGCAJ@?3?5?$AA@" = linkonce_odr dso_local unnamed_addr constant [3 x i8] c": \00", comdat, align 1
@7 = private unnamed_addr constant { [3 x i8*] } { [3 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4runtime_error@std@@6B@" to i8*), i8* bitcast (i8* (%"class.std::runtime_error"*, i32)* @"??_Gruntime_error@std@@UEAAPEAXI@Z" to i8*), i8* bitcast (i8* (%"class.std::exception"*)* @"?what@exception@std@@UEBAPEBDXZ" to i8*)] }, comdat($"??_7runtime_error@std@@6B@")
@"??_R4runtime_error@std@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor23* @"??_R0?AVruntime_error@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3runtime_error@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4runtime_error@std@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"?_Psave@?$_Facetptr@V?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@@std@@2PEBVfacet@locale@2@EB" = linkonce_odr dso_local global %"class.std::locale::facet"* null, comdat, align 8
@"?_Id_cnt@id@locale@std@@0HA" = external dso_local global i32, align 4
@"??_C@_00CNPNBAHC@?$AA@" = linkonce_odr dso_local unnamed_addr constant [1 x i8] zeroinitializer, comdat, align 1
@"??_C@_0BA@ELKIONDK@bad?5locale?5name?$AA@" = linkonce_odr dso_local unnamed_addr constant [16 x i8] c"bad locale name\00", comdat, align 1
@8 = private unnamed_addr constant { [12 x i8*] } { [12 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@6B@" to i8*), i8* bitcast (i8* (%"class.std::num_put"*, i32)* @"??_G?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEAAPEAXI@Z" to i8*), i8* bitcast (void (%"class.std::locale::facet"*)* @"?_Incref@facet@locale@std@@UEAAXXZ" to i8*), i8* bitcast (%"class.std::_Facet_base"* (%"class.std::locale::facet"*)* @"?_Decref@facet@locale@std@@UEAAPEAV_Facet_base@3@XZ" to i8*), i8* bitcast (void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i8*)* @"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEBX@Z" to i8*), i8* bitcast (void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, double)* @"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DO@Z" to i8*), i8* bitcast (void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, double)* @"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DN@Z" to i8*), i8* bitcast (void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i64)* @"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@D_K@Z" to i8*), i8* bitcast (void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i64)* @"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@D_J@Z" to i8*), i8* bitcast (void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i32)* @"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DK@Z" to i8*), i8* bitcast (void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i32)* @"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DJ@Z" to i8*), i8* bitcast (void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i1)* @"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@D_N@Z" to i8*)] }, comdat($"??_7?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@6B@")
@"??_R4?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor73* @"??_R0?AV?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R0?AV?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@@8" = linkonce_odr global %rtti.TypeDescriptor73 { i8** @"??_7type_info@@6B@", i8* null, [74 x i8] c".?AV?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@\00" }, comdat
@"??_R3?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 1, i32 4, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([5 x i32]* @"??_R2?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@8" = linkonce_odr constant [5 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@facet@locale@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@_Facet_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R17?0A@EA@_Crt_new_delete@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@"??_R1A@?0A@EA@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor73* @"??_R0?AV?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 3, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R1A@?0A@EA@facet@locale@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor22* @"??_R0?AVfacet@locale@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 2, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3facet@locale@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R0?AVfacet@locale@std@@@8" = linkonce_odr global %rtti.TypeDescriptor22 { i8** @"??_7type_info@@6B@", i8* null, [23 x i8] c".?AVfacet@locale@std@@\00" }, comdat
@"??_R3facet@locale@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 1, i32 3, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([4 x i32]* @"??_R2facet@locale@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2facet@locale@std@@8" = linkonce_odr constant [4 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@facet@locale@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@_Facet_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R17?0A@EA@_Crt_new_delete@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@"??_R1A@?0A@EA@_Facet_base@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor21* @"??_R0?AV_Facet_base@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3_Facet_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R0?AV_Facet_base@std@@@8" = linkonce_odr global %rtti.TypeDescriptor21 { i8** @"??_7type_info@@6B@", i8* null, [22 x i8] c".?AV_Facet_base@std@@\00" }, comdat
@"??_R3_Facet_base@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 0, i32 1, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([2 x i32]* @"??_R2_Facet_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2_Facet_base@std@@8" = linkonce_odr constant [2 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@_Facet_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@"??_R17?0A@EA@_Crt_new_delete@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor25* @"??_R0?AU_Crt_new_delete@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 8, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3_Crt_new_delete@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R0?AU_Crt_new_delete@std@@@8" = linkonce_odr global %rtti.TypeDescriptor25 { i8** @"??_7type_info@@6B@", i8* null, [26 x i8] c".?AU_Crt_new_delete@std@@\00" }, comdat
@"??_R3_Crt_new_delete@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 0, i32 1, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([2 x i32]* @"??_R2_Crt_new_delete@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2_Crt_new_delete@std@@8" = linkonce_odr constant [2 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@_Crt_new_delete@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@"??_R1A@?0A@EA@_Crt_new_delete@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor25* @"??_R0?AU_Crt_new_delete@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3_Crt_new_delete@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@9 = private unnamed_addr constant { [4 x i8*] } { [4 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4facet@locale@std@@6B@" to i8*), i8* bitcast (i8* (%"class.std::locale::facet"*, i32)* @"??_Gfacet@locale@std@@MEAAPEAXI@Z" to i8*), i8* bitcast (void (%"class.std::locale::facet"*)* @"?_Incref@facet@locale@std@@UEAAXXZ" to i8*), i8* bitcast (%"class.std::_Facet_base"* (%"class.std::locale::facet"*)* @"?_Decref@facet@locale@std@@UEAAPEAV_Facet_base@3@XZ" to i8*)] }, comdat($"??_7facet@locale@std@@6B@")
@"??_R4facet@locale@std@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor22* @"??_R0?AVfacet@locale@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3facet@locale@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4facet@locale@std@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@10 = private unnamed_addr constant { [4 x i8*] } { [4 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4_Facet_base@std@@6B@" to i8*), i8* bitcast (i8* (%"class.std::_Facet_base"*, i32)* @"??_G_Facet_base@std@@UEAAPEAXI@Z" to i8*), i8* bitcast (void ()* @_purecall to i8*), i8* bitcast (void ()* @_purecall to i8*)] }, comdat($"??_7_Facet_base@std@@6B@")
@"??_R4_Facet_base@std@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor21* @"??_R0?AV_Facet_base@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3_Facet_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4_Facet_base@std@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_C@_02BBAHNLBA@?$CFp?$AA@" = linkonce_odr dso_local unnamed_addr constant [3 x i8] c"%p\00", comdat, align 1
@"?_Psave@?$_Facetptr@V?$ctype@D@std@@@std@@2PEBVfacet@locale@2@EB" = linkonce_odr dso_local global %"class.std::locale::facet"* null, comdat, align 8
@"?id@?$ctype@D@std@@2V0locale@2@A" = external dso_local global %"class.std::locale::id", align 8
@11 = private unnamed_addr constant { [12 x i8*] } { [12 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4?$ctype@D@std@@6B@" to i8*), i8* bitcast (i8* (%"class.std::ctype"*, i32)* @"??_G?$ctype@D@std@@MEAAPEAXI@Z" to i8*), i8* bitcast (void (%"class.std::locale::facet"*)* @"?_Incref@facet@locale@std@@UEAAXXZ" to i8*), i8* bitcast (%"class.std::_Facet_base"* (%"class.std::locale::facet"*)* @"?_Decref@facet@locale@std@@UEAAPEAV_Facet_base@3@XZ" to i8*), i8* bitcast (i8* (%"class.std::ctype"*, i8*, i8*)* @"?do_tolower@?$ctype@D@std@@MEBAPEBDPEADPEBD@Z" to i8*), i8* bitcast (i8 (%"class.std::ctype"*, i8)* @"?do_tolower@?$ctype@D@std@@MEBADD@Z" to i8*), i8* bitcast (i8* (%"class.std::ctype"*, i8*, i8*)* @"?do_toupper@?$ctype@D@std@@MEBAPEBDPEADPEBD@Z" to i8*), i8* bitcast (i8 (%"class.std::ctype"*, i8)* @"?do_toupper@?$ctype@D@std@@MEBADD@Z" to i8*), i8* bitcast (i8* (%"class.std::ctype"*, i8*, i8*, i8*)* @"?do_widen@?$ctype@D@std@@MEBAPEBDPEBD0PEAD@Z" to i8*), i8* bitcast (i8 (%"class.std::ctype"*, i8)* @"?do_widen@?$ctype@D@std@@MEBADD@Z" to i8*), i8* bitcast (i8* (%"class.std::ctype"*, i8*, i8*, i8, i8*)* @"?do_narrow@?$ctype@D@std@@MEBAPEBDPEBD0DPEAD@Z" to i8*), i8* bitcast (i8 (%"class.std::ctype"*, i8, i8)* @"?do_narrow@?$ctype@D@std@@MEBADDD@Z" to i8*)] }, comdat($"??_7?$ctype@D@std@@6B@")
@"??_R4?$ctype@D@std@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor19* @"??_R0?AV?$ctype@D@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3?$ctype@D@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4?$ctype@D@std@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R0?AV?$ctype@D@std@@@8" = linkonce_odr global %rtti.TypeDescriptor19 { i8** @"??_7type_info@@6B@", i8* null, [20 x i8] c".?AV?$ctype@D@std@@\00" }, comdat
@"??_R3?$ctype@D@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 1, i32 5, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([6 x i32]* @"??_R2?$ctype@D@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2?$ctype@D@std@@8" = linkonce_odr constant [6 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@?$ctype@D@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@ctype_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@facet@locale@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@_Facet_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R17?0A@EA@_Crt_new_delete@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@"??_R1A@?0A@EA@?$ctype@D@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor19* @"??_R0?AV?$ctype@D@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 4, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3?$ctype@D@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R1A@?0A@EA@ctype_base@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor20* @"??_R0?AUctype_base@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 3, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3ctype_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R0?AUctype_base@std@@@8" = linkonce_odr global %rtti.TypeDescriptor20 { i8** @"??_7type_info@@6B@", i8* null, [21 x i8] c".?AUctype_base@std@@\00" }, comdat
@"??_R3ctype_base@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 1, i32 4, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([5 x i32]* @"??_R2ctype_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2ctype_base@std@@8" = linkonce_odr constant [5 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@ctype_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@facet@locale@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@_Facet_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R17?0A@EA@_Crt_new_delete@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@12 = private unnamed_addr constant { [4 x i8*] } { [4 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4ctype_base@std@@6B@" to i8*), i8* bitcast (i8* (%"struct.std::ctype_base"*, i32)* @"??_Gctype_base@std@@UEAAPEAXI@Z" to i8*), i8* bitcast (void (%"class.std::locale::facet"*)* @"?_Incref@facet@locale@std@@UEAAXXZ" to i8*), i8* bitcast (%"class.std::_Facet_base"* (%"class.std::locale::facet"*)* @"?_Decref@facet@locale@std@@UEAAPEAV_Facet_base@3@XZ" to i8*)] }, comdat($"??_7ctype_base@std@@6B@")
@"??_R4ctype_base@std@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor20* @"??_R0?AUctype_base@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3ctype_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4ctype_base@std@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"?_Psave@?$_Facetptr@V?$numpunct@D@std@@@std@@2PEBVfacet@locale@2@EB" = linkonce_odr dso_local global %"class.std::locale::facet"* null, comdat, align 8
@13 = private unnamed_addr constant { [9 x i8*] } { [9 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4?$numpunct@D@std@@6B@" to i8*), i8* bitcast (i8* (%"class.std::numpunct"*, i32)* @"??_G?$numpunct@D@std@@MEAAPEAXI@Z" to i8*), i8* bitcast (void (%"class.std::locale::facet"*)* @"?_Incref@facet@locale@std@@UEAAXXZ" to i8*), i8* bitcast (%"class.std::_Facet_base"* (%"class.std::locale::facet"*)* @"?_Decref@facet@locale@std@@UEAAPEAV_Facet_base@3@XZ" to i8*), i8* bitcast (i8 (%"class.std::numpunct"*)* @"?do_decimal_point@?$numpunct@D@std@@MEBADXZ" to i8*), i8* bitcast (i8 (%"class.std::numpunct"*)* @"?do_thousands_sep@?$numpunct@D@std@@MEBADXZ" to i8*), i8* bitcast (void (%"class.std::numpunct"*, %"class.std::basic_string"*)* @"?do_grouping@?$numpunct@D@std@@MEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ" to i8*), i8* bitcast (void (%"class.std::numpunct"*, %"class.std::basic_string"*)* @"?do_falsename@?$numpunct@D@std@@MEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ" to i8*), i8* bitcast (void (%"class.std::numpunct"*, %"class.std::basic_string"*)* @"?do_truename@?$numpunct@D@std@@MEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ" to i8*)] }, comdat($"??_7?$numpunct@D@std@@6B@")
@"??_R4?$numpunct@D@std@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor22* @"??_R0?AV?$numpunct@D@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3?$numpunct@D@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4?$numpunct@D@std@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R0?AV?$numpunct@D@std@@@8" = linkonce_odr global %rtti.TypeDescriptor22 { i8** @"??_7type_info@@6B@", i8* null, [23 x i8] c".?AV?$numpunct@D@std@@\00" }, comdat
@"??_R3?$numpunct@D@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 1, i32 4, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([5 x i32]* @"??_R2?$numpunct@D@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2?$numpunct@D@std@@8" = linkonce_odr constant [5 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@?$numpunct@D@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@facet@locale@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@_Facet_base@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R17?0A@EA@_Crt_new_delete@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@"??_R1A@?0A@EA@?$numpunct@D@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor22* @"??_R0?AV?$numpunct@D@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 3, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3?$numpunct@D@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_C@_05LAPONLG@false?$AA@" = linkonce_odr dso_local unnamed_addr constant [6 x i8] c"false\00", comdat, align 1
@"??_C@_04LOAJBDKD@true?$AA@" = linkonce_odr dso_local unnamed_addr constant [5 x i8] c"true\00", comdat, align 1
@"??_C@_0BI@CFPLBAOH@invalid?5string?5position?$AA@" = linkonce_odr dso_local unnamed_addr constant [24 x i8] c"invalid string position\00", comdat, align 1
@"?_OptionsStorage@?1??__local_stdio_printf_options@@9@4_KA" = linkonce_odr dso_local global i64 0, comdat, align 8
@"??_C@_02MDKMJEGG@eE?$AA@" = linkonce_odr dso_local unnamed_addr constant [3 x i8] c"eE\00", comdat, align 1
@"??_C@_02OOPEBDOJ@pP?$AA@" = linkonce_odr dso_local unnamed_addr constant [3 x i8] c"pP\00", comdat, align 1
@"__const.?_Fput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEBD_K@Z._Dp" = private unnamed_addr constant [2 x i8] c".\00", align 1
@"??_C@_02CLHGNPPK@Lu?$AA@" = linkonce_odr dso_local unnamed_addr constant [3 x i8] c"Lu\00", comdat, align 1
@"??_C@_02HIKPPMOK@Ld?$AA@" = linkonce_odr dso_local unnamed_addr constant [3 x i8] c"Ld\00", comdat, align 1
@"??_C@_02BDDLJJBK@lu?$AA@" = linkonce_odr dso_local unnamed_addr constant [3 x i8] c"lu\00", comdat, align 1
@"??_C@_02EAOCLKAK@ld?$AA@" = linkonce_odr dso_local unnamed_addr constant [3 x i8] c"ld\00", comdat, align 1
@"??_R0?AVbad_cast@std@@@8" = linkonce_odr global %rtti.TypeDescriptor18 { i8** @"??_7type_info@@6B@", i8* null, [19 x i8] c".?AVbad_cast@std@@\00" }, comdat
@"_CT??_R0?AVbad_cast@std@@@8??0bad_cast@std@@QEAA@AEBV01@@Z24" = linkonce_odr unnamed_addr constant %eh.CatchableType { i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor18* @"??_R0?AVbad_cast@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 -1, i32 0, i32 24, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%"class.std::bad_cast"* (%"class.std::bad_cast"*, %"class.std::bad_cast"*)* @"??0bad_cast@std@@QEAA@AEBV01@@Z" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, section ".xdata", comdat
@"_CTA2?AVbad_cast@std@@" = linkonce_odr unnamed_addr constant %eh.CatchableTypeArray.2 { i32 2, [2 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%eh.CatchableType* @"_CT??_R0?AVbad_cast@std@@@8??0bad_cast@std@@QEAA@AEBV01@@Z24" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%eh.CatchableType* @"_CT??_R0?AVexception@std@@@8??0exception@std@@QEAA@AEBV01@@Z24" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32)] }, section ".xdata", comdat
@"_TI2?AVbad_cast@std@@" = linkonce_odr unnamed_addr constant %eh.ThrowInfo { i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (void (%"class.std::bad_cast"*)* @"??1bad_cast@std@@UEAA@XZ" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%eh.CatchableTypeArray.2* @"_CTA2?AVbad_cast@std@@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, section ".xdata", comdat
@"??_C@_08EPJLHIJG@bad?5cast?$AA@" = linkonce_odr dso_local unnamed_addr constant [9 x i8] c"bad cast\00", comdat, align 1
@14 = private unnamed_addr constant { [3 x i8*] } { [3 x i8*] [i8* bitcast (%rtti.CompleteObjectLocator* @"??_R4bad_cast@std@@6B@" to i8*), i8* bitcast (i8* (%"class.std::bad_cast"*, i32)* @"??_Gbad_cast@std@@UEAAPEAXI@Z" to i8*), i8* bitcast (i8* (%"class.std::exception"*)* @"?what@exception@std@@UEBAPEBDXZ" to i8*)] }, comdat($"??_7bad_cast@std@@6B@")
@"??_R4bad_cast@std@@6B@" = linkonce_odr constant %rtti.CompleteObjectLocator { i32 1, i32 0, i32 0, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor18* @"??_R0?AVbad_cast@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3bad_cast@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.CompleteObjectLocator* @"??_R4bad_cast@std@@6B@" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R3bad_cast@std@@8" = linkonce_odr constant %rtti.ClassHierarchyDescriptor { i32 0, i32 0, i32 2, i32 trunc (i64 sub nuw nsw (i64 ptrtoint ([3 x i32]* @"??_R2bad_cast@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@"??_R2bad_cast@std@@8" = linkonce_odr constant [3 x i32] [i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@bad_cast@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.BaseClassDescriptor* @"??_R1A@?0A@EA@exception@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 0], comdat
@"??_R1A@?0A@EA@bad_cast@std@@8" = linkonce_odr constant %rtti.BaseClassDescriptor { i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.TypeDescriptor18* @"??_R0?AVbad_cast@std@@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32), i32 1, i32 0, i32 -1, i32 0, i32 64, i32 trunc (i64 sub nuw nsw (i64 ptrtoint (%rtti.ClassHierarchyDescriptor* @"??_R3bad_cast@std@@8" to i64), i64 ptrtoint (i8* @__ImageBase to i64)) to i32) }, comdat
@llvm.global_ctors = appending global [2 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 65535, void ()* @"??__E?id@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@2V0locale@2@A@@YAXXZ", i8* bitcast (%"class.std::locale::id"* @"?id@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@2V0locale@2@A" to i8*) }, { i32, void ()*, i8* } { i32 65535, void ()* @"??__E?id@?$numpunct@D@std@@2V0locale@2@A@@YAXXZ", i8* bitcast (%"class.std::locale::id"* @"?id@?$numpunct@D@std@@2V0locale@2@A" to i8*) }]
@llvm.used = appending global [2 x i8*] [i8* bitcast (%"class.std::locale::id"* @"?id@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@2V0locale@2@A" to i8*), i8* bitcast (%"class.std::locale::id"* @"?id@?$numpunct@D@std@@2V0locale@2@A" to i8*)], section "llvm.metadata"

@"??_7_Iostream_error_category2@std@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [7 x i8*] }, { [7 x i8*] }* @0, i32 0, i32 0, i32 1)
@"??_7bad_array_new_length@std@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [3 x i8*] }, { [3 x i8*] }* @1, i32 0, i32 0, i32 1)
@"??_7bad_alloc@std@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [3 x i8*] }, { [3 x i8*] }* @2, i32 0, i32 0, i32 1)
@"??_7exception@std@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [3 x i8*] }, { [3 x i8*] }* @3, i32 0, i32 0, i32 1)
@"??_7failure@ios_base@std@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [3 x i8*] }, { [3 x i8*] }* @4, i32 0, i32 0, i32 1)
@"??_7system_error@std@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [3 x i8*] }, { [3 x i8*] }* @5, i32 0, i32 0, i32 1)
@"??_7_System_error@std@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [3 x i8*] }, { [3 x i8*] }* @6, i32 0, i32 0, i32 1)
@"??_7runtime_error@std@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [3 x i8*] }, { [3 x i8*] }* @7, i32 0, i32 0, i32 1)
@"??_7?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [12 x i8*] }, { [12 x i8*] }* @8, i32 0, i32 0, i32 1)
@"??_7facet@locale@std@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [4 x i8*] }, { [4 x i8*] }* @9, i32 0, i32 0, i32 1)
@"??_7_Facet_base@std@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [4 x i8*] }, { [4 x i8*] }* @10, i32 0, i32 0, i32 1)
@"??_7?$ctype@D@std@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [12 x i8*] }, { [12 x i8*] }* @11, i32 0, i32 0, i32 1)
@"??_7ctype_base@std@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [4 x i8*] }, { [4 x i8*] }* @12, i32 0, i32 0, i32 1)
@"??_7?$numpunct@D@std@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [9 x i8*] }, { [9 x i8*] }* @13, i32 0, i32 0, i32 1)
@"??_7bad_cast@std@@6B@" = unnamed_addr alias i8*, getelementptr inbounds ({ [3 x i8*] }, { [3 x i8*] }* @14, i32 0, i32 0, i32 1)

; Function Attrs: noinline norecurse optnone uwtable
define dso_local i32 @main() #0 {
  %1 = alloca i64, align 8
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca double, align 8
  store i64 5000000000, i64* %1, align 8
  store i64 6000000000, i64* %2, align 8
  store i64 3000000000, i64* %3, align 8
  store i64 2000000000, i64* %4, align 8
  store i64 10000000000, i64* %5, align 8
  %7 = load i64, i64* %1, align 8
  %8 = load i64, i64* %2, align 8
  %9 = load i64, i64* %3, align 8
  %10 = mul nsw i64 %8, %9
  %11 = load i64, i64* %4, align 8
  %12 = sdiv i64 %10, %11
  %13 = add nsw i64 %7, %12
  %14 = load i64, i64* %5, align 8
  %15 = sub nsw i64 %13, %14
  %16 = sitofp i64 %15 to double
  store double %16, double* %6, align 8
  %17 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??$?6U?$char_traits@D@std@@@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@0@AEAV10@PEBD@Z"(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) @"?cout@std@@3V?$basic_ostream@DU?$char_traits@D@std@@@1@A", i8* getelementptr inbounds ([7 x i8], [7 x i8]* @"??_C@_06FMLHDGIC@Size?3?5?$AA@", i64 0, i64 0))
  %18 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@_K@Z"(%"class.std::basic_ostream"* %17, i64 64)
  %19 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??$?6U?$char_traits@D@std@@@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@0@AEAV10@D@Z"(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %18, i8 10)
  %20 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??$?6U?$char_traits@D@std@@@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@0@AEAV10@PEBD@Z"(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) @"?cout@std@@3V?$basic_ostream@DU?$char_traits@D@std@@@1@A", i8* getelementptr inbounds ([12 x i8], [12 x i8]* @"??_C@_0M@NKLJEELK@Max?5Long?5?3?5?$AA@", i64 0, i64 0))
  %21 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@J@Z"(%"class.std::basic_ostream"* %20, i32 2147483647)
  %22 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??$?6U?$char_traits@D@std@@@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@0@AEAV10@D@Z"(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %21, i8 10)
  %23 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??$?6U?$char_traits@D@std@@@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@0@AEAV10@PEBD@Z"(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) @"?cout@std@@3V?$basic_ostream@DU?$char_traits@D@std@@@1@A", i8* getelementptr inbounds ([17 x i8], [17 x i8]* @"??_C@_0BB@CCIDIEPP@Max?5Long?5Long?5?3?5?$AA@", i64 0, i64 0))
  %24 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@_J@Z"(%"class.std::basic_ostream"* %23, i64 9223372036854775807)
  %25 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??$?6U?$char_traits@D@std@@@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@0@AEAV10@D@Z"(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %24, i8 10)
  %26 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??$?6U?$char_traits@D@std@@@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@0@AEAV10@PEBD@Z"(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) @"?cout@std@@3V?$basic_ostream@DU?$char_traits@D@std@@@1@A", i8* getelementptr inbounds ([7 x i8], [7 x i8]* @"??_C@_06GGONACPB@Result?$AA@", i64 0, i64 0))
  %27 = load double, double* %6, align 8
  %28 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@O@Z"(%"class.std::basic_ostream"* %26, double %27)
  ret i32 0
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??$?6U?$char_traits@D@std@@@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@0@AEAV10@D@Z"(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %0, i8 %1) #1 comdat personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %3 = alloca i8, align 1
  %4 = alloca %"class.std::basic_ostream"*, align 8
  %5 = alloca i32, align 4
  %6 = alloca %"class.std::basic_ostream<char, std::char_traits<char>>::sentry", align 8
  %7 = alloca i64, align 8
  %8 = alloca i32, align 4
  %9 = alloca i32, align 4
  %10 = alloca i32, align 4
  %11 = alloca i32, align 4
  %12 = alloca i32, align 4
  %13 = alloca i32, align 4
  store i8 %1, i8* %3, align 1
  store %"class.std::basic_ostream"* %0, %"class.std::basic_ostream"** %4, align 8
  store i32 0, i32* %5, align 4
  %14 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %15 = call %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* @"??0sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@AEAV12@@Z"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6, %"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %14)
  %16 = invoke zeroext i1 @"??Bsentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEBA_NXZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6)
          to label %17 unwind label %246

17:                                               ; preds = %2
  br i1 %16, label %18, label %216

18:                                               ; preds = %17
  %19 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %20 = bitcast %"class.std::basic_ostream"* %19 to i8*
  %21 = getelementptr inbounds i8, i8* %20, i64 0
  %22 = bitcast i8* %21 to i32**
  %23 = load i32*, i32** %22, align 8
  %24 = getelementptr inbounds i32, i32* %23, i32 1
  %25 = load i32, i32* %24, align 4
  %26 = sext i32 %25 to i64
  %27 = add nsw i64 0, %26
  %28 = bitcast %"class.std::basic_ostream"* %19 to i8*
  %29 = getelementptr inbounds i8, i8* %28, i64 %27
  %30 = bitcast i8* %29 to %"class.std::ios_base"*
  %31 = invoke i64 @"?width@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %30)
          to label %32 unwind label %246

32:                                               ; preds = %18
  %33 = icmp sle i64 %31, 1
  br i1 %33, label %34, label %35

34:                                               ; preds = %32
  br label %51

35:                                               ; preds = %32
  %36 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %37 = bitcast %"class.std::basic_ostream"* %36 to i8*
  %38 = getelementptr inbounds i8, i8* %37, i64 0
  %39 = bitcast i8* %38 to i32**
  %40 = load i32*, i32** %39, align 8
  %41 = getelementptr inbounds i32, i32* %40, i32 1
  %42 = load i32, i32* %41, align 4
  %43 = sext i32 %42 to i64
  %44 = add nsw i64 0, %43
  %45 = bitcast %"class.std::basic_ostream"* %36 to i8*
  %46 = getelementptr inbounds i8, i8* %45, i64 %44
  %47 = bitcast i8* %46 to %"class.std::ios_base"*
  %48 = invoke i64 @"?width@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %47)
          to label %49 unwind label %246

49:                                               ; preds = %35
  %50 = sub nsw i64 %48, 1
  br label %51

51:                                               ; preds = %49, %34
  %52 = phi i64 [ 0, %34 ], [ %50, %49 ]
  store i64 %52, i64* %7, align 8
  %53 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %54 = bitcast %"class.std::basic_ostream"* %53 to i8*
  %55 = getelementptr inbounds i8, i8* %54, i64 0
  %56 = bitcast i8* %55 to i32**
  %57 = load i32*, i32** %56, align 8
  %58 = getelementptr inbounds i32, i32* %57, i32 1
  %59 = load i32, i32* %58, align 4
  %60 = sext i32 %59 to i64
  %61 = add nsw i64 0, %60
  %62 = bitcast %"class.std::basic_ostream"* %53 to i8*
  %63 = getelementptr inbounds i8, i8* %62, i64 %61
  %64 = bitcast i8* %63 to %"class.std::ios_base"*
  %65 = invoke i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %64)
          to label %66 unwind label %186

66:                                               ; preds = %51
  %67 = and i32 %65, 448
  %68 = icmp ne i32 %67, 64
  br i1 %68, label %69, label %119

69:                                               ; preds = %66
  br label %70

70:                                               ; preds = %115, %69
  %71 = load i32, i32* %5, align 4
  %72 = icmp eq i32 %71, 0
  br i1 %72, label %73, label %76

73:                                               ; preds = %70
  %74 = load i64, i64* %7, align 8
  %75 = icmp slt i64 0, %74
  br label %76

76:                                               ; preds = %73, %70
  %77 = phi i1 [ false, %70 ], [ %75, %73 ]
  br i1 %77, label %78, label %118

78:                                               ; preds = %76
  %79 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %80 = bitcast %"class.std::basic_ostream"* %79 to i8*
  %81 = getelementptr inbounds i8, i8* %80, i64 0
  %82 = bitcast i8* %81 to i32**
  %83 = load i32*, i32** %82, align 8
  %84 = getelementptr inbounds i32, i32* %83, i32 1
  %85 = load i32, i32* %84, align 4
  %86 = sext i32 %85 to i64
  %87 = add nsw i64 0, %86
  %88 = bitcast %"class.std::basic_ostream"* %79 to i8*
  %89 = getelementptr inbounds i8, i8* %88, i64 %87
  %90 = bitcast i8* %89 to %"class.std::basic_ios"*
  %91 = invoke %"class.std::basic_streambuf"* @"?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %90)
          to label %92 unwind label %186

92:                                               ; preds = %78
  %93 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %94 = bitcast %"class.std::basic_ostream"* %93 to i8*
  %95 = getelementptr inbounds i8, i8* %94, i64 0
  %96 = bitcast i8* %95 to i32**
  %97 = load i32*, i32** %96, align 8
  %98 = getelementptr inbounds i32, i32* %97, i32 1
  %99 = load i32, i32* %98, align 4
  %100 = sext i32 %99 to i64
  %101 = add nsw i64 0, %100
  %102 = bitcast %"class.std::basic_ostream"* %93 to i8*
  %103 = getelementptr inbounds i8, i8* %102, i64 %101
  %104 = bitcast i8* %103 to %"class.std::basic_ios"*
  %105 = invoke i8 @"?fill@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBADXZ"(%"class.std::basic_ios"* %104)
          to label %106 unwind label %186

106:                                              ; preds = %92
  %107 = invoke i32 @"?sputc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHD@Z"(%"class.std::basic_streambuf"* %91, i8 %105)
          to label %108 unwind label %186

108:                                              ; preds = %106
  store i32 %107, i32* %8, align 4
  %109 = call i32 @"?eof@?$_Narrow_char_traits@DH@std@@SAHXZ"() #5
  store i32 %109, i32* %9, align 4
  %110 = call zeroext i1 @"?eq_int_type@?$_Narrow_char_traits@DH@std@@SA_NAEBH0@Z"(i32* nonnull align 4 dereferenceable(4) %9, i32* nonnull align 4 dereferenceable(4) %8) #5
  br i1 %110, label %111, label %114

111:                                              ; preds = %108
  %112 = load i32, i32* %5, align 4
  %113 = or i32 %112, 4
  store i32 %113, i32* %5, align 4
  br label %114

114:                                              ; preds = %111, %108
  br label %115

115:                                              ; preds = %114
  %116 = load i64, i64* %7, align 8
  %117 = add nsw i64 %116, -1
  store i64 %117, i64* %7, align 8
  br label %70

118:                                              ; preds = %76
  br label %119

119:                                              ; preds = %118, %66
  %120 = load i32, i32* %5, align 4
  %121 = icmp eq i32 %120, 0
  br i1 %121, label %122, label %142

122:                                              ; preds = %119
  %123 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %124 = bitcast %"class.std::basic_ostream"* %123 to i8*
  %125 = getelementptr inbounds i8, i8* %124, i64 0
  %126 = bitcast i8* %125 to i32**
  %127 = load i32*, i32** %126, align 8
  %128 = getelementptr inbounds i32, i32* %127, i32 1
  %129 = load i32, i32* %128, align 4
  %130 = sext i32 %129 to i64
  %131 = add nsw i64 0, %130
  %132 = bitcast %"class.std::basic_ostream"* %123 to i8*
  %133 = getelementptr inbounds i8, i8* %132, i64 %131
  %134 = bitcast i8* %133 to %"class.std::basic_ios"*
  %135 = invoke %"class.std::basic_streambuf"* @"?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %134)
          to label %136 unwind label %186

136:                                              ; preds = %122
  %137 = load i8, i8* %3, align 1
  %138 = invoke i32 @"?sputc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHD@Z"(%"class.std::basic_streambuf"* %135, i8 %137)
          to label %139 unwind label %186

139:                                              ; preds = %136
  store i32 %138, i32* %10, align 4
  %140 = call i32 @"?eof@?$_Narrow_char_traits@DH@std@@SAHXZ"() #5
  store i32 %140, i32* %11, align 4
  %141 = call zeroext i1 @"?eq_int_type@?$_Narrow_char_traits@DH@std@@SA_NAEBH0@Z"(i32* nonnull align 4 dereferenceable(4) %11, i32* nonnull align 4 dereferenceable(4) %10) #5
  br label %142

142:                                              ; preds = %139, %119
  %143 = phi i1 [ false, %119 ], [ %141, %139 ]
  br i1 %143, label %144, label %147

144:                                              ; preds = %142
  %145 = load i32, i32* %5, align 4
  %146 = or i32 %145, 4
  store i32 %146, i32* %5, align 4
  br label %147

147:                                              ; preds = %144, %142
  br label %148

148:                                              ; preds = %212, %147
  %149 = load i32, i32* %5, align 4
  %150 = icmp eq i32 %149, 0
  br i1 %150, label %151, label %154

151:                                              ; preds = %148
  %152 = load i64, i64* %7, align 8
  %153 = icmp slt i64 0, %152
  br label %154

154:                                              ; preds = %151, %148
  %155 = phi i1 [ false, %148 ], [ %153, %151 ]
  br i1 %155, label %156, label %215

156:                                              ; preds = %154
  %157 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %158 = bitcast %"class.std::basic_ostream"* %157 to i8*
  %159 = getelementptr inbounds i8, i8* %158, i64 0
  %160 = bitcast i8* %159 to i32**
  %161 = load i32*, i32** %160, align 8
  %162 = getelementptr inbounds i32, i32* %161, i32 1
  %163 = load i32, i32* %162, align 4
  %164 = sext i32 %163 to i64
  %165 = add nsw i64 0, %164
  %166 = bitcast %"class.std::basic_ostream"* %157 to i8*
  %167 = getelementptr inbounds i8, i8* %166, i64 %165
  %168 = bitcast i8* %167 to %"class.std::basic_ios"*
  %169 = invoke %"class.std::basic_streambuf"* @"?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %168)
          to label %170 unwind label %186

170:                                              ; preds = %156
  %171 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %172 = bitcast %"class.std::basic_ostream"* %171 to i8*
  %173 = getelementptr inbounds i8, i8* %172, i64 0
  %174 = bitcast i8* %173 to i32**
  %175 = load i32*, i32** %174, align 8
  %176 = getelementptr inbounds i32, i32* %175, i32 1
  %177 = load i32, i32* %176, align 4
  %178 = sext i32 %177 to i64
  %179 = add nsw i64 0, %178
  %180 = bitcast %"class.std::basic_ostream"* %171 to i8*
  %181 = getelementptr inbounds i8, i8* %180, i64 %179
  %182 = bitcast i8* %181 to %"class.std::basic_ios"*
  %183 = invoke i8 @"?fill@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBADXZ"(%"class.std::basic_ios"* %182)
          to label %184 unwind label %186

184:                                              ; preds = %170
  %185 = invoke i32 @"?sputc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHD@Z"(%"class.std::basic_streambuf"* %169, i8 %183)
          to label %205 unwind label %186

186:                                              ; preds = %184, %170, %156, %136, %122, %106, %92, %78, %51
  %187 = catchswitch within none [label %188] unwind label %246

188:                                              ; preds = %186
  %189 = catchpad within %187 [i8* null, i32 64, i8* null]
  %190 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %191 = bitcast %"class.std::basic_ostream"* %190 to i8*
  %192 = getelementptr inbounds i8, i8* %191, i64 0
  %193 = bitcast i8* %192 to i32**
  %194 = load i32*, i32** %193, align 8
  %195 = getelementptr inbounds i32, i32* %194, i32 1
  %196 = load i32, i32* %195, align 4
  %197 = sext i32 %196 to i64
  %198 = add nsw i64 0, %197
  %199 = bitcast %"class.std::basic_ostream"* %190 to i8*
  %200 = getelementptr inbounds i8, i8* %199, i64 %198
  %201 = bitcast i8* %200 to %"class.std::basic_ios"*
  invoke void @"?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %201, i32 4, i1 zeroext true) [ "funclet"(token %189) ]
          to label %202 unwind label %246

202:                                              ; preds = %188
  catchret from %189 to label %203

203:                                              ; preds = %202
  br label %204

204:                                              ; preds = %203, %215
  br label %216

205:                                              ; preds = %184
  store i32 %185, i32* %12, align 4
  %206 = call i32 @"?eof@?$_Narrow_char_traits@DH@std@@SAHXZ"() #5
  store i32 %206, i32* %13, align 4
  %207 = call zeroext i1 @"?eq_int_type@?$_Narrow_char_traits@DH@std@@SA_NAEBH0@Z"(i32* nonnull align 4 dereferenceable(4) %13, i32* nonnull align 4 dereferenceable(4) %12) #5
  br i1 %207, label %208, label %211

208:                                              ; preds = %205
  %209 = load i32, i32* %5, align 4
  %210 = or i32 %209, 4
  store i32 %210, i32* %5, align 4
  br label %211

211:                                              ; preds = %208, %205
  br label %212

212:                                              ; preds = %211
  %213 = load i64, i64* %7, align 8
  %214 = add nsw i64 %213, -1
  store i64 %214, i64* %7, align 8
  br label %148

215:                                              ; preds = %154
  br label %204

216:                                              ; preds = %204, %17
  %217 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %218 = bitcast %"class.std::basic_ostream"* %217 to i8*
  %219 = getelementptr inbounds i8, i8* %218, i64 0
  %220 = bitcast i8* %219 to i32**
  %221 = load i32*, i32** %220, align 8
  %222 = getelementptr inbounds i32, i32* %221, i32 1
  %223 = load i32, i32* %222, align 4
  %224 = sext i32 %223 to i64
  %225 = add nsw i64 0, %224
  %226 = bitcast %"class.std::basic_ostream"* %217 to i8*
  %227 = getelementptr inbounds i8, i8* %226, i64 %225
  %228 = bitcast i8* %227 to %"class.std::ios_base"*
  %229 = invoke i64 @"?width@ios_base@std@@QEAA_J_J@Z"(%"class.std::ios_base"* %228, i64 0)
          to label %230 unwind label %246

230:                                              ; preds = %216
  %231 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %232 = bitcast %"class.std::basic_ostream"* %231 to i8*
  %233 = getelementptr inbounds i8, i8* %232, i64 0
  %234 = bitcast i8* %233 to i32**
  %235 = load i32*, i32** %234, align 8
  %236 = getelementptr inbounds i32, i32* %235, i32 1
  %237 = load i32, i32* %236, align 4
  %238 = sext i32 %237 to i64
  %239 = add nsw i64 0, %238
  %240 = bitcast %"class.std::basic_ostream"* %231 to i8*
  %241 = getelementptr inbounds i8, i8* %240, i64 %239
  %242 = bitcast i8* %241 to %"class.std::basic_ios"*
  %243 = load i32, i32* %5, align 4
  invoke void @"?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %242, i32 %243, i1 zeroext false)
          to label %244 unwind label %246

244:                                              ; preds = %230
  %245 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  call void @"??1sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6) #5
  ret %"class.std::basic_ostream"* %245

246:                                              ; preds = %230, %216, %188, %186, %35, %18, %2
  %247 = cleanuppad within none []
  call void @"??1sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6) #5 [ "funclet"(token %247) ]
  cleanupret from %247 unwind to caller
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??$?6U?$char_traits@D@std@@@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@0@AEAV10@PEBD@Z"(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %0, i8* %1) #1 comdat personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::basic_ostream"*, align 8
  %5 = alloca i32, align 4
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca %"class.std::basic_ostream<char, std::char_traits<char>>::sentry", align 8
  %9 = alloca i32, align 4
  %10 = alloca i32, align 4
  %11 = alloca i32, align 4
  %12 = alloca i32, align 4
  store i8* %1, i8** %3, align 8
  store %"class.std::basic_ostream"* %0, %"class.std::basic_ostream"** %4, align 8
  store i32 0, i32* %5, align 4
  %13 = load i8*, i8** %3, align 8
  %14 = call i64 @"?length@?$_Narrow_char_traits@DH@std@@SA_KQEBD@Z"(i8* %13) #5
  store i64 %14, i64* %6, align 8
  %15 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %16 = bitcast %"class.std::basic_ostream"* %15 to i8*
  %17 = getelementptr inbounds i8, i8* %16, i64 0
  %18 = bitcast i8* %17 to i32**
  %19 = load i32*, i32** %18, align 8
  %20 = getelementptr inbounds i32, i32* %19, i32 1
  %21 = load i32, i32* %20, align 4
  %22 = sext i32 %21 to i64
  %23 = add nsw i64 0, %22
  %24 = bitcast %"class.std::basic_ostream"* %15 to i8*
  %25 = getelementptr inbounds i8, i8* %24, i64 %23
  %26 = bitcast i8* %25 to %"class.std::ios_base"*
  %27 = call i64 @"?width@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %26)
  %28 = icmp sle i64 %27, 0
  br i1 %28, label %45, label %29

29:                                               ; preds = %2
  %30 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %31 = bitcast %"class.std::basic_ostream"* %30 to i8*
  %32 = getelementptr inbounds i8, i8* %31, i64 0
  %33 = bitcast i8* %32 to i32**
  %34 = load i32*, i32** %33, align 8
  %35 = getelementptr inbounds i32, i32* %34, i32 1
  %36 = load i32, i32* %35, align 4
  %37 = sext i32 %36 to i64
  %38 = add nsw i64 0, %37
  %39 = bitcast %"class.std::basic_ostream"* %30 to i8*
  %40 = getelementptr inbounds i8, i8* %39, i64 %38
  %41 = bitcast i8* %40 to %"class.std::ios_base"*
  %42 = call i64 @"?width@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %41)
  %43 = load i64, i64* %6, align 8
  %44 = icmp sle i64 %42, %43
  br i1 %44, label %45, label %46

45:                                               ; preds = %29, %2
  br label %62

46:                                               ; preds = %29
  %47 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %48 = bitcast %"class.std::basic_ostream"* %47 to i8*
  %49 = getelementptr inbounds i8, i8* %48, i64 0
  %50 = bitcast i8* %49 to i32**
  %51 = load i32*, i32** %50, align 8
  %52 = getelementptr inbounds i32, i32* %51, i32 1
  %53 = load i32, i32* %52, align 4
  %54 = sext i32 %53 to i64
  %55 = add nsw i64 0, %54
  %56 = bitcast %"class.std::basic_ostream"* %47 to i8*
  %57 = getelementptr inbounds i8, i8* %56, i64 %55
  %58 = bitcast i8* %57 to %"class.std::ios_base"*
  %59 = call i64 @"?width@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %58)
  %60 = load i64, i64* %6, align 8
  %61 = sub nsw i64 %59, %60
  br label %62

62:                                               ; preds = %46, %45
  %63 = phi i64 [ 0, %45 ], [ %61, %46 ]
  store i64 %63, i64* %7, align 8
  %64 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %65 = call %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* @"??0sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@AEAV12@@Z"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %8, %"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %64)
  %66 = invoke zeroext i1 @"??Bsentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEBA_NXZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %8)
          to label %67 unwind label %258

67:                                               ; preds = %62
  br i1 %66, label %71, label %68

68:                                               ; preds = %67
  %69 = load i32, i32* %5, align 4
  %70 = or i32 %69, 4
  store i32 %70, i32* %5, align 4
  br label %242

71:                                               ; preds = %67
  %72 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %73 = bitcast %"class.std::basic_ostream"* %72 to i8*
  %74 = getelementptr inbounds i8, i8* %73, i64 0
  %75 = bitcast i8* %74 to i32**
  %76 = load i32*, i32** %75, align 8
  %77 = getelementptr inbounds i32, i32* %76, i32 1
  %78 = load i32, i32* %77, align 4
  %79 = sext i32 %78 to i64
  %80 = add nsw i64 0, %79
  %81 = bitcast %"class.std::basic_ostream"* %72 to i8*
  %82 = getelementptr inbounds i8, i8* %81, i64 %80
  %83 = bitcast i8* %82 to %"class.std::ios_base"*
  %84 = invoke i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %83)
          to label %85 unwind label %222

85:                                               ; preds = %71
  %86 = and i32 %84, 448
  %87 = icmp ne i32 %86, 64
  br i1 %87, label %88, label %133

88:                                               ; preds = %85
  br label %89

89:                                               ; preds = %129, %88
  %90 = load i64, i64* %7, align 8
  %91 = icmp slt i64 0, %90
  br i1 %91, label %92, label %132

92:                                               ; preds = %89
  %93 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %94 = bitcast %"class.std::basic_ostream"* %93 to i8*
  %95 = getelementptr inbounds i8, i8* %94, i64 0
  %96 = bitcast i8* %95 to i32**
  %97 = load i32*, i32** %96, align 8
  %98 = getelementptr inbounds i32, i32* %97, i32 1
  %99 = load i32, i32* %98, align 4
  %100 = sext i32 %99 to i64
  %101 = add nsw i64 0, %100
  %102 = bitcast %"class.std::basic_ostream"* %93 to i8*
  %103 = getelementptr inbounds i8, i8* %102, i64 %101
  %104 = bitcast i8* %103 to %"class.std::basic_ios"*
  %105 = invoke %"class.std::basic_streambuf"* @"?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %104)
          to label %106 unwind label %222

106:                                              ; preds = %92
  %107 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %108 = bitcast %"class.std::basic_ostream"* %107 to i8*
  %109 = getelementptr inbounds i8, i8* %108, i64 0
  %110 = bitcast i8* %109 to i32**
  %111 = load i32*, i32** %110, align 8
  %112 = getelementptr inbounds i32, i32* %111, i32 1
  %113 = load i32, i32* %112, align 4
  %114 = sext i32 %113 to i64
  %115 = add nsw i64 0, %114
  %116 = bitcast %"class.std::basic_ostream"* %107 to i8*
  %117 = getelementptr inbounds i8, i8* %116, i64 %115
  %118 = bitcast i8* %117 to %"class.std::basic_ios"*
  %119 = invoke i8 @"?fill@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBADXZ"(%"class.std::basic_ios"* %118)
          to label %120 unwind label %222

120:                                              ; preds = %106
  %121 = invoke i32 @"?sputc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHD@Z"(%"class.std::basic_streambuf"* %105, i8 %119)
          to label %122 unwind label %222

122:                                              ; preds = %120
  store i32 %121, i32* %9, align 4
  %123 = call i32 @"?eof@?$_Narrow_char_traits@DH@std@@SAHXZ"() #5
  store i32 %123, i32* %10, align 4
  %124 = call zeroext i1 @"?eq_int_type@?$_Narrow_char_traits@DH@std@@SA_NAEBH0@Z"(i32* nonnull align 4 dereferenceable(4) %10, i32* nonnull align 4 dereferenceable(4) %9) #5
  br i1 %124, label %125, label %128

125:                                              ; preds = %122
  %126 = load i32, i32* %5, align 4
  %127 = or i32 %126, 4
  store i32 %127, i32* %5, align 4
  br label %132

128:                                              ; preds = %122
  br label %129

129:                                              ; preds = %128
  %130 = load i64, i64* %7, align 8
  %131 = add nsw i64 %130, -1
  store i64 %131, i64* %7, align 8
  br label %89

132:                                              ; preds = %125, %89
  br label %133

133:                                              ; preds = %132, %85
  %134 = load i32, i32* %5, align 4
  %135 = icmp eq i32 %134, 0
  br i1 %135, label %136, label %160

136:                                              ; preds = %133
  %137 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %138 = bitcast %"class.std::basic_ostream"* %137 to i8*
  %139 = getelementptr inbounds i8, i8* %138, i64 0
  %140 = bitcast i8* %139 to i32**
  %141 = load i32*, i32** %140, align 8
  %142 = getelementptr inbounds i32, i32* %141, i32 1
  %143 = load i32, i32* %142, align 4
  %144 = sext i32 %143 to i64
  %145 = add nsw i64 0, %144
  %146 = bitcast %"class.std::basic_ostream"* %137 to i8*
  %147 = getelementptr inbounds i8, i8* %146, i64 %145
  %148 = bitcast i8* %147 to %"class.std::basic_ios"*
  %149 = invoke %"class.std::basic_streambuf"* @"?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %148)
          to label %150 unwind label %222

150:                                              ; preds = %136
  %151 = load i64, i64* %6, align 8
  %152 = load i8*, i8** %3, align 8
  %153 = invoke i64 @"?sputn@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAA_JPEBD_J@Z"(%"class.std::basic_streambuf"* %149, i8* %152, i64 %151)
          to label %154 unwind label %222

154:                                              ; preds = %150
  %155 = load i64, i64* %6, align 8
  %156 = icmp ne i64 %153, %155
  br i1 %156, label %157, label %160

157:                                              ; preds = %154
  %158 = load i32, i32* %5, align 4
  %159 = or i32 %158, 4
  store i32 %159, i32* %5, align 4
  br label %160

160:                                              ; preds = %157, %154, %133
  %161 = load i32, i32* %5, align 4
  %162 = icmp eq i32 %161, 0
  br i1 %162, label %163, label %208

163:                                              ; preds = %160
  br label %164

164:                                              ; preds = %204, %163
  %165 = load i64, i64* %7, align 8
  %166 = icmp slt i64 0, %165
  br i1 %166, label %167, label %207

167:                                              ; preds = %164
  %168 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %169 = bitcast %"class.std::basic_ostream"* %168 to i8*
  %170 = getelementptr inbounds i8, i8* %169, i64 0
  %171 = bitcast i8* %170 to i32**
  %172 = load i32*, i32** %171, align 8
  %173 = getelementptr inbounds i32, i32* %172, i32 1
  %174 = load i32, i32* %173, align 4
  %175 = sext i32 %174 to i64
  %176 = add nsw i64 0, %175
  %177 = bitcast %"class.std::basic_ostream"* %168 to i8*
  %178 = getelementptr inbounds i8, i8* %177, i64 %176
  %179 = bitcast i8* %178 to %"class.std::basic_ios"*
  %180 = invoke %"class.std::basic_streambuf"* @"?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %179)
          to label %181 unwind label %222

181:                                              ; preds = %167
  %182 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %183 = bitcast %"class.std::basic_ostream"* %182 to i8*
  %184 = getelementptr inbounds i8, i8* %183, i64 0
  %185 = bitcast i8* %184 to i32**
  %186 = load i32*, i32** %185, align 8
  %187 = getelementptr inbounds i32, i32* %186, i32 1
  %188 = load i32, i32* %187, align 4
  %189 = sext i32 %188 to i64
  %190 = add nsw i64 0, %189
  %191 = bitcast %"class.std::basic_ostream"* %182 to i8*
  %192 = getelementptr inbounds i8, i8* %191, i64 %190
  %193 = bitcast i8* %192 to %"class.std::basic_ios"*
  %194 = invoke i8 @"?fill@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBADXZ"(%"class.std::basic_ios"* %193)
          to label %195 unwind label %222

195:                                              ; preds = %181
  %196 = invoke i32 @"?sputc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHD@Z"(%"class.std::basic_streambuf"* %180, i8 %194)
          to label %197 unwind label %222

197:                                              ; preds = %195
  store i32 %196, i32* %11, align 4
  %198 = call i32 @"?eof@?$_Narrow_char_traits@DH@std@@SAHXZ"() #5
  store i32 %198, i32* %12, align 4
  %199 = call zeroext i1 @"?eq_int_type@?$_Narrow_char_traits@DH@std@@SA_NAEBH0@Z"(i32* nonnull align 4 dereferenceable(4) %12, i32* nonnull align 4 dereferenceable(4) %11) #5
  br i1 %199, label %200, label %203

200:                                              ; preds = %197
  %201 = load i32, i32* %5, align 4
  %202 = or i32 %201, 4
  store i32 %202, i32* %5, align 4
  br label %207

203:                                              ; preds = %197
  br label %204

204:                                              ; preds = %203
  %205 = load i64, i64* %7, align 8
  %206 = add nsw i64 %205, -1
  store i64 %206, i64* %7, align 8
  br label %164

207:                                              ; preds = %200, %164
  br label %208

208:                                              ; preds = %207, %160
  %209 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %210 = bitcast %"class.std::basic_ostream"* %209 to i8*
  %211 = getelementptr inbounds i8, i8* %210, i64 0
  %212 = bitcast i8* %211 to i32**
  %213 = load i32*, i32** %212, align 8
  %214 = getelementptr inbounds i32, i32* %213, i32 1
  %215 = load i32, i32* %214, align 4
  %216 = sext i32 %215 to i64
  %217 = add nsw i64 0, %216
  %218 = bitcast %"class.std::basic_ostream"* %209 to i8*
  %219 = getelementptr inbounds i8, i8* %218, i64 %217
  %220 = bitcast i8* %219 to %"class.std::ios_base"*
  %221 = invoke i64 @"?width@ios_base@std@@QEAA_J_J@Z"(%"class.std::ios_base"* %220, i64 0)
          to label %241 unwind label %222

222:                                              ; preds = %208, %195, %181, %167, %150, %136, %120, %106, %92, %71
  %223 = catchswitch within none [label %224] unwind label %258

224:                                              ; preds = %222
  %225 = catchpad within %223 [i8* null, i32 64, i8* null]
  %226 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %227 = bitcast %"class.std::basic_ostream"* %226 to i8*
  %228 = getelementptr inbounds i8, i8* %227, i64 0
  %229 = bitcast i8* %228 to i32**
  %230 = load i32*, i32** %229, align 8
  %231 = getelementptr inbounds i32, i32* %230, i32 1
  %232 = load i32, i32* %231, align 4
  %233 = sext i32 %232 to i64
  %234 = add nsw i64 0, %233
  %235 = bitcast %"class.std::basic_ostream"* %226 to i8*
  %236 = getelementptr inbounds i8, i8* %235, i64 %234
  %237 = bitcast i8* %236 to %"class.std::basic_ios"*
  invoke void @"?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %237, i32 4, i1 zeroext true) [ "funclet"(token %225) ]
          to label %238 unwind label %258

238:                                              ; preds = %224
  catchret from %225 to label %239

239:                                              ; preds = %238
  br label %240

240:                                              ; preds = %239, %241
  br label %242

241:                                              ; preds = %208
  br label %240

242:                                              ; preds = %240, %68
  %243 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %244 = bitcast %"class.std::basic_ostream"* %243 to i8*
  %245 = getelementptr inbounds i8, i8* %244, i64 0
  %246 = bitcast i8* %245 to i32**
  %247 = load i32*, i32** %246, align 8
  %248 = getelementptr inbounds i32, i32* %247, i32 1
  %249 = load i32, i32* %248, align 4
  %250 = sext i32 %249 to i64
  %251 = add nsw i64 0, %250
  %252 = bitcast %"class.std::basic_ostream"* %243 to i8*
  %253 = getelementptr inbounds i8, i8* %252, i64 %251
  %254 = bitcast i8* %253 to %"class.std::basic_ios"*
  %255 = load i32, i32* %5, align 4
  invoke void @"?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %254, i32 %255, i1 zeroext false)
          to label %256 unwind label %258

256:                                              ; preds = %242
  %257 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  call void @"??1sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %8) #5
  ret %"class.std::basic_ostream"* %257

258:                                              ; preds = %242, %224, %222, %62
  %259 = cleanuppad within none []
  call void @"??1sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %8) #5 [ "funclet"(token %259) ]
  cleanupret from %259 unwind to caller
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@_K@Z"(%"class.std::basic_ostream"* %0, i64 %1) #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %3 = alloca i64, align 8
  %4 = alloca %"class.std::basic_ostream"*, align 8
  %5 = alloca i32, align 4
  %6 = alloca %"class.std::basic_ostream<char, std::char_traits<char>>::sentry", align 8
  %7 = alloca %"class.std::num_put"*, align 8
  %8 = alloca %"class.std::locale", align 8
  %9 = alloca %"class.std::ostreambuf_iterator", align 8
  %10 = alloca %"class.std::ostreambuf_iterator", align 8
  store i64 %1, i64* %3, align 8
  store %"class.std::basic_ostream"* %0, %"class.std::basic_ostream"** %4, align 8
  %11 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  store i32 0, i32* %5, align 4
  %12 = call %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* @"??0sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@AEAV12@@Z"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6, %"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %11)
  %13 = invoke zeroext i1 @"??Bsentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEBA_NXZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6)
          to label %14 unwind label %110

14:                                               ; preds = %2
  br i1 %13, label %15, label %96

15:                                               ; preds = %14
  %16 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %17 = getelementptr inbounds i8, i8* %16, i64 0
  %18 = bitcast i8* %17 to i32**
  %19 = load i32*, i32** %18, align 8
  %20 = getelementptr inbounds i32, i32* %19, i32 1
  %21 = load i32, i32* %20, align 4
  %22 = sext i32 %21 to i64
  %23 = add nsw i64 0, %22
  %24 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %25 = getelementptr inbounds i8, i8* %24, i64 %23
  %26 = bitcast i8* %25 to %"class.std::ios_base"*
  invoke void @"?getloc@ios_base@std@@QEBA?AVlocale@2@XZ"(%"class.std::ios_base"* %26, %"class.std::locale"* sret align 8 %8)
          to label %27 unwind label %110

27:                                               ; preds = %15
  %28 = invoke nonnull align 8 dereferenceable(16) %"class.std::num_put"* @"??$use_facet@V?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@@std@@YAAEBV?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@0@AEBVlocale@0@@Z"(%"class.std::locale"* nonnull align 8 dereferenceable(16) %8)
          to label %29 unwind label %93

29:                                               ; preds = %27
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %8) #5
  store %"class.std::num_put"* %28, %"class.std::num_put"** %7, align 8
  %30 = load %"class.std::num_put"*, %"class.std::num_put"** %7, align 8
  %31 = load i64, i64* %3, align 8
  %32 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %33 = getelementptr inbounds i8, i8* %32, i64 0
  %34 = bitcast i8* %33 to i32**
  %35 = load i32*, i32** %34, align 8
  %36 = getelementptr inbounds i32, i32* %35, i32 1
  %37 = load i32, i32* %36, align 4
  %38 = sext i32 %37 to i64
  %39 = add nsw i64 0, %38
  %40 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %41 = getelementptr inbounds i8, i8* %40, i64 %39
  %42 = bitcast i8* %41 to %"class.std::basic_ios"*
  %43 = invoke i8 @"?fill@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBADXZ"(%"class.std::basic_ios"* %42)
          to label %44 unwind label %70

44:                                               ; preds = %29
  %45 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %46 = getelementptr inbounds i8, i8* %45, i64 0
  %47 = bitcast i8* %46 to i32**
  %48 = load i32*, i32** %47, align 8
  %49 = getelementptr inbounds i32, i32* %48, i32 1
  %50 = load i32, i32* %49, align 4
  %51 = sext i32 %50 to i64
  %52 = add nsw i64 0, %51
  %53 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %54 = getelementptr inbounds i8, i8* %53, i64 %52
  %55 = bitcast i8* %54 to %"class.std::ios_base"*
  %56 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %57 = getelementptr inbounds i8, i8* %56, i64 0
  %58 = bitcast i8* %57 to i32**
  %59 = load i32*, i32** %58, align 8
  %60 = getelementptr inbounds i32, i32* %59, i32 1
  %61 = load i32, i32* %60, align 4
  %62 = sext i32 %61 to i64
  %63 = add nsw i64 0, %62
  %64 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %65 = getelementptr inbounds i8, i8* %64, i64 %63
  %66 = bitcast i8* %65 to %"class.std::basic_ios"*
  %67 = invoke %"class.std::basic_streambuf"* @"?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %66)
          to label %68 unwind label %70

68:                                               ; preds = %44
  %69 = call %"class.std::ostreambuf_iterator"* @"??0?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@@Z"(%"class.std::ostreambuf_iterator"* %10, %"class.std::basic_streambuf"* %67) #5
  invoke void @"?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@D_K@Z"(%"class.std::num_put"* %30, %"class.std::ostreambuf_iterator"* sret align 8 %9, %"class.std::ostreambuf_iterator"* %10, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %55, i8 %43, i64 %31)
          to label %88 unwind label %70

70:                                               ; preds = %68, %44, %29
  %71 = catchswitch within none [label %72] unwind label %110

72:                                               ; preds = %70
  %73 = catchpad within %71 [i8* null, i32 64, i8* null]
  %74 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %75 = getelementptr inbounds i8, i8* %74, i64 0
  %76 = bitcast i8* %75 to i32**
  %77 = load i32*, i32** %76, align 8
  %78 = getelementptr inbounds i32, i32* %77, i32 1
  %79 = load i32, i32* %78, align 4
  %80 = sext i32 %79 to i64
  %81 = add nsw i64 0, %80
  %82 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %83 = getelementptr inbounds i8, i8* %82, i64 %81
  %84 = bitcast i8* %83 to %"class.std::basic_ios"*
  invoke void @"?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %84, i32 4, i1 zeroext true) [ "funclet"(token %73) ]
          to label %85 unwind label %110

85:                                               ; preds = %72
  catchret from %73 to label %86

86:                                               ; preds = %85
  br label %87

87:                                               ; preds = %86, %95
  br label %96

88:                                               ; preds = %68
  %89 = call zeroext i1 @"?failed@?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEBA_NXZ"(%"class.std::ostreambuf_iterator"* %9) #5
  br i1 %89, label %90, label %95

90:                                               ; preds = %88
  %91 = load i32, i32* %5, align 4
  %92 = or i32 %91, 4
  store i32 %92, i32* %5, align 4
  br label %95

93:                                               ; preds = %27
  %94 = cleanuppad within none []
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %8) #5 [ "funclet"(token %94) ]
  cleanupret from %94 unwind label %110

95:                                               ; preds = %90, %88
  br label %87

96:                                               ; preds = %87, %14
  %97 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %98 = getelementptr inbounds i8, i8* %97, i64 0
  %99 = bitcast i8* %98 to i32**
  %100 = load i32*, i32** %99, align 8
  %101 = getelementptr inbounds i32, i32* %100, i32 1
  %102 = load i32, i32* %101, align 4
  %103 = sext i32 %102 to i64
  %104 = add nsw i64 0, %103
  %105 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %106 = getelementptr inbounds i8, i8* %105, i64 %104
  %107 = bitcast i8* %106 to %"class.std::basic_ios"*
  %108 = load i32, i32* %5, align 4
  invoke void @"?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %107, i32 %108, i1 zeroext false)
          to label %109 unwind label %110

109:                                              ; preds = %96
  call void @"??1sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6) #5
  ret %"class.std::basic_ostream"* %11

110:                                              ; preds = %96, %72, %70, %93, %15, %2
  %111 = cleanuppad within none []
  call void @"??1sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6) #5 [ "funclet"(token %111) ]
  cleanupret from %111 unwind to caller
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@J@Z"(%"class.std::basic_ostream"* %0, i32 %1) #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %3 = alloca i32, align 4
  %4 = alloca %"class.std::basic_ostream"*, align 8
  %5 = alloca i32, align 4
  %6 = alloca %"class.std::basic_ostream<char, std::char_traits<char>>::sentry", align 8
  %7 = alloca %"class.std::num_put"*, align 8
  %8 = alloca %"class.std::locale", align 8
  %9 = alloca %"class.std::ostreambuf_iterator", align 8
  %10 = alloca %"class.std::ostreambuf_iterator", align 8
  store i32 %1, i32* %3, align 4
  store %"class.std::basic_ostream"* %0, %"class.std::basic_ostream"** %4, align 8
  %11 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  store i32 0, i32* %5, align 4
  %12 = call %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* @"??0sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@AEAV12@@Z"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6, %"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %11)
  %13 = invoke zeroext i1 @"??Bsentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEBA_NXZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6)
          to label %14 unwind label %110

14:                                               ; preds = %2
  br i1 %13, label %15, label %96

15:                                               ; preds = %14
  %16 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %17 = getelementptr inbounds i8, i8* %16, i64 0
  %18 = bitcast i8* %17 to i32**
  %19 = load i32*, i32** %18, align 8
  %20 = getelementptr inbounds i32, i32* %19, i32 1
  %21 = load i32, i32* %20, align 4
  %22 = sext i32 %21 to i64
  %23 = add nsw i64 0, %22
  %24 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %25 = getelementptr inbounds i8, i8* %24, i64 %23
  %26 = bitcast i8* %25 to %"class.std::ios_base"*
  invoke void @"?getloc@ios_base@std@@QEBA?AVlocale@2@XZ"(%"class.std::ios_base"* %26, %"class.std::locale"* sret align 8 %8)
          to label %27 unwind label %110

27:                                               ; preds = %15
  %28 = invoke nonnull align 8 dereferenceable(16) %"class.std::num_put"* @"??$use_facet@V?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@@std@@YAAEBV?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@0@AEBVlocale@0@@Z"(%"class.std::locale"* nonnull align 8 dereferenceable(16) %8)
          to label %29 unwind label %93

29:                                               ; preds = %27
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %8) #5
  store %"class.std::num_put"* %28, %"class.std::num_put"** %7, align 8
  %30 = load %"class.std::num_put"*, %"class.std::num_put"** %7, align 8
  %31 = load i32, i32* %3, align 4
  %32 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %33 = getelementptr inbounds i8, i8* %32, i64 0
  %34 = bitcast i8* %33 to i32**
  %35 = load i32*, i32** %34, align 8
  %36 = getelementptr inbounds i32, i32* %35, i32 1
  %37 = load i32, i32* %36, align 4
  %38 = sext i32 %37 to i64
  %39 = add nsw i64 0, %38
  %40 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %41 = getelementptr inbounds i8, i8* %40, i64 %39
  %42 = bitcast i8* %41 to %"class.std::basic_ios"*
  %43 = invoke i8 @"?fill@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBADXZ"(%"class.std::basic_ios"* %42)
          to label %44 unwind label %70

44:                                               ; preds = %29
  %45 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %46 = getelementptr inbounds i8, i8* %45, i64 0
  %47 = bitcast i8* %46 to i32**
  %48 = load i32*, i32** %47, align 8
  %49 = getelementptr inbounds i32, i32* %48, i32 1
  %50 = load i32, i32* %49, align 4
  %51 = sext i32 %50 to i64
  %52 = add nsw i64 0, %51
  %53 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %54 = getelementptr inbounds i8, i8* %53, i64 %52
  %55 = bitcast i8* %54 to %"class.std::ios_base"*
  %56 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %57 = getelementptr inbounds i8, i8* %56, i64 0
  %58 = bitcast i8* %57 to i32**
  %59 = load i32*, i32** %58, align 8
  %60 = getelementptr inbounds i32, i32* %59, i32 1
  %61 = load i32, i32* %60, align 4
  %62 = sext i32 %61 to i64
  %63 = add nsw i64 0, %62
  %64 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %65 = getelementptr inbounds i8, i8* %64, i64 %63
  %66 = bitcast i8* %65 to %"class.std::basic_ios"*
  %67 = invoke %"class.std::basic_streambuf"* @"?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %66)
          to label %68 unwind label %70

68:                                               ; preds = %44
  %69 = call %"class.std::ostreambuf_iterator"* @"??0?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@@Z"(%"class.std::ostreambuf_iterator"* %10, %"class.std::basic_streambuf"* %67) #5
  invoke void @"?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DJ@Z"(%"class.std::num_put"* %30, %"class.std::ostreambuf_iterator"* sret align 8 %9, %"class.std::ostreambuf_iterator"* %10, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %55, i8 %43, i32 %31)
          to label %88 unwind label %70

70:                                               ; preds = %68, %44, %29
  %71 = catchswitch within none [label %72] unwind label %110

72:                                               ; preds = %70
  %73 = catchpad within %71 [i8* null, i32 64, i8* null]
  %74 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %75 = getelementptr inbounds i8, i8* %74, i64 0
  %76 = bitcast i8* %75 to i32**
  %77 = load i32*, i32** %76, align 8
  %78 = getelementptr inbounds i32, i32* %77, i32 1
  %79 = load i32, i32* %78, align 4
  %80 = sext i32 %79 to i64
  %81 = add nsw i64 0, %80
  %82 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %83 = getelementptr inbounds i8, i8* %82, i64 %81
  %84 = bitcast i8* %83 to %"class.std::basic_ios"*
  invoke void @"?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %84, i32 4, i1 zeroext true) [ "funclet"(token %73) ]
          to label %85 unwind label %110

85:                                               ; preds = %72
  catchret from %73 to label %86

86:                                               ; preds = %85
  br label %87

87:                                               ; preds = %86, %95
  br label %96

88:                                               ; preds = %68
  %89 = call zeroext i1 @"?failed@?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEBA_NXZ"(%"class.std::ostreambuf_iterator"* %9) #5
  br i1 %89, label %90, label %95

90:                                               ; preds = %88
  %91 = load i32, i32* %5, align 4
  %92 = or i32 %91, 4
  store i32 %92, i32* %5, align 4
  br label %95

93:                                               ; preds = %27
  %94 = cleanuppad within none []
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %8) #5 [ "funclet"(token %94) ]
  cleanupret from %94 unwind label %110

95:                                               ; preds = %90, %88
  br label %87

96:                                               ; preds = %87, %14
  %97 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %98 = getelementptr inbounds i8, i8* %97, i64 0
  %99 = bitcast i8* %98 to i32**
  %100 = load i32*, i32** %99, align 8
  %101 = getelementptr inbounds i32, i32* %100, i32 1
  %102 = load i32, i32* %101, align 4
  %103 = sext i32 %102 to i64
  %104 = add nsw i64 0, %103
  %105 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %106 = getelementptr inbounds i8, i8* %105, i64 %104
  %107 = bitcast i8* %106 to %"class.std::basic_ios"*
  %108 = load i32, i32* %5, align 4
  invoke void @"?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %107, i32 %108, i1 zeroext false)
          to label %109 unwind label %110

109:                                              ; preds = %96
  call void @"??1sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6) #5
  ret %"class.std::basic_ostream"* %11

110:                                              ; preds = %96, %72, %70, %93, %15, %2
  %111 = cleanuppad within none []
  call void @"??1sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6) #5 [ "funclet"(token %111) ]
  cleanupret from %111 unwind to caller
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@_J@Z"(%"class.std::basic_ostream"* %0, i64 %1) #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %3 = alloca i64, align 8
  %4 = alloca %"class.std::basic_ostream"*, align 8
  %5 = alloca i32, align 4
  %6 = alloca %"class.std::basic_ostream<char, std::char_traits<char>>::sentry", align 8
  %7 = alloca %"class.std::num_put"*, align 8
  %8 = alloca %"class.std::locale", align 8
  %9 = alloca %"class.std::ostreambuf_iterator", align 8
  %10 = alloca %"class.std::ostreambuf_iterator", align 8
  store i64 %1, i64* %3, align 8
  store %"class.std::basic_ostream"* %0, %"class.std::basic_ostream"** %4, align 8
  %11 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  store i32 0, i32* %5, align 4
  %12 = call %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* @"??0sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@AEAV12@@Z"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6, %"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %11)
  %13 = invoke zeroext i1 @"??Bsentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEBA_NXZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6)
          to label %14 unwind label %110

14:                                               ; preds = %2
  br i1 %13, label %15, label %96

15:                                               ; preds = %14
  %16 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %17 = getelementptr inbounds i8, i8* %16, i64 0
  %18 = bitcast i8* %17 to i32**
  %19 = load i32*, i32** %18, align 8
  %20 = getelementptr inbounds i32, i32* %19, i32 1
  %21 = load i32, i32* %20, align 4
  %22 = sext i32 %21 to i64
  %23 = add nsw i64 0, %22
  %24 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %25 = getelementptr inbounds i8, i8* %24, i64 %23
  %26 = bitcast i8* %25 to %"class.std::ios_base"*
  invoke void @"?getloc@ios_base@std@@QEBA?AVlocale@2@XZ"(%"class.std::ios_base"* %26, %"class.std::locale"* sret align 8 %8)
          to label %27 unwind label %110

27:                                               ; preds = %15
  %28 = invoke nonnull align 8 dereferenceable(16) %"class.std::num_put"* @"??$use_facet@V?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@@std@@YAAEBV?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@0@AEBVlocale@0@@Z"(%"class.std::locale"* nonnull align 8 dereferenceable(16) %8)
          to label %29 unwind label %93

29:                                               ; preds = %27
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %8) #5
  store %"class.std::num_put"* %28, %"class.std::num_put"** %7, align 8
  %30 = load %"class.std::num_put"*, %"class.std::num_put"** %7, align 8
  %31 = load i64, i64* %3, align 8
  %32 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %33 = getelementptr inbounds i8, i8* %32, i64 0
  %34 = bitcast i8* %33 to i32**
  %35 = load i32*, i32** %34, align 8
  %36 = getelementptr inbounds i32, i32* %35, i32 1
  %37 = load i32, i32* %36, align 4
  %38 = sext i32 %37 to i64
  %39 = add nsw i64 0, %38
  %40 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %41 = getelementptr inbounds i8, i8* %40, i64 %39
  %42 = bitcast i8* %41 to %"class.std::basic_ios"*
  %43 = invoke i8 @"?fill@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBADXZ"(%"class.std::basic_ios"* %42)
          to label %44 unwind label %70

44:                                               ; preds = %29
  %45 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %46 = getelementptr inbounds i8, i8* %45, i64 0
  %47 = bitcast i8* %46 to i32**
  %48 = load i32*, i32** %47, align 8
  %49 = getelementptr inbounds i32, i32* %48, i32 1
  %50 = load i32, i32* %49, align 4
  %51 = sext i32 %50 to i64
  %52 = add nsw i64 0, %51
  %53 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %54 = getelementptr inbounds i8, i8* %53, i64 %52
  %55 = bitcast i8* %54 to %"class.std::ios_base"*
  %56 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %57 = getelementptr inbounds i8, i8* %56, i64 0
  %58 = bitcast i8* %57 to i32**
  %59 = load i32*, i32** %58, align 8
  %60 = getelementptr inbounds i32, i32* %59, i32 1
  %61 = load i32, i32* %60, align 4
  %62 = sext i32 %61 to i64
  %63 = add nsw i64 0, %62
  %64 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %65 = getelementptr inbounds i8, i8* %64, i64 %63
  %66 = bitcast i8* %65 to %"class.std::basic_ios"*
  %67 = invoke %"class.std::basic_streambuf"* @"?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %66)
          to label %68 unwind label %70

68:                                               ; preds = %44
  %69 = call %"class.std::ostreambuf_iterator"* @"??0?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@@Z"(%"class.std::ostreambuf_iterator"* %10, %"class.std::basic_streambuf"* %67) #5
  invoke void @"?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@D_J@Z"(%"class.std::num_put"* %30, %"class.std::ostreambuf_iterator"* sret align 8 %9, %"class.std::ostreambuf_iterator"* %10, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %55, i8 %43, i64 %31)
          to label %88 unwind label %70

70:                                               ; preds = %68, %44, %29
  %71 = catchswitch within none [label %72] unwind label %110

72:                                               ; preds = %70
  %73 = catchpad within %71 [i8* null, i32 64, i8* null]
  %74 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %75 = getelementptr inbounds i8, i8* %74, i64 0
  %76 = bitcast i8* %75 to i32**
  %77 = load i32*, i32** %76, align 8
  %78 = getelementptr inbounds i32, i32* %77, i32 1
  %79 = load i32, i32* %78, align 4
  %80 = sext i32 %79 to i64
  %81 = add nsw i64 0, %80
  %82 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %83 = getelementptr inbounds i8, i8* %82, i64 %81
  %84 = bitcast i8* %83 to %"class.std::basic_ios"*
  invoke void @"?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %84, i32 4, i1 zeroext true) [ "funclet"(token %73) ]
          to label %85 unwind label %110

85:                                               ; preds = %72
  catchret from %73 to label %86

86:                                               ; preds = %85
  br label %87

87:                                               ; preds = %86, %95
  br label %96

88:                                               ; preds = %68
  %89 = call zeroext i1 @"?failed@?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEBA_NXZ"(%"class.std::ostreambuf_iterator"* %9) #5
  br i1 %89, label %90, label %95

90:                                               ; preds = %88
  %91 = load i32, i32* %5, align 4
  %92 = or i32 %91, 4
  store i32 %92, i32* %5, align 4
  br label %95

93:                                               ; preds = %27
  %94 = cleanuppad within none []
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %8) #5 [ "funclet"(token %94) ]
  cleanupret from %94 unwind label %110

95:                                               ; preds = %90, %88
  br label %87

96:                                               ; preds = %87, %14
  %97 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %98 = getelementptr inbounds i8, i8* %97, i64 0
  %99 = bitcast i8* %98 to i32**
  %100 = load i32*, i32** %99, align 8
  %101 = getelementptr inbounds i32, i32* %100, i32 1
  %102 = load i32, i32* %101, align 4
  %103 = sext i32 %102 to i64
  %104 = add nsw i64 0, %103
  %105 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %106 = getelementptr inbounds i8, i8* %105, i64 %104
  %107 = bitcast i8* %106 to %"class.std::basic_ios"*
  %108 = load i32, i32* %5, align 4
  invoke void @"?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %107, i32 %108, i1 zeroext false)
          to label %109 unwind label %110

109:                                              ; preds = %96
  call void @"??1sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6) #5
  ret %"class.std::basic_ostream"* %11

110:                                              ; preds = %96, %72, %70, %93, %15, %2
  %111 = cleanuppad within none []
  call void @"??1sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6) #5 [ "funclet"(token %111) ]
  cleanupret from %111 unwind to caller
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@O@Z"(%"class.std::basic_ostream"* %0, double %1) #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %3 = alloca double, align 8
  %4 = alloca %"class.std::basic_ostream"*, align 8
  %5 = alloca i32, align 4
  %6 = alloca %"class.std::basic_ostream<char, std::char_traits<char>>::sentry", align 8
  %7 = alloca %"class.std::num_put"*, align 8
  %8 = alloca %"class.std::locale", align 8
  %9 = alloca %"class.std::ostreambuf_iterator", align 8
  %10 = alloca %"class.std::ostreambuf_iterator", align 8
  store double %1, double* %3, align 8
  store %"class.std::basic_ostream"* %0, %"class.std::basic_ostream"** %4, align 8
  %11 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  store i32 0, i32* %5, align 4
  %12 = call %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* @"??0sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@AEAV12@@Z"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6, %"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %11)
  %13 = invoke zeroext i1 @"??Bsentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEBA_NXZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6)
          to label %14 unwind label %110

14:                                               ; preds = %2
  br i1 %13, label %15, label %96

15:                                               ; preds = %14
  %16 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %17 = getelementptr inbounds i8, i8* %16, i64 0
  %18 = bitcast i8* %17 to i32**
  %19 = load i32*, i32** %18, align 8
  %20 = getelementptr inbounds i32, i32* %19, i32 1
  %21 = load i32, i32* %20, align 4
  %22 = sext i32 %21 to i64
  %23 = add nsw i64 0, %22
  %24 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %25 = getelementptr inbounds i8, i8* %24, i64 %23
  %26 = bitcast i8* %25 to %"class.std::ios_base"*
  invoke void @"?getloc@ios_base@std@@QEBA?AVlocale@2@XZ"(%"class.std::ios_base"* %26, %"class.std::locale"* sret align 8 %8)
          to label %27 unwind label %110

27:                                               ; preds = %15
  %28 = invoke nonnull align 8 dereferenceable(16) %"class.std::num_put"* @"??$use_facet@V?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@@std@@YAAEBV?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@0@AEBVlocale@0@@Z"(%"class.std::locale"* nonnull align 8 dereferenceable(16) %8)
          to label %29 unwind label %93

29:                                               ; preds = %27
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %8) #5
  store %"class.std::num_put"* %28, %"class.std::num_put"** %7, align 8
  %30 = load %"class.std::num_put"*, %"class.std::num_put"** %7, align 8
  %31 = load double, double* %3, align 8
  %32 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %33 = getelementptr inbounds i8, i8* %32, i64 0
  %34 = bitcast i8* %33 to i32**
  %35 = load i32*, i32** %34, align 8
  %36 = getelementptr inbounds i32, i32* %35, i32 1
  %37 = load i32, i32* %36, align 4
  %38 = sext i32 %37 to i64
  %39 = add nsw i64 0, %38
  %40 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %41 = getelementptr inbounds i8, i8* %40, i64 %39
  %42 = bitcast i8* %41 to %"class.std::basic_ios"*
  %43 = invoke i8 @"?fill@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBADXZ"(%"class.std::basic_ios"* %42)
          to label %44 unwind label %70

44:                                               ; preds = %29
  %45 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %46 = getelementptr inbounds i8, i8* %45, i64 0
  %47 = bitcast i8* %46 to i32**
  %48 = load i32*, i32** %47, align 8
  %49 = getelementptr inbounds i32, i32* %48, i32 1
  %50 = load i32, i32* %49, align 4
  %51 = sext i32 %50 to i64
  %52 = add nsw i64 0, %51
  %53 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %54 = getelementptr inbounds i8, i8* %53, i64 %52
  %55 = bitcast i8* %54 to %"class.std::ios_base"*
  %56 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %57 = getelementptr inbounds i8, i8* %56, i64 0
  %58 = bitcast i8* %57 to i32**
  %59 = load i32*, i32** %58, align 8
  %60 = getelementptr inbounds i32, i32* %59, i32 1
  %61 = load i32, i32* %60, align 4
  %62 = sext i32 %61 to i64
  %63 = add nsw i64 0, %62
  %64 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %65 = getelementptr inbounds i8, i8* %64, i64 %63
  %66 = bitcast i8* %65 to %"class.std::basic_ios"*
  %67 = invoke %"class.std::basic_streambuf"* @"?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %66)
          to label %68 unwind label %70

68:                                               ; preds = %44
  %69 = call %"class.std::ostreambuf_iterator"* @"??0?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@@Z"(%"class.std::ostreambuf_iterator"* %10, %"class.std::basic_streambuf"* %67) #5
  invoke void @"?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DO@Z"(%"class.std::num_put"* %30, %"class.std::ostreambuf_iterator"* sret align 8 %9, %"class.std::ostreambuf_iterator"* %10, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %55, i8 %43, double %31)
          to label %88 unwind label %70

70:                                               ; preds = %68, %44, %29
  %71 = catchswitch within none [label %72] unwind label %110

72:                                               ; preds = %70
  %73 = catchpad within %71 [i8* null, i32 64, i8* null]
  %74 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %75 = getelementptr inbounds i8, i8* %74, i64 0
  %76 = bitcast i8* %75 to i32**
  %77 = load i32*, i32** %76, align 8
  %78 = getelementptr inbounds i32, i32* %77, i32 1
  %79 = load i32, i32* %78, align 4
  %80 = sext i32 %79 to i64
  %81 = add nsw i64 0, %80
  %82 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %83 = getelementptr inbounds i8, i8* %82, i64 %81
  %84 = bitcast i8* %83 to %"class.std::basic_ios"*
  invoke void @"?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %84, i32 4, i1 zeroext true) [ "funclet"(token %73) ]
          to label %85 unwind label %110

85:                                               ; preds = %72
  catchret from %73 to label %86

86:                                               ; preds = %85
  br label %87

87:                                               ; preds = %86, %95
  br label %96

88:                                               ; preds = %68
  %89 = call zeroext i1 @"?failed@?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEBA_NXZ"(%"class.std::ostreambuf_iterator"* %9) #5
  br i1 %89, label %90, label %95

90:                                               ; preds = %88
  %91 = load i32, i32* %5, align 4
  %92 = or i32 %91, 4
  store i32 %92, i32* %5, align 4
  br label %95

93:                                               ; preds = %27
  %94 = cleanuppad within none []
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %8) #5 [ "funclet"(token %94) ]
  cleanupret from %94 unwind label %110

95:                                               ; preds = %90, %88
  br label %87

96:                                               ; preds = %87, %14
  %97 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %98 = getelementptr inbounds i8, i8* %97, i64 0
  %99 = bitcast i8* %98 to i32**
  %100 = load i32*, i32** %99, align 8
  %101 = getelementptr inbounds i32, i32* %100, i32 1
  %102 = load i32, i32* %101, align 4
  %103 = sext i32 %102 to i64
  %104 = add nsw i64 0, %103
  %105 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %106 = getelementptr inbounds i8, i8* %105, i64 %104
  %107 = bitcast i8* %106 to %"class.std::basic_ios"*
  %108 = load i32, i32* %5, align 4
  invoke void @"?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %107, i32 %108, i1 zeroext false)
          to label %109 unwind label %110

109:                                              ; preds = %96
  call void @"??1sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6) #5
  ret %"class.std::basic_ostream"* %11

110:                                              ; preds = %96, %72, %70, %93, %15, %2
  %111 = cleanuppad within none []
  call void @"??1sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %6) #5 [ "funclet"(token %111) ]
  cleanupret from %111 unwind to caller
}

; Function Attrs: noinline uwtable
define linkonce_odr dso_local void @"??__E?id@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@2V0locale@2@A@@YAXXZ"() #2 comdat {
  %1 = call %"class.std::locale::id"* @"??0id@locale@std@@QEAA@_K@Z"(%"class.std::locale::id"* @"?id@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@2V0locale@2@A", i64 0)
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::locale::id"* @"??0id@locale@std@@QEAA@_K@Z"(%"class.std::locale::id"* returned %0, i64 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i64, align 8
  %4 = alloca %"class.std::locale::id"*, align 8
  store i64 %1, i64* %3, align 8
  store %"class.std::locale::id"* %0, %"class.std::locale::id"** %4, align 8
  %5 = load %"class.std::locale::id"*, %"class.std::locale::id"** %4, align 8
  %6 = getelementptr inbounds %"class.std::locale::id", %"class.std::locale::id"* %5, i32 0, i32 0
  %7 = load i64, i64* %3, align 8
  store i64 %7, i64* %6, align 8
  ret %"class.std::locale::id"* %5
}

; Function Attrs: noinline uwtable
define linkonce_odr dso_local void @"??__E?id@?$numpunct@D@std@@2V0locale@2@A@@YAXXZ"() #2 comdat {
  %1 = call %"class.std::locale::id"* @"??0id@locale@std@@QEAA@_K@Z"(%"class.std::locale::id"* @"?id@?$numpunct@D@std@@2V0locale@2@A", i64 0)
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i64 @"?length@?$_Narrow_char_traits@DH@std@@SA_KQEBD@Z"(i8* %0) #3 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %2 = alloca i8*, align 8
  store i8* %0, i8** %2, align 8
  %3 = load i8*, i8** %2, align 8
  %4 = invoke i64 @strlen(i8* %3)
          to label %5 unwind label %6

5:                                                ; preds = %1
  ret i64 %4

6:                                                ; preds = %1
  %7 = cleanuppad within none []
  call void @__std_terminate() #18 [ "funclet"(token %7) ]
  unreachable
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i64 @"?width@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::ios_base"*, align 8
  store %"class.std::ios_base"* %0, %"class.std::ios_base"** %2, align 8
  %3 = load %"class.std::ios_base"*, %"class.std::ios_base"** %2, align 8
  %4 = getelementptr inbounds %"class.std::ios_base", %"class.std::ios_base"* %3, i32 0, i32 6
  %5 = load i64, i64* %4, align 8
  ret i64 %5
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* @"??0sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@AEAV12@@Z"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* returned %0, %"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %1) unnamed_addr #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %3 = alloca %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"*, align 8
  %4 = alloca %"class.std::basic_ostream"*, align 8
  %5 = alloca %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"*, align 8
  %6 = alloca %"class.std::basic_ostream"*, align 8
  store %"class.std::basic_ostream"* %1, %"class.std::basic_ostream"** %4, align 8
  store %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %0, %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"** %5, align 8
  %7 = load %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"*, %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"** %5, align 8
  store %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %7, %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"** %3, align 8
  %8 = bitcast %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %7 to %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"*
  %9 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %10 = call %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"* @"??0_Sentry_base@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@AEAV12@@Z"(%"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"* %8, %"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %9)
  %11 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %12 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %13 = getelementptr inbounds i8, i8* %12, i64 0
  %14 = bitcast i8* %13 to i32**
  %15 = load i32*, i32** %14, align 8
  %16 = getelementptr inbounds i32, i32* %15, i32 1
  %17 = load i32, i32* %16, align 4
  %18 = sext i32 %17 to i64
  %19 = add nsw i64 0, %18
  %20 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %21 = getelementptr inbounds i8, i8* %20, i64 %19
  %22 = bitcast i8* %21 to %"class.std::ios_base"*
  %23 = invoke zeroext i1 @"?good@ios_base@std@@QEBA_NXZ"(%"class.std::ios_base"* %22)
          to label %24 unwind label %72

24:                                               ; preds = %2
  br i1 %23, label %27, label %25

25:                                               ; preds = %24
  %26 = getelementptr inbounds %"class.std::basic_ostream<char, std::char_traits<char>>::sentry", %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %7, i32 0, i32 1
  store i8 0, i8* %26, align 8
  br label %70

27:                                               ; preds = %24
  %28 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %29 = bitcast %"class.std::basic_ostream"* %28 to i8*
  %30 = getelementptr inbounds i8, i8* %29, i64 0
  %31 = bitcast i8* %30 to i32**
  %32 = load i32*, i32** %31, align 8
  %33 = getelementptr inbounds i32, i32* %32, i32 1
  %34 = load i32, i32* %33, align 4
  %35 = sext i32 %34 to i64
  %36 = add nsw i64 0, %35
  %37 = bitcast %"class.std::basic_ostream"* %28 to i8*
  %38 = getelementptr inbounds i8, i8* %37, i64 %36
  %39 = bitcast i8* %38 to %"class.std::basic_ios"*
  %40 = invoke %"class.std::basic_ostream"* @"?tie@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_ostream@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %39)
          to label %41 unwind label %72

41:                                               ; preds = %27
  store %"class.std::basic_ostream"* %40, %"class.std::basic_ostream"** %6, align 8
  %42 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %6, align 8
  %43 = icmp ne %"class.std::basic_ostream"* %42, null
  br i1 %43, label %44, label %48

44:                                               ; preds = %41
  %45 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %6, align 8
  %46 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %47 = icmp eq %"class.std::basic_ostream"* %45, %46
  br i1 %47, label %48, label %50

48:                                               ; preds = %44, %41
  %49 = getelementptr inbounds %"class.std::basic_ostream<char, std::char_traits<char>>::sentry", %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %7, i32 0, i32 1
  store i8 1, i8* %49, align 8
  br label %70

50:                                               ; preds = %44
  %51 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %6, align 8
  %52 = invoke nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"?flush@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@XZ"(%"class.std::basic_ostream"* %51)
          to label %53 unwind label %72

53:                                               ; preds = %50
  %54 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  %55 = bitcast %"class.std::basic_ostream"* %54 to i8*
  %56 = getelementptr inbounds i8, i8* %55, i64 0
  %57 = bitcast i8* %56 to i32**
  %58 = load i32*, i32** %57, align 8
  %59 = getelementptr inbounds i32, i32* %58, i32 1
  %60 = load i32, i32* %59, align 4
  %61 = sext i32 %60 to i64
  %62 = add nsw i64 0, %61
  %63 = bitcast %"class.std::basic_ostream"* %54 to i8*
  %64 = getelementptr inbounds i8, i8* %63, i64 %62
  %65 = bitcast i8* %64 to %"class.std::ios_base"*
  %66 = invoke zeroext i1 @"?good@ios_base@std@@QEBA_NXZ"(%"class.std::ios_base"* %65)
          to label %67 unwind label %72

67:                                               ; preds = %53
  %68 = getelementptr inbounds %"class.std::basic_ostream<char, std::char_traits<char>>::sentry", %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %7, i32 0, i32 1
  %69 = zext i1 %66 to i8
  store i8 %69, i8* %68, align 8
  br label %70

70:                                               ; preds = %67, %48, %25
  %71 = load %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"*, %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"** %3, align 8
  ret %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %71

72:                                               ; preds = %53, %50, %27, %2
  %73 = cleanuppad within none []
  %74 = bitcast %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %7 to %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"*
  call void @"??1_Sentry_base@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"* %74) #5 [ "funclet"(token %73) ]
  cleanupret from %73 unwind to caller
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local zeroext i1 @"??Bsentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEBA_NXZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"*, align 8
  store %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %0, %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"** %2, align 8
  %3 = load %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"*, %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"** %2, align 8
  %4 = getelementptr inbounds %"class.std::basic_ostream<char, std::char_traits<char>>::sentry", %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %3, i32 0, i32 1
  %5 = load i8, i8* %4, align 8
  %6 = trunc i8 %5 to i1
  ret i1 %6
}

declare dso_local i32 @__CxxFrameHandler3(...)

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::ios_base"*, align 8
  store %"class.std::ios_base"* %0, %"class.std::ios_base"** %2, align 8
  %3 = load %"class.std::ios_base"*, %"class.std::ios_base"** %2, align 8
  %4 = getelementptr inbounds %"class.std::ios_base", %"class.std::ios_base"* %3, i32 0, i32 4
  %5 = load i32, i32* %4, align 8
  ret i32 %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local zeroext i1 @"?eq_int_type@?$_Narrow_char_traits@DH@std@@SA_NAEBH0@Z"(i32* nonnull align 4 dereferenceable(4) %0, i32* nonnull align 4 dereferenceable(4) %1) #3 comdat align 2 {
  %3 = alloca i32*, align 8
  %4 = alloca i32*, align 8
  store i32* %1, i32** %3, align 8
  store i32* %0, i32** %4, align 8
  %5 = load i32*, i32** %4, align 8
  %6 = load i32, i32* %5, align 4
  %7 = load i32*, i32** %3, align 8
  %8 = load i32, i32* %7, align 4
  %9 = icmp eq i32 %6, %8
  ret i1 %9
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::basic_streambuf"* @"?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::basic_ios"*, align 8
  store %"class.std::basic_ios"* %0, %"class.std::basic_ios"** %2, align 8
  %3 = load %"class.std::basic_ios"*, %"class.std::basic_ios"** %2, align 8
  %4 = getelementptr inbounds %"class.std::basic_ios", %"class.std::basic_ios"* %3, i32 0, i32 1
  %5 = load %"class.std::basic_streambuf"*, %"class.std::basic_streambuf"** %4, align 8
  ret %"class.std::basic_streambuf"* %5
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i32 @"?sputc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHD@Z"(%"class.std::basic_streambuf"* %0, i8 %1) #1 comdat align 2 {
  %3 = alloca i8, align 1
  %4 = alloca %"class.std::basic_streambuf"*, align 8
  store i8 %1, i8* %3, align 1
  store %"class.std::basic_streambuf"* %0, %"class.std::basic_streambuf"** %4, align 8
  %5 = load %"class.std::basic_streambuf"*, %"class.std::basic_streambuf"** %4, align 8
  %6 = call i64 @"?_Pnavail@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBA_JXZ"(%"class.std::basic_streambuf"* %5)
  %7 = icmp slt i64 0, %6
  br i1 %7, label %8, label %12

8:                                                ; preds = %2
  %9 = load i8, i8* %3, align 1
  %10 = call i8* @"?_Pninc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAPEADXZ"(%"class.std::basic_streambuf"* %5)
  store i8 %9, i8* %10, align 1
  %11 = call i32 @"?to_int_type@?$_Narrow_char_traits@DH@std@@SAHAEBD@Z"(i8* nonnull align 1 dereferenceable(1) %10) #5
  br label %19

12:                                               ; preds = %2
  %13 = call i32 @"?to_int_type@?$_Narrow_char_traits@DH@std@@SAHAEBD@Z"(i8* nonnull align 1 dereferenceable(1) %3) #5
  %14 = bitcast %"class.std::basic_streambuf"* %5 to i32 (%"class.std::basic_streambuf"*, i32)***
  %15 = load i32 (%"class.std::basic_streambuf"*, i32)**, i32 (%"class.std::basic_streambuf"*, i32)*** %14, align 8
  %16 = getelementptr inbounds i32 (%"class.std::basic_streambuf"*, i32)*, i32 (%"class.std::basic_streambuf"*, i32)** %15, i64 3
  %17 = load i32 (%"class.std::basic_streambuf"*, i32)*, i32 (%"class.std::basic_streambuf"*, i32)** %16, align 8
  %18 = call i32 %17(%"class.std::basic_streambuf"* %5, i32 %13)
  br label %19

19:                                               ; preds = %12, %8
  %20 = phi i32 [ %11, %8 ], [ %18, %12 ]
  ret i32 %20
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8 @"?fill@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBADXZ"(%"class.std::basic_ios"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::basic_ios"*, align 8
  store %"class.std::basic_ios"* %0, %"class.std::basic_ios"** %2, align 8
  %3 = load %"class.std::basic_ios"*, %"class.std::basic_ios"** %2, align 8
  %4 = getelementptr inbounds %"class.std::basic_ios", %"class.std::basic_ios"* %3, i32 0, i32 3
  %5 = load i8, i8* %4, align 8
  ret i8 %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i32 @"?eof@?$_Narrow_char_traits@DH@std@@SAHXZ"() #3 comdat align 2 {
  ret i32 -1
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i64 @"?sputn@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAA_JPEBD_J@Z"(%"class.std::basic_streambuf"* %0, i8* %1, i64 %2) #1 comdat align 2 {
  %4 = alloca i64, align 8
  %5 = alloca i8*, align 8
  %6 = alloca %"class.std::basic_streambuf"*, align 8
  store i64 %2, i64* %4, align 8
  store i8* %1, i8** %5, align 8
  store %"class.std::basic_streambuf"* %0, %"class.std::basic_streambuf"** %6, align 8
  %7 = load %"class.std::basic_streambuf"*, %"class.std::basic_streambuf"** %6, align 8
  %8 = load i64, i64* %4, align 8
  %9 = load i8*, i8** %5, align 8
  %10 = bitcast %"class.std::basic_streambuf"* %7 to i64 (%"class.std::basic_streambuf"*, i8*, i64)***
  %11 = load i64 (%"class.std::basic_streambuf"*, i8*, i64)**, i64 (%"class.std::basic_streambuf"*, i8*, i64)*** %10, align 8
  %12 = getelementptr inbounds i64 (%"class.std::basic_streambuf"*, i8*, i64)*, i64 (%"class.std::basic_streambuf"*, i8*, i64)** %11, i64 9
  %13 = load i64 (%"class.std::basic_streambuf"*, i8*, i64)*, i64 (%"class.std::basic_streambuf"*, i8*, i64)** %12, align 8
  %14 = call i64 %13(%"class.std::basic_streambuf"* %7, i8* %9, i64 %8)
  ret i64 %14
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i64 @"?width@ios_base@std@@QEAA_J_J@Z"(%"class.std::ios_base"* %0, i64 %1) #3 comdat align 2 {
  %3 = alloca i64, align 8
  %4 = alloca %"class.std::ios_base"*, align 8
  %5 = alloca i64, align 8
  store i64 %1, i64* %3, align 8
  store %"class.std::ios_base"* %0, %"class.std::ios_base"** %4, align 8
  %6 = load %"class.std::ios_base"*, %"class.std::ios_base"** %4, align 8
  %7 = getelementptr inbounds %"class.std::ios_base", %"class.std::ios_base"* %6, i32 0, i32 6
  %8 = load i64, i64* %7, align 8
  store i64 %8, i64* %5, align 8
  %9 = load i64, i64* %3, align 8
  %10 = getelementptr inbounds %"class.std::ios_base", %"class.std::ios_base"* %6, i32 0, i32 6
  store i64 %9, i64* %10, align 8
  %11 = load i64, i64* %5, align 8
  ret i64 %11
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %0, i32 %1, i1 zeroext %2) #1 comdat align 2 {
  %4 = alloca i8, align 1
  %5 = alloca i32, align 4
  %6 = alloca %"class.std::basic_ios"*, align 8
  %7 = zext i1 %2 to i8
  store i8 %7, i8* %4, align 1
  store i32 %1, i32* %5, align 4
  store %"class.std::basic_ios"* %0, %"class.std::basic_ios"** %6, align 8
  %8 = load %"class.std::basic_ios"*, %"class.std::basic_ios"** %6, align 8
  %9 = load i8, i8* %4, align 1
  %10 = trunc i8 %9 to i1
  %11 = bitcast %"class.std::basic_ios"* %8 to %"class.std::ios_base"*
  %12 = call i32 @"?rdstate@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %11)
  %13 = load i32, i32* %5, align 4
  %14 = or i32 %12, %13
  call void @"?clear@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %8, i32 %14, i1 zeroext %10)
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %0) unnamed_addr #3 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %2 = alloca %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"*, align 8
  %3 = alloca i8, align 1
  store %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %0, %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"** %2, align 8
  %4 = load %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"*, %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"** %2, align 8
  %5 = call zeroext i1 @"?uncaught_exception@std@@YA_NXZ"() #5
  %6 = xor i1 %5, true
  %7 = zext i1 %6 to i8
  store i8 %7, i8* %3, align 1
  %8 = load i8, i8* %3, align 1
  %9 = trunc i8 %8 to i1
  br i1 %9, label %10, label %15

10:                                               ; preds = %1
  %11 = bitcast %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %4 to %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"*
  %12 = getelementptr inbounds %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base", %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"* %11, i32 0, i32 0
  %13 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %12, align 8
  invoke void @"?_Osfx@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAXXZ"(%"class.std::basic_ostream"* %13)
          to label %14 unwind label %17

14:                                               ; preds = %10
  br label %15

15:                                               ; preds = %14, %1
  %16 = bitcast %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %4 to %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"*
  call void @"??1_Sentry_base@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"* %16) #5
  ret void

17:                                               ; preds = %10
  %18 = cleanuppad within none []
  %19 = bitcast %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %4 to %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"*
  call void @"??1_Sentry_base@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"* %19) #5 [ "funclet"(token %18) ]
  cleanupret from %18 unwind label %20

20:                                               ; preds = %17
  %21 = cleanuppad within none []
  call void @__std_terminate() #18 [ "funclet"(token %21) ]
  unreachable
}

declare dso_local i64 @strlen(i8*) #4

declare dso_local void @__std_terminate()

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"* @"??0_Sentry_base@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@AEAV12@@Z"(%"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"* returned %0, %"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %1) unnamed_addr #1 comdat align 2 {
  %3 = alloca %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"*, align 8
  %4 = alloca %"class.std::basic_ostream"*, align 8
  %5 = alloca %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"*, align 8
  %6 = alloca %"class.std::basic_streambuf"*, align 8
  store %"class.std::basic_ostream"* %1, %"class.std::basic_ostream"** %4, align 8
  store %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"* %0, %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"** %5, align 8
  %7 = load %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"*, %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"** %5, align 8
  store %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"* %7, %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"** %3, align 8
  %8 = getelementptr inbounds %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base", %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"* %7, i32 0, i32 0
  %9 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  store %"class.std::basic_ostream"* %9, %"class.std::basic_ostream"** %8, align 8
  %10 = getelementptr inbounds %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base", %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"* %7, i32 0, i32 0
  %11 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %10, align 8
  %12 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %13 = getelementptr inbounds i8, i8* %12, i64 0
  %14 = bitcast i8* %13 to i32**
  %15 = load i32*, i32** %14, align 8
  %16 = getelementptr inbounds i32, i32* %15, i32 1
  %17 = load i32, i32* %16, align 4
  %18 = sext i32 %17 to i64
  %19 = add nsw i64 0, %18
  %20 = bitcast %"class.std::basic_ostream"* %11 to i8*
  %21 = getelementptr inbounds i8, i8* %20, i64 %19
  %22 = bitcast i8* %21 to %"class.std::basic_ios"*
  %23 = call %"class.std::basic_streambuf"* @"?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %22)
  store %"class.std::basic_streambuf"* %23, %"class.std::basic_streambuf"** %6, align 8
  %24 = load %"class.std::basic_streambuf"*, %"class.std::basic_streambuf"** %6, align 8
  %25 = icmp ne %"class.std::basic_streambuf"* %24, null
  br i1 %25, label %26, label %32

26:                                               ; preds = %2
  %27 = load %"class.std::basic_streambuf"*, %"class.std::basic_streambuf"** %6, align 8
  %28 = bitcast %"class.std::basic_streambuf"* %27 to void (%"class.std::basic_streambuf"*)***
  %29 = load void (%"class.std::basic_streambuf"*)**, void (%"class.std::basic_streambuf"*)*** %28, align 8
  %30 = getelementptr inbounds void (%"class.std::basic_streambuf"*)*, void (%"class.std::basic_streambuf"*)** %29, i64 1
  %31 = load void (%"class.std::basic_streambuf"*)*, void (%"class.std::basic_streambuf"*)** %30, align 8
  call void %31(%"class.std::basic_streambuf"* %27)
  br label %32

32:                                               ; preds = %26, %2
  %33 = load %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"*, %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"** %3, align 8
  ret %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"* %33
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local zeroext i1 @"?good@ios_base@std@@QEBA_NXZ"(%"class.std::ios_base"* %0) #1 comdat align 2 {
  %2 = alloca %"class.std::ios_base"*, align 8
  store %"class.std::ios_base"* %0, %"class.std::ios_base"** %2, align 8
  %3 = load %"class.std::ios_base"*, %"class.std::ios_base"** %2, align 8
  %4 = call i32 @"?rdstate@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %3)
  %5 = icmp eq i32 %4, 0
  ret i1 %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::basic_ostream"* @"?tie@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_ostream@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::basic_ios"*, align 8
  store %"class.std::basic_ios"* %0, %"class.std::basic_ios"** %2, align 8
  %3 = load %"class.std::basic_ios"*, %"class.std::basic_ios"** %2, align 8
  %4 = getelementptr inbounds %"class.std::basic_ios", %"class.std::basic_ios"* %3, i32 0, i32 2
  %5 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %4, align 8
  ret %"class.std::basic_ostream"* %5
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @"?flush@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@XZ"(%"class.std::basic_ostream"* %0) #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %2 = alloca %"class.std::basic_ostream"*, align 8
  %3 = alloca %"class.std::basic_streambuf"*, align 8
  %4 = alloca %"class.std::basic_ostream<char, std::char_traits<char>>::sentry", align 8
  store %"class.std::basic_ostream"* %0, %"class.std::basic_ostream"** %2, align 8
  %5 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %2, align 8
  %6 = bitcast %"class.std::basic_ostream"* %5 to i8*
  %7 = getelementptr inbounds i8, i8* %6, i64 0
  %8 = bitcast i8* %7 to i32**
  %9 = load i32*, i32** %8, align 8
  %10 = getelementptr inbounds i32, i32* %9, i32 1
  %11 = load i32, i32* %10, align 4
  %12 = sext i32 %11 to i64
  %13 = add nsw i64 0, %12
  %14 = bitcast %"class.std::basic_ostream"* %5 to i8*
  %15 = getelementptr inbounds i8, i8* %14, i64 %13
  %16 = bitcast i8* %15 to %"class.std::basic_ios"*
  %17 = call %"class.std::basic_streambuf"* @"?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %16)
  store %"class.std::basic_streambuf"* %17, %"class.std::basic_streambuf"** %3, align 8
  %18 = load %"class.std::basic_streambuf"*, %"class.std::basic_streambuf"** %3, align 8
  %19 = icmp ne %"class.std::basic_streambuf"* %18, null
  br i1 %19, label %20, label %45

20:                                               ; preds = %1
  %21 = call %"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* @"??0sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@AEAV12@@Z"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %4, %"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %5)
  %22 = invoke zeroext i1 @"??Bsentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEBA_NXZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %4)
          to label %23 unwind label %43

23:                                               ; preds = %20
  br i1 %22, label %24, label %42

24:                                               ; preds = %23
  %25 = load %"class.std::basic_streambuf"*, %"class.std::basic_streambuf"** %3, align 8
  %26 = invoke i32 @"?pubsync@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHXZ"(%"class.std::basic_streambuf"* %25)
          to label %27 unwind label %43

27:                                               ; preds = %24
  %28 = icmp eq i32 %26, -1
  br i1 %28, label %29, label %42

29:                                               ; preds = %27
  %30 = bitcast %"class.std::basic_ostream"* %5 to i8*
  %31 = getelementptr inbounds i8, i8* %30, i64 0
  %32 = bitcast i8* %31 to i32**
  %33 = load i32*, i32** %32, align 8
  %34 = getelementptr inbounds i32, i32* %33, i32 1
  %35 = load i32, i32* %34, align 4
  %36 = sext i32 %35 to i64
  %37 = add nsw i64 0, %36
  %38 = bitcast %"class.std::basic_ostream"* %5 to i8*
  %39 = getelementptr inbounds i8, i8* %38, i64 %37
  %40 = bitcast i8* %39 to %"class.std::basic_ios"*
  invoke void @"?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %40, i32 4, i1 zeroext false)
          to label %41 unwind label %43

41:                                               ; preds = %29
  br label %42

42:                                               ; preds = %41, %27, %23
  call void @"??1sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %4) #5
  br label %45

43:                                               ; preds = %29, %24, %20
  %44 = cleanuppad within none []
  call void @"??1sentry@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::sentry"* %4) #5 [ "funclet"(token %44) ]
  cleanupret from %44 unwind to caller

45:                                               ; preds = %42, %1
  ret %"class.std::basic_ostream"* %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1_Sentry_base@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@XZ"(%"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"* %0) unnamed_addr #3 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %2 = alloca %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"*, align 8
  %3 = alloca %"class.std::basic_streambuf"*, align 8
  store %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"* %0, %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"** %2, align 8
  %4 = load %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"*, %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"** %2, align 8
  %5 = getelementptr inbounds %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base", %"class.std::basic_ostream<char, std::char_traits<char>>::_Sentry_base"* %4, i32 0, i32 0
  %6 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %5, align 8
  %7 = bitcast %"class.std::basic_ostream"* %6 to i8*
  %8 = getelementptr inbounds i8, i8* %7, i64 0
  %9 = bitcast i8* %8 to i32**
  %10 = load i32*, i32** %9, align 8
  %11 = getelementptr inbounds i32, i32* %10, i32 1
  %12 = load i32, i32* %11, align 4
  %13 = sext i32 %12 to i64
  %14 = add nsw i64 0, %13
  %15 = bitcast %"class.std::basic_ostream"* %6 to i8*
  %16 = getelementptr inbounds i8, i8* %15, i64 %14
  %17 = bitcast i8* %16 to %"class.std::basic_ios"*
  %18 = invoke %"class.std::basic_streambuf"* @"?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %17)
          to label %19 unwind label %30

19:                                               ; preds = %1
  store %"class.std::basic_streambuf"* %18, %"class.std::basic_streambuf"** %3, align 8
  %20 = load %"class.std::basic_streambuf"*, %"class.std::basic_streambuf"** %3, align 8
  %21 = icmp ne %"class.std::basic_streambuf"* %20, null
  br i1 %21, label %22, label %29

22:                                               ; preds = %19
  %23 = load %"class.std::basic_streambuf"*, %"class.std::basic_streambuf"** %3, align 8
  %24 = bitcast %"class.std::basic_streambuf"* %23 to void (%"class.std::basic_streambuf"*)***
  %25 = load void (%"class.std::basic_streambuf"*)**, void (%"class.std::basic_streambuf"*)*** %24, align 8
  %26 = getelementptr inbounds void (%"class.std::basic_streambuf"*)*, void (%"class.std::basic_streambuf"*)** %25, i64 2
  %27 = load void (%"class.std::basic_streambuf"*)*, void (%"class.std::basic_streambuf"*)** %26, align 8
  invoke void %27(%"class.std::basic_streambuf"* %23)
          to label %28 unwind label %30

28:                                               ; preds = %22
  br label %29

29:                                               ; preds = %28, %19
  ret void

30:                                               ; preds = %22, %1
  %31 = cleanuppad within none []
  call void @__std_terminate() #18 [ "funclet"(token %31) ]
  unreachable
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i32 @"?rdstate@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::ios_base"*, align 8
  store %"class.std::ios_base"* %0, %"class.std::ios_base"** %2, align 8
  %3 = load %"class.std::ios_base"*, %"class.std::ios_base"** %2, align 8
  %4 = getelementptr inbounds %"class.std::ios_base", %"class.std::ios_base"* %3, i32 0, i32 2
  %5 = load i32, i32* %4, align 8
  ret i32 %5
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i32 @"?pubsync@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHXZ"(%"class.std::basic_streambuf"* %0) #1 comdat align 2 {
  %2 = alloca %"class.std::basic_streambuf"*, align 8
  store %"class.std::basic_streambuf"* %0, %"class.std::basic_streambuf"** %2, align 8
  %3 = load %"class.std::basic_streambuf"*, %"class.std::basic_streambuf"** %2, align 8
  %4 = bitcast %"class.std::basic_streambuf"* %3 to i32 (%"class.std::basic_streambuf"*)***
  %5 = load i32 (%"class.std::basic_streambuf"*)**, i32 (%"class.std::basic_streambuf"*)*** %4, align 8
  %6 = getelementptr inbounds i32 (%"class.std::basic_streambuf"*)*, i32 (%"class.std::basic_streambuf"*)** %5, i64 13
  %7 = load i32 (%"class.std::basic_streambuf"*)*, i32 (%"class.std::basic_streambuf"*)** %6, align 8
  %8 = call i32 %7(%"class.std::basic_streambuf"* %3)
  ret i32 %8
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i64 @"?_Pnavail@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBA_JXZ"(%"class.std::basic_streambuf"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::basic_streambuf"*, align 8
  store %"class.std::basic_streambuf"* %0, %"class.std::basic_streambuf"** %2, align 8
  %3 = load %"class.std::basic_streambuf"*, %"class.std::basic_streambuf"** %2, align 8
  %4 = getelementptr inbounds %"class.std::basic_streambuf", %"class.std::basic_streambuf"* %3, i32 0, i32 8
  %5 = load i8**, i8*** %4, align 8
  %6 = load i8*, i8** %5, align 8
  %7 = icmp ne i8* %6, null
  br i1 %7, label %8, label %12

8:                                                ; preds = %1
  %9 = getelementptr inbounds %"class.std::basic_streambuf", %"class.std::basic_streambuf"* %3, i32 0, i32 12
  %10 = load i32*, i32** %9, align 8
  %11 = load i32, i32* %10, align 4
  br label %13

12:                                               ; preds = %1
  br label %13

13:                                               ; preds = %12, %8
  %14 = phi i32 [ %11, %8 ], [ 0, %12 ]
  %15 = sext i32 %14 to i64
  ret i64 %15
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i32 @"?to_int_type@?$_Narrow_char_traits@DH@std@@SAHAEBD@Z"(i8* nonnull align 1 dereferenceable(1) %0) #3 comdat align 2 {
  %2 = alloca i8*, align 8
  store i8* %0, i8** %2, align 8
  %3 = load i8*, i8** %2, align 8
  %4 = load i8, i8* %3, align 1
  %5 = zext i8 %4 to i32
  ret i32 %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"?_Pninc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAPEADXZ"(%"class.std::basic_streambuf"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::basic_streambuf"*, align 8
  store %"class.std::basic_streambuf"* %0, %"class.std::basic_streambuf"** %2, align 8
  %3 = load %"class.std::basic_streambuf"*, %"class.std::basic_streambuf"** %2, align 8
  %4 = getelementptr inbounds %"class.std::basic_streambuf", %"class.std::basic_streambuf"* %3, i32 0, i32 12
  %5 = load i32*, i32** %4, align 8
  %6 = load i32, i32* %5, align 4
  %7 = add nsw i32 %6, -1
  store i32 %7, i32* %5, align 4
  %8 = getelementptr inbounds %"class.std::basic_streambuf", %"class.std::basic_streambuf"* %3, i32 0, i32 8
  %9 = load i8**, i8*** %8, align 8
  %10 = load i8*, i8** %9, align 8
  %11 = getelementptr inbounds i8, i8* %10, i32 1
  store i8* %11, i8** %9, align 8
  ret i8* %10
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?clear@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %0, i32 %1, i1 zeroext %2) #1 comdat align 2 {
  %4 = alloca i8, align 1
  %5 = alloca i32, align 4
  %6 = alloca %"class.std::basic_ios"*, align 8
  %7 = zext i1 %2 to i8
  store i8 %7, i8* %4, align 1
  store i32 %1, i32* %5, align 4
  store %"class.std::basic_ios"* %0, %"class.std::basic_ios"** %6, align 8
  %8 = load %"class.std::basic_ios"*, %"class.std::basic_ios"** %6, align 8
  %9 = bitcast %"class.std::basic_ios"* %8 to %"class.std::ios_base"*
  %10 = load i8, i8* %4, align 1
  %11 = trunc i8 %10 to i1
  %12 = load i32, i32* %5, align 4
  %13 = getelementptr inbounds %"class.std::basic_ios", %"class.std::basic_ios"* %8, i32 0, i32 1
  %14 = load %"class.std::basic_streambuf"*, %"class.std::basic_streambuf"** %13, align 8
  %15 = icmp ne %"class.std::basic_streambuf"* %14, null
  %16 = zext i1 %15 to i64
  %17 = select i1 %15, i32 0, i32 4
  %18 = or i32 %12, %17
  call void @"?clear@ios_base@std@@QEAAXH_N@Z"(%"class.std::ios_base"* %9, i32 %18, i1 zeroext %11)
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?clear@ios_base@std@@QEAAXH_N@Z"(%"class.std::ios_base"* %0, i32 %1, i1 zeroext %2) #1 comdat align 2 {
  %4 = alloca i8, align 1
  %5 = alloca i32, align 4
  %6 = alloca %"class.std::ios_base"*, align 8
  %7 = alloca i32, align 4
  %8 = alloca i8*, align 8
  %9 = alloca %"class.std::ios_base::failure", align 8
  %10 = alloca %"class.std::error_code", align 8
  %11 = zext i1 %2 to i8
  store i8 %11, i8* %4, align 1
  store i32 %1, i32* %5, align 4
  store %"class.std::ios_base"* %0, %"class.std::ios_base"** %6, align 8
  %12 = load %"class.std::ios_base"*, %"class.std::ios_base"** %6, align 8
  %13 = load i32, i32* %5, align 4
  %14 = and i32 %13, 23
  store i32 %14, i32* %5, align 4
  %15 = load i32, i32* %5, align 4
  %16 = getelementptr inbounds %"class.std::ios_base", %"class.std::ios_base"* %12, i32 0, i32 2
  store i32 %15, i32* %16, align 8
  %17 = load i32, i32* %5, align 4
  %18 = getelementptr inbounds %"class.std::ios_base", %"class.std::ios_base"* %12, i32 0, i32 3
  %19 = load i32, i32* %18, align 4
  %20 = and i32 %17, %19
  store i32 %20, i32* %7, align 4
  %21 = load i32, i32* %7, align 4
  %22 = icmp ne i32 %21, 0
  br i1 %22, label %23, label %43

23:                                               ; preds = %3
  %24 = load i8, i8* %4, align 1
  %25 = trunc i8 %24 to i1
  br i1 %25, label %26, label %27

26:                                               ; preds = %23
  call void @_CxxThrowException(i8* null, %eh.ThrowInfo* null) #19
  unreachable

27:                                               ; preds = %23
  %28 = load i32, i32* %7, align 4
  %29 = and i32 %28, 4
  %30 = icmp ne i32 %29, 0
  br i1 %30, label %31, label %32

31:                                               ; preds = %27
  store i8* getelementptr inbounds ([21 x i8], [21 x i8]* @"??_C@_0BF@PHHKMMFD@ios_base?3?3badbit?5set?$AA@", i64 0, i64 0), i8** %8, align 8
  br label %39

32:                                               ; preds = %27
  %33 = load i32, i32* %7, align 4
  %34 = and i32 %33, 2
  %35 = icmp ne i32 %34, 0
  br i1 %35, label %36, label %37

36:                                               ; preds = %32
  store i8* getelementptr inbounds ([22 x i8], [22 x i8]* @"??_C@_0BG@FMKFHCIL@ios_base?3?3failbit?5set?$AA@", i64 0, i64 0), i8** %8, align 8
  br label %38

37:                                               ; preds = %32
  store i8* getelementptr inbounds ([21 x i8], [21 x i8]* @"??_C@_0BF@OOHOMBOF@ios_base?3?3eofbit?5set?$AA@", i64 0, i64 0), i8** %8, align 8
  br label %38

38:                                               ; preds = %37, %36
  br label %39

39:                                               ; preds = %38, %31
  call void @"?make_error_code@std@@YA?AVerror_code@1@W4io_errc@1@@Z"(%"class.std::error_code"* sret align 8 %10, i32 1) #5
  %40 = load i8*, i8** %8, align 8
  %41 = call %"class.std::ios_base::failure"* @"??0failure@ios_base@std@@QEAA@PEBDAEBVerror_code@2@@Z"(%"class.std::ios_base::failure"* %9, i8* %40, %"class.std::error_code"* nonnull align 8 dereferenceable(16) %10)
  %42 = bitcast %"class.std::ios_base::failure"* %9 to i8*
  call void @_CxxThrowException(i8* %42, %eh.ThrowInfo* @"_TI5?AVfailure@ios_base@std@@") #19
  unreachable

43:                                               ; preds = %3
  ret void
}

declare dso_local void @_CxxThrowException(i8*, %eh.ThrowInfo*)

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?make_error_code@std@@YA?AVerror_code@1@W4io_errc@1@@Z"(%"class.std::error_code"* noalias sret align 8 %0, i32 %1) #3 comdat {
  %3 = alloca i8*, align 8
  %4 = alloca i32, align 4
  %5 = bitcast %"class.std::error_code"* %0 to i8*
  store i8* %5, i8** %3, align 8
  store i32 %1, i32* %4, align 4
  %6 = call nonnull align 8 dereferenceable(16) %"class.std::error_category"* @"?iostream_category@std@@YAAEBVerror_category@1@XZ"() #5
  %7 = load i32, i32* %4, align 4
  %8 = call %"class.std::error_code"* @"??0error_code@std@@QEAA@HAEBVerror_category@1@@Z"(%"class.std::error_code"* %0, i32 %7, %"class.std::error_category"* nonnull align 8 dereferenceable(16) %6) #5
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local %"class.std::ios_base::failure"* @"??0failure@ios_base@std@@QEAA@PEBDAEBVerror_code@2@@Z"(%"class.std::ios_base::failure"* returned %0, i8* %1, %"class.std::error_code"* nonnull align 8 dereferenceable(16) %2) unnamed_addr #1 comdat align 2 {
  %4 = alloca %"class.std::error_code"*, align 8
  %5 = alloca i8*, align 8
  %6 = alloca %"class.std::ios_base::failure"*, align 8
  %7 = alloca %"class.std::error_code", align 8
  store %"class.std::error_code"* %2, %"class.std::error_code"** %4, align 8
  store i8* %1, i8** %5, align 8
  store %"class.std::ios_base::failure"* %0, %"class.std::ios_base::failure"** %6, align 8
  %8 = load %"class.std::ios_base::failure"*, %"class.std::ios_base::failure"** %6, align 8
  %9 = bitcast %"class.std::ios_base::failure"* %8 to %"class.std::system_error"*
  %10 = load i8*, i8** %5, align 8
  %11 = load %"class.std::error_code"*, %"class.std::error_code"** %4, align 8
  %12 = bitcast %"class.std::error_code"* %7 to i8*
  %13 = bitcast %"class.std::error_code"* %11 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %12, i8* align 8 %13, i64 16, i1 false)
  %14 = call %"class.std::system_error"* @"??0system_error@std@@QEAA@Verror_code@1@PEBD@Z"(%"class.std::system_error"* %9, %"class.std::error_code"* %7, i8* %10)
  %15 = bitcast %"class.std::ios_base::failure"* %8 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7failure@ios_base@std@@6B@" to i32 (...)**), i32 (...)*** %15, align 8
  ret %"class.std::ios_base::failure"* %8
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::ios_base::failure"* @"??0failure@ios_base@std@@QEAA@AEBV012@@Z"(%"class.std::ios_base::failure"* returned %0, %"class.std::ios_base::failure"* nonnull align 8 dereferenceable(40) %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca %"class.std::ios_base::failure"*, align 8
  %4 = alloca %"class.std::ios_base::failure"*, align 8
  store %"class.std::ios_base::failure"* %1, %"class.std::ios_base::failure"** %3, align 8
  store %"class.std::ios_base::failure"* %0, %"class.std::ios_base::failure"** %4, align 8
  %5 = load %"class.std::ios_base::failure"*, %"class.std::ios_base::failure"** %4, align 8
  %6 = bitcast %"class.std::ios_base::failure"* %5 to %"class.std::system_error"*
  %7 = load %"class.std::ios_base::failure"*, %"class.std::ios_base::failure"** %3, align 8
  %8 = bitcast %"class.std::ios_base::failure"* %7 to %"class.std::system_error"*
  %9 = call %"class.std::system_error"* @"??0system_error@std@@QEAA@AEBV01@@Z"(%"class.std::system_error"* %6, %"class.std::system_error"* nonnull align 8 dereferenceable(40) %8) #5
  %10 = bitcast %"class.std::ios_base::failure"* %5 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7failure@ios_base@std@@6B@" to i32 (...)**), i32 (...)*** %10, align 8
  ret %"class.std::ios_base::failure"* %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::system_error"* @"??0system_error@std@@QEAA@AEBV01@@Z"(%"class.std::system_error"* returned %0, %"class.std::system_error"* nonnull align 8 dereferenceable(40) %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca %"class.std::system_error"*, align 8
  %4 = alloca %"class.std::system_error"*, align 8
  store %"class.std::system_error"* %1, %"class.std::system_error"** %3, align 8
  store %"class.std::system_error"* %0, %"class.std::system_error"** %4, align 8
  %5 = load %"class.std::system_error"*, %"class.std::system_error"** %4, align 8
  %6 = bitcast %"class.std::system_error"* %5 to %"class.std::_System_error"*
  %7 = load %"class.std::system_error"*, %"class.std::system_error"** %3, align 8
  %8 = bitcast %"class.std::system_error"* %7 to %"class.std::_System_error"*
  %9 = call %"class.std::_System_error"* @"??0_System_error@std@@QEAA@AEBV01@@Z"(%"class.std::_System_error"* %6, %"class.std::_System_error"* nonnull align 8 dereferenceable(40) %8) #5
  %10 = bitcast %"class.std::system_error"* %5 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7system_error@std@@6B@" to i32 (...)**), i32 (...)*** %10, align 8
  ret %"class.std::system_error"* %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::_System_error"* @"??0_System_error@std@@QEAA@AEBV01@@Z"(%"class.std::_System_error"* returned %0, %"class.std::_System_error"* nonnull align 8 dereferenceable(40) %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca %"class.std::_System_error"*, align 8
  %4 = alloca %"class.std::_System_error"*, align 8
  store %"class.std::_System_error"* %1, %"class.std::_System_error"** %3, align 8
  store %"class.std::_System_error"* %0, %"class.std::_System_error"** %4, align 8
  %5 = load %"class.std::_System_error"*, %"class.std::_System_error"** %4, align 8
  %6 = bitcast %"class.std::_System_error"* %5 to %"class.std::runtime_error"*
  %7 = load %"class.std::_System_error"*, %"class.std::_System_error"** %3, align 8
  %8 = bitcast %"class.std::_System_error"* %7 to %"class.std::runtime_error"*
  %9 = call %"class.std::runtime_error"* @"??0runtime_error@std@@QEAA@AEBV01@@Z"(%"class.std::runtime_error"* %6, %"class.std::runtime_error"* nonnull align 8 dereferenceable(24) %8) #5
  %10 = bitcast %"class.std::_System_error"* %5 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7_System_error@std@@6B@" to i32 (...)**), i32 (...)*** %10, align 8
  %11 = getelementptr inbounds %"class.std::_System_error", %"class.std::_System_error"* %5, i32 0, i32 1
  %12 = load %"class.std::_System_error"*, %"class.std::_System_error"** %3, align 8
  %13 = getelementptr inbounds %"class.std::_System_error", %"class.std::_System_error"* %12, i32 0, i32 1
  %14 = bitcast %"class.std::error_code"* %11 to i8*
  %15 = bitcast %"class.std::error_code"* %13 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %14, i8* align 8 %15, i64 16, i1 false)
  ret %"class.std::_System_error"* %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::runtime_error"* @"??0runtime_error@std@@QEAA@AEBV01@@Z"(%"class.std::runtime_error"* returned %0, %"class.std::runtime_error"* nonnull align 8 dereferenceable(24) %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca %"class.std::runtime_error"*, align 8
  %4 = alloca %"class.std::runtime_error"*, align 8
  store %"class.std::runtime_error"* %1, %"class.std::runtime_error"** %3, align 8
  store %"class.std::runtime_error"* %0, %"class.std::runtime_error"** %4, align 8
  %5 = load %"class.std::runtime_error"*, %"class.std::runtime_error"** %4, align 8
  %6 = bitcast %"class.std::runtime_error"* %5 to %"class.std::exception"*
  %7 = load %"class.std::runtime_error"*, %"class.std::runtime_error"** %3, align 8
  %8 = bitcast %"class.std::runtime_error"* %7 to %"class.std::exception"*
  %9 = call %"class.std::exception"* @"??0exception@std@@QEAA@AEBV01@@Z"(%"class.std::exception"* %6, %"class.std::exception"* nonnull align 8 dereferenceable(24) %8) #5
  %10 = bitcast %"class.std::runtime_error"* %5 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7runtime_error@std@@6B@" to i32 (...)**), i32 (...)*** %10, align 8
  ret %"class.std::runtime_error"* %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::exception"* @"??0exception@std@@QEAA@AEBV01@@Z"(%"class.std::exception"* returned %0, %"class.std::exception"* nonnull align 8 dereferenceable(24) %1) unnamed_addr #3 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %3 = alloca %"class.std::exception"*, align 8
  %4 = alloca %"class.std::exception"*, align 8
  store %"class.std::exception"* %1, %"class.std::exception"** %3, align 8
  store %"class.std::exception"* %0, %"class.std::exception"** %4, align 8
  %5 = load %"class.std::exception"*, %"class.std::exception"** %4, align 8
  %6 = bitcast %"class.std::exception"* %5 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7exception@std@@6B@" to i32 (...)**), i32 (...)*** %6, align 8
  %7 = getelementptr inbounds %"class.std::exception", %"class.std::exception"* %5, i32 0, i32 1
  %8 = bitcast %struct.__std_exception_data* %7 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 8 %8, i8 0, i64 16, i1 false)
  %9 = getelementptr inbounds %"class.std::exception", %"class.std::exception"* %5, i32 0, i32 1
  %10 = load %"class.std::exception"*, %"class.std::exception"** %3, align 8
  %11 = getelementptr inbounds %"class.std::exception", %"class.std::exception"* %10, i32 0, i32 1
  invoke void @__std_exception_copy(%struct.__std_exception_data* %11, %struct.__std_exception_data* %9)
          to label %12 unwind label %13

12:                                               ; preds = %2
  ret %"class.std::exception"* %5

13:                                               ; preds = %2
  %14 = cleanuppad within none []
  call void @__std_terminate() #18 [ "funclet"(token %14) ]
  unreachable
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1failure@ios_base@std@@UEAA@XZ"(%"class.std::ios_base::failure"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::ios_base::failure"*, align 8
  store %"class.std::ios_base::failure"* %0, %"class.std::ios_base::failure"** %2, align 8
  %3 = load %"class.std::ios_base::failure"*, %"class.std::ios_base::failure"** %2, align 8
  %4 = bitcast %"class.std::ios_base::failure"* %3 to %"class.std::system_error"*
  call void @"??1system_error@std@@UEAA@XZ"(%"class.std::system_error"* %4) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(16) %"class.std::error_category"* @"?iostream_category@std@@YAAEBVerror_category@1@XZ"() #3 comdat {
  %1 = call nonnull align 8 dereferenceable(16) %"class.std::_Iostream_error_category2"* @"??$_Immortalize_memcpy_image@V_Iostream_error_category2@std@@@std@@YAAEBV_Iostream_error_category2@0@XZ"() #5
  %2 = bitcast %"class.std::_Iostream_error_category2"* %1 to %"class.std::error_category"*
  ret %"class.std::error_category"* %2
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::error_code"* @"??0error_code@std@@QEAA@HAEBVerror_category@1@@Z"(%"class.std::error_code"* returned %0, i32 %1, %"class.std::error_category"* nonnull align 8 dereferenceable(16) %2) unnamed_addr #3 comdat align 2 {
  %4 = alloca %"class.std::error_category"*, align 8
  %5 = alloca i32, align 4
  %6 = alloca %"class.std::error_code"*, align 8
  store %"class.std::error_category"* %2, %"class.std::error_category"** %4, align 8
  store i32 %1, i32* %5, align 4
  store %"class.std::error_code"* %0, %"class.std::error_code"** %6, align 8
  %7 = load %"class.std::error_code"*, %"class.std::error_code"** %6, align 8
  %8 = getelementptr inbounds %"class.std::error_code", %"class.std::error_code"* %7, i32 0, i32 0
  %9 = load i32, i32* %5, align 4
  store i32 %9, i32* %8, align 8
  %10 = getelementptr inbounds %"class.std::error_code", %"class.std::error_code"* %7, i32 0, i32 1
  %11 = load %"class.std::error_category"*, %"class.std::error_category"** %4, align 8
  store %"class.std::error_category"* %11, %"class.std::error_category"** %10, align 8
  ret %"class.std::error_code"* %7
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(16) %"class.std::_Iostream_error_category2"* @"??$_Immortalize_memcpy_image@V_Iostream_error_category2@std@@@std@@YAAEBV_Iostream_error_category2@0@XZ"() #3 comdat {
  %1 = load atomic i32, i32* @"?$TSS0@?1???$_Immortalize_memcpy_image@V_Iostream_error_category2@std@@@std@@YAAEBV_Iostream_error_category2@0@XZ@4HA" unordered, align 4
  %2 = load i32, i32* @_Init_thread_epoch, align 4
  %3 = icmp sgt i32 %1, %2
  br i1 %3, label %4, label %9, !prof !8

4:                                                ; preds = %0
  call void @_Init_thread_header(i32* @"?$TSS0@?1???$_Immortalize_memcpy_image@V_Iostream_error_category2@std@@@std@@YAAEBV_Iostream_error_category2@0@XZ@4HA") #5
  %5 = load atomic i32, i32* @"?$TSS0@?1???$_Immortalize_memcpy_image@V_Iostream_error_category2@std@@@std@@YAAEBV_Iostream_error_category2@0@XZ@4HA" unordered, align 4
  %6 = icmp eq i32 %5, -1
  br i1 %6, label %7, label %9

7:                                                ; preds = %4
  %8 = call i32 @atexit(void ()* @"??__F_Static@?1???$_Immortalize_memcpy_image@V_Iostream_error_category2@std@@@std@@YAAEBV_Iostream_error_category2@1@XZ@YAXXZ") #5
  call void @_Init_thread_footer(i32* @"?$TSS0@?1???$_Immortalize_memcpy_image@V_Iostream_error_category2@std@@@std@@YAAEBV_Iostream_error_category2@0@XZ@4HA") #5
  br label %9

9:                                                ; preds = %7, %4, %0
  ret %"class.std::_Iostream_error_category2"* bitcast ({ i8**, i64 }* @"?_Static@?1???$_Immortalize_memcpy_image@V_Iostream_error_category2@std@@@std@@YAAEBV_Iostream_error_category2@1@XZ@4V21@A" to %"class.std::_Iostream_error_category2"*)
}

; Function Attrs: nounwind
declare dso_local void @_Init_thread_header(i32*) #5

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1_Iostream_error_category2@std@@UEAA@XZ"(%"class.std::_Iostream_error_category2"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::_Iostream_error_category2"*, align 8
  store %"class.std::_Iostream_error_category2"* %0, %"class.std::_Iostream_error_category2"** %2, align 8
  %3 = load %"class.std::_Iostream_error_category2"*, %"class.std::_Iostream_error_category2"** %2, align 8
  %4 = bitcast %"class.std::_Iostream_error_category2"* %3 to %"class.std::error_category"*
  call void @"??1error_category@std@@UEAA@XZ"(%"class.std::error_category"* %4) #5
  ret void
}

; Function Attrs: noinline uwtable
define internal void @"??__F_Static@?1???$_Immortalize_memcpy_image@V_Iostream_error_category2@std@@@std@@YAAEBV_Iostream_error_category2@1@XZ@YAXXZ"() #2 {
  call void @"??1_Iostream_error_category2@std@@UEAA@XZ"(%"class.std::_Iostream_error_category2"* bitcast ({ i8**, i64 }* @"?_Static@?1???$_Immortalize_memcpy_image@V_Iostream_error_category2@std@@@std@@YAAEBV_Iostream_error_category2@1@XZ@4V21@A" to %"class.std::_Iostream_error_category2"*))
  ret void
}

; Function Attrs: nounwind
declare dso_local i32 @atexit(void ()*) #5

; Function Attrs: nounwind
declare dso_local void @_Init_thread_footer(i32*) #5

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??_G_Iostream_error_category2@std@@UEAAPEAXI@Z"(%"class.std::_Iostream_error_category2"* %0, i32 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca i32, align 4
  %5 = alloca %"class.std::_Iostream_error_category2"*, align 8
  store i32 %1, i32* %4, align 4
  store %"class.std::_Iostream_error_category2"* %0, %"class.std::_Iostream_error_category2"** %5, align 8
  %6 = load %"class.std::_Iostream_error_category2"*, %"class.std::_Iostream_error_category2"** %5, align 8
  %7 = bitcast %"class.std::_Iostream_error_category2"* %6 to i8*
  store i8* %7, i8** %3, align 8
  %8 = load i32, i32* %4, align 4
  call void @"??1_Iostream_error_category2@std@@UEAA@XZ"(%"class.std::_Iostream_error_category2"* %6) #5
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %12, label %10

10:                                               ; preds = %2
  %11 = bitcast %"class.std::_Iostream_error_category2"* %6 to i8*
  call void @"??3@YAXPEAX@Z"(i8* %11) #20
  br label %12

12:                                               ; preds = %10, %2
  %13 = load i8*, i8** %3, align 8
  ret i8* %13
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"?name@_Iostream_error_category2@std@@UEBAPEBDXZ"(%"class.std::_Iostream_error_category2"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::_Iostream_error_category2"*, align 8
  store %"class.std::_Iostream_error_category2"* %0, %"class.std::_Iostream_error_category2"** %2, align 8
  %3 = load %"class.std::_Iostream_error_category2"*, %"class.std::_Iostream_error_category2"** %2, align 8
  ret i8* getelementptr inbounds ([9 x i8], [9 x i8]* @"??_C@_08LLGCOLLL@iostream?$AA@", i64 0, i64 0)
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?message@_Iostream_error_category2@std@@UEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@H@Z"(%"class.std::_Iostream_error_category2"* %0, %"class.std::basic_string"* noalias sret align 8 %1, i32 %2) unnamed_addr #1 comdat align 2 {
  %4 = alloca i8*, align 8
  %5 = alloca i32, align 4
  %6 = alloca %"class.std::_Iostream_error_category2"*, align 8
  %7 = alloca i64, align 8
  %8 = bitcast %"class.std::basic_string"* %1 to i8*
  store i8* %8, i8** %4, align 8
  store i32 %2, i32* %5, align 4
  store %"class.std::_Iostream_error_category2"* %0, %"class.std::_Iostream_error_category2"** %6, align 8
  %9 = load %"class.std::_Iostream_error_category2"*, %"class.std::_Iostream_error_category2"** %6, align 8
  %10 = load i32, i32* %5, align 4
  %11 = icmp eq i32 %10, 1
  br i1 %11, label %12, label %14

12:                                               ; preds = %3
  store i64 21, i64* %7, align 8
  %13 = call %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@QEBD_K@Z"(%"class.std::basic_string"* %1, i8* getelementptr inbounds ([22 x i8], [22 x i8]* @"?_Iostream_error@?4??message@_Iostream_error_category2@std@@UEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@3@H@Z@4QBDB", i64 0, i64 0), i64 21)
  br label %18

14:                                               ; preds = %3
  %15 = load i32, i32* %5, align 4
  %16 = call i8* @"?_Syserror_map@std@@YAPEBDH@Z"(i32 %15)
  %17 = call %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@QEBD@Z"(%"class.std::basic_string"* %1, i8* %16)
  br label %18

18:                                               ; preds = %14, %12
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?default_error_condition@error_category@std@@UEBA?AVerror_condition@2@H@Z"(%"class.std::error_category"* %0, %"class.std::error_condition"* noalias sret align 8 %1, i32 %2) unnamed_addr #3 comdat align 2 {
  %4 = alloca i8*, align 8
  %5 = alloca i32, align 4
  %6 = alloca %"class.std::error_category"*, align 8
  %7 = bitcast %"class.std::error_condition"* %1 to i8*
  store i8* %7, i8** %4, align 8
  store i32 %2, i32* %5, align 4
  store %"class.std::error_category"* %0, %"class.std::error_category"** %6, align 8
  %8 = load %"class.std::error_category"*, %"class.std::error_category"** %6, align 8
  %9 = load i32, i32* %5, align 4
  %10 = call %"class.std::error_condition"* @"??0error_condition@std@@QEAA@HAEBVerror_category@1@@Z"(%"class.std::error_condition"* %1, i32 %9, %"class.std::error_category"* nonnull align 8 dereferenceable(16) %8) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local zeroext i1 @"?equivalent@error_category@std@@UEBA_NAEBVerror_code@2@H@Z"(%"class.std::error_category"* %0, %"class.std::error_code"* nonnull align 8 dereferenceable(16) %1, i32 %2) unnamed_addr #3 comdat align 2 {
  %4 = alloca i32, align 4
  %5 = alloca %"class.std::error_code"*, align 8
  %6 = alloca %"class.std::error_category"*, align 8
  store i32 %2, i32* %4, align 4
  store %"class.std::error_code"* %1, %"class.std::error_code"** %5, align 8
  store %"class.std::error_category"* %0, %"class.std::error_category"** %6, align 8
  %7 = load %"class.std::error_category"*, %"class.std::error_category"** %6, align 8
  %8 = load %"class.std::error_code"*, %"class.std::error_code"** %5, align 8
  %9 = call nonnull align 8 dereferenceable(16) %"class.std::error_category"* @"?category@error_code@std@@QEBAAEBVerror_category@2@XZ"(%"class.std::error_code"* %8) #5
  %10 = call zeroext i1 @"??8error_category@std@@QEBA_NAEBV01@@Z"(%"class.std::error_category"* %7, %"class.std::error_category"* nonnull align 8 dereferenceable(16) %9) #5
  br i1 %10, label %11, label %16

11:                                               ; preds = %3
  %12 = load %"class.std::error_code"*, %"class.std::error_code"** %5, align 8
  %13 = call i32 @"?value@error_code@std@@QEBAHXZ"(%"class.std::error_code"* %12) #5
  %14 = load i32, i32* %4, align 4
  %15 = icmp eq i32 %13, %14
  br label %16

16:                                               ; preds = %11, %3
  %17 = phi i1 [ false, %3 ], [ %15, %11 ]
  ret i1 %17
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local zeroext i1 @"?equivalent@error_category@std@@UEBA_NHAEBVerror_condition@2@@Z"(%"class.std::error_category"* %0, i32 %1, %"class.std::error_condition"* nonnull align 8 dereferenceable(16) %2) unnamed_addr #3 comdat align 2 {
  %4 = alloca %"class.std::error_condition"*, align 8
  %5 = alloca i32, align 4
  %6 = alloca %"class.std::error_category"*, align 8
  %7 = alloca %"class.std::error_condition", align 8
  store %"class.std::error_condition"* %2, %"class.std::error_condition"** %4, align 8
  store i32 %1, i32* %5, align 4
  store %"class.std::error_category"* %0, %"class.std::error_category"** %6, align 8
  %8 = load %"class.std::error_category"*, %"class.std::error_category"** %6, align 8
  %9 = load %"class.std::error_condition"*, %"class.std::error_condition"** %4, align 8
  %10 = load i32, i32* %5, align 4
  %11 = bitcast %"class.std::error_category"* %8 to void (%"class.std::error_category"*, %"class.std::error_condition"*, i32)***
  %12 = load void (%"class.std::error_category"*, %"class.std::error_condition"*, i32)**, void (%"class.std::error_category"*, %"class.std::error_condition"*, i32)*** %11, align 8
  %13 = getelementptr inbounds void (%"class.std::error_category"*, %"class.std::error_condition"*, i32)*, void (%"class.std::error_category"*, %"class.std::error_condition"*, i32)** %12, i64 3
  %14 = load void (%"class.std::error_category"*, %"class.std::error_condition"*, i32)*, void (%"class.std::error_category"*, %"class.std::error_condition"*, i32)** %13, align 8
  call void %14(%"class.std::error_category"* %8, %"class.std::error_condition"* sret align 8 %7, i32 %10) #5
  %15 = call zeroext i1 @"??8std@@YA_NAEBVerror_condition@0@0@Z"(%"class.std::error_condition"* nonnull align 8 dereferenceable(16) %7, %"class.std::error_condition"* nonnull align 8 dereferenceable(16) %9) #5
  ret i1 %15
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1error_category@std@@UEAA@XZ"(%"class.std::error_category"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::error_category"*, align 8
  store %"class.std::error_category"* %0, %"class.std::error_category"** %2, align 8
  %3 = load %"class.std::error_category"*, %"class.std::error_category"** %2, align 8
  ret void
}

; Function Attrs: nobuiltin nounwind
declare dso_local void @"??3@YAXPEAX@Z"(i8*) #6

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@QEBD_K@Z"(%"class.std::basic_string"* returned %0, i8* %1, i64 %2) unnamed_addr #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %4 = alloca i64, align 8
  %5 = alloca i8*, align 8
  %6 = alloca %"class.std::basic_string"*, align 8
  %7 = alloca %"struct.std::_Zero_then_variadic_args_t", align 1
  %8 = alloca %"struct.std::_Fake_allocator"*, align 8
  %9 = alloca %"struct.std::_Fake_proxy_ptr_impl", align 1
  store i64 %2, i64* %4, align 8
  store i8* %1, i8** %5, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %6, align 8
  %10 = load %"class.std::basic_string"*, %"class.std::basic_string"** %6, align 8
  %11 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %10, i32 0, i32 0
  %12 = getelementptr inbounds %"struct.std::_Zero_then_variadic_args_t", %"struct.std::_Zero_then_variadic_args_t"* %7, i32 0, i32 0
  %13 = load i8, i8* %12, align 1
  %14 = call %"class.std::_Compressed_pair"* @"??$?0$$V@?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@U_Zero_then_variadic_args_t@1@@Z"(%"class.std::_Compressed_pair"* %11, i8 %13) #5
  store %"struct.std::_Fake_allocator"* @"?_Fake_alloc@std@@3U_Fake_allocator@1@B", %"struct.std::_Fake_allocator"** %8, align 8
  %15 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %10, i32 0, i32 0
  %16 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %15, i32 0, i32 0
  %17 = bitcast %"class.std::_String_val"* %16 to %"struct.std::_Container_base0"*
  %18 = call %"struct.std::_Fake_proxy_ptr_impl"* @"??0_Fake_proxy_ptr_impl@std@@QEAA@AEBU_Fake_allocator@1@AEBU_Container_base0@1@@Z"(%"struct.std::_Fake_proxy_ptr_impl"* %9, %"struct.std::_Fake_allocator"* nonnull align 1 dereferenceable(1) @"?_Fake_alloc@std@@3U_Fake_allocator@1@B", %"struct.std::_Container_base0"* nonnull align 1 dereferenceable(1) %17) #5
  call void @"?_Tidy_init@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXXZ"(%"class.std::basic_string"* %10) #5
  %19 = load i64, i64* %4, align 8
  %20 = load i8*, i8** %5, align 8
  %21 = invoke nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@QEBD_K@Z"(%"class.std::basic_string"* %10, i8* %20, i64 %19)
          to label %22 unwind label %23

22:                                               ; preds = %3
  call void @"?_Release@_Fake_proxy_ptr_impl@std@@QEAAXXZ"(%"struct.std::_Fake_proxy_ptr_impl"* %9) #5
  ret %"class.std::basic_string"* %10

23:                                               ; preds = %3
  %24 = cleanuppad within none []
  call void @"??1?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@XZ"(%"class.std::_Compressed_pair"* %11) #5 [ "funclet"(token %24) ]
  cleanupret from %24 unwind to caller
}

declare dso_local i8* @"?_Syserror_map@std@@YAPEBDH@Z"(i32) #4

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@QEBD@Z"(%"class.std::basic_string"* returned %0, i8* %1) unnamed_addr #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::basic_string"*, align 8
  %5 = alloca %"struct.std::_Zero_then_variadic_args_t", align 1
  %6 = alloca %"struct.std::_Fake_allocator"*, align 8
  %7 = alloca %"struct.std::_Fake_proxy_ptr_impl", align 1
  store i8* %1, i8** %3, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %4, align 8
  %8 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  %9 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %8, i32 0, i32 0
  %10 = getelementptr inbounds %"struct.std::_Zero_then_variadic_args_t", %"struct.std::_Zero_then_variadic_args_t"* %5, i32 0, i32 0
  %11 = load i8, i8* %10, align 1
  %12 = call %"class.std::_Compressed_pair"* @"??$?0$$V@?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@U_Zero_then_variadic_args_t@1@@Z"(%"class.std::_Compressed_pair"* %9, i8 %11) #5
  store %"struct.std::_Fake_allocator"* @"?_Fake_alloc@std@@3U_Fake_allocator@1@B", %"struct.std::_Fake_allocator"** %6, align 8
  %13 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %8, i32 0, i32 0
  %14 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %13, i32 0, i32 0
  %15 = bitcast %"class.std::_String_val"* %14 to %"struct.std::_Container_base0"*
  %16 = call %"struct.std::_Fake_proxy_ptr_impl"* @"??0_Fake_proxy_ptr_impl@std@@QEAA@AEBU_Fake_allocator@1@AEBU_Container_base0@1@@Z"(%"struct.std::_Fake_proxy_ptr_impl"* %7, %"struct.std::_Fake_allocator"* nonnull align 1 dereferenceable(1) @"?_Fake_alloc@std@@3U_Fake_allocator@1@B", %"struct.std::_Container_base0"* nonnull align 1 dereferenceable(1) %15) #5
  call void @"?_Tidy_init@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXXZ"(%"class.std::basic_string"* %8) #5
  %17 = load i8*, i8** %3, align 8
  %18 = invoke nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@QEBD@Z"(%"class.std::basic_string"* %8, i8* %17)
          to label %19 unwind label %20

19:                                               ; preds = %2
  call void @"?_Release@_Fake_proxy_ptr_impl@std@@QEAAXXZ"(%"struct.std::_Fake_proxy_ptr_impl"* %7) #5
  ret %"class.std::basic_string"* %8

20:                                               ; preds = %2
  %21 = cleanuppad within none []
  call void @"??1?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@XZ"(%"class.std::_Compressed_pair"* %9) #5 [ "funclet"(token %21) ]
  cleanupret from %21 unwind to caller
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::_Compressed_pair"* @"??$?0$$V@?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@U_Zero_then_variadic_args_t@1@@Z"(%"class.std::_Compressed_pair"* returned %0, i8 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca %"struct.std::_Zero_then_variadic_args_t", align 1
  %4 = alloca %"class.std::_Compressed_pair"*, align 8
  %5 = getelementptr inbounds %"struct.std::_Zero_then_variadic_args_t", %"struct.std::_Zero_then_variadic_args_t"* %3, i32 0, i32 0
  store i8 %1, i8* %5, align 1
  store %"class.std::_Compressed_pair"* %0, %"class.std::_Compressed_pair"** %4, align 8
  %6 = load %"class.std::_Compressed_pair"*, %"class.std::_Compressed_pair"** %4, align 8
  %7 = bitcast %"class.std::_Compressed_pair"* %6 to %"class.std::allocator"*
  %8 = call %"class.std::allocator"* @"??0?$allocator@D@std@@QEAA@XZ"(%"class.std::allocator"* %7) #5
  %9 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %6, i32 0, i32 0
  %10 = call %"class.std::_String_val"* @"??0?$_String_val@U?$_Simple_types@D@std@@@std@@QEAA@XZ"(%"class.std::_String_val"* %9) #5
  ret %"class.std::_Compressed_pair"* %6
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"struct.std::_Fake_proxy_ptr_impl"* @"??0_Fake_proxy_ptr_impl@std@@QEAA@AEBU_Fake_allocator@1@AEBU_Container_base0@1@@Z"(%"struct.std::_Fake_proxy_ptr_impl"* returned %0, %"struct.std::_Fake_allocator"* nonnull align 1 dereferenceable(1) %1, %"struct.std::_Container_base0"* nonnull align 1 dereferenceable(1) %2) unnamed_addr #3 comdat align 2 {
  %4 = alloca %"struct.std::_Container_base0"*, align 8
  %5 = alloca %"struct.std::_Fake_allocator"*, align 8
  %6 = alloca %"struct.std::_Fake_proxy_ptr_impl"*, align 8
  store %"struct.std::_Container_base0"* %2, %"struct.std::_Container_base0"** %4, align 8
  store %"struct.std::_Fake_allocator"* %1, %"struct.std::_Fake_allocator"** %5, align 8
  store %"struct.std::_Fake_proxy_ptr_impl"* %0, %"struct.std::_Fake_proxy_ptr_impl"** %6, align 8
  %7 = load %"struct.std::_Fake_proxy_ptr_impl"*, %"struct.std::_Fake_proxy_ptr_impl"** %6, align 8
  ret %"struct.std::_Fake_proxy_ptr_impl"* %7
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?_Tidy_init@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXXZ"(%"class.std::basic_string"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::basic_string"*, align 8
  %3 = alloca %"class.std::_String_val"*, align 8
  %4 = alloca i8, align 1
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %2, align 8
  %5 = load %"class.std::basic_string"*, %"class.std::basic_string"** %2, align 8
  %6 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %5, i32 0, i32 0
  %7 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %6, i32 0, i32 0
  store %"class.std::_String_val"* %7, %"class.std::_String_val"** %3, align 8
  %8 = load %"class.std::_String_val"*, %"class.std::_String_val"** %3, align 8
  %9 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %8, i32 0, i32 1
  store i64 0, i64* %9, align 8
  %10 = load %"class.std::_String_val"*, %"class.std::_String_val"** %3, align 8
  %11 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %10, i32 0, i32 2
  store i64 15, i64* %11, align 8
  store i8 0, i8* %4, align 1
  %12 = load %"class.std::_String_val"*, %"class.std::_String_val"** %3, align 8
  %13 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %12, i32 0, i32 0
  %14 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %13 to [16 x i8]*
  %15 = getelementptr inbounds [16 x i8], [16 x i8]* %14, i64 0, i64 0
  call void @"?assign@?$_Narrow_char_traits@DH@std@@SAXAEADAEBD@Z"(i8* nonnull align 1 dereferenceable(1) %15, i8* nonnull align 1 dereferenceable(1) %4) #5
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@QEBD_K@Z"(%"class.std::basic_string"* %0, i8* %1, i64 %2) #1 comdat align 2 {
  %4 = alloca %"class.std::basic_string"*, align 8
  %5 = alloca i64, align 8
  %6 = alloca i8*, align 8
  %7 = alloca %"class.std::basic_string"*, align 8
  %8 = alloca i8*, align 8
  %9 = alloca i8, align 1
  %10 = alloca %class.anon, align 1
  store i64 %2, i64* %5, align 8
  store i8* %1, i8** %6, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %7, align 8
  %11 = load %"class.std::basic_string"*, %"class.std::basic_string"** %7, align 8
  %12 = load i64, i64* %5, align 8
  %13 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %11, i32 0, i32 0
  %14 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %13, i32 0, i32 0
  %15 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %14, i32 0, i32 2
  %16 = load i64, i64* %15, align 8
  %17 = icmp ule i64 %12, %16
  br i1 %17, label %18, label %33

18:                                               ; preds = %3
  %19 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %11, i32 0, i32 0
  %20 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %19, i32 0, i32 0
  %21 = call i8* @"?_Myptr@?$_String_val@U?$_Simple_types@D@std@@@std@@QEAAPEADXZ"(%"class.std::_String_val"* %20) #5
  store i8* %21, i8** %8, align 8
  %22 = load i64, i64* %5, align 8
  %23 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %11, i32 0, i32 0
  %24 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %23, i32 0, i32 0
  %25 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %24, i32 0, i32 1
  store i64 %22, i64* %25, align 8
  %26 = load i64, i64* %5, align 8
  %27 = load i8*, i8** %6, align 8
  %28 = load i8*, i8** %8, align 8
  %29 = call i8* @"?move@?$_Char_traits@DH@std@@SAPEADQEADQEBD_K@Z"(i8* %28, i8* %27, i64 %26) #5
  store i8 0, i8* %9, align 1
  %30 = load i8*, i8** %8, align 8
  %31 = load i64, i64* %5, align 8
  %32 = getelementptr inbounds i8, i8* %30, i64 %31
  call void @"?assign@?$_Narrow_char_traits@DH@std@@SAXAEADAEBD@Z"(i8* nonnull align 1 dereferenceable(1) %32, i8* nonnull align 1 dereferenceable(1) %9) #5
  store %"class.std::basic_string"* %11, %"class.std::basic_string"** %4, align 8
  br label %39

33:                                               ; preds = %3
  %34 = load i8*, i8** %6, align 8
  %35 = load i64, i64* %5, align 8
  %36 = getelementptr inbounds %class.anon, %class.anon* %10, i32 0, i32 0
  %37 = load i8, i8* %36, align 1
  %38 = call nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"??$_Reallocate_for@V<lambda_1>@?0??assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV34@QEBD_K@Z@PEBD@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV01@_KV<lambda_1>@?0??assign@01@QEAAAEAV01@QEBD0@Z@PEBD@Z"(%"class.std::basic_string"* %11, i64 %35, i8 %37, i8* %34)
  store %"class.std::basic_string"* %38, %"class.std::basic_string"** %4, align 8
  br label %39

39:                                               ; preds = %33, %18
  %40 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  ret %"class.std::basic_string"* %40
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?_Release@_Fake_proxy_ptr_impl@std@@QEAAXXZ"(%"struct.std::_Fake_proxy_ptr_impl"* %0) #3 comdat align 2 {
  %2 = alloca %"struct.std::_Fake_proxy_ptr_impl"*, align 8
  store %"struct.std::_Fake_proxy_ptr_impl"* %0, %"struct.std::_Fake_proxy_ptr_impl"** %2, align 8
  %3 = load %"struct.std::_Fake_proxy_ptr_impl"*, %"struct.std::_Fake_proxy_ptr_impl"** %2, align 8
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@XZ"(%"class.std::_Compressed_pair"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::_Compressed_pair"*, align 8
  store %"class.std::_Compressed_pair"* %0, %"class.std::_Compressed_pair"** %2, align 8
  %3 = load %"class.std::_Compressed_pair"*, %"class.std::_Compressed_pair"** %2, align 8
  %4 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %3, i32 0, i32 0
  call void @"??1?$_String_val@U?$_Simple_types@D@std@@@std@@QEAA@XZ"(%"class.std::_String_val"* %4) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::allocator"* @"??0?$allocator@D@std@@QEAA@XZ"(%"class.std::allocator"* returned %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::allocator"*, align 8
  store %"class.std::allocator"* %0, %"class.std::allocator"** %2, align 8
  %3 = load %"class.std::allocator"*, %"class.std::allocator"** %2, align 8
  ret %"class.std::allocator"* %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::_String_val"* @"??0?$_String_val@U?$_Simple_types@D@std@@@std@@QEAA@XZ"(%"class.std::_String_val"* returned %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::_String_val"*, align 8
  store %"class.std::_String_val"* %0, %"class.std::_String_val"** %2, align 8
  %3 = load %"class.std::_String_val"*, %"class.std::_String_val"** %2, align 8
  %4 = bitcast %"class.std::_String_val"* %3 to %"struct.std::_Container_base0"*
  %5 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %3, i32 0, i32 0
  %6 = call %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* @"??0_Bxty@?$_String_val@U?$_Simple_types@D@std@@@std@@QEAA@XZ"(%"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %5) #5
  %7 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %3, i32 0, i32 1
  store i64 0, i64* %7, align 8
  %8 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %3, i32 0, i32 2
  store i64 0, i64* %8, align 8
  ret %"class.std::_String_val"* %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* @"??0_Bxty@?$_String_val@U?$_Simple_types@D@std@@@std@@QEAA@XZ"(%"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* returned %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"*, align 8
  store %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %0, %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"** %2, align 8
  %3 = load %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"*, %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"** %2, align 8
  %4 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %3 to i8**
  store i8* null, i8** %4, align 8
  ret %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?assign@?$_Narrow_char_traits@DH@std@@SAXAEADAEBD@Z"(i8* nonnull align 1 dereferenceable(1) %0, i8* nonnull align 1 dereferenceable(1) %1) #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca i8*, align 8
  store i8* %1, i8** %3, align 8
  store i8* %0, i8** %4, align 8
  %5 = load i8*, i8** %3, align 8
  %6 = load i8, i8* %5, align 1
  %7 = load i8*, i8** %4, align 8
  store i8 %6, i8* %7, align 1
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"?_Myptr@?$_String_val@U?$_Simple_types@D@std@@@std@@QEAAPEADXZ"(%"class.std::_String_val"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::_String_val"*, align 8
  %3 = alloca i8*, align 8
  store %"class.std::_String_val"* %0, %"class.std::_String_val"** %2, align 8
  %4 = load %"class.std::_String_val"*, %"class.std::_String_val"** %2, align 8
  %5 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %4, i32 0, i32 0
  %6 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %5 to [16 x i8]*
  %7 = getelementptr inbounds [16 x i8], [16 x i8]* %6, i64 0, i64 0
  store i8* %7, i8** %3, align 8
  %8 = call zeroext i1 @"?_Large_string_engaged@?$_String_val@U?$_Simple_types@D@std@@@std@@QEBA_NXZ"(%"class.std::_String_val"* %4) #5
  br i1 %8, label %9, label %14

9:                                                ; preds = %1
  %10 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %4, i32 0, i32 0
  %11 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %10 to i8**
  %12 = load i8*, i8** %11, align 8
  %13 = call i8* @"??$_Unfancy@D@std@@YAPEADPEAD@Z"(i8* %12) #5
  store i8* %13, i8** %3, align 8
  br label %14

14:                                               ; preds = %9, %1
  %15 = load i8*, i8** %3, align 8
  ret i8* %15
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"?move@?$_Char_traits@DH@std@@SAPEADQEADQEBD_K@Z"(i8* %0, i8* %1, i64 %2) #3 comdat align 2 {
  %4 = alloca i64, align 8
  %5 = alloca i8*, align 8
  %6 = alloca i8*, align 8
  store i64 %2, i64* %4, align 8
  store i8* %1, i8** %5, align 8
  store i8* %0, i8** %6, align 8
  %7 = load i8*, i8** %6, align 8
  %8 = load i8*, i8** %5, align 8
  %9 = load i64, i64* %4, align 8
  %10 = mul i64 %9, 1
  call void @llvm.memmove.p0i8.p0i8.i64(i8* align 1 %7, i8* align 1 %8, i64 %10, i1 false)
  %11 = load i8*, i8** %6, align 8
  ret i8* %11
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"??$_Reallocate_for@V<lambda_1>@?0??assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV34@QEBD_K@Z@PEBD@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV01@_KV<lambda_1>@?0??assign@01@QEAAAEAV01@QEBD0@Z@PEBD@Z"(%"class.std::basic_string"* %0, i64 %1, i8 %2, i8* %3) #1 comdat align 2 {
  %5 = alloca %class.anon, align 1
  %6 = alloca i8*, align 8
  %7 = alloca i64, align 8
  %8 = alloca %"class.std::basic_string"*, align 8
  %9 = alloca i64, align 8
  %10 = alloca i64, align 8
  %11 = alloca %"class.std::allocator"*, align 8
  %12 = alloca i8*, align 8
  %13 = getelementptr inbounds %class.anon, %class.anon* %5, i32 0, i32 0
  store i8 %2, i8* %13, align 1
  store i8* %3, i8** %6, align 8
  store i64 %1, i64* %7, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %8, align 8
  %14 = load %"class.std::basic_string"*, %"class.std::basic_string"** %8, align 8
  %15 = load i64, i64* %7, align 8
  %16 = call i64 @"?max_size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %14) #5
  %17 = icmp ugt i64 %15, %16
  br i1 %17, label %18, label %19

18:                                               ; preds = %4
  call void @"?_Xlen_string@std@@YAXXZ"() #19
  unreachable

19:                                               ; preds = %4
  %20 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %21 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %20, i32 0, i32 0
  %22 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %21, i32 0, i32 2
  %23 = load i64, i64* %22, align 8
  store i64 %23, i64* %9, align 8
  %24 = load i64, i64* %7, align 8
  %25 = call i64 @"?_Calculate_growth@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEBA_K_K@Z"(%"class.std::basic_string"* %14, i64 %24) #5
  store i64 %25, i64* %10, align 8
  %26 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Getal@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV?$allocator@D@2@XZ"(%"class.std::basic_string"* %14) #5
  store %"class.std::allocator"* %26, %"class.std::allocator"** %11, align 8
  %27 = load %"class.std::allocator"*, %"class.std::allocator"** %11, align 8
  %28 = load i64, i64* %10, align 8
  %29 = add i64 %28, 1
  %30 = call i8* @"?allocate@?$allocator@D@std@@QEAAPEAD_K@Z"(%"class.std::allocator"* %27, i64 %29)
  store i8* %30, i8** %12, align 8
  %31 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %32 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %31, i32 0, i32 0
  %33 = bitcast %"class.std::_String_val"* %32 to %"struct.std::_Container_base0"*
  call void @"?_Orphan_all@_Container_base0@std@@QEAAXXZ"(%"struct.std::_Container_base0"* %33) #5
  %34 = load i64, i64* %7, align 8
  %35 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %36 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %35, i32 0, i32 0
  %37 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %36, i32 0, i32 1
  store i64 %34, i64* %37, align 8
  %38 = load i64, i64* %10, align 8
  %39 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %40 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %39, i32 0, i32 0
  %41 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %40, i32 0, i32 2
  store i64 %38, i64* %41, align 8
  %42 = load i8*, i8** %6, align 8
  %43 = load i64, i64* %7, align 8
  %44 = load i8*, i8** %12, align 8
  %45 = call i8* @"??$_Unfancy@D@std@@YAPEADPEAD@Z"(i8* %44) #5
  call void @"??R<lambda_1>@?0??assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@QEBD_K@Z@QEBA?A?<auto>@@QEAD10@Z"(%class.anon* %5, i8* %45, i64 %43, i8* %42)
  %46 = load i64, i64* %9, align 8
  %47 = icmp ule i64 16, %46
  br i1 %47, label %48, label %62

48:                                               ; preds = %19
  %49 = load %"class.std::allocator"*, %"class.std::allocator"** %11, align 8
  %50 = load i64, i64* %9, align 8
  %51 = add i64 %50, 1
  %52 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %53 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %52, i32 0, i32 0
  %54 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %53, i32 0, i32 0
  %55 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %54 to i8**
  %56 = load i8*, i8** %55, align 8
  call void @"?deallocate@?$allocator@D@std@@QEAAXQEAD_K@Z"(%"class.std::allocator"* %49, i8* %56, i64 %51)
  %57 = load i8*, i8** %12, align 8
  %58 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %59 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %58, i32 0, i32 0
  %60 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %59, i32 0, i32 0
  %61 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %60 to i8**
  store i8* %57, i8** %61, align 8
  br label %67

62:                                               ; preds = %19
  %63 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %64 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %63, i32 0, i32 0
  %65 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %64, i32 0, i32 0
  %66 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %65 to i8**
  call void @"??$_Construct_in_place@PEADAEBQEAD@std@@YAXAEAPEADAEBQEAD@Z"(i8** nonnull align 8 dereferenceable(8) %66, i8** nonnull align 8 dereferenceable(8) %12) #5
  br label %67

67:                                               ; preds = %62, %48
  ret %"class.std::basic_string"* %14
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local zeroext i1 @"?_Large_string_engaged@?$_String_val@U?$_Simple_types@D@std@@@std@@QEBA_NXZ"(%"class.std::_String_val"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::_String_val"*, align 8
  store %"class.std::_String_val"* %0, %"class.std::_String_val"** %2, align 8
  %3 = load %"class.std::_String_val"*, %"class.std::_String_val"** %2, align 8
  %4 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %3, i32 0, i32 2
  %5 = load i64, i64* %4, align 8
  %6 = icmp ule i64 16, %5
  ret i1 %6
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??$_Unfancy@D@std@@YAPEADPEAD@Z"(i8* %0) #3 comdat {
  %2 = alloca i8*, align 8
  store i8* %0, i8** %2, align 8
  %3 = load i8*, i8** %2, align 8
  ret i8* %3
}

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.memmove.p0i8.p0i8.i64(i8* nocapture, i8* nocapture readonly, i64, i1 immarg) #7

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i64 @"?max_size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::basic_string"*, align 8
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %2, align 8
  %8 = load %"class.std::basic_string"*, %"class.std::basic_string"** %2, align 8
  %9 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Getal@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEBAAEBV?$allocator@D@2@XZ"(%"class.std::basic_string"* %8) #5
  %10 = call i64 @"?max_size@?$_Default_allocator_traits@V?$allocator@D@std@@@std@@SA_KAEBV?$allocator@D@2@@Z"(%"class.std::allocator"* nonnull align 1 dereferenceable(1) %9) #5
  store i64 %10, i64* %3, align 8
  store i64 16, i64* %5, align 8
  %11 = call nonnull align 8 dereferenceable(8) i64* @"??$max@_K@std@@YAAEB_KAEB_K0@Z"(i64* nonnull align 8 dereferenceable(8) %3, i64* nonnull align 8 dereferenceable(8) %5) #5
  %12 = load i64, i64* %11, align 8
  store i64 %12, i64* %4, align 8
  %13 = load i64, i64* %4, align 8
  %14 = sub i64 %13, 1
  store i64 %14, i64* %6, align 8
  %15 = call i64 @"?max@?$numeric_limits@_J@std@@SA_JXZ"() #5
  store i64 %15, i64* %7, align 8
  %16 = call nonnull align 8 dereferenceable(8) i64* @"??$min@_K@std@@YAAEB_KAEB_K0@Z"(i64* nonnull align 8 dereferenceable(8) %7, i64* nonnull align 8 dereferenceable(8) %6) #5
  %17 = load i64, i64* %16, align 8
  ret i64 %17
}

; Function Attrs: noinline noreturn optnone uwtable
define linkonce_odr dso_local void @"?_Xlen_string@std@@YAXXZ"() #8 comdat {
  call void @"?_Xlength_error@std@@YAXPEBD@Z"(i8* getelementptr inbounds ([16 x i8], [16 x i8]* @"??_C@_0BA@JFNIOLAK@string?5too?5long?$AA@", i64 0, i64 0)) #19
  unreachable
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i64 @"?_Calculate_growth@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEBA_K_K@Z"(%"class.std::basic_string"* %0, i64 %1) #3 comdat align 2 {
  %3 = alloca i64, align 8
  %4 = alloca %"class.std::basic_string"*, align 8
  store i64 %1, i64* %3, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %4, align 8
  %5 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  %6 = call i64 @"?max_size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %5) #5
  %7 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %5, i32 0, i32 0
  %8 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %7, i32 0, i32 0
  %9 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %8, i32 0, i32 2
  %10 = load i64, i64* %9, align 8
  %11 = load i64, i64* %3, align 8
  %12 = call i64 @"?_Calculate_growth@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@CA_K_K00@Z"(i64 %11, i64 %10, i64 %6) #5
  ret i64 %12
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Getal@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV?$allocator@D@2@XZ"(%"class.std::basic_string"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::basic_string"*, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %2, align 8
  %3 = load %"class.std::basic_string"*, %"class.std::basic_string"** %2, align 8
  %4 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %3, i32 0, i32 0
  %5 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Get_first@?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAAAEAV?$allocator@D@2@XZ"(%"class.std::_Compressed_pair"* %4) #5
  ret %"class.std::allocator"* %5
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i8* @"?allocate@?$allocator@D@std@@QEAAPEAD_K@Z"(%"class.std::allocator"* %0, i64 %1) #1 comdat align 2 {
  %3 = alloca i64, align 8
  %4 = alloca %"class.std::allocator"*, align 8
  store i64 %1, i64* %3, align 8
  store %"class.std::allocator"* %0, %"class.std::allocator"** %4, align 8
  %5 = load %"class.std::allocator"*, %"class.std::allocator"** %4, align 8
  %6 = load i64, i64* %3, align 8
  %7 = call i64 @"??$_Get_size_of_n@$00@std@@YA_K_K@Z"(i64 %6)
  %8 = call i8* @"??$_Allocate@$0BA@U_Default_allocate_traits@std@@$0A@@std@@YAPEAX_K@Z"(i64 %7)
  ret i8* %8
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?_Orphan_all@_Container_base0@std@@QEAAXXZ"(%"struct.std::_Container_base0"* %0) #3 comdat align 2 {
  %2 = alloca %"struct.std::_Container_base0"*, align 8
  store %"struct.std::_Container_base0"* %0, %"struct.std::_Container_base0"** %2, align 8
  %3 = load %"struct.std::_Container_base0"*, %"struct.std::_Container_base0"** %2, align 8
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??R<lambda_1>@?0??assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@QEBD_K@Z@QEBA?A?<auto>@@QEAD10@Z"(%class.anon* %0, i8* %1, i64 %2, i8* %3) #3 comdat align 2 {
  %5 = alloca i8*, align 8
  %6 = alloca i64, align 8
  %7 = alloca i8*, align 8
  %8 = alloca %class.anon*, align 8
  %9 = alloca i8, align 1
  store i8* %3, i8** %5, align 8
  store i64 %2, i64* %6, align 8
  store i8* %1, i8** %7, align 8
  store %class.anon* %0, %class.anon** %8, align 8
  %10 = load %class.anon*, %class.anon** %8, align 8
  %11 = load i64, i64* %6, align 8
  %12 = load i8*, i8** %5, align 8
  %13 = load i8*, i8** %7, align 8
  %14 = call i8* @"?copy@?$_Char_traits@DH@std@@SAPEADQEADQEBD_K@Z"(i8* %13, i8* %12, i64 %11) #5
  store i8 0, i8* %9, align 1
  %15 = load i8*, i8** %7, align 8
  %16 = load i64, i64* %6, align 8
  %17 = getelementptr inbounds i8, i8* %15, i64 %16
  call void @"?assign@?$_Narrow_char_traits@DH@std@@SAXAEADAEBD@Z"(i8* nonnull align 1 dereferenceable(1) %17, i8* nonnull align 1 dereferenceable(1) %9) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?deallocate@?$allocator@D@std@@QEAAXQEAD_K@Z"(%"class.std::allocator"* %0, i8* %1, i64 %2) #3 comdat align 2 {
  %4 = alloca i64, align 8
  %5 = alloca i8*, align 8
  %6 = alloca %"class.std::allocator"*, align 8
  store i64 %2, i64* %4, align 8
  store i8* %1, i8** %5, align 8
  store %"class.std::allocator"* %0, %"class.std::allocator"** %6, align 8
  %7 = load %"class.std::allocator"*, %"class.std::allocator"** %6, align 8
  %8 = load i64, i64* %4, align 8
  %9 = mul i64 1, %8
  %10 = load i8*, i8** %5, align 8
  call void @"??$_Deallocate@$0BA@$0A@@std@@YAXPEAX_K@Z"(i8* %10, i64 %9) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??$_Construct_in_place@PEADAEBQEAD@std@@YAXAEAPEADAEBQEAD@Z"(i8** nonnull align 8 dereferenceable(8) %0, i8** nonnull align 8 dereferenceable(8) %1) #3 comdat {
  %3 = alloca i8**, align 8
  %4 = alloca i8**, align 8
  store i8** %1, i8*** %3, align 8
  store i8** %0, i8*** %4, align 8
  %5 = load i8**, i8*** %4, align 8
  %6 = call i8** @"??$addressof@PEAD@std@@YAPEAPEADAEAPEAD@Z"(i8** nonnull align 8 dereferenceable(8) %5) #5
  %7 = call i8* @"??$_Voidify_iter@PEAPEAD@std@@YAPEAXPEAPEAD@Z"(i8** %6) #5
  %8 = bitcast i8* %7 to i8**
  %9 = load i8**, i8*** %3, align 8
  %10 = call nonnull align 8 dereferenceable(8) i8** @"??$forward@AEBQEAD@std@@YAAEBQEADAEBQEAD@Z"(i8** nonnull align 8 dereferenceable(8) %9) #5
  %11 = load i8*, i8** %10, align 8
  store i8* %11, i8** %8, align 8
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i64 @"?max_size@?$_Default_allocator_traits@V?$allocator@D@std@@@std@@SA_KAEBV?$allocator@D@2@@Z"(%"class.std::allocator"* nonnull align 1 dereferenceable(1) %0) #3 comdat align 2 {
  %2 = alloca %"class.std::allocator"*, align 8
  store %"class.std::allocator"* %0, %"class.std::allocator"** %2, align 8
  ret i64 -1
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Getal@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEBAAEBV?$allocator@D@2@XZ"(%"class.std::basic_string"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::basic_string"*, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %2, align 8
  %3 = load %"class.std::basic_string"*, %"class.std::basic_string"** %2, align 8
  %4 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %3, i32 0, i32 0
  %5 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Get_first@?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEBAAEBV?$allocator@D@2@XZ"(%"class.std::_Compressed_pair"* %4) #5
  ret %"class.std::allocator"* %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(8) i64* @"??$max@_K@std@@YAAEB_KAEB_K0@Z"(i64* nonnull align 8 dereferenceable(8) %0, i64* nonnull align 8 dereferenceable(8) %1) #3 comdat {
  %3 = alloca i64*, align 8
  %4 = alloca i64*, align 8
  store i64* %1, i64** %3, align 8
  store i64* %0, i64** %4, align 8
  %5 = load i64*, i64** %4, align 8
  %6 = load i64, i64* %5, align 8
  %7 = load i64*, i64** %3, align 8
  %8 = load i64, i64* %7, align 8
  %9 = icmp ult i64 %6, %8
  br i1 %9, label %10, label %12

10:                                               ; preds = %2
  %11 = load i64*, i64** %3, align 8
  br label %14

12:                                               ; preds = %2
  %13 = load i64*, i64** %4, align 8
  br label %14

14:                                               ; preds = %12, %10
  %15 = phi i64* [ %11, %10 ], [ %13, %12 ]
  ret i64* %15
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(8) i64* @"??$min@_K@std@@YAAEB_KAEB_K0@Z"(i64* nonnull align 8 dereferenceable(8) %0, i64* nonnull align 8 dereferenceable(8) %1) #3 comdat {
  %3 = alloca i64*, align 8
  %4 = alloca i64*, align 8
  store i64* %1, i64** %3, align 8
  store i64* %0, i64** %4, align 8
  %5 = load i64*, i64** %3, align 8
  %6 = load i64, i64* %5, align 8
  %7 = load i64*, i64** %4, align 8
  %8 = load i64, i64* %7, align 8
  %9 = icmp ult i64 %6, %8
  br i1 %9, label %10, label %12

10:                                               ; preds = %2
  %11 = load i64*, i64** %3, align 8
  br label %14

12:                                               ; preds = %2
  %13 = load i64*, i64** %4, align 8
  br label %14

14:                                               ; preds = %12, %10
  %15 = phi i64* [ %11, %10 ], [ %13, %12 ]
  ret i64* %15
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i64 @"?max@?$numeric_limits@_J@std@@SA_JXZ"() #3 comdat align 2 {
  ret i64 9223372036854775807
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Get_first@?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEBAAEBV?$allocator@D@2@XZ"(%"class.std::_Compressed_pair"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::_Compressed_pair"*, align 8
  store %"class.std::_Compressed_pair"* %0, %"class.std::_Compressed_pair"** %2, align 8
  %3 = load %"class.std::_Compressed_pair"*, %"class.std::_Compressed_pair"** %2, align 8
  %4 = bitcast %"class.std::_Compressed_pair"* %3 to %"class.std::allocator"*
  ret %"class.std::allocator"* %4
}

; Function Attrs: noreturn
declare dso_local void @"?_Xlength_error@std@@YAXPEBD@Z"(i8*) #9

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i64 @"?_Calculate_growth@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@CA_K_K00@Z"(i64 %0, i64 %1, i64 %2) #3 comdat align 2 {
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  %9 = alloca i64, align 8
  store i64 %2, i64* %5, align 8
  store i64 %1, i64* %6, align 8
  store i64 %0, i64* %7, align 8
  %10 = load i64, i64* %7, align 8
  %11 = or i64 %10, 15
  store i64 %11, i64* %8, align 8
  %12 = load i64, i64* %8, align 8
  %13 = load i64, i64* %5, align 8
  %14 = icmp ugt i64 %12, %13
  br i1 %14, label %15, label %17

15:                                               ; preds = %3
  %16 = load i64, i64* %5, align 8
  store i64 %16, i64* %4, align 8
  br label %33

17:                                               ; preds = %3
  %18 = load i64, i64* %6, align 8
  %19 = load i64, i64* %5, align 8
  %20 = load i64, i64* %6, align 8
  %21 = udiv i64 %20, 2
  %22 = sub i64 %19, %21
  %23 = icmp ugt i64 %18, %22
  br i1 %23, label %24, label %26

24:                                               ; preds = %17
  %25 = load i64, i64* %5, align 8
  store i64 %25, i64* %4, align 8
  br label %33

26:                                               ; preds = %17
  %27 = load i64, i64* %6, align 8
  %28 = load i64, i64* %6, align 8
  %29 = udiv i64 %28, 2
  %30 = add i64 %27, %29
  store i64 %30, i64* %9, align 8
  %31 = call nonnull align 8 dereferenceable(8) i64* @"??$max@_K@std@@YAAEB_KAEB_K0@Z"(i64* nonnull align 8 dereferenceable(8) %8, i64* nonnull align 8 dereferenceable(8) %9) #5
  %32 = load i64, i64* %31, align 8
  store i64 %32, i64* %4, align 8
  br label %33

33:                                               ; preds = %26, %24, %15
  %34 = load i64, i64* %4, align 8
  ret i64 %34
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Get_first@?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAAAEAV?$allocator@D@2@XZ"(%"class.std::_Compressed_pair"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::_Compressed_pair"*, align 8
  store %"class.std::_Compressed_pair"* %0, %"class.std::_Compressed_pair"** %2, align 8
  %3 = load %"class.std::_Compressed_pair"*, %"class.std::_Compressed_pair"** %2, align 8
  %4 = bitcast %"class.std::_Compressed_pair"* %3 to %"class.std::allocator"*
  ret %"class.std::allocator"* %4
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i8* @"??$_Allocate@$0BA@U_Default_allocate_traits@std@@$0A@@std@@YAPEAX_K@Z"(i64 %0) #1 comdat {
  %2 = alloca i8*, align 8
  %3 = alloca i64, align 8
  store i64 %0, i64* %3, align 8
  %4 = load i64, i64* %3, align 8
  %5 = icmp uge i64 %4, 4096
  br i1 %5, label %6, label %9

6:                                                ; preds = %1
  %7 = load i64, i64* %3, align 8
  %8 = call i8* @"??$_Allocate_manually_vector_aligned@U_Default_allocate_traits@std@@@std@@YAPEAX_K@Z"(i64 %7)
  store i8* %8, i8** %2, align 8
  br label %16

9:                                                ; preds = %1
  %10 = load i64, i64* %3, align 8
  %11 = icmp ne i64 %10, 0
  br i1 %11, label %12, label %15

12:                                               ; preds = %9
  %13 = load i64, i64* %3, align 8
  %14 = call i8* @"?_Allocate@_Default_allocate_traits@std@@SAPEAX_K@Z"(i64 %13)
  store i8* %14, i8** %2, align 8
  br label %16

15:                                               ; preds = %9
  store i8* null, i8** %2, align 8
  br label %16

16:                                               ; preds = %15, %12, %6
  %17 = load i8*, i8** %2, align 8
  ret i8* %17
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i64 @"??$_Get_size_of_n@$00@std@@YA_K_K@Z"(i64 %0) #3 comdat {
  %2 = alloca i64, align 8
  %3 = alloca i8, align 1
  store i64 %0, i64* %2, align 8
  store i8 0, i8* %3, align 1
  %4 = load i64, i64* %2, align 8
  %5 = mul i64 %4, 1
  ret i64 %5
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i8* @"??$_Allocate_manually_vector_aligned@U_Default_allocate_traits@std@@@std@@YAPEAX_K@Z"(i64 %0) #1 comdat {
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i8*, align 8
  store i64 %0, i64* %2, align 8
  %6 = load i64, i64* %2, align 8
  %7 = add i64 39, %6
  store i64 %7, i64* %3, align 8
  %8 = load i64, i64* %3, align 8
  %9 = load i64, i64* %2, align 8
  %10 = icmp ule i64 %8, %9
  br i1 %10, label %11, label %12

11:                                               ; preds = %1
  call void @"?_Throw_bad_array_new_length@std@@YAXXZ"() #19
  unreachable

12:                                               ; preds = %1
  %13 = load i64, i64* %3, align 8
  %14 = call i8* @"?_Allocate@_Default_allocate_traits@std@@SAPEAX_K@Z"(i64 %13)
  %15 = ptrtoint i8* %14 to i64
  store i64 %15, i64* %4, align 8
  br label %16

16:                                               ; preds = %12
  %17 = load i64, i64* %4, align 8
  %18 = icmp ne i64 %17, 0
  br i1 %18, label %19, label %20

19:                                               ; preds = %16
  br label %23

20:                                               ; preds = %16
  br label %21

21:                                               ; preds = %20
  call void @_invalid_parameter_noinfo_noreturn() #19
  unreachable

22:                                               ; No predecessors!
  br label %23

23:                                               ; preds = %22, %19
  br label %24

24:                                               ; preds = %23
  %25 = load i64, i64* %4, align 8
  %26 = add i64 %25, 39
  %27 = and i64 %26, -32
  %28 = inttoptr i64 %27 to i8*
  store i8* %28, i8** %5, align 8
  %29 = load i64, i64* %4, align 8
  %30 = load i8*, i8** %5, align 8
  %31 = bitcast i8* %30 to i64*
  %32 = getelementptr inbounds i64, i64* %31, i64 -1
  store i64 %29, i64* %32, align 8
  %33 = load i8*, i8** %5, align 8
  ret i8* %33
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i8* @"?_Allocate@_Default_allocate_traits@std@@SAPEAX_K@Z"(i64 %0) #1 comdat align 2 {
  %2 = alloca i64, align 8
  store i64 %0, i64* %2, align 8
  %3 = load i64, i64* %2, align 8
  %4 = call noalias nonnull i8* @"??2@YAPEAX_K@Z"(i64 %3) #21
  ret i8* %4
}

; Function Attrs: noinline noreturn optnone uwtable
define linkonce_odr dso_local void @"?_Throw_bad_array_new_length@std@@YAXXZ"() #8 comdat {
  %1 = alloca %"class.std::bad_array_new_length", align 8
  %2 = call %"class.std::bad_array_new_length"* @"??0bad_array_new_length@std@@QEAA@XZ"(%"class.std::bad_array_new_length"* %1) #5
  %3 = bitcast %"class.std::bad_array_new_length"* %1 to i8*
  call void @_CxxThrowException(i8* %3, %eh.ThrowInfo* @"_TI3?AVbad_array_new_length@std@@") #19
  unreachable
}

; Function Attrs: noreturn
declare dso_local void @_invalid_parameter_noinfo_noreturn() #9

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::bad_array_new_length"* @"??0bad_array_new_length@std@@QEAA@XZ"(%"class.std::bad_array_new_length"* returned %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::bad_array_new_length"*, align 8
  store %"class.std::bad_array_new_length"* %0, %"class.std::bad_array_new_length"** %2, align 8
  %3 = load %"class.std::bad_array_new_length"*, %"class.std::bad_array_new_length"** %2, align 8
  %4 = bitcast %"class.std::bad_array_new_length"* %3 to %"class.std::bad_alloc"*
  %5 = call %"class.std::bad_alloc"* @"??0bad_alloc@std@@AEAA@QEBD@Z"(%"class.std::bad_alloc"* %4, i8* getelementptr inbounds ([21 x i8], [21 x i8]* @"??_C@_0BF@KINCDENJ@bad?5array?5new?5length?$AA@", i64 0, i64 0)) #5
  %6 = bitcast %"class.std::bad_array_new_length"* %3 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7bad_array_new_length@std@@6B@" to i32 (...)**), i32 (...)*** %6, align 8
  ret %"class.std::bad_array_new_length"* %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::bad_array_new_length"* @"??0bad_array_new_length@std@@QEAA@AEBV01@@Z"(%"class.std::bad_array_new_length"* returned %0, %"class.std::bad_array_new_length"* nonnull align 8 dereferenceable(24) %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca %"class.std::bad_array_new_length"*, align 8
  %4 = alloca %"class.std::bad_array_new_length"*, align 8
  store %"class.std::bad_array_new_length"* %1, %"class.std::bad_array_new_length"** %3, align 8
  store %"class.std::bad_array_new_length"* %0, %"class.std::bad_array_new_length"** %4, align 8
  %5 = load %"class.std::bad_array_new_length"*, %"class.std::bad_array_new_length"** %4, align 8
  %6 = bitcast %"class.std::bad_array_new_length"* %5 to %"class.std::bad_alloc"*
  %7 = load %"class.std::bad_array_new_length"*, %"class.std::bad_array_new_length"** %3, align 8
  %8 = bitcast %"class.std::bad_array_new_length"* %7 to %"class.std::bad_alloc"*
  %9 = call %"class.std::bad_alloc"* @"??0bad_alloc@std@@QEAA@AEBV01@@Z"(%"class.std::bad_alloc"* %6, %"class.std::bad_alloc"* nonnull align 8 dereferenceable(24) %8) #5
  %10 = bitcast %"class.std::bad_array_new_length"* %5 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7bad_array_new_length@std@@6B@" to i32 (...)**), i32 (...)*** %10, align 8
  ret %"class.std::bad_array_new_length"* %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::bad_alloc"* @"??0bad_alloc@std@@QEAA@AEBV01@@Z"(%"class.std::bad_alloc"* returned %0, %"class.std::bad_alloc"* nonnull align 8 dereferenceable(24) %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca %"class.std::bad_alloc"*, align 8
  %4 = alloca %"class.std::bad_alloc"*, align 8
  store %"class.std::bad_alloc"* %1, %"class.std::bad_alloc"** %3, align 8
  store %"class.std::bad_alloc"* %0, %"class.std::bad_alloc"** %4, align 8
  %5 = load %"class.std::bad_alloc"*, %"class.std::bad_alloc"** %4, align 8
  %6 = bitcast %"class.std::bad_alloc"* %5 to %"class.std::exception"*
  %7 = load %"class.std::bad_alloc"*, %"class.std::bad_alloc"** %3, align 8
  %8 = bitcast %"class.std::bad_alloc"* %7 to %"class.std::exception"*
  %9 = call %"class.std::exception"* @"??0exception@std@@QEAA@AEBV01@@Z"(%"class.std::exception"* %6, %"class.std::exception"* nonnull align 8 dereferenceable(24) %8) #5
  %10 = bitcast %"class.std::bad_alloc"* %5 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7bad_alloc@std@@6B@" to i32 (...)**), i32 (...)*** %10, align 8
  ret %"class.std::bad_alloc"* %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1bad_array_new_length@std@@UEAA@XZ"(%"class.std::bad_array_new_length"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::bad_array_new_length"*, align 8
  store %"class.std::bad_array_new_length"* %0, %"class.std::bad_array_new_length"** %2, align 8
  %3 = load %"class.std::bad_array_new_length"*, %"class.std::bad_array_new_length"** %2, align 8
  %4 = bitcast %"class.std::bad_array_new_length"* %3 to %"class.std::bad_alloc"*
  call void @"??1bad_alloc@std@@UEAA@XZ"(%"class.std::bad_alloc"* %4) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::bad_alloc"* @"??0bad_alloc@std@@AEAA@QEBD@Z"(%"class.std::bad_alloc"* returned %0, i8* %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::bad_alloc"*, align 8
  store i8* %1, i8** %3, align 8
  store %"class.std::bad_alloc"* %0, %"class.std::bad_alloc"** %4, align 8
  %5 = load %"class.std::bad_alloc"*, %"class.std::bad_alloc"** %4, align 8
  %6 = bitcast %"class.std::bad_alloc"* %5 to %"class.std::exception"*
  %7 = load i8*, i8** %3, align 8
  %8 = call %"class.std::exception"* @"??0exception@std@@QEAA@QEBDH@Z"(%"class.std::exception"* %6, i8* %7, i32 1) #5
  %9 = bitcast %"class.std::bad_alloc"* %5 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7bad_alloc@std@@6B@" to i32 (...)**), i32 (...)*** %9, align 8
  ret %"class.std::bad_alloc"* %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??_Gbad_array_new_length@std@@UEAAPEAXI@Z"(%"class.std::bad_array_new_length"* %0, i32 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca i32, align 4
  %5 = alloca %"class.std::bad_array_new_length"*, align 8
  store i32 %1, i32* %4, align 4
  store %"class.std::bad_array_new_length"* %0, %"class.std::bad_array_new_length"** %5, align 8
  %6 = load %"class.std::bad_array_new_length"*, %"class.std::bad_array_new_length"** %5, align 8
  %7 = bitcast %"class.std::bad_array_new_length"* %6 to i8*
  store i8* %7, i8** %3, align 8
  %8 = load i32, i32* %4, align 4
  call void @"??1bad_array_new_length@std@@UEAA@XZ"(%"class.std::bad_array_new_length"* %6) #5
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %12, label %10

10:                                               ; preds = %2
  %11 = bitcast %"class.std::bad_array_new_length"* %6 to i8*
  call void @"??3@YAXPEAX@Z"(i8* %11) #20
  br label %12

12:                                               ; preds = %10, %2
  %13 = load i8*, i8** %3, align 8
  ret i8* %13
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"?what@exception@std@@UEBAPEBDXZ"(%"class.std::exception"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::exception"*, align 8
  store %"class.std::exception"* %0, %"class.std::exception"** %2, align 8
  %3 = load %"class.std::exception"*, %"class.std::exception"** %2, align 8
  %4 = getelementptr inbounds %"class.std::exception", %"class.std::exception"* %3, i32 0, i32 1
  %5 = getelementptr inbounds %struct.__std_exception_data, %struct.__std_exception_data* %4, i32 0, i32 0
  %6 = load i8*, i8** %5, align 8
  %7 = icmp ne i8* %6, null
  br i1 %7, label %8, label %12

8:                                                ; preds = %1
  %9 = getelementptr inbounds %"class.std::exception", %"class.std::exception"* %3, i32 0, i32 1
  %10 = getelementptr inbounds %struct.__std_exception_data, %struct.__std_exception_data* %9, i32 0, i32 0
  %11 = load i8*, i8** %10, align 8
  br label %13

12:                                               ; preds = %1
  br label %13

13:                                               ; preds = %12, %8
  %14 = phi i8* [ %11, %8 ], [ getelementptr inbounds ([18 x i8], [18 x i8]* @"??_C@_0BC@EOODALEL@Unknown?5exception?$AA@", i64 0, i64 0), %12 ]
  ret i8* %14
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::exception"* @"??0exception@std@@QEAA@QEBDH@Z"(%"class.std::exception"* returned %0, i8* %1, i32 %2) unnamed_addr #3 comdat align 2 {
  %4 = alloca i32, align 4
  %5 = alloca i8*, align 8
  %6 = alloca %"class.std::exception"*, align 8
  store i32 %2, i32* %4, align 4
  store i8* %1, i8** %5, align 8
  store %"class.std::exception"* %0, %"class.std::exception"** %6, align 8
  %7 = load %"class.std::exception"*, %"class.std::exception"** %6, align 8
  %8 = bitcast %"class.std::exception"* %7 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7exception@std@@6B@" to i32 (...)**), i32 (...)*** %8, align 8
  %9 = getelementptr inbounds %"class.std::exception", %"class.std::exception"* %7, i32 0, i32 1
  %10 = bitcast %struct.__std_exception_data* %9 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 8 %10, i8 0, i64 16, i1 false)
  %11 = load i8*, i8** %5, align 8
  %12 = getelementptr inbounds %"class.std::exception", %"class.std::exception"* %7, i32 0, i32 1
  %13 = getelementptr inbounds %struct.__std_exception_data, %struct.__std_exception_data* %12, i32 0, i32 0
  store i8* %11, i8** %13, align 8
  ret %"class.std::exception"* %7
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??_Gbad_alloc@std@@UEAAPEAXI@Z"(%"class.std::bad_alloc"* %0, i32 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca i32, align 4
  %5 = alloca %"class.std::bad_alloc"*, align 8
  store i32 %1, i32* %4, align 4
  store %"class.std::bad_alloc"* %0, %"class.std::bad_alloc"** %5, align 8
  %6 = load %"class.std::bad_alloc"*, %"class.std::bad_alloc"** %5, align 8
  %7 = bitcast %"class.std::bad_alloc"* %6 to i8*
  store i8* %7, i8** %3, align 8
  %8 = load i32, i32* %4, align 4
  call void @"??1bad_alloc@std@@UEAA@XZ"(%"class.std::bad_alloc"* %6) #5
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %12, label %10

10:                                               ; preds = %2
  %11 = bitcast %"class.std::bad_alloc"* %6 to i8*
  call void @"??3@YAXPEAX@Z"(i8* %11) #20
  br label %12

12:                                               ; preds = %10, %2
  %13 = load i8*, i8** %3, align 8
  ret i8* %13
}

; Function Attrs: argmemonly nounwind willreturn writeonly
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #10

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??_Gexception@std@@UEAAPEAXI@Z"(%"class.std::exception"* %0, i32 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca i32, align 4
  %5 = alloca %"class.std::exception"*, align 8
  store i32 %1, i32* %4, align 4
  store %"class.std::exception"* %0, %"class.std::exception"** %5, align 8
  %6 = load %"class.std::exception"*, %"class.std::exception"** %5, align 8
  %7 = bitcast %"class.std::exception"* %6 to i8*
  store i8* %7, i8** %3, align 8
  %8 = load i32, i32* %4, align 4
  call void @"??1exception@std@@UEAA@XZ"(%"class.std::exception"* %6) #5
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %12, label %10

10:                                               ; preds = %2
  %11 = bitcast %"class.std::exception"* %6 to i8*
  call void @"??3@YAXPEAX@Z"(i8* %11) #20
  br label %12

12:                                               ; preds = %10, %2
  %13 = load i8*, i8** %3, align 8
  ret i8* %13
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1exception@std@@UEAA@XZ"(%"class.std::exception"* %0) unnamed_addr #3 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %2 = alloca %"class.std::exception"*, align 8
  store %"class.std::exception"* %0, %"class.std::exception"** %2, align 8
  %3 = load %"class.std::exception"*, %"class.std::exception"** %2, align 8
  %4 = bitcast %"class.std::exception"* %3 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7exception@std@@6B@" to i32 (...)**), i32 (...)*** %4, align 8
  %5 = getelementptr inbounds %"class.std::exception", %"class.std::exception"* %3, i32 0, i32 1
  invoke void @__std_exception_destroy(%struct.__std_exception_data* %5)
          to label %6 unwind label %7

6:                                                ; preds = %1
  ret void

7:                                                ; preds = %1
  %8 = cleanuppad within none []
  call void @__std_terminate() #18 [ "funclet"(token %8) ]
  unreachable
}

declare dso_local void @__std_exception_destroy(%struct.__std_exception_data*) #4

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1bad_alloc@std@@UEAA@XZ"(%"class.std::bad_alloc"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::bad_alloc"*, align 8
  store %"class.std::bad_alloc"* %0, %"class.std::bad_alloc"** %2, align 8
  %3 = load %"class.std::bad_alloc"*, %"class.std::bad_alloc"** %2, align 8
  %4 = bitcast %"class.std::bad_alloc"* %3 to %"class.std::exception"*
  call void @"??1exception@std@@UEAA@XZ"(%"class.std::exception"* %4) #5
  ret void
}

; Function Attrs: nobuiltin allocsize(0)
declare dso_local nonnull i8* @"??2@YAPEAX_K@Z"(i64) #11

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"?copy@?$_Char_traits@DH@std@@SAPEADQEADQEBD_K@Z"(i8* %0, i8* %1, i64 %2) #3 comdat align 2 {
  %4 = alloca i64, align 8
  %5 = alloca i8*, align 8
  %6 = alloca i8*, align 8
  store i64 %2, i64* %4, align 8
  store i8* %1, i8** %5, align 8
  store i8* %0, i8** %6, align 8
  %7 = load i8*, i8** %6, align 8
  %8 = load i8*, i8** %5, align 8
  %9 = load i64, i64* %4, align 8
  %10 = mul i64 %9, 1
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %7, i8* align 1 %8, i64 %10, i1 false)
  %11 = load i8*, i8** %6, align 8
  ret i8* %11
}

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #7

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??$_Deallocate@$0BA@$0A@@std@@YAXPEAX_K@Z"(i8* %0, i64 %1) #3 comdat personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %3 = alloca i64, align 8
  %4 = alloca i8*, align 8
  store i64 %1, i64* %3, align 8
  store i8* %0, i8** %4, align 8
  %5 = load i64, i64* %3, align 8
  %6 = icmp uge i64 %5, 4096
  br i1 %6, label %7, label %9

7:                                                ; preds = %2
  invoke void @"?_Adjust_manually_vector_aligned@std@@YAXAEAPEAXAEA_K@Z"(i8** nonnull align 8 dereferenceable(8) %4, i64* nonnull align 8 dereferenceable(8) %3)
          to label %8 unwind label %12

8:                                                ; preds = %7
  br label %9

9:                                                ; preds = %8, %2
  %10 = load i64, i64* %3, align 8
  %11 = load i8*, i8** %4, align 8
  call void @"??3@YAXPEAX_K@Z"(i8* %11, i64 %10) #5
  ret void

12:                                               ; preds = %7
  %13 = cleanuppad within none []
  call void @__std_terminate() #18 [ "funclet"(token %13) ]
  unreachable
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?_Adjust_manually_vector_aligned@std@@YAXAEAPEAXAEA_K@Z"(i8** nonnull align 8 dereferenceable(8) %0, i64* nonnull align 8 dereferenceable(8) %1) #1 comdat {
  %3 = alloca i64*, align 8
  %4 = alloca i8**, align 8
  %5 = alloca i64*, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  store i64* %1, i64** %3, align 8
  store i8** %0, i8*** %4, align 8
  %9 = load i64*, i64** %3, align 8
  %10 = load i64, i64* %9, align 8
  %11 = add i64 %10, 39
  store i64 %11, i64* %9, align 8
  %12 = load i8**, i8*** %4, align 8
  %13 = load i8*, i8** %12, align 8
  %14 = bitcast i8* %13 to i64*
  store i64* %14, i64** %5, align 8
  %15 = load i64*, i64** %5, align 8
  %16 = getelementptr inbounds i64, i64* %15, i64 -1
  %17 = load i64, i64* %16, align 8
  store i64 %17, i64* %6, align 8
  store i64 8, i64* %7, align 8
  %18 = load i8**, i8*** %4, align 8
  %19 = load i8*, i8** %18, align 8
  %20 = ptrtoint i8* %19 to i64
  %21 = load i64, i64* %6, align 8
  %22 = sub i64 %20, %21
  store i64 %22, i64* %8, align 8
  br label %23

23:                                               ; preds = %2
  %24 = load i64, i64* %8, align 8
  %25 = icmp uge i64 %24, 8
  br i1 %25, label %26, label %30

26:                                               ; preds = %23
  %27 = load i64, i64* %8, align 8
  %28 = icmp ule i64 %27, 39
  br i1 %28, label %29, label %30

29:                                               ; preds = %26
  br label %33

30:                                               ; preds = %26, %23
  br label %31

31:                                               ; preds = %30
  call void @_invalid_parameter_noinfo_noreturn() #19
  unreachable

32:                                               ; No predecessors!
  br label %33

33:                                               ; preds = %32, %29
  br label %34

34:                                               ; preds = %33
  %35 = load i64, i64* %6, align 8
  %36 = inttoptr i64 %35 to i8*
  %37 = load i8**, i8*** %4, align 8
  store i8* %36, i8** %37, align 8
  ret void
}

; Function Attrs: nounwind
declare dso_local void @"??3@YAXPEAX_K@Z"(i8*, i64) #12

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??$_Voidify_iter@PEAPEAD@std@@YAPEAXPEAPEAD@Z"(i8** %0) #3 comdat {
  %2 = alloca i8**, align 8
  store i8** %0, i8*** %2, align 8
  %3 = load i8**, i8*** %2, align 8
  %4 = bitcast i8** %3 to i8*
  ret i8* %4
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8** @"??$addressof@PEAD@std@@YAPEAPEADAEAPEAD@Z"(i8** nonnull align 8 dereferenceable(8) %0) #3 comdat {
  %2 = alloca i8**, align 8
  store i8** %0, i8*** %2, align 8
  %3 = load i8**, i8*** %2, align 8
  ret i8** %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(8) i8** @"??$forward@AEBQEAD@std@@YAAEBQEADAEBQEAD@Z"(i8** nonnull align 8 dereferenceable(8) %0) #3 comdat {
  %2 = alloca i8**, align 8
  store i8** %0, i8*** %2, align 8
  %3 = load i8**, i8*** %2, align 8
  ret i8** %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1?$_String_val@U?$_Simple_types@D@std@@@std@@QEAA@XZ"(%"class.std::_String_val"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::_String_val"*, align 8
  store %"class.std::_String_val"* %0, %"class.std::_String_val"** %2, align 8
  %3 = load %"class.std::_String_val"*, %"class.std::_String_val"** %2, align 8
  %4 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %3, i32 0, i32 0
  call void @"??1_Bxty@?$_String_val@U?$_Simple_types@D@std@@@std@@QEAA@XZ"(%"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %4) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1_Bxty@?$_String_val@U?$_Simple_types@D@std@@@std@@QEAA@XZ"(%"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"*, align 8
  store %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %0, %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"** %2, align 8
  %3 = load %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"*, %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"** %2, align 8
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@QEBD@Z"(%"class.std::basic_string"* %0, i8* %1) #1 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::basic_string"*, align 8
  store i8* %1, i8** %3, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %4, align 8
  %5 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  %6 = load i8*, i8** %3, align 8
  %7 = call i64 @"?length@?$_Narrow_char_traits@DH@std@@SA_KQEBD@Z"(i8* %6) #5
  %8 = call i64 @"??$_Convert_size@_K@std@@YA_K_K@Z"(i64 %7) #5
  %9 = load i8*, i8** %3, align 8
  %10 = call nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@QEBD_K@Z"(%"class.std::basic_string"* %5, i8* %9, i64 %8)
  ret %"class.std::basic_string"* %10
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i64 @"??$_Convert_size@_K@std@@YA_K_K@Z"(i64 %0) #3 comdat {
  %2 = alloca i64, align 8
  store i64 %0, i64* %2, align 8
  %3 = load i64, i64* %2, align 8
  ret i64 %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::error_condition"* @"??0error_condition@std@@QEAA@HAEBVerror_category@1@@Z"(%"class.std::error_condition"* returned %0, i32 %1, %"class.std::error_category"* nonnull align 8 dereferenceable(16) %2) unnamed_addr #3 comdat align 2 {
  %4 = alloca %"class.std::error_category"*, align 8
  %5 = alloca i32, align 4
  %6 = alloca %"class.std::error_condition"*, align 8
  store %"class.std::error_category"* %2, %"class.std::error_category"** %4, align 8
  store i32 %1, i32* %5, align 4
  store %"class.std::error_condition"* %0, %"class.std::error_condition"** %6, align 8
  %7 = load %"class.std::error_condition"*, %"class.std::error_condition"** %6, align 8
  %8 = getelementptr inbounds %"class.std::error_condition", %"class.std::error_condition"* %7, i32 0, i32 0
  %9 = load i32, i32* %5, align 4
  store i32 %9, i32* %8, align 8
  %10 = getelementptr inbounds %"class.std::error_condition", %"class.std::error_condition"* %7, i32 0, i32 1
  %11 = load %"class.std::error_category"*, %"class.std::error_category"** %4, align 8
  store %"class.std::error_category"* %11, %"class.std::error_category"** %10, align 8
  ret %"class.std::error_condition"* %7
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local zeroext i1 @"??8error_category@std@@QEBA_NAEBV01@@Z"(%"class.std::error_category"* %0, %"class.std::error_category"* nonnull align 8 dereferenceable(16) %1) #3 comdat align 2 {
  %3 = alloca %"class.std::error_category"*, align 8
  %4 = alloca %"class.std::error_category"*, align 8
  store %"class.std::error_category"* %1, %"class.std::error_category"** %3, align 8
  store %"class.std::error_category"* %0, %"class.std::error_category"** %4, align 8
  %5 = load %"class.std::error_category"*, %"class.std::error_category"** %4, align 8
  %6 = getelementptr inbounds %"class.std::error_category", %"class.std::error_category"* %5, i32 0, i32 1
  %7 = load i64, i64* %6, align 8
  %8 = load %"class.std::error_category"*, %"class.std::error_category"** %3, align 8
  %9 = getelementptr inbounds %"class.std::error_category", %"class.std::error_category"* %8, i32 0, i32 1
  %10 = load i64, i64* %9, align 8
  %11 = icmp eq i64 %7, %10
  ret i1 %11
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(16) %"class.std::error_category"* @"?category@error_code@std@@QEBAAEBVerror_category@2@XZ"(%"class.std::error_code"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::error_code"*, align 8
  store %"class.std::error_code"* %0, %"class.std::error_code"** %2, align 8
  %3 = load %"class.std::error_code"*, %"class.std::error_code"** %2, align 8
  %4 = getelementptr inbounds %"class.std::error_code", %"class.std::error_code"* %3, i32 0, i32 1
  %5 = load %"class.std::error_category"*, %"class.std::error_category"** %4, align 8
  ret %"class.std::error_category"* %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i32 @"?value@error_code@std@@QEBAHXZ"(%"class.std::error_code"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::error_code"*, align 8
  store %"class.std::error_code"* %0, %"class.std::error_code"** %2, align 8
  %3 = load %"class.std::error_code"*, %"class.std::error_code"** %2, align 8
  %4 = getelementptr inbounds %"class.std::error_code", %"class.std::error_code"* %3, i32 0, i32 0
  %5 = load i32, i32* %4, align 8
  ret i32 %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local zeroext i1 @"??8std@@YA_NAEBVerror_condition@0@0@Z"(%"class.std::error_condition"* nonnull align 8 dereferenceable(16) %0, %"class.std::error_condition"* nonnull align 8 dereferenceable(16) %1) #3 comdat {
  %3 = alloca %"class.std::error_condition"*, align 8
  %4 = alloca %"class.std::error_condition"*, align 8
  store %"class.std::error_condition"* %1, %"class.std::error_condition"** %3, align 8
  store %"class.std::error_condition"* %0, %"class.std::error_condition"** %4, align 8
  %5 = load %"class.std::error_condition"*, %"class.std::error_condition"** %4, align 8
  %6 = call nonnull align 8 dereferenceable(16) %"class.std::error_category"* @"?category@error_condition@std@@QEBAAEBVerror_category@2@XZ"(%"class.std::error_condition"* %5) #5
  %7 = load %"class.std::error_condition"*, %"class.std::error_condition"** %3, align 8
  %8 = call nonnull align 8 dereferenceable(16) %"class.std::error_category"* @"?category@error_condition@std@@QEBAAEBVerror_category@2@XZ"(%"class.std::error_condition"* %7) #5
  %9 = call zeroext i1 @"??8error_category@std@@QEBA_NAEBV01@@Z"(%"class.std::error_category"* %6, %"class.std::error_category"* nonnull align 8 dereferenceable(16) %8) #5
  br i1 %9, label %10, label %16

10:                                               ; preds = %2
  %11 = load %"class.std::error_condition"*, %"class.std::error_condition"** %4, align 8
  %12 = call i32 @"?value@error_condition@std@@QEBAHXZ"(%"class.std::error_condition"* %11) #5
  %13 = load %"class.std::error_condition"*, %"class.std::error_condition"** %3, align 8
  %14 = call i32 @"?value@error_condition@std@@QEBAHXZ"(%"class.std::error_condition"* %13) #5
  %15 = icmp eq i32 %12, %14
  br label %16

16:                                               ; preds = %10, %2
  %17 = phi i1 [ false, %2 ], [ %15, %10 ]
  ret i1 %17
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(16) %"class.std::error_category"* @"?category@error_condition@std@@QEBAAEBVerror_category@2@XZ"(%"class.std::error_condition"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::error_condition"*, align 8
  store %"class.std::error_condition"* %0, %"class.std::error_condition"** %2, align 8
  %3 = load %"class.std::error_condition"*, %"class.std::error_condition"** %2, align 8
  %4 = getelementptr inbounds %"class.std::error_condition", %"class.std::error_condition"* %3, i32 0, i32 1
  %5 = load %"class.std::error_category"*, %"class.std::error_category"** %4, align 8
  ret %"class.std::error_category"* %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i32 @"?value@error_condition@std@@QEBAHXZ"(%"class.std::error_condition"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::error_condition"*, align 8
  store %"class.std::error_condition"* %0, %"class.std::error_condition"** %2, align 8
  %3 = load %"class.std::error_condition"*, %"class.std::error_condition"** %2, align 8
  %4 = getelementptr inbounds %"class.std::error_condition", %"class.std::error_condition"* %3, i32 0, i32 0
  %5 = load i32, i32* %4, align 8
  ret i32 %5
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local %"class.std::system_error"* @"??0system_error@std@@QEAA@Verror_code@1@PEBD@Z"(%"class.std::system_error"* returned %0, %"class.std::error_code"* %1, i8* %2) unnamed_addr #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %4 = alloca i8*, align 8
  %5 = alloca %"class.std::system_error"*, align 8
  %6 = alloca %"class.std::basic_string", align 8
  %7 = alloca %"class.std::error_code", align 8
  store i8* %2, i8** %4, align 8
  store %"class.std::system_error"* %0, %"class.std::system_error"** %5, align 8
  %8 = load %"class.std::system_error"*, %"class.std::system_error"** %5, align 8
  %9 = bitcast %"class.std::system_error"* %8 to %"class.std::_System_error"*
  %10 = load i8*, i8** %4, align 8
  %11 = call %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@QEBD@Z"(%"class.std::basic_string"* %6, i8* %10)
  %12 = bitcast %"class.std::error_code"* %7 to i8*
  %13 = bitcast %"class.std::error_code"* %1 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %12, i8* align 8 %13, i64 16, i1 false)
  %14 = invoke %"class.std::_System_error"* @"??0_System_error@std@@IEAA@Verror_code@1@AEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@1@@Z"(%"class.std::_System_error"* %9, %"class.std::error_code"* %7, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %6)
          to label %15 unwind label %17

15:                                               ; preds = %3
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %6) #5
  %16 = bitcast %"class.std::system_error"* %8 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7system_error@std@@6B@" to i32 (...)**), i32 (...)*** %16, align 8
  ret %"class.std::system_error"* %8

17:                                               ; preds = %3
  %18 = cleanuppad within none []
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %6) #5 [ "funclet"(token %18) ]
  cleanupret from %18 unwind to caller
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??_Gfailure@ios_base@std@@UEAAPEAXI@Z"(%"class.std::ios_base::failure"* %0, i32 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca i32, align 4
  %5 = alloca %"class.std::ios_base::failure"*, align 8
  store i32 %1, i32* %4, align 4
  store %"class.std::ios_base::failure"* %0, %"class.std::ios_base::failure"** %5, align 8
  %6 = load %"class.std::ios_base::failure"*, %"class.std::ios_base::failure"** %5, align 8
  %7 = bitcast %"class.std::ios_base::failure"* %6 to i8*
  store i8* %7, i8** %3, align 8
  %8 = load i32, i32* %4, align 4
  call void @"??1failure@ios_base@std@@UEAA@XZ"(%"class.std::ios_base::failure"* %6) #5
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %12, label %10

10:                                               ; preds = %2
  %11 = bitcast %"class.std::ios_base::failure"* %6 to i8*
  call void @"??3@YAXPEAX@Z"(i8* %11) #20
  br label %12

12:                                               ; preds = %10, %2
  %13 = load i8*, i8** %3, align 8
  ret i8* %13
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local %"class.std::_System_error"* @"??0_System_error@std@@IEAA@Verror_code@1@AEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@1@@Z"(%"class.std::_System_error"* returned %0, %"class.std::error_code"* %1, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %2) unnamed_addr #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %4 = alloca %"class.std::basic_string"*, align 8
  %5 = alloca %"class.std::_System_error"*, align 8
  %6 = alloca %"class.std::basic_string", align 8
  %7 = alloca %"class.std::basic_string", align 8
  %8 = alloca %"class.std::error_code", align 8
  store %"class.std::basic_string"* %2, %"class.std::basic_string"** %4, align 8
  store %"class.std::_System_error"* %0, %"class.std::_System_error"** %5, align 8
  %9 = load %"class.std::_System_error"*, %"class.std::_System_error"** %5, align 8
  %10 = bitcast %"class.std::_System_error"* %9 to %"class.std::runtime_error"*
  %11 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  %12 = call %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@AEBV01@@Z"(%"class.std::basic_string"* %7, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %11)
  %13 = bitcast %"class.std::error_code"* %8 to i8*
  %14 = bitcast %"class.std::error_code"* %1 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %13, i8* align 8 %14, i64 16, i1 false)
  call void @"?_Makestr@_System_error@std@@CA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@Verror_code@2@V32@@Z"(%"class.std::basic_string"* sret align 8 %6, %"class.std::error_code"* %8, %"class.std::basic_string"* %7)
  %15 = invoke %"class.std::runtime_error"* @"??0runtime_error@std@@QEAA@AEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@1@@Z"(%"class.std::runtime_error"* %10, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %6)
          to label %16 unwind label %21

16:                                               ; preds = %3
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %6) #5
  %17 = bitcast %"class.std::_System_error"* %9 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7_System_error@std@@6B@" to i32 (...)**), i32 (...)*** %17, align 8
  %18 = getelementptr inbounds %"class.std::_System_error", %"class.std::_System_error"* %9, i32 0, i32 1
  %19 = bitcast %"class.std::error_code"* %18 to i8*
  %20 = bitcast %"class.std::error_code"* %1 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %19, i8* align 8 %20, i64 16, i1 false)
  ret %"class.std::_System_error"* %9

21:                                               ; preds = %3
  %22 = cleanuppad within none []
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %6) #5 [ "funclet"(token %22) ]
  cleanupret from %22 unwind to caller
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::basic_string"*, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %2, align 8
  %3 = load %"class.std::basic_string"*, %"class.std::basic_string"** %2, align 8
  call void @"?_Tidy_deallocate@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXXZ"(%"class.std::basic_string"* %3) #5
  %4 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %3, i32 0, i32 0
  call void @"??1?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@XZ"(%"class.std::_Compressed_pair"* %4) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??_Gsystem_error@std@@UEAAPEAXI@Z"(%"class.std::system_error"* %0, i32 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca i32, align 4
  %5 = alloca %"class.std::system_error"*, align 8
  store i32 %1, i32* %4, align 4
  store %"class.std::system_error"* %0, %"class.std::system_error"** %5, align 8
  %6 = load %"class.std::system_error"*, %"class.std::system_error"** %5, align 8
  %7 = bitcast %"class.std::system_error"* %6 to i8*
  store i8* %7, i8** %3, align 8
  %8 = load i32, i32* %4, align 4
  call void @"??1system_error@std@@UEAA@XZ"(%"class.std::system_error"* %6) #5
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %12, label %10

10:                                               ; preds = %2
  %11 = bitcast %"class.std::system_error"* %6 to i8*
  call void @"??3@YAXPEAX@Z"(i8* %11) #20
  br label %12

12:                                               ; preds = %10, %2
  %13 = load i8*, i8** %3, align 8
  ret i8* %13
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?_Makestr@_System_error@std@@CA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@Verror_code@2@V32@@Z"(%"class.std::basic_string"* noalias sret align 8 %0, %"class.std::error_code"* %1, %"class.std::basic_string"* %2) #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %4 = alloca i8*, align 8
  %5 = alloca %"class.std::basic_string", align 8
  %6 = bitcast %"class.std::basic_string"* %0 to i8*
  store i8* %6, i8** %4, align 8
  %7 = call zeroext i1 @"?empty@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_NXZ"(%"class.std::basic_string"* %2) #5
  br i1 %7, label %11, label %8

8:                                                ; preds = %3
  %9 = invoke nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@QEBD@Z"(%"class.std::basic_string"* %2, i8* getelementptr inbounds ([3 x i8], [3 x i8]* @"??_C@_02LMMGGCAJ@?3?5?$AA@", i64 0, i64 0))
          to label %10 unwind label %18

10:                                               ; preds = %8
  br label %11

11:                                               ; preds = %10, %3
  invoke void @"?message@error_code@std@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ"(%"class.std::error_code"* %1, %"class.std::basic_string"* sret align 8 %5)
          to label %12 unwind label %18

12:                                               ; preds = %11
  %13 = invoke nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@AEBV12@@Z"(%"class.std::basic_string"* %2, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %5)
          to label %14 unwind label %16

14:                                               ; preds = %12
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %5) #5
  %15 = call %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@$$QEAV01@@Z"(%"class.std::basic_string"* %0, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %2) #5
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %2) #5
  ret void

16:                                               ; preds = %12
  %17 = cleanuppad within none []
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %5) #5 [ "funclet"(token %17) ]
  cleanupret from %17 unwind label %18

18:                                               ; preds = %16, %11, %8
  %19 = cleanuppad within none []
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %2) #5 [ "funclet"(token %19) ]
  cleanupret from %19 unwind to caller
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@AEBV01@@Z"(%"class.std::basic_string"* returned %0, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %1) unnamed_addr #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %3 = alloca %"class.std::basic_string"*, align 8
  %4 = alloca %"class.std::basic_string"*, align 8
  %5 = alloca %"class.std::allocator", align 1
  %6 = alloca %"struct.std::_One_then_variadic_args_t", align 1
  %7 = alloca %"struct.std::_Fake_allocator"*, align 8
  %8 = alloca %"struct.std::_Fake_proxy_ptr_impl", align 1
  store %"class.std::basic_string"* %1, %"class.std::basic_string"** %3, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %4, align 8
  %9 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  %10 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %9, i32 0, i32 0
  %11 = load %"class.std::basic_string"*, %"class.std::basic_string"** %3, align 8
  %12 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Getal@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEBAAEBV?$allocator@D@2@XZ"(%"class.std::basic_string"* %11) #5
  call void @"?select_on_container_copy_construction@?$_Default_allocator_traits@V?$allocator@D@std@@@std@@SA?AV?$allocator@D@2@AEBV32@@Z"(%"class.std::allocator"* sret align 1 %5, %"class.std::allocator"* nonnull align 1 dereferenceable(1) %12)
  %13 = getelementptr inbounds %"struct.std::_One_then_variadic_args_t", %"struct.std::_One_then_variadic_args_t"* %6, i32 0, i32 0
  %14 = load i8, i8* %13, align 1
  %15 = call %"class.std::_Compressed_pair"* @"??$?0V?$allocator@D@std@@$$V@?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@U_One_then_variadic_args_t@1@$$QEAV?$allocator@D@1@@Z"(%"class.std::_Compressed_pair"* %10, i8 %14, %"class.std::allocator"* nonnull align 1 dereferenceable(1) %5) #5
  store %"struct.std::_Fake_allocator"* @"?_Fake_alloc@std@@3U_Fake_allocator@1@B", %"struct.std::_Fake_allocator"** %7, align 8
  %16 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %9, i32 0, i32 0
  %17 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %16, i32 0, i32 0
  %18 = bitcast %"class.std::_String_val"* %17 to %"struct.std::_Container_base0"*
  %19 = call %"struct.std::_Fake_proxy_ptr_impl"* @"??0_Fake_proxy_ptr_impl@std@@QEAA@AEBU_Fake_allocator@1@AEBU_Container_base0@1@@Z"(%"struct.std::_Fake_proxy_ptr_impl"* %8, %"struct.std::_Fake_allocator"* nonnull align 1 dereferenceable(1) @"?_Fake_alloc@std@@3U_Fake_allocator@1@B", %"struct.std::_Container_base0"* nonnull align 1 dereferenceable(1) %18) #5
  %20 = load %"class.std::basic_string"*, %"class.std::basic_string"** %3, align 8
  invoke void @"?_Construct_lv_contents@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXAEBV12@@Z"(%"class.std::basic_string"* %9, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %20)
          to label %21 unwind label %22

21:                                               ; preds = %2
  call void @"?_Release@_Fake_proxy_ptr_impl@std@@QEAAXXZ"(%"struct.std::_Fake_proxy_ptr_impl"* %8) #5
  ret %"class.std::basic_string"* %9

22:                                               ; preds = %2
  %23 = cleanuppad within none []
  call void @"??1?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@XZ"(%"class.std::_Compressed_pair"* %10) #5 [ "funclet"(token %23) ]
  cleanupret from %23 unwind to caller
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::runtime_error"* @"??0runtime_error@std@@QEAA@AEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@1@@Z"(%"class.std::runtime_error"* returned %0, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca %"class.std::basic_string"*, align 8
  %4 = alloca %"class.std::runtime_error"*, align 8
  store %"class.std::basic_string"* %1, %"class.std::basic_string"** %3, align 8
  store %"class.std::runtime_error"* %0, %"class.std::runtime_error"** %4, align 8
  %5 = load %"class.std::runtime_error"*, %"class.std::runtime_error"** %4, align 8
  %6 = bitcast %"class.std::runtime_error"* %5 to %"class.std::exception"*
  %7 = load %"class.std::basic_string"*, %"class.std::basic_string"** %3, align 8
  %8 = call i8* @"?c_str@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBAPEBDXZ"(%"class.std::basic_string"* %7) #5
  %9 = call %"class.std::exception"* @"??0exception@std@@QEAA@QEBD@Z"(%"class.std::exception"* %6, i8* %8) #5
  %10 = bitcast %"class.std::runtime_error"* %5 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7runtime_error@std@@6B@" to i32 (...)**), i32 (...)*** %10, align 8
  ret %"class.std::runtime_error"* %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??_G_System_error@std@@UEAAPEAXI@Z"(%"class.std::_System_error"* %0, i32 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca i32, align 4
  %5 = alloca %"class.std::_System_error"*, align 8
  store i32 %1, i32* %4, align 4
  store %"class.std::_System_error"* %0, %"class.std::_System_error"** %5, align 8
  %6 = load %"class.std::_System_error"*, %"class.std::_System_error"** %5, align 8
  %7 = bitcast %"class.std::_System_error"* %6 to i8*
  store i8* %7, i8** %3, align 8
  %8 = load i32, i32* %4, align 4
  call void @"??1_System_error@std@@UEAA@XZ"(%"class.std::_System_error"* %6) #5
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %12, label %10

10:                                               ; preds = %2
  %11 = bitcast %"class.std::_System_error"* %6 to i8*
  call void @"??3@YAXPEAX@Z"(i8* %11) #20
  br label %12

12:                                               ; preds = %10, %2
  %13 = load i8*, i8** %3, align 8
  ret i8* %13
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local zeroext i1 @"?empty@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_NXZ"(%"class.std::basic_string"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::basic_string"*, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %2, align 8
  %3 = load %"class.std::basic_string"*, %"class.std::basic_string"** %2, align 8
  %4 = call i64 @"?size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %3) #5
  %5 = icmp eq i64 %4, 0
  ret i1 %5
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@QEBD@Z"(%"class.std::basic_string"* %0, i8* %1) #1 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::basic_string"*, align 8
  store i8* %1, i8** %3, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %4, align 8
  %5 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  %6 = load i8*, i8** %3, align 8
  %7 = call i64 @"?length@?$_Narrow_char_traits@DH@std@@SA_KQEBD@Z"(i8* %6) #5
  %8 = call i64 @"??$_Convert_size@_K@std@@YA_K_K@Z"(i64 %7) #5
  %9 = load i8*, i8** %3, align 8
  %10 = call nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@QEBD_K@Z"(%"class.std::basic_string"* %5, i8* %9, i64 %8)
  ret %"class.std::basic_string"* %10
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@AEBV12@@Z"(%"class.std::basic_string"* %0, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %1) #1 comdat align 2 {
  %3 = alloca %"class.std::basic_string"*, align 8
  %4 = alloca %"class.std::basic_string"*, align 8
  store %"class.std::basic_string"* %1, %"class.std::basic_string"** %3, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %4, align 8
  %5 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  %6 = load %"class.std::basic_string"*, %"class.std::basic_string"** %3, align 8
  %7 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %6, i32 0, i32 0
  %8 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %7, i32 0, i32 0
  %9 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %8, i32 0, i32 1
  %10 = load i64, i64* %9, align 8
  %11 = load %"class.std::basic_string"*, %"class.std::basic_string"** %3, align 8
  %12 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %11, i32 0, i32 0
  %13 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %12, i32 0, i32 0
  %14 = call i8* @"?_Myptr@?$_String_val@U?$_Simple_types@D@std@@@std@@QEBAPEBDXZ"(%"class.std::_String_val"* %13) #5
  %15 = call nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@QEBD_K@Z"(%"class.std::basic_string"* %5, i8* %14, i64 %10)
  ret %"class.std::basic_string"* %15
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?message@error_code@std@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ"(%"class.std::error_code"* %0, %"class.std::basic_string"* noalias sret align 8 %1) #1 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::error_code"*, align 8
  %5 = bitcast %"class.std::basic_string"* %1 to i8*
  store i8* %5, i8** %3, align 8
  store %"class.std::error_code"* %0, %"class.std::error_code"** %4, align 8
  %6 = load %"class.std::error_code"*, %"class.std::error_code"** %4, align 8
  %7 = call nonnull align 8 dereferenceable(16) %"class.std::error_category"* @"?category@error_code@std@@QEBAAEBVerror_category@2@XZ"(%"class.std::error_code"* %6) #5
  %8 = call i32 @"?value@error_code@std@@QEBAHXZ"(%"class.std::error_code"* %6) #5
  %9 = bitcast %"class.std::error_category"* %7 to void (%"class.std::error_category"*, %"class.std::basic_string"*, i32)***
  %10 = load void (%"class.std::error_category"*, %"class.std::basic_string"*, i32)**, void (%"class.std::error_category"*, %"class.std::basic_string"*, i32)*** %9, align 8
  %11 = getelementptr inbounds void (%"class.std::error_category"*, %"class.std::basic_string"*, i32)*, void (%"class.std::error_category"*, %"class.std::basic_string"*, i32)** %10, i64 2
  %12 = load void (%"class.std::error_category"*, %"class.std::basic_string"*, i32)*, void (%"class.std::error_category"*, %"class.std::basic_string"*, i32)** %11, align 8
  call void %12(%"class.std::error_category"* %7, %"class.std::basic_string"* sret align 8 %1, i32 %8)
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@$$QEAV01@@Z"(%"class.std::basic_string"* returned %0, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca %"class.std::basic_string"*, align 8
  %4 = alloca %"class.std::basic_string"*, align 8
  %5 = alloca %"struct.std::_One_then_variadic_args_t", align 1
  store %"class.std::basic_string"* %1, %"class.std::basic_string"** %3, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %4, align 8
  %6 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  %7 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %6, i32 0, i32 0
  %8 = load %"class.std::basic_string"*, %"class.std::basic_string"** %3, align 8
  %9 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Getal@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV?$allocator@D@2@XZ"(%"class.std::basic_string"* %8) #5
  %10 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"??$move@AEAV?$allocator@D@std@@@std@@YA$$QEAV?$allocator@D@0@AEAV10@@Z"(%"class.std::allocator"* nonnull align 1 dereferenceable(1) %9) #5
  %11 = getelementptr inbounds %"struct.std::_One_then_variadic_args_t", %"struct.std::_One_then_variadic_args_t"* %5, i32 0, i32 0
  %12 = load i8, i8* %11, align 1
  %13 = call %"class.std::_Compressed_pair"* @"??$?0V?$allocator@D@std@@$$V@?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@U_One_then_variadic_args_t@1@$$QEAV?$allocator@D@1@@Z"(%"class.std::_Compressed_pair"* %7, i8 %12, %"class.std::allocator"* nonnull align 1 dereferenceable(1) %10) #5
  %14 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %6, i32 0, i32 0
  %15 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %14, i32 0, i32 0
  %16 = bitcast %"class.std::_String_val"* %15 to %"struct.std::_Container_base0"*
  call void @"?_Alloc_proxy@_Container_base0@std@@QEAAXAEBU_Fake_allocator@2@@Z"(%"struct.std::_Container_base0"* %16, %"struct.std::_Fake_allocator"* nonnull align 1 dereferenceable(1) @"?_Fake_alloc@std@@3U_Fake_allocator@1@B") #5
  %17 = load %"class.std::basic_string"*, %"class.std::basic_string"** %3, align 8
  call void @"?_Take_contents@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXAEAV12@@Z"(%"class.std::basic_string"* %6, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %17) #5
  ret %"class.std::basic_string"* %6
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i64 @"?size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::basic_string"*, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %2, align 8
  %3 = load %"class.std::basic_string"*, %"class.std::basic_string"** %2, align 8
  %4 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %3, i32 0, i32 0
  %5 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %4, i32 0, i32 0
  %6 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %5, i32 0, i32 1
  %7 = load i64, i64* %6, align 8
  ret i64 %7
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@QEBD_K@Z"(%"class.std::basic_string"* %0, i8* %1, i64 %2) #1 comdat align 2 {
  %4 = alloca %"class.std::basic_string"*, align 8
  %5 = alloca i64, align 8
  %6 = alloca i8*, align 8
  %7 = alloca %"class.std::basic_string"*, align 8
  %8 = alloca i64, align 8
  %9 = alloca i8*, align 8
  %10 = alloca i8, align 1
  %11 = alloca %class.anon.0, align 1
  store i64 %2, i64* %5, align 8
  store i8* %1, i8** %6, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %7, align 8
  %12 = load %"class.std::basic_string"*, %"class.std::basic_string"** %7, align 8
  %13 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %12, i32 0, i32 0
  %14 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %13, i32 0, i32 0
  %15 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %14, i32 0, i32 1
  %16 = load i64, i64* %15, align 8
  store i64 %16, i64* %8, align 8
  %17 = load i64, i64* %5, align 8
  %18 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %12, i32 0, i32 0
  %19 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %18, i32 0, i32 0
  %20 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %19, i32 0, i32 2
  %21 = load i64, i64* %20, align 8
  %22 = load i64, i64* %8, align 8
  %23 = sub i64 %21, %22
  %24 = icmp ule i64 %17, %23
  br i1 %24, label %25, label %46

25:                                               ; preds = %3
  %26 = load i64, i64* %8, align 8
  %27 = load i64, i64* %5, align 8
  %28 = add i64 %26, %27
  %29 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %12, i32 0, i32 0
  %30 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %29, i32 0, i32 0
  %31 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %30, i32 0, i32 1
  store i64 %28, i64* %31, align 8
  %32 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %12, i32 0, i32 0
  %33 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %32, i32 0, i32 0
  %34 = call i8* @"?_Myptr@?$_String_val@U?$_Simple_types@D@std@@@std@@QEAAPEADXZ"(%"class.std::_String_val"* %33) #5
  store i8* %34, i8** %9, align 8
  %35 = load i64, i64* %5, align 8
  %36 = load i8*, i8** %6, align 8
  %37 = load i8*, i8** %9, align 8
  %38 = load i64, i64* %8, align 8
  %39 = getelementptr inbounds i8, i8* %37, i64 %38
  %40 = call i8* @"?move@?$_Char_traits@DH@std@@SAPEADQEADQEBD_K@Z"(i8* %39, i8* %36, i64 %35) #5
  store i8 0, i8* %10, align 1
  %41 = load i8*, i8** %9, align 8
  %42 = load i64, i64* %8, align 8
  %43 = load i64, i64* %5, align 8
  %44 = add i64 %42, %43
  %45 = getelementptr inbounds i8, i8* %41, i64 %44
  call void @"?assign@?$_Narrow_char_traits@DH@std@@SAXAEADAEBD@Z"(i8* nonnull align 1 dereferenceable(1) %45, i8* nonnull align 1 dereferenceable(1) %10) #5
  store %"class.std::basic_string"* %12, %"class.std::basic_string"** %4, align 8
  br label %53

46:                                               ; preds = %3
  %47 = load i64, i64* %5, align 8
  %48 = load i8*, i8** %6, align 8
  %49 = load i64, i64* %5, align 8
  %50 = getelementptr inbounds %class.anon.0, %class.anon.0* %11, i32 0, i32 0
  %51 = load i8, i8* %50, align 1
  %52 = call nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"??$_Reallocate_grow_by@V<lambda_1>@?0??append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV34@QEBD_K@Z@PEBD_K@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV01@_KV<lambda_1>@?0??append@01@QEAAAEAV01@QEBD0@Z@PEBD_K@Z"(%"class.std::basic_string"* %12, i64 %49, i8 %51, i8* %48, i64 %47)
  store %"class.std::basic_string"* %52, %"class.std::basic_string"** %4, align 8
  br label %53

53:                                               ; preds = %46, %25
  %54 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  ret %"class.std::basic_string"* %54
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"??$_Reallocate_grow_by@V<lambda_1>@?0??append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV34@QEBD_K@Z@PEBD_K@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV01@_KV<lambda_1>@?0??append@01@QEAAAEAV01@QEBD0@Z@PEBD_K@Z"(%"class.std::basic_string"* %0, i64 %1, i8 %2, i8* %3, i64 %4) #1 comdat align 2 {
  %6 = alloca %class.anon.0, align 1
  %7 = alloca i64, align 8
  %8 = alloca i8*, align 8
  %9 = alloca i64, align 8
  %10 = alloca %"class.std::basic_string"*, align 8
  %11 = alloca %"class.std::_String_val"*, align 8
  %12 = alloca i64, align 8
  %13 = alloca i64, align 8
  %14 = alloca i64, align 8
  %15 = alloca i64, align 8
  %16 = alloca %"class.std::allocator"*, align 8
  %17 = alloca i8*, align 8
  %18 = alloca i8*, align 8
  %19 = alloca i8*, align 8
  %20 = getelementptr inbounds %class.anon.0, %class.anon.0* %6, i32 0, i32 0
  store i8 %2, i8* %20, align 1
  store i64 %4, i64* %7, align 8
  store i8* %3, i8** %8, align 8
  store i64 %1, i64* %9, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %10, align 8
  %21 = load %"class.std::basic_string"*, %"class.std::basic_string"** %10, align 8
  %22 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %21, i32 0, i32 0
  %23 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %22, i32 0, i32 0
  store %"class.std::_String_val"* %23, %"class.std::_String_val"** %11, align 8
  %24 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %25 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %24, i32 0, i32 1
  %26 = load i64, i64* %25, align 8
  store i64 %26, i64* %12, align 8
  %27 = call i64 @"?max_size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %21) #5
  %28 = load i64, i64* %12, align 8
  %29 = sub i64 %27, %28
  %30 = load i64, i64* %9, align 8
  %31 = icmp ult i64 %29, %30
  br i1 %31, label %32, label %33

32:                                               ; preds = %5
  call void @"?_Xlen_string@std@@YAXXZ"() #19
  unreachable

33:                                               ; preds = %5
  %34 = load i64, i64* %12, align 8
  %35 = load i64, i64* %9, align 8
  %36 = add i64 %34, %35
  store i64 %36, i64* %13, align 8
  %37 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %38 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %37, i32 0, i32 2
  %39 = load i64, i64* %38, align 8
  store i64 %39, i64* %14, align 8
  %40 = load i64, i64* %13, align 8
  %41 = call i64 @"?_Calculate_growth@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEBA_K_K@Z"(%"class.std::basic_string"* %21, i64 %40) #5
  store i64 %41, i64* %15, align 8
  %42 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Getal@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV?$allocator@D@2@XZ"(%"class.std::basic_string"* %21) #5
  store %"class.std::allocator"* %42, %"class.std::allocator"** %16, align 8
  %43 = load %"class.std::allocator"*, %"class.std::allocator"** %16, align 8
  %44 = load i64, i64* %15, align 8
  %45 = add i64 %44, 1
  %46 = call i8* @"?allocate@?$allocator@D@std@@QEAAPEAD_K@Z"(%"class.std::allocator"* %43, i64 %45)
  store i8* %46, i8** %17, align 8
  %47 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %48 = bitcast %"class.std::_String_val"* %47 to %"struct.std::_Container_base0"*
  call void @"?_Orphan_all@_Container_base0@std@@QEAAXXZ"(%"struct.std::_Container_base0"* %48) #5
  %49 = load i64, i64* %13, align 8
  %50 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %51 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %50, i32 0, i32 1
  store i64 %49, i64* %51, align 8
  %52 = load i64, i64* %15, align 8
  %53 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %54 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %53, i32 0, i32 2
  store i64 %52, i64* %54, align 8
  %55 = load i8*, i8** %17, align 8
  %56 = call i8* @"??$_Unfancy@D@std@@YAPEADPEAD@Z"(i8* %55) #5
  store i8* %56, i8** %18, align 8
  %57 = load i64, i64* %14, align 8
  %58 = icmp ule i64 16, %57
  br i1 %58, label %59, label %78

59:                                               ; preds = %33
  %60 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %61 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %60, i32 0, i32 0
  %62 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %61 to i8**
  %63 = load i8*, i8** %62, align 8
  store i8* %63, i8** %19, align 8
  %64 = load i64, i64* %7, align 8
  %65 = load i8*, i8** %8, align 8
  %66 = load i64, i64* %12, align 8
  %67 = load i8*, i8** %19, align 8
  %68 = call i8* @"??$_Unfancy@D@std@@YAPEADPEAD@Z"(i8* %67) #5
  %69 = load i8*, i8** %18, align 8
  call void @"??R<lambda_1>@?0??append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@QEBD_K@Z@QEBA?A?<auto>@@QEAD0101@Z"(%class.anon.0* %6, i8* %69, i8* %68, i64 %66, i8* %65, i64 %64)
  %70 = load %"class.std::allocator"*, %"class.std::allocator"** %16, align 8
  %71 = load i64, i64* %14, align 8
  %72 = add i64 %71, 1
  %73 = load i8*, i8** %19, align 8
  call void @"?deallocate@?$allocator@D@std@@QEAAXQEAD_K@Z"(%"class.std::allocator"* %70, i8* %73, i64 %72)
  %74 = load i8*, i8** %17, align 8
  %75 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %76 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %75, i32 0, i32 0
  %77 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %76 to i8**
  store i8* %74, i8** %77, align 8
  br label %90

78:                                               ; preds = %33
  %79 = load i64, i64* %7, align 8
  %80 = load i8*, i8** %8, align 8
  %81 = load i64, i64* %12, align 8
  %82 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %83 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %82, i32 0, i32 0
  %84 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %83 to [16 x i8]*
  %85 = getelementptr inbounds [16 x i8], [16 x i8]* %84, i64 0, i64 0
  %86 = load i8*, i8** %18, align 8
  call void @"??R<lambda_1>@?0??append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@QEBD_K@Z@QEBA?A?<auto>@@QEAD0101@Z"(%class.anon.0* %6, i8* %86, i8* %85, i64 %81, i8* %80, i64 %79)
  %87 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %88 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %87, i32 0, i32 0
  %89 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %88 to i8**
  call void @"??$_Construct_in_place@PEADAEBQEAD@std@@YAXAEAPEADAEBQEAD@Z"(i8** nonnull align 8 dereferenceable(8) %89, i8** nonnull align 8 dereferenceable(8) %17) #5
  br label %90

90:                                               ; preds = %78, %59
  ret %"class.std::basic_string"* %21
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??R<lambda_1>@?0??append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@QEBD_K@Z@QEBA?A?<auto>@@QEAD0101@Z"(%class.anon.0* %0, i8* %1, i8* %2, i64 %3, i8* %4, i64 %5) #3 comdat align 2 {
  %7 = alloca i64, align 8
  %8 = alloca i8*, align 8
  %9 = alloca i64, align 8
  %10 = alloca i8*, align 8
  %11 = alloca i8*, align 8
  %12 = alloca %class.anon.0*, align 8
  %13 = alloca i8, align 1
  store i64 %5, i64* %7, align 8
  store i8* %4, i8** %8, align 8
  store i64 %3, i64* %9, align 8
  store i8* %2, i8** %10, align 8
  store i8* %1, i8** %11, align 8
  store %class.anon.0* %0, %class.anon.0** %12, align 8
  %14 = load %class.anon.0*, %class.anon.0** %12, align 8
  %15 = load i64, i64* %9, align 8
  %16 = load i8*, i8** %10, align 8
  %17 = load i8*, i8** %11, align 8
  %18 = call i8* @"?copy@?$_Char_traits@DH@std@@SAPEADQEADQEBD_K@Z"(i8* %17, i8* %16, i64 %15) #5
  %19 = load i64, i64* %7, align 8
  %20 = load i8*, i8** %8, align 8
  %21 = load i8*, i8** %11, align 8
  %22 = load i64, i64* %9, align 8
  %23 = getelementptr inbounds i8, i8* %21, i64 %22
  %24 = call i8* @"?copy@?$_Char_traits@DH@std@@SAPEADQEADQEBD_K@Z"(i8* %23, i8* %20, i64 %19) #5
  store i8 0, i8* %13, align 1
  %25 = load i8*, i8** %11, align 8
  %26 = load i64, i64* %9, align 8
  %27 = load i64, i64* %7, align 8
  %28 = add i64 %26, %27
  %29 = getelementptr inbounds i8, i8* %25, i64 %28
  call void @"?assign@?$_Narrow_char_traits@DH@std@@SAXAEADAEBD@Z"(i8* nonnull align 1 dereferenceable(1) %29, i8* nonnull align 1 dereferenceable(1) %13) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"?_Myptr@?$_String_val@U?$_Simple_types@D@std@@@std@@QEBAPEBDXZ"(%"class.std::_String_val"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::_String_val"*, align 8
  %3 = alloca i8*, align 8
  store %"class.std::_String_val"* %0, %"class.std::_String_val"** %2, align 8
  %4 = load %"class.std::_String_val"*, %"class.std::_String_val"** %2, align 8
  %5 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %4, i32 0, i32 0
  %6 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %5 to [16 x i8]*
  %7 = getelementptr inbounds [16 x i8], [16 x i8]* %6, i64 0, i64 0
  store i8* %7, i8** %3, align 8
  %8 = call zeroext i1 @"?_Large_string_engaged@?$_String_val@U?$_Simple_types@D@std@@@std@@QEBA_NXZ"(%"class.std::_String_val"* %4) #5
  br i1 %8, label %9, label %14

9:                                                ; preds = %1
  %10 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %4, i32 0, i32 0
  %11 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %10 to i8**
  %12 = load i8*, i8** %11, align 8
  %13 = call i8* @"??$_Unfancy@D@std@@YAPEADPEAD@Z"(i8* %12) #5
  store i8* %13, i8** %3, align 8
  br label %14

14:                                               ; preds = %9, %1
  %15 = load i8*, i8** %3, align 8
  ret i8* %15
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"??$move@AEAV?$allocator@D@std@@@std@@YA$$QEAV?$allocator@D@0@AEAV10@@Z"(%"class.std::allocator"* nonnull align 1 dereferenceable(1) %0) #3 comdat {
  %2 = alloca %"class.std::allocator"*, align 8
  store %"class.std::allocator"* %0, %"class.std::allocator"** %2, align 8
  %3 = load %"class.std::allocator"*, %"class.std::allocator"** %2, align 8
  ret %"class.std::allocator"* %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::_Compressed_pair"* @"??$?0V?$allocator@D@std@@$$V@?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@U_One_then_variadic_args_t@1@$$QEAV?$allocator@D@1@@Z"(%"class.std::_Compressed_pair"* returned %0, i8 %1, %"class.std::allocator"* nonnull align 1 dereferenceable(1) %2) unnamed_addr #3 comdat align 2 {
  %4 = alloca %"struct.std::_One_then_variadic_args_t", align 1
  %5 = alloca %"class.std::allocator"*, align 8
  %6 = alloca %"class.std::_Compressed_pair"*, align 8
  %7 = getelementptr inbounds %"struct.std::_One_then_variadic_args_t", %"struct.std::_One_then_variadic_args_t"* %4, i32 0, i32 0
  store i8 %1, i8* %7, align 1
  store %"class.std::allocator"* %2, %"class.std::allocator"** %5, align 8
  store %"class.std::_Compressed_pair"* %0, %"class.std::_Compressed_pair"** %6, align 8
  %8 = load %"class.std::_Compressed_pair"*, %"class.std::_Compressed_pair"** %6, align 8
  %9 = bitcast %"class.std::_Compressed_pair"* %8 to %"class.std::allocator"*
  %10 = load %"class.std::allocator"*, %"class.std::allocator"** %5, align 8
  %11 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"??$forward@V?$allocator@D@std@@@std@@YA$$QEAV?$allocator@D@0@AEAV10@@Z"(%"class.std::allocator"* nonnull align 1 dereferenceable(1) %10) #5
  %12 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %8, i32 0, i32 0
  %13 = call %"class.std::_String_val"* @"??0?$_String_val@U?$_Simple_types@D@std@@@std@@QEAA@XZ"(%"class.std::_String_val"* %12) #5
  ret %"class.std::_Compressed_pair"* %8
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?_Alloc_proxy@_Container_base0@std@@QEAAXAEBU_Fake_allocator@2@@Z"(%"struct.std::_Container_base0"* %0, %"struct.std::_Fake_allocator"* nonnull align 1 dereferenceable(1) %1) #3 comdat align 2 {
  %3 = alloca %"struct.std::_Fake_allocator"*, align 8
  %4 = alloca %"struct.std::_Container_base0"*, align 8
  store %"struct.std::_Fake_allocator"* %1, %"struct.std::_Fake_allocator"** %3, align 8
  store %"struct.std::_Container_base0"* %0, %"struct.std::_Container_base0"** %4, align 8
  %5 = load %"struct.std::_Container_base0"*, %"struct.std::_Container_base0"** %4, align 8
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?_Take_contents@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXAEAV12@@Z"(%"class.std::basic_string"* %0, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %1) #3 comdat align 2 {
  %3 = alloca %"class.std::basic_string"*, align 8
  %4 = alloca %"class.std::basic_string"*, align 8
  %5 = alloca %"class.std::_String_val"*, align 8
  %6 = alloca %"class.std::_String_val"*, align 8
  store %"class.std::basic_string"* %1, %"class.std::basic_string"** %3, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %4, align 8
  %7 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  %8 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %7, i32 0, i32 0
  %9 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %8, i32 0, i32 0
  store %"class.std::_String_val"* %9, %"class.std::_String_val"** %5, align 8
  %10 = load %"class.std::basic_string"*, %"class.std::basic_string"** %3, align 8
  %11 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %10, i32 0, i32 0
  %12 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %11, i32 0, i32 0
  store %"class.std::_String_val"* %12, %"class.std::_String_val"** %6, align 8
  %13 = load %"class.std::basic_string"*, %"class.std::basic_string"** %3, align 8
  call void @"?_Memcpy_val_from@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXAEBV12@@Z"(%"class.std::basic_string"* %7, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %13) #5
  %14 = load %"class.std::basic_string"*, %"class.std::basic_string"** %3, align 8
  call void @"?_Tidy_init@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXXZ"(%"class.std::basic_string"* %14) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"??$forward@V?$allocator@D@std@@@std@@YA$$QEAV?$allocator@D@0@AEAV10@@Z"(%"class.std::allocator"* nonnull align 1 dereferenceable(1) %0) #3 comdat {
  %2 = alloca %"class.std::allocator"*, align 8
  store %"class.std::allocator"* %0, %"class.std::allocator"** %2, align 8
  %3 = load %"class.std::allocator"*, %"class.std::allocator"** %2, align 8
  ret %"class.std::allocator"* %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?_Memcpy_val_from@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXAEBV12@@Z"(%"class.std::basic_string"* %0, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %1) #3 comdat align 2 {
  %3 = alloca %"class.std::basic_string"*, align 8
  %4 = alloca %"class.std::basic_string"*, align 8
  %5 = alloca i8*, align 8
  %6 = alloca i8*, align 8
  store %"class.std::basic_string"* %1, %"class.std::basic_string"** %3, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %4, align 8
  %7 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  %8 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %7, i32 0, i32 0
  %9 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %8, i32 0, i32 0
  %10 = call %"class.std::_String_val"* @"??$addressof@V?$_String_val@U?$_Simple_types@D@std@@@std@@@std@@YAPEAV?$_String_val@U?$_Simple_types@D@std@@@0@AEAV10@@Z"(%"class.std::_String_val"* nonnull align 8 dereferenceable(32) %9) #5
  %11 = bitcast %"class.std::_String_val"* %10 to i8*
  %12 = getelementptr inbounds i8, i8* %11, i64 0
  store i8* %12, i8** %5, align 8
  %13 = load %"class.std::basic_string"*, %"class.std::basic_string"** %3, align 8
  %14 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %13, i32 0, i32 0
  %15 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %14, i32 0, i32 0
  %16 = call %"class.std::_String_val"* @"??$addressof@$$CBV?$_String_val@U?$_Simple_types@D@std@@@std@@@std@@YAPEBV?$_String_val@U?$_Simple_types@D@std@@@0@AEBV10@@Z"(%"class.std::_String_val"* nonnull align 8 dereferenceable(32) %15) #5
  %17 = bitcast %"class.std::_String_val"* %16 to i8*
  %18 = getelementptr inbounds i8, i8* %17, i64 0
  store i8* %18, i8** %6, align 8
  %19 = load i8*, i8** %5, align 8
  %20 = load i8*, i8** %6, align 8
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %19, i8* align 1 %20, i64 32, i1 false)
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::_String_val"* @"??$addressof@V?$_String_val@U?$_Simple_types@D@std@@@std@@@std@@YAPEAV?$_String_val@U?$_Simple_types@D@std@@@0@AEAV10@@Z"(%"class.std::_String_val"* nonnull align 8 dereferenceable(32) %0) #3 comdat {
  %2 = alloca %"class.std::_String_val"*, align 8
  store %"class.std::_String_val"* %0, %"class.std::_String_val"** %2, align 8
  %3 = load %"class.std::_String_val"*, %"class.std::_String_val"** %2, align 8
  ret %"class.std::_String_val"* %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::_String_val"* @"??$addressof@$$CBV?$_String_val@U?$_Simple_types@D@std@@@std@@@std@@YAPEBV?$_String_val@U?$_Simple_types@D@std@@@0@AEBV10@@Z"(%"class.std::_String_val"* nonnull align 8 dereferenceable(32) %0) #3 comdat {
  %2 = alloca %"class.std::_String_val"*, align 8
  store %"class.std::_String_val"* %0, %"class.std::_String_val"** %2, align 8
  %3 = load %"class.std::_String_val"*, %"class.std::_String_val"** %2, align 8
  ret %"class.std::_String_val"* %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?select_on_container_copy_construction@?$_Default_allocator_traits@V?$allocator@D@std@@@std@@SA?AV?$allocator@D@2@AEBV32@@Z"(%"class.std::allocator"* noalias sret align 1 %0, %"class.std::allocator"* nonnull align 1 dereferenceable(1) %1) #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::allocator"*, align 8
  %5 = bitcast %"class.std::allocator"* %0 to i8*
  store i8* %5, i8** %3, align 8
  store %"class.std::allocator"* %1, %"class.std::allocator"** %4, align 8
  %6 = load %"class.std::allocator"*, %"class.std::allocator"** %4, align 8
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?_Construct_lv_contents@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXAEBV12@@Z"(%"class.std::basic_string"* %0, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %1) #1 comdat align 2 {
  %3 = alloca %"class.std::basic_string"*, align 8
  %4 = alloca %"class.std::basic_string"*, align 8
  %5 = alloca %"class.std::_String_val"*, align 8
  %6 = alloca i64, align 8
  %7 = alloca i8*, align 8
  %8 = alloca %"class.std::_String_val"*, align 8
  %9 = alloca i8, align 1
  %10 = alloca %"class.std::allocator"*, align 8
  %11 = alloca i64, align 8
  %12 = alloca i64, align 8
  %13 = alloca i64, align 8
  %14 = alloca i8*, align 8
  store %"class.std::basic_string"* %1, %"class.std::basic_string"** %3, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %4, align 8
  %15 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  %16 = load %"class.std::basic_string"*, %"class.std::basic_string"** %3, align 8
  %17 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %16, i32 0, i32 0
  %18 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %17, i32 0, i32 0
  store %"class.std::_String_val"* %18, %"class.std::_String_val"** %5, align 8
  %19 = load %"class.std::_String_val"*, %"class.std::_String_val"** %5, align 8
  %20 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %19, i32 0, i32 1
  %21 = load i64, i64* %20, align 8
  store i64 %21, i64* %6, align 8
  %22 = load %"class.std::_String_val"*, %"class.std::_String_val"** %5, align 8
  %23 = call i8* @"?_Myptr@?$_String_val@U?$_Simple_types@D@std@@@std@@QEBAPEBDXZ"(%"class.std::_String_val"* %22) #5
  store i8* %23, i8** %7, align 8
  %24 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %15, i32 0, i32 0
  %25 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %24, i32 0, i32 0
  store %"class.std::_String_val"* %25, %"class.std::_String_val"** %8, align 8
  %26 = load i64, i64* %6, align 8
  %27 = icmp ult i64 %26, 16
  %28 = zext i1 %27 to i8
  store i8 %28, i8* %9, align 1
  %29 = load i8, i8* %9, align 1
  %30 = trunc i8 %29 to i1
  br i1 %30, label %31, label %43

31:                                               ; preds = %2
  %32 = load i8*, i8** %7, align 8
  %33 = load %"class.std::_String_val"*, %"class.std::_String_val"** %8, align 8
  %34 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %33, i32 0, i32 0
  %35 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %34 to [16 x i8]*
  %36 = getelementptr inbounds [16 x i8], [16 x i8]* %35, i64 0, i64 0
  %37 = call i8* @"?copy@?$_Char_traits@DH@std@@SAPEADQEADQEBD_K@Z"(i8* %36, i8* %32, i64 16) #5
  %38 = load i64, i64* %6, align 8
  %39 = load %"class.std::_String_val"*, %"class.std::_String_val"** %8, align 8
  %40 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %39, i32 0, i32 1
  store i64 %38, i64* %40, align 8
  %41 = load %"class.std::_String_val"*, %"class.std::_String_val"** %8, align 8
  %42 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %41, i32 0, i32 2
  store i64 15, i64* %42, align 8
  br label %69

43:                                               ; preds = %2
  %44 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Getal@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV?$allocator@D@2@XZ"(%"class.std::basic_string"* %15) #5
  store %"class.std::allocator"* %44, %"class.std::allocator"** %10, align 8
  %45 = call i64 @"?max_size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %15) #5
  store i64 %45, i64* %12, align 8
  %46 = load i64, i64* %6, align 8
  %47 = or i64 %46, 15
  store i64 %47, i64* %13, align 8
  %48 = call nonnull align 8 dereferenceable(8) i64* @"??$min@_K@std@@YAAEB_KAEB_K0@Z"(i64* nonnull align 8 dereferenceable(8) %13, i64* nonnull align 8 dereferenceable(8) %12) #5
  %49 = load i64, i64* %48, align 8
  store i64 %49, i64* %11, align 8
  %50 = load %"class.std::allocator"*, %"class.std::allocator"** %10, align 8
  %51 = load i64, i64* %11, align 8
  %52 = add i64 %51, 1
  %53 = call i8* @"?allocate@?$allocator@D@std@@QEAAPEAD_K@Z"(%"class.std::allocator"* %50, i64 %52)
  store i8* %53, i8** %14, align 8
  %54 = load %"class.std::_String_val"*, %"class.std::_String_val"** %8, align 8
  %55 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %54, i32 0, i32 0
  %56 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %55 to i8**
  call void @"??$_Construct_in_place@PEADAEBQEAD@std@@YAXAEAPEADAEBQEAD@Z"(i8** nonnull align 8 dereferenceable(8) %56, i8** nonnull align 8 dereferenceable(8) %14) #5
  %57 = load i64, i64* %6, align 8
  %58 = add i64 %57, 1
  %59 = load i8*, i8** %7, align 8
  %60 = load i8*, i8** %14, align 8
  %61 = call i8* @"??$_Unfancy@D@std@@YAPEADPEAD@Z"(i8* %60) #5
  %62 = call i8* @"?copy@?$_Char_traits@DH@std@@SAPEADQEADQEBD_K@Z"(i8* %61, i8* %59, i64 %58) #5
  %63 = load i64, i64* %6, align 8
  %64 = load %"class.std::_String_val"*, %"class.std::_String_val"** %8, align 8
  %65 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %64, i32 0, i32 1
  store i64 %63, i64* %65, align 8
  %66 = load i64, i64* %11, align 8
  %67 = load %"class.std::_String_val"*, %"class.std::_String_val"** %8, align 8
  %68 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %67, i32 0, i32 2
  store i64 %66, i64* %68, align 8
  br label %69

69:                                               ; preds = %43, %31
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"?c_str@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBAPEBDXZ"(%"class.std::basic_string"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::basic_string"*, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %2, align 8
  %3 = load %"class.std::basic_string"*, %"class.std::basic_string"** %2, align 8
  %4 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %3, i32 0, i32 0
  %5 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %4, i32 0, i32 0
  %6 = call i8* @"?_Myptr@?$_String_val@U?$_Simple_types@D@std@@@std@@QEBAPEBDXZ"(%"class.std::_String_val"* %5) #5
  ret i8* %6
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::exception"* @"??0exception@std@@QEAA@QEBD@Z"(%"class.std::exception"* returned %0, i8* %1) unnamed_addr #3 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::exception"*, align 8
  %5 = alloca %struct.__std_exception_data, align 8
  store i8* %1, i8** %3, align 8
  store %"class.std::exception"* %0, %"class.std::exception"** %4, align 8
  %6 = load %"class.std::exception"*, %"class.std::exception"** %4, align 8
  %7 = bitcast %"class.std::exception"* %6 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7exception@std@@6B@" to i32 (...)**), i32 (...)*** %7, align 8
  %8 = getelementptr inbounds %"class.std::exception", %"class.std::exception"* %6, i32 0, i32 1
  %9 = bitcast %struct.__std_exception_data* %8 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 8 %9, i8 0, i64 16, i1 false)
  %10 = getelementptr inbounds %struct.__std_exception_data, %struct.__std_exception_data* %5, i32 0, i32 0
  %11 = load i8*, i8** %3, align 8
  store i8* %11, i8** %10, align 8
  %12 = getelementptr inbounds %struct.__std_exception_data, %struct.__std_exception_data* %5, i32 0, i32 1
  store i8 1, i8* %12, align 8
  %13 = getelementptr inbounds %"class.std::exception", %"class.std::exception"* %6, i32 0, i32 1
  invoke void @__std_exception_copy(%struct.__std_exception_data* %5, %struct.__std_exception_data* %13)
          to label %14 unwind label %15

14:                                               ; preds = %2
  ret %"class.std::exception"* %6

15:                                               ; preds = %2
  %16 = cleanuppad within none []
  call void @__std_terminate() #18 [ "funclet"(token %16) ]
  unreachable
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??_Gruntime_error@std@@UEAAPEAXI@Z"(%"class.std::runtime_error"* %0, i32 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca i32, align 4
  %5 = alloca %"class.std::runtime_error"*, align 8
  store i32 %1, i32* %4, align 4
  store %"class.std::runtime_error"* %0, %"class.std::runtime_error"** %5, align 8
  %6 = load %"class.std::runtime_error"*, %"class.std::runtime_error"** %5, align 8
  %7 = bitcast %"class.std::runtime_error"* %6 to i8*
  store i8* %7, i8** %3, align 8
  %8 = load i32, i32* %4, align 4
  call void @"??1runtime_error@std@@UEAA@XZ"(%"class.std::runtime_error"* %6) #5
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %12, label %10

10:                                               ; preds = %2
  %11 = bitcast %"class.std::runtime_error"* %6 to i8*
  call void @"??3@YAXPEAX@Z"(i8* %11) #20
  br label %12

12:                                               ; preds = %10, %2
  %13 = load i8*, i8** %3, align 8
  ret i8* %13
}

declare dso_local void @__std_exception_copy(%struct.__std_exception_data*, %struct.__std_exception_data*) #4

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1runtime_error@std@@UEAA@XZ"(%"class.std::runtime_error"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::runtime_error"*, align 8
  store %"class.std::runtime_error"* %0, %"class.std::runtime_error"** %2, align 8
  %3 = load %"class.std::runtime_error"*, %"class.std::runtime_error"** %2, align 8
  %4 = bitcast %"class.std::runtime_error"* %3 to %"class.std::exception"*
  call void @"??1exception@std@@UEAA@XZ"(%"class.std::exception"* %4) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1_System_error@std@@UEAA@XZ"(%"class.std::_System_error"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::_System_error"*, align 8
  store %"class.std::_System_error"* %0, %"class.std::_System_error"** %2, align 8
  %3 = load %"class.std::_System_error"*, %"class.std::_System_error"** %2, align 8
  %4 = bitcast %"class.std::_System_error"* %3 to %"class.std::runtime_error"*
  call void @"??1runtime_error@std@@UEAA@XZ"(%"class.std::runtime_error"* %4) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?_Tidy_deallocate@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXXZ"(%"class.std::basic_string"* %0) #3 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %2 = alloca %"class.std::basic_string"*, align 8
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::allocator"*, align 8
  %5 = alloca i8, align 1
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %2, align 8
  %6 = load %"class.std::basic_string"*, %"class.std::basic_string"** %2, align 8
  %7 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %6, i32 0, i32 0
  %8 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %7, i32 0, i32 0
  %9 = bitcast %"class.std::_String_val"* %8 to %"struct.std::_Container_base0"*
  call void @"?_Orphan_all@_Container_base0@std@@QEAAXXZ"(%"struct.std::_Container_base0"* %9) #5
  %10 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %6, i32 0, i32 0
  %11 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %10, i32 0, i32 0
  %12 = call zeroext i1 @"?_Large_string_engaged@?$_String_val@U?$_Simple_types@D@std@@@std@@QEBA_NXZ"(%"class.std::_String_val"* %11) #5
  br i1 %12, label %13, label %32

13:                                               ; preds = %1
  %14 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %6, i32 0, i32 0
  %15 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %14, i32 0, i32 0
  %16 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %15, i32 0, i32 0
  %17 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %16 to i8**
  %18 = load i8*, i8** %17, align 8
  store i8* %18, i8** %3, align 8
  %19 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Getal@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV?$allocator@D@2@XZ"(%"class.std::basic_string"* %6) #5
  store %"class.std::allocator"* %19, %"class.std::allocator"** %4, align 8
  %20 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %6, i32 0, i32 0
  %21 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %20, i32 0, i32 0
  %22 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %21, i32 0, i32 0
  %23 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %22 to i8**
  call void @"??$_Destroy_in_place@PEAD@std@@YAXAEAPEAD@Z"(i8** nonnull align 8 dereferenceable(8) %23) #5
  %24 = load %"class.std::allocator"*, %"class.std::allocator"** %4, align 8
  %25 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %6, i32 0, i32 0
  %26 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %25, i32 0, i32 0
  %27 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %26, i32 0, i32 2
  %28 = load i64, i64* %27, align 8
  %29 = add i64 %28, 1
  %30 = load i8*, i8** %3, align 8
  invoke void @"?deallocate@?$allocator@D@std@@QEAAXQEAD_K@Z"(%"class.std::allocator"* %24, i8* %30, i64 %29)
          to label %31 unwind label %44

31:                                               ; preds = %13
  br label %32

32:                                               ; preds = %31, %1
  %33 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %6, i32 0, i32 0
  %34 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %33, i32 0, i32 0
  %35 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %34, i32 0, i32 1
  store i64 0, i64* %35, align 8
  %36 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %6, i32 0, i32 0
  %37 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %36, i32 0, i32 0
  %38 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %37, i32 0, i32 2
  store i64 15, i64* %38, align 8
  store i8 0, i8* %5, align 1
  %39 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %6, i32 0, i32 0
  %40 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %39, i32 0, i32 0
  %41 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %40, i32 0, i32 0
  %42 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %41 to [16 x i8]*
  %43 = getelementptr inbounds [16 x i8], [16 x i8]* %42, i64 0, i64 0
  call void @"?assign@?$_Narrow_char_traits@DH@std@@SAXAEADAEBD@Z"(i8* nonnull align 1 dereferenceable(1) %43, i8* nonnull align 1 dereferenceable(1) %5) #5
  ret void

44:                                               ; preds = %13
  %45 = cleanuppad within none []
  call void @__std_terminate() #18 [ "funclet"(token %45) ]
  unreachable
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??$_Destroy_in_place@PEAD@std@@YAXAEAPEAD@Z"(i8** nonnull align 8 dereferenceable(8) %0) #3 comdat {
  %2 = alloca i8**, align 8
  store i8** %0, i8*** %2, align 8
  %3 = load i8**, i8*** %2, align 8
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1system_error@std@@UEAA@XZ"(%"class.std::system_error"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::system_error"*, align 8
  store %"class.std::system_error"* %0, %"class.std::system_error"** %2, align 8
  %3 = load %"class.std::system_error"*, %"class.std::system_error"** %2, align 8
  %4 = bitcast %"class.std::system_error"* %3 to %"class.std::_System_error"*
  call void @"??1_System_error@std@@UEAA@XZ"(%"class.std::_System_error"* %4) #5
  ret void
}

; Function Attrs: nounwind
declare dso_local zeroext i1 @"?uncaught_exception@std@@YA_NXZ"() #12

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?_Osfx@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAXXZ"(%"class.std::basic_ostream"* %0) #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %2 = alloca %"class.std::basic_ostream"*, align 8
  store %"class.std::basic_ostream"* %0, %"class.std::basic_ostream"** %2, align 8
  %3 = load %"class.std::basic_ostream"*, %"class.std::basic_ostream"** %2, align 8
  %4 = bitcast %"class.std::basic_ostream"* %3 to i8*
  %5 = getelementptr inbounds i8, i8* %4, i64 0
  %6 = bitcast i8* %5 to i32**
  %7 = load i32*, i32** %6, align 8
  %8 = getelementptr inbounds i32, i32* %7, i32 1
  %9 = load i32, i32* %8, align 4
  %10 = sext i32 %9 to i64
  %11 = add nsw i64 0, %10
  %12 = bitcast %"class.std::basic_ostream"* %3 to i8*
  %13 = getelementptr inbounds i8, i8* %12, i64 %11
  %14 = bitcast i8* %13 to %"class.std::ios_base"*
  %15 = invoke zeroext i1 @"?good@ios_base@std@@QEBA_NXZ"(%"class.std::ios_base"* %14)
          to label %16 unwind label %62

16:                                               ; preds = %1
  br i1 %15, label %17, label %70

17:                                               ; preds = %16
  %18 = bitcast %"class.std::basic_ostream"* %3 to i8*
  %19 = getelementptr inbounds i8, i8* %18, i64 0
  %20 = bitcast i8* %19 to i32**
  %21 = load i32*, i32** %20, align 8
  %22 = getelementptr inbounds i32, i32* %21, i32 1
  %23 = load i32, i32* %22, align 4
  %24 = sext i32 %23 to i64
  %25 = add nsw i64 0, %24
  %26 = bitcast %"class.std::basic_ostream"* %3 to i8*
  %27 = getelementptr inbounds i8, i8* %26, i64 %25
  %28 = bitcast i8* %27 to %"class.std::ios_base"*
  %29 = invoke i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %28)
          to label %30 unwind label %62

30:                                               ; preds = %17
  %31 = and i32 %29, 2
  %32 = icmp ne i32 %31, 0
  br i1 %32, label %33, label %70

33:                                               ; preds = %30
  %34 = bitcast %"class.std::basic_ostream"* %3 to i8*
  %35 = getelementptr inbounds i8, i8* %34, i64 0
  %36 = bitcast i8* %35 to i32**
  %37 = load i32*, i32** %36, align 8
  %38 = getelementptr inbounds i32, i32* %37, i32 1
  %39 = load i32, i32* %38, align 4
  %40 = sext i32 %39 to i64
  %41 = add nsw i64 0, %40
  %42 = bitcast %"class.std::basic_ostream"* %3 to i8*
  %43 = getelementptr inbounds i8, i8* %42, i64 %41
  %44 = bitcast i8* %43 to %"class.std::basic_ios"*
  %45 = invoke %"class.std::basic_streambuf"* @"?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ"(%"class.std::basic_ios"* %44)
          to label %46 unwind label %62

46:                                               ; preds = %33
  %47 = invoke i32 @"?pubsync@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHXZ"(%"class.std::basic_streambuf"* %45)
          to label %48 unwind label %62

48:                                               ; preds = %46
  %49 = icmp eq i32 %47, -1
  br i1 %49, label %50, label %69

50:                                               ; preds = %48
  %51 = bitcast %"class.std::basic_ostream"* %3 to i8*
  %52 = getelementptr inbounds i8, i8* %51, i64 0
  %53 = bitcast i8* %52 to i32**
  %54 = load i32*, i32** %53, align 8
  %55 = getelementptr inbounds i32, i32* %54, i32 1
  %56 = load i32, i32* %55, align 4
  %57 = sext i32 %56 to i64
  %58 = add nsw i64 0, %57
  %59 = bitcast %"class.std::basic_ostream"* %3 to i8*
  %60 = getelementptr inbounds i8, i8* %59, i64 %58
  %61 = bitcast i8* %60 to %"class.std::basic_ios"*
  invoke void @"?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z"(%"class.std::basic_ios"* %61, i32 4, i1 zeroext false)
          to label %68 unwind label %62

62:                                               ; preds = %50, %46, %33, %17, %1
  %63 = catchswitch within none [label %64] unwind to caller

64:                                               ; preds = %62
  %65 = catchpad within %63 [i8* null, i32 64, i8* null]
  catchret from %65 to label %66

66:                                               ; preds = %64
  br label %67

67:                                               ; preds = %66, %70
  ret void

68:                                               ; preds = %50
  br label %69

69:                                               ; preds = %68, %48
  br label %70

70:                                               ; preds = %69, %30, %16
  br label %67
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(16) %"class.std::num_put"* @"??$use_facet@V?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@@std@@YAAEBV?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@0@AEBVlocale@0@@Z"(%"class.std::locale"* nonnull align 8 dereferenceable(16) %0) #1 comdat personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %2 = alloca %"class.std::locale"*, align 8
  %3 = alloca %"class.std::_Lockit", align 4
  %4 = alloca %"class.std::locale::facet"*, align 8
  %5 = alloca i64, align 8
  %6 = alloca %"class.std::locale::facet"*, align 8
  %7 = alloca %"class.std::locale::facet"*, align 8
  %8 = alloca %"class.std::unique_ptr", align 8
  store %"class.std::locale"* %0, %"class.std::locale"** %2, align 8
  %9 = call %"class.std::_Lockit"* @"??0_Lockit@std@@QEAA@H@Z"(%"class.std::_Lockit"* %3, i32 0) #5
  %10 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** @"?_Psave@?$_Facetptr@V?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@@std@@2PEBVfacet@locale@2@EB", align 8
  store %"class.std::locale::facet"* %10, %"class.std::locale::facet"** %4, align 8
  %11 = invoke i64 @"??Bid@locale@std@@QEAA_KXZ"(%"class.std::locale::id"* @"?id@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@2V0locale@2@A")
          to label %12 unwind label %54

12:                                               ; preds = %1
  store i64 %11, i64* %5, align 8
  %13 = load %"class.std::locale"*, %"class.std::locale"** %2, align 8
  %14 = load i64, i64* %5, align 8
  %15 = invoke %"class.std::locale::facet"* @"?_Getfacet@locale@std@@QEBAPEBVfacet@12@_K@Z"(%"class.std::locale"* %13, i64 %14)
          to label %16 unwind label %54

16:                                               ; preds = %12
  store %"class.std::locale::facet"* %15, %"class.std::locale::facet"** %6, align 8
  %17 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %6, align 8
  %18 = icmp ne %"class.std::locale::facet"* %17, null
  br i1 %18, label %51, label %19

19:                                               ; preds = %16
  %20 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %4, align 8
  %21 = icmp ne %"class.std::locale::facet"* %20, null
  br i1 %21, label %22, label %24

22:                                               ; preds = %19
  %23 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %4, align 8
  store %"class.std::locale::facet"* %23, %"class.std::locale::facet"** %6, align 8
  br label %50

24:                                               ; preds = %19
  %25 = load %"class.std::locale"*, %"class.std::locale"** %2, align 8
  %26 = invoke i64 @"?_Getcat@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z"(%"class.std::locale::facet"** %4, %"class.std::locale"* %25)
          to label %27 unwind label %54

27:                                               ; preds = %24
  %28 = icmp eq i64 %26, -1
  br i1 %28, label %29, label %31

29:                                               ; preds = %27
  invoke void @"?_Throw_bad_cast@std@@YAXXZ"() #19
          to label %30 unwind label %54

30:                                               ; preds = %29
  unreachable

31:                                               ; preds = %27
  %32 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %4, align 8
  store %"class.std::locale::facet"* %32, %"class.std::locale::facet"** %7, align 8
  %33 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %7, align 8
  %34 = bitcast %"class.std::locale::facet"* %33 to %"class.std::_Facet_base"*
  %35 = call %"class.std::unique_ptr"* @"??$?0U?$default_delete@V_Facet_base@std@@@std@@$0A@@?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAA@PEAV_Facet_base@1@@Z"(%"class.std::unique_ptr"* %8, %"class.std::_Facet_base"* %34) #5
  %36 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %7, align 8
  %37 = bitcast %"class.std::locale::facet"* %36 to %"class.std::_Facet_base"*
  invoke void @"?_Facet_Register@std@@YAXPEAV_Facet_base@1@@Z"(%"class.std::_Facet_base"* %37)
          to label %38 unwind label %47

38:                                               ; preds = %31
  %39 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %7, align 8
  %40 = bitcast %"class.std::locale::facet"* %39 to void (%"class.std::locale::facet"*)***
  %41 = load void (%"class.std::locale::facet"*)**, void (%"class.std::locale::facet"*)*** %40, align 8
  %42 = getelementptr inbounds void (%"class.std::locale::facet"*)*, void (%"class.std::locale::facet"*)** %41, i64 1
  %43 = load void (%"class.std::locale::facet"*)*, void (%"class.std::locale::facet"*)** %42, align 8
  call void %43(%"class.std::locale::facet"* %39) #5
  %44 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %4, align 8
  store %"class.std::locale::facet"* %44, %"class.std::locale::facet"** @"?_Psave@?$_Facetptr@V?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@@std@@2PEBVfacet@locale@2@EB", align 8
  %45 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %4, align 8
  store %"class.std::locale::facet"* %45, %"class.std::locale::facet"** %6, align 8
  %46 = call %"class.std::_Facet_base"* @"?release@?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAAPEAV_Facet_base@2@XZ"(%"class.std::unique_ptr"* %8) #5
  call void @"??1?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAA@XZ"(%"class.std::unique_ptr"* %8) #5
  br label %49

47:                                               ; preds = %31
  %48 = cleanuppad within none []
  call void @"??1?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAA@XZ"(%"class.std::unique_ptr"* %8) #5 [ "funclet"(token %48) ]
  cleanupret from %48 unwind label %54

49:                                               ; preds = %38
  br label %50

50:                                               ; preds = %49, %22
  br label %51

51:                                               ; preds = %50, %16
  %52 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %6, align 8
  %53 = bitcast %"class.std::locale::facet"* %52 to %"class.std::num_put"*
  call void @"??1_Lockit@std@@QEAA@XZ"(%"class.std::_Lockit"* %3) #5
  ret %"class.std::num_put"* %53

54:                                               ; preds = %47, %29, %24, %12, %1
  %55 = cleanuppad within none []
  call void @"??1_Lockit@std@@QEAA@XZ"(%"class.std::_Lockit"* %3) #5 [ "funclet"(token %55) ]
  cleanupret from %55 unwind to caller
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?getloc@ios_base@std@@QEBA?AVlocale@2@XZ"(%"class.std::ios_base"* %0, %"class.std::locale"* noalias sret align 8 %1) #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::ios_base"*, align 8
  %5 = bitcast %"class.std::locale"* %1 to i8*
  store i8* %5, i8** %3, align 8
  store %"class.std::ios_base"* %0, %"class.std::ios_base"** %4, align 8
  %6 = load %"class.std::ios_base"*, %"class.std::ios_base"** %4, align 8
  %7 = getelementptr inbounds %"class.std::ios_base", %"class.std::ios_base"* %6, i32 0, i32 9
  %8 = load %"class.std::locale"*, %"class.std::locale"** %7, align 8
  %9 = call %"class.std::locale"* @"??0locale@std@@QEAA@AEBV01@@Z"(%"class.std::locale"* %1, %"class.std::locale"* nonnull align 8 dereferenceable(16) %8) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::locale"*, align 8
  store %"class.std::locale"* %0, %"class.std::locale"** %2, align 8
  %3 = load %"class.std::locale"*, %"class.std::locale"** %2, align 8
  %4 = getelementptr inbounds %"class.std::locale", %"class.std::locale"* %3, i32 0, i32 1
  %5 = load %"class.std::locale::_Locimp"*, %"class.std::locale::_Locimp"** %4, align 8
  %6 = icmp ne %"class.std::locale::_Locimp"* %5, null
  br i1 %6, label %7, label %24

7:                                                ; preds = %1
  %8 = getelementptr inbounds %"class.std::locale", %"class.std::locale"* %3, i32 0, i32 1
  %9 = load %"class.std::locale::_Locimp"*, %"class.std::locale::_Locimp"** %8, align 8
  %10 = bitcast %"class.std::locale::_Locimp"* %9 to %"class.std::locale::facet"*
  %11 = bitcast %"class.std::locale::facet"* %10 to %"class.std::_Facet_base"* (%"class.std::locale::facet"*)***
  %12 = load %"class.std::_Facet_base"* (%"class.std::locale::facet"*)**, %"class.std::_Facet_base"* (%"class.std::locale::facet"*)*** %11, align 8
  %13 = getelementptr inbounds %"class.std::_Facet_base"* (%"class.std::locale::facet"*)*, %"class.std::_Facet_base"* (%"class.std::locale::facet"*)** %12, i64 2
  %14 = load %"class.std::_Facet_base"* (%"class.std::locale::facet"*)*, %"class.std::_Facet_base"* (%"class.std::locale::facet"*)** %13, align 8
  %15 = call %"class.std::_Facet_base"* %14(%"class.std::locale::facet"* %10) #5
  %16 = icmp eq %"class.std::_Facet_base"* %15, null
  br i1 %16, label %23, label %17

17:                                               ; preds = %7
  %18 = bitcast %"class.std::_Facet_base"* %15 to i8* (%"class.std::_Facet_base"*, i32)***
  %19 = load i8* (%"class.std::_Facet_base"*, i32)**, i8* (%"class.std::_Facet_base"*, i32)*** %18, align 8
  %20 = getelementptr inbounds i8* (%"class.std::_Facet_base"*, i32)*, i8* (%"class.std::_Facet_base"*, i32)** %19, i64 0
  %21 = load i8* (%"class.std::_Facet_base"*, i32)*, i8* (%"class.std::_Facet_base"*, i32)** %20, align 8
  %22 = call i8* %21(%"class.std::_Facet_base"* %15, i32 1) #5
  br label %23

23:                                               ; preds = %17, %7
  br label %24

24:                                               ; preds = %23, %1
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@D_K@Z"(%"class.std::num_put"* %0, %"class.std::ostreambuf_iterator"* noalias sret align 8 %1, %"class.std::ostreambuf_iterator"* %2, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %3, i8 %4, i64 %5) #1 comdat align 2 {
  %7 = alloca i8*, align 8
  %8 = alloca i64, align 8
  %9 = alloca i8, align 1
  %10 = alloca %"class.std::ios_base"*, align 8
  %11 = alloca %"class.std::num_put"*, align 8
  %12 = alloca %"class.std::ostreambuf_iterator", align 8
  %13 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  store i8* %13, i8** %7, align 8
  store i64 %5, i64* %8, align 8
  store i8 %4, i8* %9, align 1
  store %"class.std::ios_base"* %3, %"class.std::ios_base"** %10, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %11, align 8
  %14 = load %"class.std::num_put"*, %"class.std::num_put"** %11, align 8
  %15 = load i64, i64* %8, align 8
  %16 = load i8, i8* %9, align 1
  %17 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %18 = bitcast %"class.std::ostreambuf_iterator"* %12 to i8*
  %19 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %18, i8* align 8 %19, i64 16, i1 false)
  %20 = bitcast %"class.std::num_put"* %14 to void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i64)***
  %21 = load void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i64)**, void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i64)*** %20, align 8
  %22 = getelementptr inbounds void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i64)*, void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i64)** %21, i64 6
  %23 = load void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i64)*, void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i64)** %22, align 8
  call void %23(%"class.std::num_put"* %14, %"class.std::ostreambuf_iterator"* sret align 8 %1, %"class.std::ostreambuf_iterator"* %12, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %17, i8 %16, i64 %15)
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::ostreambuf_iterator"* @"??0?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@@Z"(%"class.std::ostreambuf_iterator"* returned %0, %"class.std::basic_streambuf"* %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca %"class.std::basic_streambuf"*, align 8
  %4 = alloca %"class.std::ostreambuf_iterator"*, align 8
  store %"class.std::basic_streambuf"* %1, %"class.std::basic_streambuf"** %3, align 8
  store %"class.std::ostreambuf_iterator"* %0, %"class.std::ostreambuf_iterator"** %4, align 8
  %5 = load %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"** %4, align 8
  %6 = getelementptr inbounds %"class.std::ostreambuf_iterator", %"class.std::ostreambuf_iterator"* %5, i32 0, i32 0
  store i8 0, i8* %6, align 8
  %7 = getelementptr inbounds %"class.std::ostreambuf_iterator", %"class.std::ostreambuf_iterator"* %5, i32 0, i32 1
  %8 = load %"class.std::basic_streambuf"*, %"class.std::basic_streambuf"** %3, align 8
  store %"class.std::basic_streambuf"* %8, %"class.std::basic_streambuf"** %7, align 8
  ret %"class.std::ostreambuf_iterator"* %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local zeroext i1 @"?failed@?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEBA_NXZ"(%"class.std::ostreambuf_iterator"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::ostreambuf_iterator"*, align 8
  store %"class.std::ostreambuf_iterator"* %0, %"class.std::ostreambuf_iterator"** %2, align 8
  %3 = load %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"** %2, align 8
  %4 = getelementptr inbounds %"class.std::ostreambuf_iterator", %"class.std::ostreambuf_iterator"* %3, i32 0, i32 0
  %5 = load i8, i8* %4, align 8
  %6 = trunc i8 %5 to i1
  ret i1 %6
}

; Function Attrs: nounwind
declare dso_local %"class.std::_Lockit"* @"??0_Lockit@std@@QEAA@H@Z"(%"class.std::_Lockit"* returned, i32) unnamed_addr #12

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i64 @"??Bid@locale@std@@QEAA_KXZ"(%"class.std::locale::id"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::locale::id"*, align 8
  %3 = alloca %"class.std::_Lockit", align 4
  store %"class.std::locale::id"* %0, %"class.std::locale::id"** %2, align 8
  %4 = load %"class.std::locale::id"*, %"class.std::locale::id"** %2, align 8
  %5 = getelementptr inbounds %"class.std::locale::id", %"class.std::locale::id"* %4, i32 0, i32 0
  %6 = load i64, i64* %5, align 8
  %7 = icmp eq i64 %6, 0
  br i1 %7, label %8, label %19

8:                                                ; preds = %1
  %9 = call %"class.std::_Lockit"* @"??0_Lockit@std@@QEAA@H@Z"(%"class.std::_Lockit"* %3, i32 0) #5
  %10 = getelementptr inbounds %"class.std::locale::id", %"class.std::locale::id"* %4, i32 0, i32 0
  %11 = load i64, i64* %10, align 8
  %12 = icmp eq i64 %11, 0
  br i1 %12, label %13, label %18

13:                                               ; preds = %8
  %14 = load i32, i32* @"?_Id_cnt@id@locale@std@@0HA", align 4
  %15 = add nsw i32 %14, 1
  store i32 %15, i32* @"?_Id_cnt@id@locale@std@@0HA", align 4
  %16 = sext i32 %15 to i64
  %17 = getelementptr inbounds %"class.std::locale::id", %"class.std::locale::id"* %4, i32 0, i32 0
  store i64 %16, i64* %17, align 8
  br label %18

18:                                               ; preds = %13, %8
  call void @"??1_Lockit@std@@QEAA@XZ"(%"class.std::_Lockit"* %3) #5
  br label %19

19:                                               ; preds = %18, %1
  %20 = getelementptr inbounds %"class.std::locale::id", %"class.std::locale::id"* %4, i32 0, i32 0
  %21 = load i64, i64* %20, align 8
  ret i64 %21
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local %"class.std::locale::facet"* @"?_Getfacet@locale@std@@QEBAPEBVfacet@12@_K@Z"(%"class.std::locale"* %0, i64 %1) #1 comdat align 2 {
  %3 = alloca %"class.std::locale::facet"*, align 8
  %4 = alloca i64, align 8
  %5 = alloca %"class.std::locale"*, align 8
  %6 = alloca %"class.std::locale::facet"*, align 8
  %7 = alloca %"class.std::locale::_Locimp"*, align 8
  store i64 %1, i64* %4, align 8
  store %"class.std::locale"* %0, %"class.std::locale"** %5, align 8
  %8 = load %"class.std::locale"*, %"class.std::locale"** %5, align 8
  %9 = load i64, i64* %4, align 8
  %10 = getelementptr inbounds %"class.std::locale", %"class.std::locale"* %8, i32 0, i32 1
  %11 = load %"class.std::locale::_Locimp"*, %"class.std::locale::_Locimp"** %10, align 8
  %12 = getelementptr inbounds %"class.std::locale::_Locimp", %"class.std::locale::_Locimp"* %11, i32 0, i32 2
  %13 = load i64, i64* %12, align 8
  %14 = icmp ult i64 %9, %13
  br i1 %14, label %15, label %23

15:                                               ; preds = %2
  %16 = getelementptr inbounds %"class.std::locale", %"class.std::locale"* %8, i32 0, i32 1
  %17 = load %"class.std::locale::_Locimp"*, %"class.std::locale::_Locimp"** %16, align 8
  %18 = getelementptr inbounds %"class.std::locale::_Locimp", %"class.std::locale::_Locimp"* %17, i32 0, i32 1
  %19 = load %"class.std::locale::facet"**, %"class.std::locale::facet"*** %18, align 8
  %20 = load i64, i64* %4, align 8
  %21 = getelementptr inbounds %"class.std::locale::facet"*, %"class.std::locale::facet"** %19, i64 %20
  %22 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %21, align 8
  br label %24

23:                                               ; preds = %2
  br label %24

24:                                               ; preds = %23, %15
  %25 = phi %"class.std::locale::facet"* [ %22, %15 ], [ null, %23 ]
  store %"class.std::locale::facet"* %25, %"class.std::locale::facet"** %6, align 8
  %26 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %6, align 8
  %27 = icmp ne %"class.std::locale::facet"* %26, null
  br i1 %27, label %34, label %28

28:                                               ; preds = %24
  %29 = getelementptr inbounds %"class.std::locale", %"class.std::locale"* %8, i32 0, i32 1
  %30 = load %"class.std::locale::_Locimp"*, %"class.std::locale::_Locimp"** %29, align 8
  %31 = getelementptr inbounds %"class.std::locale::_Locimp", %"class.std::locale::_Locimp"* %30, i32 0, i32 4
  %32 = load i8, i8* %31, align 4
  %33 = trunc i8 %32 to i1
  br i1 %33, label %36, label %34

34:                                               ; preds = %28, %24
  %35 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %6, align 8
  store %"class.std::locale::facet"* %35, %"class.std::locale::facet"** %3, align 8
  br label %51

36:                                               ; preds = %28
  %37 = call %"class.std::locale::_Locimp"* @"?_Getgloballocale@locale@std@@CAPEAV_Locimp@12@XZ"()
  store %"class.std::locale::_Locimp"* %37, %"class.std::locale::_Locimp"** %7, align 8
  %38 = load i64, i64* %4, align 8
  %39 = load %"class.std::locale::_Locimp"*, %"class.std::locale::_Locimp"** %7, align 8
  %40 = getelementptr inbounds %"class.std::locale::_Locimp", %"class.std::locale::_Locimp"* %39, i32 0, i32 2
  %41 = load i64, i64* %40, align 8
  %42 = icmp ult i64 %38, %41
  br i1 %42, label %43, label %50

43:                                               ; preds = %36
  %44 = load %"class.std::locale::_Locimp"*, %"class.std::locale::_Locimp"** %7, align 8
  %45 = getelementptr inbounds %"class.std::locale::_Locimp", %"class.std::locale::_Locimp"* %44, i32 0, i32 1
  %46 = load %"class.std::locale::facet"**, %"class.std::locale::facet"*** %45, align 8
  %47 = load i64, i64* %4, align 8
  %48 = getelementptr inbounds %"class.std::locale::facet"*, %"class.std::locale::facet"** %46, i64 %47
  %49 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %48, align 8
  store %"class.std::locale::facet"* %49, %"class.std::locale::facet"** %3, align 8
  br label %51

50:                                               ; preds = %36
  store %"class.std::locale::facet"* null, %"class.std::locale::facet"** %3, align 8
  br label %51

51:                                               ; preds = %50, %43, %34
  %52 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %3, align 8
  ret %"class.std::locale::facet"* %52
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i64 @"?_Getcat@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z"(%"class.std::locale::facet"** %0, %"class.std::locale"* %1) #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %3 = alloca %"class.std::locale"*, align 8
  %4 = alloca %"class.std::locale::facet"**, align 8
  %5 = alloca %"class.std::_Locinfo", align 8
  %6 = alloca i1, align 1
  store %"class.std::locale"* %1, %"class.std::locale"** %3, align 8
  store %"class.std::locale::facet"** %0, %"class.std::locale::facet"*** %4, align 8
  %7 = load %"class.std::locale::facet"**, %"class.std::locale::facet"*** %4, align 8
  %8 = icmp ne %"class.std::locale::facet"** %7, null
  br i1 %8, label %9, label %32

9:                                                ; preds = %2
  %10 = load %"class.std::locale::facet"**, %"class.std::locale::facet"*** %4, align 8
  %11 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %10, align 8
  %12 = icmp ne %"class.std::locale::facet"* %11, null
  br i1 %12, label %32, label %13

13:                                               ; preds = %9
  %14 = call noalias nonnull i8* @"??2@YAPEAX_K@Z"(i64 16) #22
  store i1 true, i1* %6, align 1
  %15 = bitcast i8* %14 to %"class.std::num_put"*
  %16 = load %"class.std::locale"*, %"class.std::locale"** %3, align 8
  %17 = invoke i8* @"?c_str@locale@std@@QEBAPEBDXZ"(%"class.std::locale"* %16)
          to label %18 unwind label %27

18:                                               ; preds = %13
  %19 = invoke %"class.std::_Locinfo"* @"??0_Locinfo@std@@QEAA@PEBD@Z"(%"class.std::_Locinfo"* %5, i8* %17)
          to label %20 unwind label %27

20:                                               ; preds = %18
  %21 = invoke %"class.std::num_put"* @"??0?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z"(%"class.std::num_put"* %15, %"class.std::_Locinfo"* nonnull align 8 dereferenceable(104) %5, i64 0)
          to label %22 unwind label %25

22:                                               ; preds = %20
  store i1 false, i1* %6, align 1
  %23 = bitcast %"class.std::num_put"* %15 to %"class.std::locale::facet"*
  %24 = load %"class.std::locale::facet"**, %"class.std::locale::facet"*** %4, align 8
  store %"class.std::locale::facet"* %23, %"class.std::locale::facet"** %24, align 8
  call void @"??1_Locinfo@std@@QEAA@XZ"(%"class.std::_Locinfo"* %5) #5
  br label %32

25:                                               ; preds = %20
  %26 = cleanuppad within none []
  call void @"??1_Locinfo@std@@QEAA@XZ"(%"class.std::_Locinfo"* %5) #5 [ "funclet"(token %26) ]
  cleanupret from %26 unwind label %27

27:                                               ; preds = %25, %18, %13
  %28 = cleanuppad within none []
  %29 = load i1, i1* %6, align 1
  br i1 %29, label %30, label %31

30:                                               ; preds = %27
  call void @"??3@YAXPEAX@Z"(i8* %14) #20 [ "funclet"(token %28) ]
  br label %31

31:                                               ; preds = %30, %27
  cleanupret from %28 unwind to caller

32:                                               ; preds = %22, %9, %2
  ret i64 4
}

; Function Attrs: noinline noreturn optnone uwtable
define linkonce_odr dso_local void @"?_Throw_bad_cast@std@@YAXXZ"() #8 comdat {
  %1 = alloca %"class.std::bad_cast", align 8
  %2 = call %"class.std::bad_cast"* @"??0bad_cast@std@@QEAA@XZ"(%"class.std::bad_cast"* %1) #5
  %3 = bitcast %"class.std::bad_cast"* %1 to i8*
  call void @_CxxThrowException(i8* %3, %eh.ThrowInfo* @"_TI2?AVbad_cast@std@@") #19
  unreachable
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::unique_ptr"* @"??$?0U?$default_delete@V_Facet_base@std@@@std@@$0A@@?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAA@PEAV_Facet_base@1@@Z"(%"class.std::unique_ptr"* returned %0, %"class.std::_Facet_base"* %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca %"class.std::_Facet_base"*, align 8
  %4 = alloca %"class.std::unique_ptr"*, align 8
  %5 = alloca %"struct.std::_Zero_then_variadic_args_t", align 1
  store %"class.std::_Facet_base"* %1, %"class.std::_Facet_base"** %3, align 8
  store %"class.std::unique_ptr"* %0, %"class.std::unique_ptr"** %4, align 8
  %6 = load %"class.std::unique_ptr"*, %"class.std::unique_ptr"** %4, align 8
  %7 = getelementptr inbounds %"class.std::unique_ptr", %"class.std::unique_ptr"* %6, i32 0, i32 0
  %8 = getelementptr inbounds %"struct.std::_Zero_then_variadic_args_t", %"struct.std::_Zero_then_variadic_args_t"* %5, i32 0, i32 0
  %9 = load i8, i8* %8, align 1
  %10 = call %"class.std::_Compressed_pair.2"* @"??$?0AEAPEAV_Facet_base@std@@@?$_Compressed_pair@U?$default_delete@V_Facet_base@std@@@std@@PEAV_Facet_base@2@$00@std@@QEAA@U_Zero_then_variadic_args_t@1@AEAPEAV_Facet_base@1@@Z"(%"class.std::_Compressed_pair.2"* %7, i8 %9, %"class.std::_Facet_base"** nonnull align 8 dereferenceable(8) %3) #5
  ret %"class.std::unique_ptr"* %6
}

declare dso_local void @"?_Facet_Register@std@@YAXPEAV_Facet_base@1@@Z"(%"class.std::_Facet_base"*) #4

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::_Facet_base"* @"?release@?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAAPEAV_Facet_base@2@XZ"(%"class.std::unique_ptr"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::unique_ptr"*, align 8
  %3 = alloca i8*, align 8
  store %"class.std::unique_ptr"* %0, %"class.std::unique_ptr"** %2, align 8
  %4 = load %"class.std::unique_ptr"*, %"class.std::unique_ptr"** %2, align 8
  store i8* null, i8** %3, align 8
  %5 = getelementptr inbounds %"class.std::unique_ptr", %"class.std::unique_ptr"* %4, i32 0, i32 0
  %6 = getelementptr inbounds %"class.std::_Compressed_pair.2", %"class.std::_Compressed_pair.2"* %5, i32 0, i32 0
  %7 = call %"class.std::_Facet_base"* @"??$exchange@PEAV_Facet_base@std@@$$T@std@@YAPEAV_Facet_base@0@AEAPEAV10@$$QEA$$T@Z"(%"class.std::_Facet_base"** nonnull align 8 dereferenceable(8) %6, i8** nonnull align 8 dereferenceable(8) %3) #5
  ret %"class.std::_Facet_base"* %7
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAA@XZ"(%"class.std::unique_ptr"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::unique_ptr"*, align 8
  store %"class.std::unique_ptr"* %0, %"class.std::unique_ptr"** %2, align 8
  %3 = load %"class.std::unique_ptr"*, %"class.std::unique_ptr"** %2, align 8
  %4 = getelementptr inbounds %"class.std::unique_ptr", %"class.std::unique_ptr"* %3, i32 0, i32 0
  %5 = getelementptr inbounds %"class.std::_Compressed_pair.2", %"class.std::_Compressed_pair.2"* %4, i32 0, i32 0
  %6 = load %"class.std::_Facet_base"*, %"class.std::_Facet_base"** %5, align 8
  %7 = icmp ne %"class.std::_Facet_base"* %6, null
  br i1 %7, label %8, label %14

8:                                                ; preds = %1
  %9 = getelementptr inbounds %"class.std::unique_ptr", %"class.std::unique_ptr"* %3, i32 0, i32 0
  %10 = call nonnull align 1 dereferenceable(1) %"struct.std::default_delete"* @"?_Get_first@?$_Compressed_pair@U?$default_delete@V_Facet_base@std@@@std@@PEAV_Facet_base@2@$00@std@@QEAAAEAU?$default_delete@V_Facet_base@std@@@2@XZ"(%"class.std::_Compressed_pair.2"* %9) #5
  %11 = getelementptr inbounds %"class.std::unique_ptr", %"class.std::unique_ptr"* %3, i32 0, i32 0
  %12 = getelementptr inbounds %"class.std::_Compressed_pair.2", %"class.std::_Compressed_pair.2"* %11, i32 0, i32 0
  %13 = load %"class.std::_Facet_base"*, %"class.std::_Facet_base"** %12, align 8
  call void @"??R?$default_delete@V_Facet_base@std@@@std@@QEBAXPEAV_Facet_base@1@@Z"(%"struct.std::default_delete"* %10, %"class.std::_Facet_base"* %13) #5
  br label %14

14:                                               ; preds = %8, %1
  ret void
}

; Function Attrs: nounwind
declare dso_local void @"??1_Lockit@std@@QEAA@XZ"(%"class.std::_Lockit"*) unnamed_addr #12

declare dso_local %"class.std::locale::_Locimp"* @"?_Getgloballocale@locale@std@@CAPEAV_Locimp@12@XZ"() #4

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"?c_str@locale@std@@QEBAPEBDXZ"(%"class.std::locale"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::locale"*, align 8
  store %"class.std::locale"* %0, %"class.std::locale"** %2, align 8
  %3 = load %"class.std::locale"*, %"class.std::locale"** %2, align 8
  %4 = getelementptr inbounds %"class.std::locale", %"class.std::locale"* %3, i32 0, i32 1
  %5 = load %"class.std::locale::_Locimp"*, %"class.std::locale::_Locimp"** %4, align 8
  %6 = icmp ne %"class.std::locale::_Locimp"* %5, null
  br i1 %6, label %7, label %12

7:                                                ; preds = %1
  %8 = getelementptr inbounds %"class.std::locale", %"class.std::locale"* %3, i32 0, i32 1
  %9 = load %"class.std::locale::_Locimp"*, %"class.std::locale::_Locimp"** %8, align 8
  %10 = getelementptr inbounds %"class.std::locale::_Locimp", %"class.std::locale::_Locimp"* %9, i32 0, i32 5
  %11 = call i8* @"?c_str@?$_Yarn@D@std@@QEBAPEBDXZ"(%"class.std::_Yarn"* %10) #5
  br label %13

12:                                               ; preds = %1
  br label %13

13:                                               ; preds = %12, %7
  %14 = phi i8* [ %11, %7 ], [ getelementptr inbounds ([1 x i8], [1 x i8]* @"??_C@_00CNPNBAHC@?$AA@", i64 0, i64 0), %12 ]
  ret i8* %14
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local %"class.std::_Locinfo"* @"??0_Locinfo@std@@QEAA@PEBD@Z"(%"class.std::_Locinfo"* returned %0, i8* %1) unnamed_addr #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::_Locinfo"*, align 8
  store i8* %1, i8** %3, align 8
  store %"class.std::_Locinfo"* %0, %"class.std::_Locinfo"** %4, align 8
  %5 = load %"class.std::_Locinfo"*, %"class.std::_Locinfo"** %4, align 8
  %6 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %5, i32 0, i32 0
  %7 = call %"class.std::_Lockit"* @"??0_Lockit@std@@QEAA@H@Z"(%"class.std::_Lockit"* %6, i32 0) #5
  %8 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %5, i32 0, i32 1
  %9 = call %"class.std::_Yarn"* @"??0?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %8) #5
  %10 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %5, i32 0, i32 2
  %11 = call %"class.std::_Yarn"* @"??0?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %10) #5
  %12 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %5, i32 0, i32 3
  %13 = call %"class.std::_Yarn.3"* @"??0?$_Yarn@_W@std@@QEAA@XZ"(%"class.std::_Yarn.3"* %12) #5
  %14 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %5, i32 0, i32 4
  %15 = call %"class.std::_Yarn.3"* @"??0?$_Yarn@_W@std@@QEAA@XZ"(%"class.std::_Yarn.3"* %14) #5
  %16 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %5, i32 0, i32 5
  %17 = call %"class.std::_Yarn"* @"??0?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %16) #5
  %18 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %5, i32 0, i32 6
  %19 = call %"class.std::_Yarn"* @"??0?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %18) #5
  %20 = load i8*, i8** %3, align 8
  %21 = icmp ne i8* %20, null
  br i1 %21, label %22, label %25

22:                                               ; preds = %2
  %23 = load i8*, i8** %3, align 8
  invoke void @"?_Locinfo_ctor@_Locinfo@std@@SAXPEAV12@PEBD@Z"(%"class.std::_Locinfo"* %5, i8* %23)
          to label %24 unwind label %27

24:                                               ; preds = %22
  ret %"class.std::_Locinfo"* %5

25:                                               ; preds = %2
  invoke void @"?_Xruntime_error@std@@YAXPEBD@Z"(i8* getelementptr inbounds ([16 x i8], [16 x i8]* @"??_C@_0BA@ELKIONDK@bad?5locale?5name?$AA@", i64 0, i64 0)) #19
          to label %26 unwind label %27

26:                                               ; preds = %25
  unreachable

27:                                               ; preds = %25, %22
  %28 = cleanuppad within none []
  call void @"??1?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %18) #5 [ "funclet"(token %28) ]
  cleanupret from %28 unwind label %29

29:                                               ; preds = %27
  %30 = cleanuppad within none []
  call void @"??1?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %16) #5 [ "funclet"(token %30) ]
  cleanupret from %30 unwind label %31

31:                                               ; preds = %29
  %32 = cleanuppad within none []
  call void @"??1?$_Yarn@_W@std@@QEAA@XZ"(%"class.std::_Yarn.3"* %14) #5 [ "funclet"(token %32) ]
  cleanupret from %32 unwind label %33

33:                                               ; preds = %31
  %34 = cleanuppad within none []
  call void @"??1?$_Yarn@_W@std@@QEAA@XZ"(%"class.std::_Yarn.3"* %12) #5 [ "funclet"(token %34) ]
  cleanupret from %34 unwind label %35

35:                                               ; preds = %33
  %36 = cleanuppad within none []
  call void @"??1?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %10) #5 [ "funclet"(token %36) ]
  cleanupret from %36 unwind label %37

37:                                               ; preds = %35
  %38 = cleanuppad within none []
  call void @"??1?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %8) #5 [ "funclet"(token %38) ]
  cleanupret from %38 unwind label %39

39:                                               ; preds = %37
  %40 = cleanuppad within none []
  call void @"??1_Lockit@std@@QEAA@XZ"(%"class.std::_Lockit"* %6) #5 [ "funclet"(token %40) ]
  cleanupret from %40 unwind to caller
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local %"class.std::num_put"* @"??0?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z"(%"class.std::num_put"* returned %0, %"class.std::_Locinfo"* nonnull align 8 dereferenceable(104) %1, i64 %2) unnamed_addr #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %4 = alloca i64, align 8
  %5 = alloca %"class.std::_Locinfo"*, align 8
  %6 = alloca %"class.std::num_put"*, align 8
  store i64 %2, i64* %4, align 8
  store %"class.std::_Locinfo"* %1, %"class.std::_Locinfo"** %5, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %6, align 8
  %7 = load %"class.std::num_put"*, %"class.std::num_put"** %6, align 8
  %8 = bitcast %"class.std::num_put"* %7 to %"class.std::locale::facet"*
  %9 = load i64, i64* %4, align 8
  %10 = call %"class.std::locale::facet"* @"??0facet@locale@std@@IEAA@_K@Z"(%"class.std::locale::facet"* %8, i64 %9)
  %11 = bitcast %"class.std::num_put"* %7 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@6B@" to i32 (...)**), i32 (...)*** %11, align 8
  %12 = load %"class.std::_Locinfo"*, %"class.std::_Locinfo"** %5, align 8
  invoke void @"?_Init@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@IEAAXAEBV_Locinfo@2@@Z"(%"class.std::num_put"* %7, %"class.std::_Locinfo"* nonnull align 8 dereferenceable(104) %12)
          to label %13 unwind label %14

13:                                               ; preds = %3
  ret %"class.std::num_put"* %7

14:                                               ; preds = %3
  %15 = cleanuppad within none []
  %16 = bitcast %"class.std::num_put"* %7 to %"class.std::locale::facet"*
  call void @"??1facet@locale@std@@MEAA@XZ"(%"class.std::locale::facet"* %16) #5 [ "funclet"(token %15) ]
  cleanupret from %15 unwind to caller
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1_Locinfo@std@@QEAA@XZ"(%"class.std::_Locinfo"* %0) unnamed_addr #3 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %2 = alloca %"class.std::_Locinfo"*, align 8
  store %"class.std::_Locinfo"* %0, %"class.std::_Locinfo"** %2, align 8
  %3 = load %"class.std::_Locinfo"*, %"class.std::_Locinfo"** %2, align 8
  invoke void @"?_Locinfo_dtor@_Locinfo@std@@SAXPEAV12@@Z"(%"class.std::_Locinfo"* %3)
          to label %4 unwind label %12

4:                                                ; preds = %1
  %5 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %3, i32 0, i32 6
  call void @"??1?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %5) #5
  %6 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %3, i32 0, i32 5
  call void @"??1?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %6) #5
  %7 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %3, i32 0, i32 4
  call void @"??1?$_Yarn@_W@std@@QEAA@XZ"(%"class.std::_Yarn.3"* %7) #5
  %8 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %3, i32 0, i32 3
  call void @"??1?$_Yarn@_W@std@@QEAA@XZ"(%"class.std::_Yarn.3"* %8) #5
  %9 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %3, i32 0, i32 2
  call void @"??1?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %9) #5
  %10 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %3, i32 0, i32 1
  call void @"??1?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %10) #5
  %11 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %3, i32 0, i32 0
  call void @"??1_Lockit@std@@QEAA@XZ"(%"class.std::_Lockit"* %11) #5
  ret void

12:                                               ; preds = %1
  %13 = cleanuppad within none []
  %14 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %3, i32 0, i32 6
  call void @"??1?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %14) #5 [ "funclet"(token %13) ]
  cleanupret from %13 unwind label %15

15:                                               ; preds = %12
  %16 = cleanuppad within none []
  %17 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %3, i32 0, i32 5
  call void @"??1?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %17) #5 [ "funclet"(token %16) ]
  cleanupret from %16 unwind label %18

18:                                               ; preds = %15
  %19 = cleanuppad within none []
  %20 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %3, i32 0, i32 4
  call void @"??1?$_Yarn@_W@std@@QEAA@XZ"(%"class.std::_Yarn.3"* %20) #5 [ "funclet"(token %19) ]
  cleanupret from %19 unwind label %21

21:                                               ; preds = %18
  %22 = cleanuppad within none []
  %23 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %3, i32 0, i32 3
  call void @"??1?$_Yarn@_W@std@@QEAA@XZ"(%"class.std::_Yarn.3"* %23) #5 [ "funclet"(token %22) ]
  cleanupret from %22 unwind label %24

24:                                               ; preds = %21
  %25 = cleanuppad within none []
  %26 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %3, i32 0, i32 2
  call void @"??1?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %26) #5 [ "funclet"(token %25) ]
  cleanupret from %25 unwind label %27

27:                                               ; preds = %24
  %28 = cleanuppad within none []
  %29 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %3, i32 0, i32 1
  call void @"??1?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %29) #5 [ "funclet"(token %28) ]
  cleanupret from %28 unwind label %30

30:                                               ; preds = %27
  %31 = cleanuppad within none []
  %32 = getelementptr inbounds %"class.std::_Locinfo", %"class.std::_Locinfo"* %3, i32 0, i32 0
  call void @"??1_Lockit@std@@QEAA@XZ"(%"class.std::_Lockit"* %32) #5 [ "funclet"(token %31) ]
  cleanupret from %31 unwind label %33

33:                                               ; preds = %30
  %34 = cleanuppad within none []
  call void @__std_terminate() #18 [ "funclet"(token %34) ]
  unreachable
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"?c_str@?$_Yarn@D@std@@QEBAPEBDXZ"(%"class.std::_Yarn"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::_Yarn"*, align 8
  store %"class.std::_Yarn"* %0, %"class.std::_Yarn"** %2, align 8
  %3 = load %"class.std::_Yarn"*, %"class.std::_Yarn"** %2, align 8
  %4 = getelementptr inbounds %"class.std::_Yarn", %"class.std::_Yarn"* %3, i32 0, i32 0
  %5 = load i8*, i8** %4, align 8
  %6 = icmp ne i8* %5, null
  br i1 %6, label %7, label %10

7:                                                ; preds = %1
  %8 = getelementptr inbounds %"class.std::_Yarn", %"class.std::_Yarn"* %3, i32 0, i32 0
  %9 = load i8*, i8** %8, align 8
  br label %12

10:                                               ; preds = %1
  %11 = getelementptr inbounds %"class.std::_Yarn", %"class.std::_Yarn"* %3, i32 0, i32 1
  br label %12

12:                                               ; preds = %10, %7
  %13 = phi i8* [ %9, %7 ], [ %11, %10 ]
  ret i8* %13
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::_Yarn"* @"??0?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* returned %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::_Yarn"*, align 8
  store %"class.std::_Yarn"* %0, %"class.std::_Yarn"** %2, align 8
  %3 = load %"class.std::_Yarn"*, %"class.std::_Yarn"** %2, align 8
  %4 = getelementptr inbounds %"class.std::_Yarn", %"class.std::_Yarn"* %3, i32 0, i32 0
  store i8* null, i8** %4, align 8
  %5 = getelementptr inbounds %"class.std::_Yarn", %"class.std::_Yarn"* %3, i32 0, i32 1
  store i8 0, i8* %5, align 8
  ret %"class.std::_Yarn"* %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::_Yarn.3"* @"??0?$_Yarn@_W@std@@QEAA@XZ"(%"class.std::_Yarn.3"* returned %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::_Yarn.3"*, align 8
  store %"class.std::_Yarn.3"* %0, %"class.std::_Yarn.3"** %2, align 8
  %3 = load %"class.std::_Yarn.3"*, %"class.std::_Yarn.3"** %2, align 8
  %4 = getelementptr inbounds %"class.std::_Yarn.3", %"class.std::_Yarn.3"* %3, i32 0, i32 0
  store i16* null, i16** %4, align 8
  %5 = getelementptr inbounds %"class.std::_Yarn.3", %"class.std::_Yarn.3"* %3, i32 0, i32 1
  store i16 0, i16* %5, align 8
  ret %"class.std::_Yarn.3"* %3
}

declare dso_local void @"?_Locinfo_ctor@_Locinfo@std@@SAXPEAV12@PEBD@Z"(%"class.std::_Locinfo"*, i8*) #4

; Function Attrs: noreturn
declare dso_local void @"?_Xruntime_error@std@@YAXPEBD@Z"(i8*) #9

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1?$_Yarn@D@std@@QEAA@XZ"(%"class.std::_Yarn"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::_Yarn"*, align 8
  store %"class.std::_Yarn"* %0, %"class.std::_Yarn"** %2, align 8
  %3 = load %"class.std::_Yarn"*, %"class.std::_Yarn"** %2, align 8
  call void @"?_Tidy@?$_Yarn@D@std@@AEAAXXZ"(%"class.std::_Yarn"* %3) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1?$_Yarn@_W@std@@QEAA@XZ"(%"class.std::_Yarn.3"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::_Yarn.3"*, align 8
  store %"class.std::_Yarn.3"* %0, %"class.std::_Yarn.3"** %2, align 8
  %3 = load %"class.std::_Yarn.3"*, %"class.std::_Yarn.3"** %2, align 8
  call void @"?_Tidy@?$_Yarn@_W@std@@AEAAXXZ"(%"class.std::_Yarn.3"* %3) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?_Tidy@?$_Yarn@D@std@@AEAAXXZ"(%"class.std::_Yarn"* %0) #3 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %2 = alloca %"class.std::_Yarn"*, align 8
  store %"class.std::_Yarn"* %0, %"class.std::_Yarn"** %2, align 8
  %3 = load %"class.std::_Yarn"*, %"class.std::_Yarn"** %2, align 8
  %4 = getelementptr inbounds %"class.std::_Yarn", %"class.std::_Yarn"* %3, i32 0, i32 0
  %5 = load i8*, i8** %4, align 8
  %6 = icmp ne i8* %5, null
  br i1 %6, label %7, label %11

7:                                                ; preds = %1
  %8 = getelementptr inbounds %"class.std::_Yarn", %"class.std::_Yarn"* %3, i32 0, i32 0
  %9 = load i8*, i8** %8, align 8
  invoke void @free(i8* %9)
          to label %10 unwind label %13

10:                                               ; preds = %7
  br label %11

11:                                               ; preds = %10, %1
  %12 = getelementptr inbounds %"class.std::_Yarn", %"class.std::_Yarn"* %3, i32 0, i32 0
  store i8* null, i8** %12, align 8
  ret void

13:                                               ; preds = %7
  %14 = cleanuppad within none []
  call void @__std_terminate() #18 [ "funclet"(token %14) ]
  unreachable
}

declare dso_local void @free(i8*) #4

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?_Tidy@?$_Yarn@_W@std@@AEAAXXZ"(%"class.std::_Yarn.3"* %0) #3 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %2 = alloca %"class.std::_Yarn.3"*, align 8
  store %"class.std::_Yarn.3"* %0, %"class.std::_Yarn.3"** %2, align 8
  %3 = load %"class.std::_Yarn.3"*, %"class.std::_Yarn.3"** %2, align 8
  %4 = getelementptr inbounds %"class.std::_Yarn.3", %"class.std::_Yarn.3"* %3, i32 0, i32 0
  %5 = load i16*, i16** %4, align 8
  %6 = icmp ne i16* %5, null
  br i1 %6, label %7, label %12

7:                                                ; preds = %1
  %8 = getelementptr inbounds %"class.std::_Yarn.3", %"class.std::_Yarn.3"* %3, i32 0, i32 0
  %9 = load i16*, i16** %8, align 8
  %10 = bitcast i16* %9 to i8*
  invoke void @free(i8* %10)
          to label %11 unwind label %14

11:                                               ; preds = %7
  br label %12

12:                                               ; preds = %11, %1
  %13 = getelementptr inbounds %"class.std::_Yarn.3", %"class.std::_Yarn.3"* %3, i32 0, i32 0
  store i16* null, i16** %13, align 8
  ret void

14:                                               ; preds = %7
  %15 = cleanuppad within none []
  call void @__std_terminate() #18 [ "funclet"(token %15) ]
  unreachable
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::locale::facet"* @"??0facet@locale@std@@IEAA@_K@Z"(%"class.std::locale::facet"* returned %0, i64 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i64, align 8
  %4 = alloca %"class.std::locale::facet"*, align 8
  store i64 %1, i64* %3, align 8
  store %"class.std::locale::facet"* %0, %"class.std::locale::facet"** %4, align 8
  %5 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %4, align 8
  %6 = bitcast %"class.std::locale::facet"* %5 to %"class.std::_Facet_base"*
  %7 = call %"class.std::_Facet_base"* @"??0_Facet_base@std@@QEAA@XZ"(%"class.std::_Facet_base"* %6) #5
  %8 = bitcast %"class.std::locale::facet"* %5 to i8*
  %9 = getelementptr inbounds i8, i8* %8, i64 8
  %10 = bitcast i8* %9 to %"struct.std::_Crt_new_delete"*
  %11 = bitcast %"class.std::locale::facet"* %5 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7facet@locale@std@@6B@" to i32 (...)**), i32 (...)*** %11, align 8
  %12 = getelementptr inbounds %"class.std::locale::facet", %"class.std::locale::facet"* %5, i32 0, i32 1
  %13 = load i64, i64* %3, align 8
  %14 = trunc i64 %13 to i32
  store i32 %14, i32* %12, align 8
  ret %"class.std::locale::facet"* %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?_Init@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@IEAAXAEBV_Locinfo@2@@Z"(%"class.std::num_put"* %0, %"class.std::_Locinfo"* nonnull align 8 dereferenceable(104) %1) #3 comdat align 2 {
  %3 = alloca %"class.std::_Locinfo"*, align 8
  %4 = alloca %"class.std::num_put"*, align 8
  store %"class.std::_Locinfo"* %1, %"class.std::_Locinfo"** %3, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %4, align 8
  %5 = load %"class.std::num_put"*, %"class.std::num_put"** %4, align 8
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1facet@locale@std@@MEAA@XZ"(%"class.std::locale::facet"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::locale::facet"*, align 8
  store %"class.std::locale::facet"* %0, %"class.std::locale::facet"** %2, align 8
  %3 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %2, align 8
  %4 = bitcast %"class.std::locale::facet"* %3 to %"class.std::_Facet_base"*
  call void @"??1_Facet_base@std@@UEAA@XZ"(%"class.std::_Facet_base"* %4) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??_G?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEAAPEAXI@Z"(%"class.std::num_put"* %0, i32 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca i32, align 4
  %5 = alloca %"class.std::num_put"*, align 8
  store i32 %1, i32* %4, align 4
  store %"class.std::num_put"* %0, %"class.std::num_put"** %5, align 8
  %6 = load %"class.std::num_put"*, %"class.std::num_put"** %5, align 8
  %7 = bitcast %"class.std::num_put"* %6 to i8*
  store i8* %7, i8** %3, align 8
  %8 = load i32, i32* %4, align 4
  call void @"??1?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEAA@XZ"(%"class.std::num_put"* %6) #5
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %12, label %10

10:                                               ; preds = %2
  %11 = bitcast %"class.std::num_put"* %6 to i8*
  call void @"??3@YAXPEAX@Z"(i8* %11) #20
  br label %12

12:                                               ; preds = %10, %2
  %13 = load i8*, i8** %3, align 8
  ret i8* %13
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?_Incref@facet@locale@std@@UEAAXXZ"(%"class.std::locale::facet"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::locale::facet"*, align 8
  store %"class.std::locale::facet"* %0, %"class.std::locale::facet"** %2, align 8
  %3 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %2, align 8
  %4 = getelementptr inbounds %"class.std::locale::facet", %"class.std::locale::facet"* %3, i32 0, i32 1
  %5 = atomicrmw add i32* %4, i32 1 seq_cst
  %6 = add i32 %5, 1
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::_Facet_base"* @"?_Decref@facet@locale@std@@UEAAPEAV_Facet_base@3@XZ"(%"class.std::locale::facet"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::_Facet_base"*, align 8
  %3 = alloca %"class.std::locale::facet"*, align 8
  store %"class.std::locale::facet"* %0, %"class.std::locale::facet"** %3, align 8
  %4 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %3, align 8
  %5 = getelementptr inbounds %"class.std::locale::facet", %"class.std::locale::facet"* %4, i32 0, i32 1
  %6 = atomicrmw sub i32* %5, i32 1 seq_cst
  %7 = sub i32 %6, 1
  %8 = icmp eq i32 %7, 0
  br i1 %8, label %9, label %11

9:                                                ; preds = %1
  %10 = bitcast %"class.std::locale::facet"* %4 to %"class.std::_Facet_base"*
  store %"class.std::_Facet_base"* %10, %"class.std::_Facet_base"** %2, align 8
  br label %12

11:                                               ; preds = %1
  store %"class.std::_Facet_base"* null, %"class.std::_Facet_base"** %2, align 8
  br label %12

12:                                               ; preds = %11, %9
  %13 = load %"class.std::_Facet_base"*, %"class.std::_Facet_base"** %2, align 8
  ret %"class.std::_Facet_base"* %13
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEBX@Z"(%"class.std::num_put"* %0, %"class.std::ostreambuf_iterator"* noalias sret align 8 %1, %"class.std::ostreambuf_iterator"* %2, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %3, i8 %4, i8* %5) unnamed_addr #1 comdat align 2 {
  %7 = alloca i8*, align 8
  %8 = alloca i8*, align 8
  %9 = alloca i8, align 1
  %10 = alloca %"class.std::ios_base"*, align 8
  %11 = alloca %"class.std::num_put"*, align 8
  %12 = alloca [64 x i8], align 16
  %13 = alloca %"class.std::ostreambuf_iterator", align 8
  %14 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  store i8* %14, i8** %7, align 8
  store i8* %5, i8** %8, align 8
  store i8 %4, i8* %9, align 1
  store %"class.std::ios_base"* %3, %"class.std::ios_base"** %10, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %11, align 8
  %15 = load %"class.std::num_put"*, %"class.std::num_put"** %11, align 8
  %16 = load i8*, i8** %8, align 8
  %17 = getelementptr inbounds [64 x i8], [64 x i8]* %12, i64 0, i64 0
  %18 = call i32 (i8*, i64, i8*, ...) @sprintf_s(i8* %17, i64 64, i8* getelementptr inbounds ([3 x i8], [3 x i8]* @"??_C@_02BBAHNLBA@?$CFp?$AA@", i64 0, i64 0), i8* %16)
  %19 = sext i32 %18 to i64
  %20 = getelementptr inbounds [64 x i8], [64 x i8]* %12, i64 0, i64 0
  %21 = load i8, i8* %9, align 1
  %22 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %23 = bitcast %"class.std::ostreambuf_iterator"* %13 to i8*
  %24 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %23, i8* align 8 %24, i64 16, i1 false)
  call void @"?_Iput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEAD_K@Z"(%"class.std::num_put"* %15, %"class.std::ostreambuf_iterator"* sret align 8 %1, %"class.std::ostreambuf_iterator"* %13, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %22, i8 %21, i8* %20, i64 %19)
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DO@Z"(%"class.std::num_put"* %0, %"class.std::ostreambuf_iterator"* noalias sret align 8 %1, %"class.std::ostreambuf_iterator"* %2, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %3, i8 %4, double %5) unnamed_addr #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %7 = alloca i8*, align 8
  %8 = alloca double, align 8
  %9 = alloca i8, align 1
  %10 = alloca %"class.std::ios_base"*, align 8
  %11 = alloca %"class.std::num_put"*, align 8
  %12 = alloca %"class.std::basic_string", align 8
  %13 = alloca [8 x i8], align 1
  %14 = alloca i32, align 4
  %15 = alloca i8, align 1
  %16 = alloca i8, align 1
  %17 = alloca i64, align 8
  %18 = alloca i32, align 4
  %19 = alloca i64, align 8
  %20 = alloca i32, align 4
  %21 = alloca i64, align 8
  %22 = alloca %"class.std::ostreambuf_iterator", align 8
  %23 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  store i8* %23, i8** %7, align 8
  store double %5, double* %8, align 8
  store i8 %4, i8* %9, align 1
  store %"class.std::ios_base"* %3, %"class.std::ios_base"** %10, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %11, align 8
  %24 = load %"class.std::num_put"*, %"class.std::num_put"** %11, align 8
  %25 = call %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %12) #5
  %26 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %27 = invoke i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %26)
          to label %28 unwind label %92

28:                                               ; preds = %6
  %29 = and i32 %27, 12288
  store i32 %29, i32* %14, align 4
  %30 = load i32, i32* %14, align 4
  %31 = icmp eq i32 %30, 8192
  %32 = zext i1 %31 to i8
  store i8 %32, i8* %15, align 1
  %33 = load i32, i32* %14, align 4
  %34 = icmp eq i32 %33, 12288
  %35 = zext i1 %34 to i8
  store i8 %35, i8* %16, align 1
  %36 = load i8, i8* %16, align 1
  %37 = trunc i8 %36 to i1
  br i1 %37, label %38, label %39

38:                                               ; preds = %28
  br label %43

39:                                               ; preds = %28
  %40 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %41 = invoke i64 @"?precision@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %40)
          to label %42 unwind label %92

42:                                               ; preds = %39
  br label %43

43:                                               ; preds = %42, %38
  %44 = phi i64 [ -1, %38 ], [ %41, %42 ]
  store i64 %44, i64* %17, align 8
  %45 = load i32, i32* %14, align 4
  %46 = load i64, i64* %17, align 8
  %47 = invoke i32 @"??$_Float_put_desired_precision@O@std@@YAH_JH@Z"(i64 %46, i32 %45)
          to label %48 unwind label %92

48:                                               ; preds = %43
  store i32 %47, i32* %18, align 4
  %49 = load i32, i32* %18, align 4
  %50 = sext i32 %49 to i64
  store i64 %50, i64* %19, align 8
  %51 = load i8, i8* %15, align 1
  %52 = trunc i8 %51 to i1
  br i1 %52, label %53, label %67

53:                                               ; preds = %48
  %54 = load double, double* %8, align 8
  %55 = call double @fabsl(double %54) #23
  %56 = fcmp olt double 1.000000e+10, %55
  br i1 %56, label %57, label %67

57:                                               ; preds = %53
  %58 = load double, double* %8, align 8
  %59 = call double @frexpl(double %58, i32* %20) #5
  %60 = load i32, i32* %20, align 4
  %61 = call i32 @abs(i32 %60) #23
  %62 = mul nsw i32 %61, 30103
  %63 = sdiv i32 %62, 100000
  %64 = sext i32 %63 to i64
  %65 = load i64, i64* %19, align 8
  %66 = add i64 %65, %64
  store i64 %66, i64* %19, align 8
  br label %67

67:                                               ; preds = %57, %53, %48
  %68 = load i64, i64* %19, align 8
  %69 = add i64 %68, 50
  invoke void @"?resize@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAX_KD@Z"(%"class.std::basic_string"* %12, i64 %69, i8 0)
          to label %70 unwind label %92

70:                                               ; preds = %67
  %71 = load double, double* %8, align 8
  %72 = load i64, i64* %17, align 8
  %73 = trunc i64 %72 to i32
  %74 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %75 = invoke i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %74)
          to label %76 unwind label %92

76:                                               ; preds = %70
  %77 = getelementptr inbounds [8 x i8], [8 x i8]* %13, i64 0, i64 0
  %78 = invoke i8* @"?_Ffmt@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAPEADPEADDH@Z"(%"class.std::num_put"* %24, i8* %77, i8 76, i32 %75)
          to label %79 unwind label %92

79:                                               ; preds = %76
  %80 = call i64 @"?size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %12) #5
  %81 = call nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAD_K@Z"(%"class.std::basic_string"* %12, i64 0) #5
  %82 = invoke i32 (i8*, i64, i8*, ...) @sprintf_s(i8* %81, i64 %80, i8* %78, i32 %73, double %71)
          to label %83 unwind label %92

83:                                               ; preds = %79
  %84 = sext i32 %82 to i64
  store i64 %84, i64* %21, align 8
  %85 = load i64, i64* %21, align 8
  %86 = call i8* @"?c_str@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBAPEBDXZ"(%"class.std::basic_string"* %12) #5
  %87 = load i8, i8* %9, align 1
  %88 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %89 = bitcast %"class.std::ostreambuf_iterator"* %22 to i8*
  %90 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %89, i8* align 8 %90, i64 16, i1 false)
  invoke void @"?_Fput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEBD_K@Z"(%"class.std::num_put"* %24, %"class.std::ostreambuf_iterator"* sret align 8 %1, %"class.std::ostreambuf_iterator"* %22, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %88, i8 %87, i8* %86, i64 %85)
          to label %91 unwind label %92

91:                                               ; preds = %83
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %12) #5
  ret void

92:                                               ; preds = %83, %79, %76, %70, %67, %43, %39, %6
  %93 = cleanuppad within none []
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %12) #5 [ "funclet"(token %93) ]
  cleanupret from %93 unwind to caller
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DN@Z"(%"class.std::num_put"* %0, %"class.std::ostreambuf_iterator"* noalias sret align 8 %1, %"class.std::ostreambuf_iterator"* %2, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %3, i8 %4, double %5) unnamed_addr #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %7 = alloca i8*, align 8
  %8 = alloca double, align 8
  %9 = alloca i8, align 1
  %10 = alloca %"class.std::ios_base"*, align 8
  %11 = alloca %"class.std::num_put"*, align 8
  %12 = alloca %"class.std::basic_string", align 8
  %13 = alloca [8 x i8], align 1
  %14 = alloca i32, align 4
  %15 = alloca i8, align 1
  %16 = alloca i8, align 1
  %17 = alloca i64, align 8
  %18 = alloca i32, align 4
  %19 = alloca i64, align 8
  %20 = alloca i32, align 4
  %21 = alloca i64, align 8
  %22 = alloca %"class.std::ostreambuf_iterator", align 8
  %23 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  store i8* %23, i8** %7, align 8
  store double %5, double* %8, align 8
  store i8 %4, i8* %9, align 1
  store %"class.std::ios_base"* %3, %"class.std::ios_base"** %10, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %11, align 8
  %24 = load %"class.std::num_put"*, %"class.std::num_put"** %11, align 8
  %25 = call %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %12) #5
  %26 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %27 = invoke i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %26)
          to label %28 unwind label %92

28:                                               ; preds = %6
  %29 = and i32 %27, 12288
  store i32 %29, i32* %14, align 4
  %30 = load i32, i32* %14, align 4
  %31 = icmp eq i32 %30, 8192
  %32 = zext i1 %31 to i8
  store i8 %32, i8* %15, align 1
  %33 = load i32, i32* %14, align 4
  %34 = icmp eq i32 %33, 12288
  %35 = zext i1 %34 to i8
  store i8 %35, i8* %16, align 1
  %36 = load i8, i8* %16, align 1
  %37 = trunc i8 %36 to i1
  br i1 %37, label %38, label %39

38:                                               ; preds = %28
  br label %43

39:                                               ; preds = %28
  %40 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %41 = invoke i64 @"?precision@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %40)
          to label %42 unwind label %92

42:                                               ; preds = %39
  br label %43

43:                                               ; preds = %42, %38
  %44 = phi i64 [ -1, %38 ], [ %41, %42 ]
  store i64 %44, i64* %17, align 8
  %45 = load i32, i32* %14, align 4
  %46 = load i64, i64* %17, align 8
  %47 = invoke i32 @"??$_Float_put_desired_precision@N@std@@YAH_JH@Z"(i64 %46, i32 %45)
          to label %48 unwind label %92

48:                                               ; preds = %43
  store i32 %47, i32* %18, align 4
  %49 = load i32, i32* %18, align 4
  %50 = sext i32 %49 to i64
  store i64 %50, i64* %19, align 8
  %51 = load i8, i8* %15, align 1
  %52 = trunc i8 %51 to i1
  br i1 %52, label %53, label %67

53:                                               ; preds = %48
  %54 = load double, double* %8, align 8
  %55 = call double @llvm.fabs.f64(double %54)
  %56 = fcmp olt double 1.000000e+10, %55
  br i1 %56, label %57, label %67

57:                                               ; preds = %53
  %58 = load double, double* %8, align 8
  %59 = call double @frexp(double %58, i32* %20) #5
  %60 = load i32, i32* %20, align 4
  %61 = call i32 @abs(i32 %60) #23
  %62 = mul nsw i32 %61, 30103
  %63 = sdiv i32 %62, 100000
  %64 = sext i32 %63 to i64
  %65 = load i64, i64* %19, align 8
  %66 = add i64 %65, %64
  store i64 %66, i64* %19, align 8
  br label %67

67:                                               ; preds = %57, %53, %48
  %68 = load i64, i64* %19, align 8
  %69 = add i64 %68, 50
  invoke void @"?resize@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAX_KD@Z"(%"class.std::basic_string"* %12, i64 %69, i8 0)
          to label %70 unwind label %92

70:                                               ; preds = %67
  %71 = load double, double* %8, align 8
  %72 = load i64, i64* %17, align 8
  %73 = trunc i64 %72 to i32
  %74 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %75 = invoke i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %74)
          to label %76 unwind label %92

76:                                               ; preds = %70
  %77 = getelementptr inbounds [8 x i8], [8 x i8]* %13, i64 0, i64 0
  %78 = invoke i8* @"?_Ffmt@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAPEADPEADDH@Z"(%"class.std::num_put"* %24, i8* %77, i8 0, i32 %75)
          to label %79 unwind label %92

79:                                               ; preds = %76
  %80 = call i64 @"?size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %12) #5
  %81 = call nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAD_K@Z"(%"class.std::basic_string"* %12, i64 0) #5
  %82 = invoke i32 (i8*, i64, i8*, ...) @sprintf_s(i8* %81, i64 %80, i8* %78, i32 %73, double %71)
          to label %83 unwind label %92

83:                                               ; preds = %79
  %84 = sext i32 %82 to i64
  store i64 %84, i64* %21, align 8
  %85 = load i64, i64* %21, align 8
  %86 = call i8* @"?c_str@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBAPEBDXZ"(%"class.std::basic_string"* %12) #5
  %87 = load i8, i8* %9, align 1
  %88 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %89 = bitcast %"class.std::ostreambuf_iterator"* %22 to i8*
  %90 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %89, i8* align 8 %90, i64 16, i1 false)
  invoke void @"?_Fput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEBD_K@Z"(%"class.std::num_put"* %24, %"class.std::ostreambuf_iterator"* sret align 8 %1, %"class.std::ostreambuf_iterator"* %22, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %88, i8 %87, i8* %86, i64 %85)
          to label %91 unwind label %92

91:                                               ; preds = %83
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %12) #5
  ret void

92:                                               ; preds = %83, %79, %76, %70, %67, %43, %39, %6
  %93 = cleanuppad within none []
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %12) #5 [ "funclet"(token %93) ]
  cleanupret from %93 unwind to caller
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@D_K@Z"(%"class.std::num_put"* %0, %"class.std::ostreambuf_iterator"* noalias sret align 8 %1, %"class.std::ostreambuf_iterator"* %2, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %3, i8 %4, i64 %5) unnamed_addr #1 comdat align 2 {
  %7 = alloca i8*, align 8
  %8 = alloca i64, align 8
  %9 = alloca i8, align 1
  %10 = alloca %"class.std::ios_base"*, align 8
  %11 = alloca %"class.std::num_put"*, align 8
  %12 = alloca [64 x i8], align 16
  %13 = alloca [8 x i8], align 1
  %14 = alloca %"class.std::ostreambuf_iterator", align 8
  %15 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  store i8* %15, i8** %7, align 8
  store i64 %5, i64* %8, align 8
  store i8 %4, i8* %9, align 1
  store %"class.std::ios_base"* %3, %"class.std::ios_base"** %10, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %11, align 8
  %16 = load %"class.std::num_put"*, %"class.std::num_put"** %11, align 8
  %17 = load i64, i64* %8, align 8
  %18 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %19 = call i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %18)
  %20 = getelementptr inbounds [8 x i8], [8 x i8]* %13, i64 0, i64 0
  %21 = call i8* @"?_Ifmt@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAPEADPEADPEBDH@Z"(%"class.std::num_put"* %16, i8* %20, i8* getelementptr inbounds ([3 x i8], [3 x i8]* @"??_C@_02CLHGNPPK@Lu?$AA@", i64 0, i64 0), i32 %19)
  %22 = getelementptr inbounds [64 x i8], [64 x i8]* %12, i64 0, i64 0
  %23 = call i32 (i8*, i64, i8*, ...) @sprintf_s(i8* %22, i64 64, i8* %21, i64 %17)
  %24 = sext i32 %23 to i64
  %25 = getelementptr inbounds [64 x i8], [64 x i8]* %12, i64 0, i64 0
  %26 = load i8, i8* %9, align 1
  %27 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %28 = bitcast %"class.std::ostreambuf_iterator"* %14 to i8*
  %29 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %28, i8* align 8 %29, i64 16, i1 false)
  call void @"?_Iput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEAD_K@Z"(%"class.std::num_put"* %16, %"class.std::ostreambuf_iterator"* sret align 8 %1, %"class.std::ostreambuf_iterator"* %14, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %27, i8 %26, i8* %25, i64 %24)
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@D_J@Z"(%"class.std::num_put"* %0, %"class.std::ostreambuf_iterator"* noalias sret align 8 %1, %"class.std::ostreambuf_iterator"* %2, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %3, i8 %4, i64 %5) unnamed_addr #1 comdat align 2 {
  %7 = alloca i8*, align 8
  %8 = alloca i64, align 8
  %9 = alloca i8, align 1
  %10 = alloca %"class.std::ios_base"*, align 8
  %11 = alloca %"class.std::num_put"*, align 8
  %12 = alloca [64 x i8], align 16
  %13 = alloca [8 x i8], align 1
  %14 = alloca %"class.std::ostreambuf_iterator", align 8
  %15 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  store i8* %15, i8** %7, align 8
  store i64 %5, i64* %8, align 8
  store i8 %4, i8* %9, align 1
  store %"class.std::ios_base"* %3, %"class.std::ios_base"** %10, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %11, align 8
  %16 = load %"class.std::num_put"*, %"class.std::num_put"** %11, align 8
  %17 = load i64, i64* %8, align 8
  %18 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %19 = call i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %18)
  %20 = getelementptr inbounds [8 x i8], [8 x i8]* %13, i64 0, i64 0
  %21 = call i8* @"?_Ifmt@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAPEADPEADPEBDH@Z"(%"class.std::num_put"* %16, i8* %20, i8* getelementptr inbounds ([3 x i8], [3 x i8]* @"??_C@_02HIKPPMOK@Ld?$AA@", i64 0, i64 0), i32 %19)
  %22 = getelementptr inbounds [64 x i8], [64 x i8]* %12, i64 0, i64 0
  %23 = call i32 (i8*, i64, i8*, ...) @sprintf_s(i8* %22, i64 64, i8* %21, i64 %17)
  %24 = sext i32 %23 to i64
  %25 = getelementptr inbounds [64 x i8], [64 x i8]* %12, i64 0, i64 0
  %26 = load i8, i8* %9, align 1
  %27 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %28 = bitcast %"class.std::ostreambuf_iterator"* %14 to i8*
  %29 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %28, i8* align 8 %29, i64 16, i1 false)
  call void @"?_Iput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEAD_K@Z"(%"class.std::num_put"* %16, %"class.std::ostreambuf_iterator"* sret align 8 %1, %"class.std::ostreambuf_iterator"* %14, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %27, i8 %26, i8* %25, i64 %24)
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DK@Z"(%"class.std::num_put"* %0, %"class.std::ostreambuf_iterator"* noalias sret align 8 %1, %"class.std::ostreambuf_iterator"* %2, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %3, i8 %4, i32 %5) unnamed_addr #1 comdat align 2 {
  %7 = alloca i8*, align 8
  %8 = alloca i32, align 4
  %9 = alloca i8, align 1
  %10 = alloca %"class.std::ios_base"*, align 8
  %11 = alloca %"class.std::num_put"*, align 8
  %12 = alloca [64 x i8], align 16
  %13 = alloca [6 x i8], align 1
  %14 = alloca %"class.std::ostreambuf_iterator", align 8
  %15 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  store i8* %15, i8** %7, align 8
  store i32 %5, i32* %8, align 4
  store i8 %4, i8* %9, align 1
  store %"class.std::ios_base"* %3, %"class.std::ios_base"** %10, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %11, align 8
  %16 = load %"class.std::num_put"*, %"class.std::num_put"** %11, align 8
  %17 = load i32, i32* %8, align 4
  %18 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %19 = call i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %18)
  %20 = getelementptr inbounds [6 x i8], [6 x i8]* %13, i64 0, i64 0
  %21 = call i8* @"?_Ifmt@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAPEADPEADPEBDH@Z"(%"class.std::num_put"* %16, i8* %20, i8* getelementptr inbounds ([3 x i8], [3 x i8]* @"??_C@_02BDDLJJBK@lu?$AA@", i64 0, i64 0), i32 %19)
  %22 = getelementptr inbounds [64 x i8], [64 x i8]* %12, i64 0, i64 0
  %23 = call i32 (i8*, i64, i8*, ...) @sprintf_s(i8* %22, i64 64, i8* %21, i32 %17)
  %24 = sext i32 %23 to i64
  %25 = getelementptr inbounds [64 x i8], [64 x i8]* %12, i64 0, i64 0
  %26 = load i8, i8* %9, align 1
  %27 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %28 = bitcast %"class.std::ostreambuf_iterator"* %14 to i8*
  %29 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %28, i8* align 8 %29, i64 16, i1 false)
  call void @"?_Iput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEAD_K@Z"(%"class.std::num_put"* %16, %"class.std::ostreambuf_iterator"* sret align 8 %1, %"class.std::ostreambuf_iterator"* %14, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %27, i8 %26, i8* %25, i64 %24)
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DJ@Z"(%"class.std::num_put"* %0, %"class.std::ostreambuf_iterator"* noalias sret align 8 %1, %"class.std::ostreambuf_iterator"* %2, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %3, i8 %4, i32 %5) unnamed_addr #1 comdat align 2 {
  %7 = alloca i8*, align 8
  %8 = alloca i32, align 4
  %9 = alloca i8, align 1
  %10 = alloca %"class.std::ios_base"*, align 8
  %11 = alloca %"class.std::num_put"*, align 8
  %12 = alloca [64 x i8], align 16
  %13 = alloca [6 x i8], align 1
  %14 = alloca %"class.std::ostreambuf_iterator", align 8
  %15 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  store i8* %15, i8** %7, align 8
  store i32 %5, i32* %8, align 4
  store i8 %4, i8* %9, align 1
  store %"class.std::ios_base"* %3, %"class.std::ios_base"** %10, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %11, align 8
  %16 = load %"class.std::num_put"*, %"class.std::num_put"** %11, align 8
  %17 = load i32, i32* %8, align 4
  %18 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %19 = call i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %18)
  %20 = getelementptr inbounds [6 x i8], [6 x i8]* %13, i64 0, i64 0
  %21 = call i8* @"?_Ifmt@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAPEADPEADPEBDH@Z"(%"class.std::num_put"* %16, i8* %20, i8* getelementptr inbounds ([3 x i8], [3 x i8]* @"??_C@_02EAOCLKAK@ld?$AA@", i64 0, i64 0), i32 %19)
  %22 = getelementptr inbounds [64 x i8], [64 x i8]* %12, i64 0, i64 0
  %23 = call i32 (i8*, i64, i8*, ...) @sprintf_s(i8* %22, i64 64, i8* %21, i32 %17)
  %24 = sext i32 %23 to i64
  %25 = getelementptr inbounds [64 x i8], [64 x i8]* %12, i64 0, i64 0
  %26 = load i8, i8* %9, align 1
  %27 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %28 = bitcast %"class.std::ostreambuf_iterator"* %14 to i8*
  %29 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %28, i8* align 8 %29, i64 16, i1 false)
  call void @"?_Iput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEAD_K@Z"(%"class.std::num_put"* %16, %"class.std::ostreambuf_iterator"* sret align 8 %1, %"class.std::ostreambuf_iterator"* %14, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %27, i8 %26, i8* %25, i64 %24)
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?do_put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@D_N@Z"(%"class.std::num_put"* %0, %"class.std::ostreambuf_iterator"* noalias sret align 8 %1, %"class.std::ostreambuf_iterator"* %2, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %3, i8 %4, i1 zeroext %5) unnamed_addr #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %7 = alloca i8*, align 8
  %8 = alloca i8, align 1
  %9 = alloca i8, align 1
  %10 = alloca %"class.std::ios_base"*, align 8
  %11 = alloca %"class.std::num_put"*, align 8
  %12 = alloca %"class.std::ostreambuf_iterator", align 8
  %13 = alloca %"class.std::numpunct"*, align 8
  %14 = alloca %"class.std::locale", align 8
  %15 = alloca %"class.std::basic_string", align 8
  %16 = alloca %"class.std::basic_string", align 8
  %17 = alloca %"class.std::basic_string", align 8
  %18 = alloca i64, align 8
  %19 = alloca %"class.std::ostreambuf_iterator", align 8
  %20 = alloca %"class.std::ostreambuf_iterator", align 8
  %21 = alloca %"class.std::ostreambuf_iterator", align 8
  %22 = alloca %"class.std::ostreambuf_iterator", align 8
  %23 = alloca %"class.std::ostreambuf_iterator", align 8
  %24 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  store i8* %24, i8** %7, align 8
  %25 = zext i1 %5 to i8
  store i8 %25, i8* %8, align 1
  store i8 %4, i8* %9, align 1
  store %"class.std::ios_base"* %3, %"class.std::ios_base"** %10, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %11, align 8
  %26 = load %"class.std::num_put"*, %"class.std::num_put"** %11, align 8
  %27 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %28 = call i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %27)
  %29 = and i32 %28, 16384
  %30 = icmp ne i32 %29, 0
  br i1 %30, label %43, label %31

31:                                               ; preds = %6
  %32 = load i8, i8* %8, align 1
  %33 = trunc i8 %32 to i1
  %34 = zext i1 %33 to i32
  %35 = load i8, i8* %9, align 1
  %36 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %37 = bitcast %"class.std::ostreambuf_iterator"* %12 to i8*
  %38 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %37, i8* align 8 %38, i64 16, i1 false)
  %39 = bitcast %"class.std::num_put"* %26 to void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i32)***
  %40 = load void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i32)**, void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i32)*** %39, align 8
  %41 = getelementptr inbounds void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i32)*, void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i32)** %40, i64 9
  %42 = load void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i32)*, void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i32)** %41, align 8
  call void %42(%"class.std::num_put"* %26, %"class.std::ostreambuf_iterator"* sret align 8 %1, %"class.std::ostreambuf_iterator"* %12, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %36, i8 %35, i32 %34)
  br label %110

43:                                               ; preds = %6
  %44 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  call void @"?getloc@ios_base@std@@QEBA?AVlocale@2@XZ"(%"class.std::ios_base"* %44, %"class.std::locale"* sret align 8 %14)
  %45 = invoke nonnull align 8 dereferenceable(48) %"class.std::numpunct"* @"??$use_facet@V?$numpunct@D@std@@@std@@YAAEBV?$numpunct@D@0@AEBVlocale@0@@Z"(%"class.std::locale"* nonnull align 8 dereferenceable(16) %14)
          to label %46 unwind label %54

46:                                               ; preds = %43
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %14) #5
  store %"class.std::numpunct"* %45, %"class.std::numpunct"** %13, align 8
  %47 = call %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %15) #5
  %48 = load i8, i8* %8, align 1
  %49 = trunc i8 %48 to i1
  br i1 %49, label %50, label %56

50:                                               ; preds = %46
  %51 = load %"class.std::numpunct"*, %"class.std::numpunct"** %13, align 8
  invoke void @"?truename@?$numpunct@D@std@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ"(%"class.std::numpunct"* %51, %"class.std::basic_string"* sret align 8 %16)
          to label %52 unwind label %108

52:                                               ; preds = %50
  %53 = call nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@$$QEAV12@@Z"(%"class.std::basic_string"* %15, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %16) #5
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %16) #5
  br label %60

54:                                               ; preds = %43
  %55 = cleanuppad within none []
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %14) #5 [ "funclet"(token %55) ]
  cleanupret from %55 unwind to caller

56:                                               ; preds = %46
  %57 = load %"class.std::numpunct"*, %"class.std::numpunct"** %13, align 8
  invoke void @"?falsename@?$numpunct@D@std@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ"(%"class.std::numpunct"* %57, %"class.std::basic_string"* sret align 8 %17)
          to label %58 unwind label %108

58:                                               ; preds = %56
  %59 = call nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@$$QEAV12@@Z"(%"class.std::basic_string"* %15, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %17) #5
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %17) #5
  br label %60

60:                                               ; preds = %58, %52
  %61 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %62 = invoke i64 @"?width@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %61)
          to label %63 unwind label %108

63:                                               ; preds = %60
  %64 = icmp sle i64 %62, 0
  br i1 %64, label %71, label %65

65:                                               ; preds = %63
  %66 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %67 = invoke i64 @"?width@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %66)
          to label %68 unwind label %108

68:                                               ; preds = %65
  %69 = call i64 @"?size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %15) #5
  %70 = icmp ule i64 %67, %69
  br i1 %70, label %71, label %72

71:                                               ; preds = %68, %63
  store i64 0, i64* %18, align 8
  br label %78

72:                                               ; preds = %68
  %73 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %74 = invoke i64 @"?width@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %73)
          to label %75 unwind label %108

75:                                               ; preds = %72
  %76 = call i64 @"?size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %15) #5
  %77 = sub i64 %74, %76
  store i64 %77, i64* %18, align 8
  br label %78

78:                                               ; preds = %75, %71
  %79 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %80 = invoke i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %79)
          to label %81 unwind label %108

81:                                               ; preds = %78
  %82 = and i32 %80, 448
  %83 = icmp ne i32 %82, 64
  br i1 %83, label %84, label %92

84:                                               ; preds = %81
  %85 = load i64, i64* %18, align 8
  %86 = load i8, i8* %9, align 1
  %87 = bitcast %"class.std::ostreambuf_iterator"* %20 to i8*
  %88 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %87, i8* align 8 %88, i64 16, i1 false)
  invoke void @"?_Rep@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@D_K@Z"(%"class.std::num_put"* %26, %"class.std::ostreambuf_iterator"* sret align 8 %19, %"class.std::ostreambuf_iterator"* %20, i8 %86, i64 %85)
          to label %89 unwind label %108

89:                                               ; preds = %84
  %90 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  %91 = bitcast %"class.std::ostreambuf_iterator"* %19 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %90, i8* align 8 %91, i64 16, i1 false)
  store i64 0, i64* %18, align 8
  br label %92

92:                                               ; preds = %89, %81
  %93 = call i64 @"?size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %15) #5
  %94 = call i8* @"?c_str@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBAPEBDXZ"(%"class.std::basic_string"* %15) #5
  %95 = bitcast %"class.std::ostreambuf_iterator"* %22 to i8*
  %96 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %95, i8* align 8 %96, i64 16, i1 false)
  invoke void @"?_Put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@PEBD_K@Z"(%"class.std::num_put"* %26, %"class.std::ostreambuf_iterator"* sret align 8 %21, %"class.std::ostreambuf_iterator"* %22, i8* %94, i64 %93)
          to label %97 unwind label %108

97:                                               ; preds = %92
  %98 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  %99 = bitcast %"class.std::ostreambuf_iterator"* %21 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %98, i8* align 8 %99, i64 16, i1 false)
  %100 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %101 = invoke i64 @"?width@ios_base@std@@QEAA_J_J@Z"(%"class.std::ios_base"* %100, i64 0)
          to label %102 unwind label %108

102:                                              ; preds = %97
  %103 = load i64, i64* %18, align 8
  %104 = load i8, i8* %9, align 1
  %105 = bitcast %"class.std::ostreambuf_iterator"* %23 to i8*
  %106 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %105, i8* align 8 %106, i64 16, i1 false)
  invoke void @"?_Rep@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@D_K@Z"(%"class.std::num_put"* %26, %"class.std::ostreambuf_iterator"* sret align 8 %1, %"class.std::ostreambuf_iterator"* %23, i8 %104, i64 %103)
          to label %107 unwind label %108

107:                                              ; preds = %102
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %15) #5
  br label %110

108:                                              ; preds = %102, %97, %92, %84, %78, %72, %65, %60, %56, %50
  %109 = cleanuppad within none []
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %15) #5 [ "funclet"(token %109) ]
  cleanupret from %109 unwind to caller

110:                                              ; preds = %107, %31
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::_Facet_base"* @"??0_Facet_base@std@@QEAA@XZ"(%"class.std::_Facet_base"* returned %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::_Facet_base"*, align 8
  store %"class.std::_Facet_base"* %0, %"class.std::_Facet_base"** %2, align 8
  %3 = load %"class.std::_Facet_base"*, %"class.std::_Facet_base"** %2, align 8
  %4 = bitcast %"class.std::_Facet_base"* %3 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7_Facet_base@std@@6B@" to i32 (...)**), i32 (...)*** %4, align 8
  ret %"class.std::_Facet_base"* %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??_Gfacet@locale@std@@MEAAPEAXI@Z"(%"class.std::locale::facet"* %0, i32 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca i32, align 4
  %5 = alloca %"class.std::locale::facet"*, align 8
  store i32 %1, i32* %4, align 4
  store %"class.std::locale::facet"* %0, %"class.std::locale::facet"** %5, align 8
  %6 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %5, align 8
  %7 = bitcast %"class.std::locale::facet"* %6 to i8*
  store i8* %7, i8** %3, align 8
  %8 = load i32, i32* %4, align 4
  call void @"??1facet@locale@std@@MEAA@XZ"(%"class.std::locale::facet"* %6) #5
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %12, label %10

10:                                               ; preds = %2
  %11 = bitcast %"class.std::locale::facet"* %6 to i8*
  call void @"??3@YAXPEAX@Z"(i8* %11) #20
  br label %12

12:                                               ; preds = %10, %2
  %13 = load i8*, i8** %3, align 8
  ret i8* %13
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??_G_Facet_base@std@@UEAAPEAXI@Z"(%"class.std::_Facet_base"* %0, i32 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca i32, align 4
  %5 = alloca %"class.std::_Facet_base"*, align 8
  store i32 %1, i32* %4, align 4
  store %"class.std::_Facet_base"* %0, %"class.std::_Facet_base"** %5, align 8
  %6 = load %"class.std::_Facet_base"*, %"class.std::_Facet_base"** %5, align 8
  %7 = bitcast %"class.std::_Facet_base"* %6 to i8*
  store i8* %7, i8** %3, align 8
  %8 = load i32, i32* %4, align 4
  call void @llvm.trap() #18
  unreachable
}

declare dso_local void @_purecall() unnamed_addr

; Function Attrs: cold noreturn nounwind
declare void @llvm.trap() #13

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1_Facet_base@std@@UEAA@XZ"(%"class.std::_Facet_base"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::_Facet_base"*, align 8
  store %"class.std::_Facet_base"* %0, %"class.std::_Facet_base"** %2, align 8
  %3 = load %"class.std::_Facet_base"*, %"class.std::_Facet_base"** %2, align 8
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@MEAA@XZ"(%"class.std::num_put"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::num_put"*, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %2, align 8
  %3 = load %"class.std::num_put"*, %"class.std::num_put"** %2, align 8
  %4 = bitcast %"class.std::num_put"* %3 to %"class.std::locale::facet"*
  call void @"??1facet@locale@std@@MEAA@XZ"(%"class.std::locale::facet"* %4) #5
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?_Iput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEAD_K@Z"(%"class.std::num_put"* %0, %"class.std::ostreambuf_iterator"* noalias sret align 8 %1, %"class.std::ostreambuf_iterator"* %2, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %3, i8 %4, i8* %5, i64 %6) #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %8 = alloca i8*, align 8
  %9 = alloca i64, align 8
  %10 = alloca i8*, align 8
  %11 = alloca i8, align 1
  %12 = alloca %"class.std::ios_base"*, align 8
  %13 = alloca %"class.std::num_put"*, align 8
  %14 = alloca i64, align 8
  %15 = alloca %"class.std::ctype"*, align 8
  %16 = alloca %"class.std::locale", align 8
  %17 = alloca %"class.std::basic_string", align 8
  %18 = alloca %"class.std::numpunct"*, align 8
  %19 = alloca %"class.std::locale", align 8
  %20 = alloca %"class.std::basic_string", align 8
  %21 = alloca i8*, align 8
  %22 = alloca i8, align 1
  %23 = alloca i64, align 8
  %24 = alloca i32, align 4
  %25 = alloca %"class.std::ostreambuf_iterator", align 8
  %26 = alloca %"class.std::ostreambuf_iterator", align 8
  %27 = alloca %"class.std::ostreambuf_iterator", align 8
  %28 = alloca %"class.std::ostreambuf_iterator", align 8
  %29 = alloca %"class.std::ostreambuf_iterator", align 8
  %30 = alloca %"class.std::ostreambuf_iterator", align 8
  %31 = alloca %"class.std::ostreambuf_iterator", align 8
  %32 = alloca %"class.std::ostreambuf_iterator", align 8
  %33 = alloca %"class.std::ostreambuf_iterator", align 8
  %34 = alloca %"class.std::ostreambuf_iterator", align 8
  %35 = alloca %"class.std::ostreambuf_iterator", align 8
  %36 = alloca %"class.std::ostreambuf_iterator", align 8
  %37 = alloca %"class.std::ostreambuf_iterator", align 8
  %38 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  store i8* %38, i8** %8, align 8
  store i64 %6, i64* %9, align 8
  store i8* %5, i8** %10, align 8
  store i8 %4, i8* %11, align 1
  store %"class.std::ios_base"* %3, %"class.std::ios_base"** %12, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %13, align 8
  %39 = load %"class.std::num_put"*, %"class.std::num_put"** %13, align 8
  %40 = load i64, i64* %9, align 8
  %41 = icmp ult i64 0, %40
  br i1 %41, label %42, label %54

42:                                               ; preds = %7
  %43 = load i8*, i8** %10, align 8
  %44 = load i8, i8* %43, align 1
  %45 = sext i8 %44 to i32
  %46 = icmp eq i32 %45, 43
  br i1 %46, label %52, label %47

47:                                               ; preds = %42
  %48 = load i8*, i8** %10, align 8
  %49 = load i8, i8* %48, align 1
  %50 = sext i8 %49 to i32
  %51 = icmp eq i32 %50, 45
  br label %52

52:                                               ; preds = %47, %42
  %53 = phi i1 [ true, %42 ], [ %51, %47 ]
  br label %54

54:                                               ; preds = %52, %7
  %55 = phi i1 [ false, %7 ], [ %53, %52 ]
  %56 = zext i1 %55 to i64
  store i64 %56, i64* %14, align 8
  %57 = load %"class.std::ios_base"*, %"class.std::ios_base"** %12, align 8
  %58 = call i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %57)
  %59 = and i32 %58, 3584
  %60 = icmp eq i32 %59, 2048
  br i1 %60, label %61, label %92

61:                                               ; preds = %54
  %62 = load i64, i64* %14, align 8
  %63 = add i64 %62, 2
  %64 = load i64, i64* %9, align 8
  %65 = icmp ule i64 %63, %64
  br i1 %65, label %66, label %92

66:                                               ; preds = %61
  %67 = load i8*, i8** %10, align 8
  %68 = load i64, i64* %14, align 8
  %69 = getelementptr inbounds i8, i8* %67, i64 %68
  %70 = load i8, i8* %69, align 1
  %71 = sext i8 %70 to i32
  %72 = icmp eq i32 %71, 48
  br i1 %72, label %73, label %92

73:                                               ; preds = %66
  %74 = load i8*, i8** %10, align 8
  %75 = load i64, i64* %14, align 8
  %76 = add i64 %75, 1
  %77 = getelementptr inbounds i8, i8* %74, i64 %76
  %78 = load i8, i8* %77, align 1
  %79 = sext i8 %78 to i32
  %80 = icmp eq i32 %79, 120
  br i1 %80, label %89, label %81

81:                                               ; preds = %73
  %82 = load i8*, i8** %10, align 8
  %83 = load i64, i64* %14, align 8
  %84 = add i64 %83, 1
  %85 = getelementptr inbounds i8, i8* %82, i64 %84
  %86 = load i8, i8* %85, align 1
  %87 = sext i8 %86 to i32
  %88 = icmp eq i32 %87, 88
  br i1 %88, label %89, label %92

89:                                               ; preds = %81, %73
  %90 = load i64, i64* %14, align 8
  %91 = add i64 %90, 2
  store i64 %91, i64* %14, align 8
  br label %92

92:                                               ; preds = %89, %81, %66, %61, %54
  %93 = load %"class.std::ios_base"*, %"class.std::ios_base"** %12, align 8
  call void @"?getloc@ios_base@std@@QEBA?AVlocale@2@XZ"(%"class.std::ios_base"* %93, %"class.std::locale"* sret align 8 %16)
  %94 = invoke nonnull align 8 dereferenceable(48) %"class.std::ctype"* @"??$use_facet@V?$ctype@D@std@@@std@@YAAEBV?$ctype@D@0@AEBVlocale@0@@Z"(%"class.std::locale"* nonnull align 8 dereferenceable(16) %16)
          to label %95 unwind label %164

95:                                               ; preds = %92
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %16) #5
  store %"class.std::ctype"* %94, %"class.std::ctype"** %15, align 8
  %96 = load i64, i64* %9, align 8
  %97 = call %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@_KD@Z"(%"class.std::basic_string"* %17, i64 %96, i8 0)
  %98 = load %"class.std::ctype"*, %"class.std::ctype"** %15, align 8
  %99 = call nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAD_K@Z"(%"class.std::basic_string"* %17, i64 0) #5
  %100 = load i8*, i8** %10, align 8
  %101 = load i64, i64* %9, align 8
  %102 = getelementptr inbounds i8, i8* %100, i64 %101
  %103 = load i8*, i8** %10, align 8
  %104 = invoke i8* @"?widen@?$ctype@D@std@@QEBAPEBDPEBD0PEAD@Z"(%"class.std::ctype"* %98, i8* %103, i8* %102, i8* %99)
          to label %105 unwind label %262

105:                                              ; preds = %95
  %106 = load %"class.std::ios_base"*, %"class.std::ios_base"** %12, align 8
  invoke void @"?getloc@ios_base@std@@QEBA?AVlocale@2@XZ"(%"class.std::ios_base"* %106, %"class.std::locale"* sret align 8 %19)
          to label %107 unwind label %262

107:                                              ; preds = %105
  %108 = invoke nonnull align 8 dereferenceable(48) %"class.std::numpunct"* @"??$use_facet@V?$numpunct@D@std@@@std@@YAAEBV?$numpunct@D@0@AEBVlocale@0@@Z"(%"class.std::locale"* nonnull align 8 dereferenceable(16) %19)
          to label %109 unwind label %166

109:                                              ; preds = %107
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %19) #5
  store %"class.std::numpunct"* %108, %"class.std::numpunct"** %18, align 8
  %110 = load %"class.std::numpunct"*, %"class.std::numpunct"** %18, align 8
  invoke void @"?grouping@?$numpunct@D@std@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ"(%"class.std::numpunct"* %110, %"class.std::basic_string"* sret align 8 %20)
          to label %111 unwind label %262

111:                                              ; preds = %109
  %112 = call nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBAAEBD_K@Z"(%"class.std::basic_string"* %20, i64 0) #5
  store i8* %112, i8** %21, align 8
  %113 = load i8*, i8** %21, align 8
  %114 = load i8, i8* %113, align 1
  %115 = sext i8 %114 to i32
  %116 = icmp ne i32 %115, 127
  br i1 %116, label %117, label %170

117:                                              ; preds = %111
  %118 = load i8*, i8** %21, align 8
  %119 = load i8, i8* %118, align 1
  %120 = sext i8 %119 to i32
  %121 = icmp slt i32 0, %120
  br i1 %121, label %122, label %170

122:                                              ; preds = %117
  %123 = load %"class.std::numpunct"*, %"class.std::numpunct"** %18, align 8
  %124 = invoke i8 @"?thousands_sep@?$numpunct@D@std@@QEBADXZ"(%"class.std::numpunct"* %123)
          to label %125 unwind label %260

125:                                              ; preds = %122
  store i8 %124, i8* %22, align 1
  br label %126

126:                                              ; preds = %168, %125
  %127 = load i8*, i8** %21, align 8
  %128 = load i8, i8* %127, align 1
  %129 = sext i8 %128 to i32
  %130 = icmp ne i32 %129, 127
  br i1 %130, label %131, label %144

131:                                              ; preds = %126
  %132 = load i8*, i8** %21, align 8
  %133 = load i8, i8* %132, align 1
  %134 = sext i8 %133 to i32
  %135 = icmp slt i32 0, %134
  br i1 %135, label %136, label %144

136:                                              ; preds = %131
  %137 = load i8*, i8** %21, align 8
  %138 = load i8, i8* %137, align 1
  %139 = sext i8 %138 to i64
  %140 = load i64, i64* %9, align 8
  %141 = load i64, i64* %14, align 8
  %142 = sub i64 %140, %141
  %143 = icmp ult i64 %139, %142
  br label %144

144:                                              ; preds = %136, %131, %126
  %145 = phi i1 [ false, %131 ], [ false, %126 ], [ %143, %136 ]
  br i1 %145, label %146, label %169

146:                                              ; preds = %144
  %147 = load i8*, i8** %21, align 8
  %148 = load i8, i8* %147, align 1
  %149 = sext i8 %148 to i64
  %150 = load i64, i64* %9, align 8
  %151 = sub i64 %150, %149
  store i64 %151, i64* %9, align 8
  %152 = load i8, i8* %22, align 1
  %153 = load i64, i64* %9, align 8
  %154 = invoke nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?insert@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@_K0D@Z"(%"class.std::basic_string"* %17, i64 %153, i64 1, i8 %152)
          to label %155 unwind label %260

155:                                              ; preds = %146
  %156 = load i8*, i8** %21, align 8
  %157 = getelementptr inbounds i8, i8* %156, i64 1
  %158 = load i8, i8* %157, align 1
  %159 = sext i8 %158 to i32
  %160 = icmp slt i32 0, %159
  br i1 %160, label %161, label %168

161:                                              ; preds = %155
  %162 = load i8*, i8** %21, align 8
  %163 = getelementptr inbounds i8, i8* %162, i32 1
  store i8* %163, i8** %21, align 8
  br label %168

164:                                              ; preds = %92
  %165 = cleanuppad within none []
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %16) #5 [ "funclet"(token %165) ]
  cleanupret from %165 unwind to caller

166:                                              ; preds = %107
  %167 = cleanuppad within none []
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %19) #5 [ "funclet"(token %167) ]
  cleanupret from %167 unwind label %262

168:                                              ; preds = %161, %155
  br label %126

169:                                              ; preds = %144
  br label %170

170:                                              ; preds = %169, %117, %111
  %171 = call i64 @"?size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %17) #5
  store i64 %171, i64* %9, align 8
  %172 = load %"class.std::ios_base"*, %"class.std::ios_base"** %12, align 8
  %173 = invoke i64 @"?width@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %172)
          to label %174 unwind label %260

174:                                              ; preds = %170
  %175 = icmp sle i64 %173, 0
  br i1 %175, label %182, label %176

176:                                              ; preds = %174
  %177 = load %"class.std::ios_base"*, %"class.std::ios_base"** %12, align 8
  %178 = invoke i64 @"?width@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %177)
          to label %179 unwind label %260

179:                                              ; preds = %176
  %180 = load i64, i64* %9, align 8
  %181 = icmp ule i64 %178, %180
  br i1 %181, label %182, label %183

182:                                              ; preds = %179, %174
  store i64 0, i64* %23, align 8
  br label %189

183:                                              ; preds = %179
  %184 = load %"class.std::ios_base"*, %"class.std::ios_base"** %12, align 8
  %185 = invoke i64 @"?width@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %184)
          to label %186 unwind label %260

186:                                              ; preds = %183
  %187 = load i64, i64* %9, align 8
  %188 = sub i64 %185, %187
  store i64 %188, i64* %23, align 8
  br label %189

189:                                              ; preds = %186, %182
  %190 = load %"class.std::ios_base"*, %"class.std::ios_base"** %12, align 8
  %191 = invoke i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %190)
          to label %192 unwind label %260

192:                                              ; preds = %189
  %193 = and i32 %191, 448
  store i32 %193, i32* %24, align 4
  %194 = load i32, i32* %24, align 4
  %195 = icmp ne i32 %194, 64
  br i1 %195, label %196, label %214

196:                                              ; preds = %192
  %197 = load i32, i32* %24, align 4
  %198 = icmp ne i32 %197, 256
  br i1 %198, label %199, label %214

199:                                              ; preds = %196
  %200 = load i64, i64* %23, align 8
  %201 = load i8, i8* %11, align 1
  %202 = bitcast %"class.std::ostreambuf_iterator"* %26 to i8*
  %203 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %202, i8* align 8 %203, i64 16, i1 false)
  invoke void @"?_Rep@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@D_K@Z"(%"class.std::num_put"* %39, %"class.std::ostreambuf_iterator"* sret align 8 %25, %"class.std::ostreambuf_iterator"* %26, i8 %201, i64 %200)
          to label %204 unwind label %260

204:                                              ; preds = %199
  %205 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  %206 = bitcast %"class.std::ostreambuf_iterator"* %25 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %205, i8* align 8 %206, i64 16, i1 false)
  store i64 0, i64* %23, align 8
  %207 = load i64, i64* %14, align 8
  %208 = call nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAD_K@Z"(%"class.std::basic_string"* %17, i64 0) #5
  %209 = bitcast %"class.std::ostreambuf_iterator"* %28 to i8*
  %210 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %209, i8* align 8 %210, i64 16, i1 false)
  invoke void @"?_Put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@PEBD_K@Z"(%"class.std::num_put"* %39, %"class.std::ostreambuf_iterator"* sret align 8 %27, %"class.std::ostreambuf_iterator"* %28, i8* %208, i64 %207)
          to label %211 unwind label %260

211:                                              ; preds = %204
  %212 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  %213 = bitcast %"class.std::ostreambuf_iterator"* %27 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %212, i8* align 8 %213, i64 16, i1 false)
  br label %241

214:                                              ; preds = %196, %192
  %215 = load i32, i32* %24, align 4
  %216 = icmp eq i32 %215, 256
  br i1 %216, label %217, label %232

217:                                              ; preds = %214
  %218 = load i64, i64* %14, align 8
  %219 = call nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAD_K@Z"(%"class.std::basic_string"* %17, i64 0) #5
  %220 = bitcast %"class.std::ostreambuf_iterator"* %30 to i8*
  %221 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %220, i8* align 8 %221, i64 16, i1 false)
  invoke void @"?_Put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@PEBD_K@Z"(%"class.std::num_put"* %39, %"class.std::ostreambuf_iterator"* sret align 8 %29, %"class.std::ostreambuf_iterator"* %30, i8* %219, i64 %218)
          to label %222 unwind label %260

222:                                              ; preds = %217
  %223 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  %224 = bitcast %"class.std::ostreambuf_iterator"* %29 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %223, i8* align 8 %224, i64 16, i1 false)
  %225 = load i64, i64* %23, align 8
  %226 = load i8, i8* %11, align 1
  %227 = bitcast %"class.std::ostreambuf_iterator"* %32 to i8*
  %228 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %227, i8* align 8 %228, i64 16, i1 false)
  invoke void @"?_Rep@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@D_K@Z"(%"class.std::num_put"* %39, %"class.std::ostreambuf_iterator"* sret align 8 %31, %"class.std::ostreambuf_iterator"* %32, i8 %226, i64 %225)
          to label %229 unwind label %260

229:                                              ; preds = %222
  %230 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  %231 = bitcast %"class.std::ostreambuf_iterator"* %31 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %230, i8* align 8 %231, i64 16, i1 false)
  store i64 0, i64* %23, align 8
  br label %240

232:                                              ; preds = %214
  %233 = load i64, i64* %14, align 8
  %234 = call nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAD_K@Z"(%"class.std::basic_string"* %17, i64 0) #5
  %235 = bitcast %"class.std::ostreambuf_iterator"* %34 to i8*
  %236 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %235, i8* align 8 %236, i64 16, i1 false)
  invoke void @"?_Put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@PEBD_K@Z"(%"class.std::num_put"* %39, %"class.std::ostreambuf_iterator"* sret align 8 %33, %"class.std::ostreambuf_iterator"* %34, i8* %234, i64 %233)
          to label %237 unwind label %260

237:                                              ; preds = %232
  %238 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  %239 = bitcast %"class.std::ostreambuf_iterator"* %33 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %238, i8* align 8 %239, i64 16, i1 false)
  br label %240

240:                                              ; preds = %237, %229
  br label %241

241:                                              ; preds = %240, %211
  %242 = load i64, i64* %9, align 8
  %243 = load i64, i64* %14, align 8
  %244 = sub i64 %242, %243
  %245 = load i64, i64* %14, align 8
  %246 = call nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAD_K@Z"(%"class.std::basic_string"* %17, i64 %245) #5
  %247 = bitcast %"class.std::ostreambuf_iterator"* %36 to i8*
  %248 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %247, i8* align 8 %248, i64 16, i1 false)
  invoke void @"?_Put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@PEBD_K@Z"(%"class.std::num_put"* %39, %"class.std::ostreambuf_iterator"* sret align 8 %35, %"class.std::ostreambuf_iterator"* %36, i8* %246, i64 %244)
          to label %249 unwind label %260

249:                                              ; preds = %241
  %250 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  %251 = bitcast %"class.std::ostreambuf_iterator"* %35 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %250, i8* align 8 %251, i64 16, i1 false)
  %252 = load %"class.std::ios_base"*, %"class.std::ios_base"** %12, align 8
  %253 = invoke i64 @"?width@ios_base@std@@QEAA_J_J@Z"(%"class.std::ios_base"* %252, i64 0)
          to label %254 unwind label %260

254:                                              ; preds = %249
  %255 = load i64, i64* %23, align 8
  %256 = load i8, i8* %11, align 1
  %257 = bitcast %"class.std::ostreambuf_iterator"* %37 to i8*
  %258 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %257, i8* align 8 %258, i64 16, i1 false)
  invoke void @"?_Rep@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@D_K@Z"(%"class.std::num_put"* %39, %"class.std::ostreambuf_iterator"* sret align 8 %1, %"class.std::ostreambuf_iterator"* %37, i8 %256, i64 %255)
          to label %259 unwind label %260

259:                                              ; preds = %254
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %20) #5
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %17) #5
  ret void

260:                                              ; preds = %254, %249, %241, %232, %222, %217, %204, %199, %189, %183, %176, %170, %146, %122
  %261 = cleanuppad within none []
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %20) #5 [ "funclet"(token %261) ]
  cleanupret from %261 unwind label %262

262:                                              ; preds = %260, %109, %166, %105, %95
  %263 = cleanuppad within none []
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %17) #5 [ "funclet"(token %263) ]
  cleanupret from %263 unwind to caller
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i32 @sprintf_s(i8* %0, i64 %1, i8* %2, ...) #1 comdat {
  %4 = alloca i8*, align 8
  %5 = alloca i64, align 8
  %6 = alloca i8*, align 8
  %7 = alloca i32, align 4
  %8 = alloca i8*, align 8
  store i8* %2, i8** %4, align 8
  store i64 %1, i64* %5, align 8
  store i8* %0, i8** %6, align 8
  %9 = bitcast i8** %8 to i8*
  call void @llvm.va_start(i8* %9)
  %10 = load i8*, i8** %8, align 8
  %11 = load i8*, i8** %4, align 8
  %12 = load i64, i64* %5, align 8
  %13 = load i8*, i8** %6, align 8
  %14 = call i32 @_vsprintf_s_l(i8* %13, i64 %12, i8* %11, %struct.__crt_locale_pointers* null, i8* %10)
  store i32 %14, i32* %7, align 4
  %15 = bitcast i8** %8 to i8*
  call void @llvm.va_end(i8* %15)
  %16 = load i32, i32* %7, align 4
  ret i32 %16
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(48) %"class.std::ctype"* @"??$use_facet@V?$ctype@D@std@@@std@@YAAEBV?$ctype@D@0@AEBVlocale@0@@Z"(%"class.std::locale"* nonnull align 8 dereferenceable(16) %0) #1 comdat personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %2 = alloca %"class.std::locale"*, align 8
  %3 = alloca %"class.std::_Lockit", align 4
  %4 = alloca %"class.std::locale::facet"*, align 8
  %5 = alloca i64, align 8
  %6 = alloca %"class.std::locale::facet"*, align 8
  %7 = alloca %"class.std::locale::facet"*, align 8
  %8 = alloca %"class.std::unique_ptr", align 8
  store %"class.std::locale"* %0, %"class.std::locale"** %2, align 8
  %9 = call %"class.std::_Lockit"* @"??0_Lockit@std@@QEAA@H@Z"(%"class.std::_Lockit"* %3, i32 0) #5
  %10 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** @"?_Psave@?$_Facetptr@V?$ctype@D@std@@@std@@2PEBVfacet@locale@2@EB", align 8
  store %"class.std::locale::facet"* %10, %"class.std::locale::facet"** %4, align 8
  %11 = invoke i64 @"??Bid@locale@std@@QEAA_KXZ"(%"class.std::locale::id"* @"?id@?$ctype@D@std@@2V0locale@2@A")
          to label %12 unwind label %54

12:                                               ; preds = %1
  store i64 %11, i64* %5, align 8
  %13 = load %"class.std::locale"*, %"class.std::locale"** %2, align 8
  %14 = load i64, i64* %5, align 8
  %15 = invoke %"class.std::locale::facet"* @"?_Getfacet@locale@std@@QEBAPEBVfacet@12@_K@Z"(%"class.std::locale"* %13, i64 %14)
          to label %16 unwind label %54

16:                                               ; preds = %12
  store %"class.std::locale::facet"* %15, %"class.std::locale::facet"** %6, align 8
  %17 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %6, align 8
  %18 = icmp ne %"class.std::locale::facet"* %17, null
  br i1 %18, label %51, label %19

19:                                               ; preds = %16
  %20 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %4, align 8
  %21 = icmp ne %"class.std::locale::facet"* %20, null
  br i1 %21, label %22, label %24

22:                                               ; preds = %19
  %23 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %4, align 8
  store %"class.std::locale::facet"* %23, %"class.std::locale::facet"** %6, align 8
  br label %50

24:                                               ; preds = %19
  %25 = load %"class.std::locale"*, %"class.std::locale"** %2, align 8
  %26 = invoke i64 @"?_Getcat@?$ctype@D@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z"(%"class.std::locale::facet"** %4, %"class.std::locale"* %25)
          to label %27 unwind label %54

27:                                               ; preds = %24
  %28 = icmp eq i64 %26, -1
  br i1 %28, label %29, label %31

29:                                               ; preds = %27
  invoke void @"?_Throw_bad_cast@std@@YAXXZ"() #19
          to label %30 unwind label %54

30:                                               ; preds = %29
  unreachable

31:                                               ; preds = %27
  %32 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %4, align 8
  store %"class.std::locale::facet"* %32, %"class.std::locale::facet"** %7, align 8
  %33 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %7, align 8
  %34 = bitcast %"class.std::locale::facet"* %33 to %"class.std::_Facet_base"*
  %35 = call %"class.std::unique_ptr"* @"??$?0U?$default_delete@V_Facet_base@std@@@std@@$0A@@?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAA@PEAV_Facet_base@1@@Z"(%"class.std::unique_ptr"* %8, %"class.std::_Facet_base"* %34) #5
  %36 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %7, align 8
  %37 = bitcast %"class.std::locale::facet"* %36 to %"class.std::_Facet_base"*
  invoke void @"?_Facet_Register@std@@YAXPEAV_Facet_base@1@@Z"(%"class.std::_Facet_base"* %37)
          to label %38 unwind label %47

38:                                               ; preds = %31
  %39 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %7, align 8
  %40 = bitcast %"class.std::locale::facet"* %39 to void (%"class.std::locale::facet"*)***
  %41 = load void (%"class.std::locale::facet"*)**, void (%"class.std::locale::facet"*)*** %40, align 8
  %42 = getelementptr inbounds void (%"class.std::locale::facet"*)*, void (%"class.std::locale::facet"*)** %41, i64 1
  %43 = load void (%"class.std::locale::facet"*)*, void (%"class.std::locale::facet"*)** %42, align 8
  call void %43(%"class.std::locale::facet"* %39) #5
  %44 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %4, align 8
  store %"class.std::locale::facet"* %44, %"class.std::locale::facet"** @"?_Psave@?$_Facetptr@V?$ctype@D@std@@@std@@2PEBVfacet@locale@2@EB", align 8
  %45 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %4, align 8
  store %"class.std::locale::facet"* %45, %"class.std::locale::facet"** %6, align 8
  %46 = call %"class.std::_Facet_base"* @"?release@?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAAPEAV_Facet_base@2@XZ"(%"class.std::unique_ptr"* %8) #5
  call void @"??1?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAA@XZ"(%"class.std::unique_ptr"* %8) #5
  br label %49

47:                                               ; preds = %31
  %48 = cleanuppad within none []
  call void @"??1?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAA@XZ"(%"class.std::unique_ptr"* %8) #5 [ "funclet"(token %48) ]
  cleanupret from %48 unwind label %54

49:                                               ; preds = %38
  br label %50

50:                                               ; preds = %49, %22
  br label %51

51:                                               ; preds = %50, %16
  %52 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %6, align 8
  %53 = bitcast %"class.std::locale::facet"* %52 to %"class.std::ctype"*
  call void @"??1_Lockit@std@@QEAA@XZ"(%"class.std::_Lockit"* %3) #5
  ret %"class.std::ctype"* %53

54:                                               ; preds = %47, %29, %24, %12, %1
  %55 = cleanuppad within none []
  call void @"??1_Lockit@std@@QEAA@XZ"(%"class.std::_Lockit"* %3) #5 [ "funclet"(token %55) ]
  cleanupret from %55 unwind to caller
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@_KD@Z"(%"class.std::basic_string"* returned %0, i64 %1, i8 %2) unnamed_addr #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %4 = alloca i8, align 1
  %5 = alloca i64, align 8
  %6 = alloca %"class.std::basic_string"*, align 8
  %7 = alloca %"struct.std::_Zero_then_variadic_args_t", align 1
  %8 = alloca %"struct.std::_Fake_allocator"*, align 8
  %9 = alloca %"struct.std::_Fake_proxy_ptr_impl", align 1
  store i8 %2, i8* %4, align 1
  store i64 %1, i64* %5, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %6, align 8
  %10 = load %"class.std::basic_string"*, %"class.std::basic_string"** %6, align 8
  %11 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %10, i32 0, i32 0
  %12 = getelementptr inbounds %"struct.std::_Zero_then_variadic_args_t", %"struct.std::_Zero_then_variadic_args_t"* %7, i32 0, i32 0
  %13 = load i8, i8* %12, align 1
  %14 = call %"class.std::_Compressed_pair"* @"??$?0$$V@?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@U_Zero_then_variadic_args_t@1@@Z"(%"class.std::_Compressed_pair"* %11, i8 %13) #5
  store %"struct.std::_Fake_allocator"* @"?_Fake_alloc@std@@3U_Fake_allocator@1@B", %"struct.std::_Fake_allocator"** %8, align 8
  %15 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %10, i32 0, i32 0
  %16 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %15, i32 0, i32 0
  %17 = bitcast %"class.std::_String_val"* %16 to %"struct.std::_Container_base0"*
  %18 = call %"struct.std::_Fake_proxy_ptr_impl"* @"??0_Fake_proxy_ptr_impl@std@@QEAA@AEBU_Fake_allocator@1@AEBU_Container_base0@1@@Z"(%"struct.std::_Fake_proxy_ptr_impl"* %9, %"struct.std::_Fake_allocator"* nonnull align 1 dereferenceable(1) @"?_Fake_alloc@std@@3U_Fake_allocator@1@B", %"struct.std::_Container_base0"* nonnull align 1 dereferenceable(1) %17) #5
  call void @"?_Tidy_init@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXXZ"(%"class.std::basic_string"* %10) #5
  %19 = load i8, i8* %4, align 1
  %20 = load i64, i64* %5, align 8
  %21 = invoke nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@_KD@Z"(%"class.std::basic_string"* %10, i64 %20, i8 %19)
          to label %22 unwind label %23

22:                                               ; preds = %3
  call void @"?_Release@_Fake_proxy_ptr_impl@std@@QEAAXXZ"(%"struct.std::_Fake_proxy_ptr_impl"* %9) #5
  ret %"class.std::basic_string"* %10

23:                                               ; preds = %3
  %24 = cleanuppad within none []
  call void @"??1?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@XZ"(%"class.std::_Compressed_pair"* %11) #5 [ "funclet"(token %24) ]
  cleanupret from %24 unwind to caller
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i8* @"?widen@?$ctype@D@std@@QEBAPEBDPEBD0PEAD@Z"(%"class.std::ctype"* %0, i8* %1, i8* %2, i8* %3) #1 comdat align 2 {
  %5 = alloca i8*, align 8
  %6 = alloca i8*, align 8
  %7 = alloca i8*, align 8
  %8 = alloca %"class.std::ctype"*, align 8
  store i8* %3, i8** %5, align 8
  store i8* %2, i8** %6, align 8
  store i8* %1, i8** %7, align 8
  store %"class.std::ctype"* %0, %"class.std::ctype"** %8, align 8
  %9 = load %"class.std::ctype"*, %"class.std::ctype"** %8, align 8
  %10 = load i8*, i8** %5, align 8
  %11 = load i8*, i8** %6, align 8
  %12 = load i8*, i8** %7, align 8
  %13 = bitcast %"class.std::ctype"* %9 to i8* (%"class.std::ctype"*, i8*, i8*, i8*)***
  %14 = load i8* (%"class.std::ctype"*, i8*, i8*, i8*)**, i8* (%"class.std::ctype"*, i8*, i8*, i8*)*** %13, align 8
  %15 = getelementptr inbounds i8* (%"class.std::ctype"*, i8*, i8*, i8*)*, i8* (%"class.std::ctype"*, i8*, i8*, i8*)** %14, i64 7
  %16 = load i8* (%"class.std::ctype"*, i8*, i8*, i8*)*, i8* (%"class.std::ctype"*, i8*, i8*, i8*)** %15, align 8
  %17 = call i8* %16(%"class.std::ctype"* %9, i8* %12, i8* %11, i8* %10)
  ret i8* %17
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAD_K@Z"(%"class.std::basic_string"* %0, i64 %1) #3 comdat align 2 {
  %3 = alloca i64, align 8
  %4 = alloca %"class.std::basic_string"*, align 8
  store i64 %1, i64* %3, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %4, align 8
  %5 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  %6 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %5, i32 0, i32 0
  %7 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %6, i32 0, i32 0
  %8 = call i8* @"?_Myptr@?$_String_val@U?$_Simple_types@D@std@@@std@@QEAAPEADXZ"(%"class.std::_String_val"* %7) #5
  %9 = load i64, i64* %3, align 8
  %10 = getelementptr inbounds i8, i8* %8, i64 %9
  ret i8* %10
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(48) %"class.std::numpunct"* @"??$use_facet@V?$numpunct@D@std@@@std@@YAAEBV?$numpunct@D@0@AEBVlocale@0@@Z"(%"class.std::locale"* nonnull align 8 dereferenceable(16) %0) #1 comdat personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %2 = alloca %"class.std::locale"*, align 8
  %3 = alloca %"class.std::_Lockit", align 4
  %4 = alloca %"class.std::locale::facet"*, align 8
  %5 = alloca i64, align 8
  %6 = alloca %"class.std::locale::facet"*, align 8
  %7 = alloca %"class.std::locale::facet"*, align 8
  %8 = alloca %"class.std::unique_ptr", align 8
  store %"class.std::locale"* %0, %"class.std::locale"** %2, align 8
  %9 = call %"class.std::_Lockit"* @"??0_Lockit@std@@QEAA@H@Z"(%"class.std::_Lockit"* %3, i32 0) #5
  %10 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** @"?_Psave@?$_Facetptr@V?$numpunct@D@std@@@std@@2PEBVfacet@locale@2@EB", align 8
  store %"class.std::locale::facet"* %10, %"class.std::locale::facet"** %4, align 8
  %11 = invoke i64 @"??Bid@locale@std@@QEAA_KXZ"(%"class.std::locale::id"* @"?id@?$numpunct@D@std@@2V0locale@2@A")
          to label %12 unwind label %54

12:                                               ; preds = %1
  store i64 %11, i64* %5, align 8
  %13 = load %"class.std::locale"*, %"class.std::locale"** %2, align 8
  %14 = load i64, i64* %5, align 8
  %15 = invoke %"class.std::locale::facet"* @"?_Getfacet@locale@std@@QEBAPEBVfacet@12@_K@Z"(%"class.std::locale"* %13, i64 %14)
          to label %16 unwind label %54

16:                                               ; preds = %12
  store %"class.std::locale::facet"* %15, %"class.std::locale::facet"** %6, align 8
  %17 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %6, align 8
  %18 = icmp ne %"class.std::locale::facet"* %17, null
  br i1 %18, label %51, label %19

19:                                               ; preds = %16
  %20 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %4, align 8
  %21 = icmp ne %"class.std::locale::facet"* %20, null
  br i1 %21, label %22, label %24

22:                                               ; preds = %19
  %23 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %4, align 8
  store %"class.std::locale::facet"* %23, %"class.std::locale::facet"** %6, align 8
  br label %50

24:                                               ; preds = %19
  %25 = load %"class.std::locale"*, %"class.std::locale"** %2, align 8
  %26 = invoke i64 @"?_Getcat@?$numpunct@D@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z"(%"class.std::locale::facet"** %4, %"class.std::locale"* %25)
          to label %27 unwind label %54

27:                                               ; preds = %24
  %28 = icmp eq i64 %26, -1
  br i1 %28, label %29, label %31

29:                                               ; preds = %27
  invoke void @"?_Throw_bad_cast@std@@YAXXZ"() #19
          to label %30 unwind label %54

30:                                               ; preds = %29
  unreachable

31:                                               ; preds = %27
  %32 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %4, align 8
  store %"class.std::locale::facet"* %32, %"class.std::locale::facet"** %7, align 8
  %33 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %7, align 8
  %34 = bitcast %"class.std::locale::facet"* %33 to %"class.std::_Facet_base"*
  %35 = call %"class.std::unique_ptr"* @"??$?0U?$default_delete@V_Facet_base@std@@@std@@$0A@@?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAA@PEAV_Facet_base@1@@Z"(%"class.std::unique_ptr"* %8, %"class.std::_Facet_base"* %34) #5
  %36 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %7, align 8
  %37 = bitcast %"class.std::locale::facet"* %36 to %"class.std::_Facet_base"*
  invoke void @"?_Facet_Register@std@@YAXPEAV_Facet_base@1@@Z"(%"class.std::_Facet_base"* %37)
          to label %38 unwind label %47

38:                                               ; preds = %31
  %39 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %7, align 8
  %40 = bitcast %"class.std::locale::facet"* %39 to void (%"class.std::locale::facet"*)***
  %41 = load void (%"class.std::locale::facet"*)**, void (%"class.std::locale::facet"*)*** %40, align 8
  %42 = getelementptr inbounds void (%"class.std::locale::facet"*)*, void (%"class.std::locale::facet"*)** %41, i64 1
  %43 = load void (%"class.std::locale::facet"*)*, void (%"class.std::locale::facet"*)** %42, align 8
  call void %43(%"class.std::locale::facet"* %39) #5
  %44 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %4, align 8
  store %"class.std::locale::facet"* %44, %"class.std::locale::facet"** @"?_Psave@?$_Facetptr@V?$numpunct@D@std@@@std@@2PEBVfacet@locale@2@EB", align 8
  %45 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %4, align 8
  store %"class.std::locale::facet"* %45, %"class.std::locale::facet"** %6, align 8
  %46 = call %"class.std::_Facet_base"* @"?release@?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAAPEAV_Facet_base@2@XZ"(%"class.std::unique_ptr"* %8) #5
  call void @"??1?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAA@XZ"(%"class.std::unique_ptr"* %8) #5
  br label %49

47:                                               ; preds = %31
  %48 = cleanuppad within none []
  call void @"??1?$unique_ptr@V_Facet_base@std@@U?$default_delete@V_Facet_base@std@@@2@@std@@QEAA@XZ"(%"class.std::unique_ptr"* %8) #5 [ "funclet"(token %48) ]
  cleanupret from %48 unwind label %54

49:                                               ; preds = %38
  br label %50

50:                                               ; preds = %49, %22
  br label %51

51:                                               ; preds = %50, %16
  %52 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %6, align 8
  %53 = bitcast %"class.std::locale::facet"* %52 to %"class.std::numpunct"*
  call void @"??1_Lockit@std@@QEAA@XZ"(%"class.std::_Lockit"* %3) #5
  ret %"class.std::numpunct"* %53

54:                                               ; preds = %47, %29, %24, %12, %1
  %55 = cleanuppad within none []
  call void @"??1_Lockit@std@@QEAA@XZ"(%"class.std::_Lockit"* %3) #5 [ "funclet"(token %55) ]
  cleanupret from %55 unwind to caller
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?grouping@?$numpunct@D@std@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ"(%"class.std::numpunct"* %0, %"class.std::basic_string"* noalias sret align 8 %1) #1 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::numpunct"*, align 8
  %5 = bitcast %"class.std::basic_string"* %1 to i8*
  store i8* %5, i8** %3, align 8
  store %"class.std::numpunct"* %0, %"class.std::numpunct"** %4, align 8
  %6 = load %"class.std::numpunct"*, %"class.std::numpunct"** %4, align 8
  %7 = bitcast %"class.std::numpunct"* %6 to void (%"class.std::numpunct"*, %"class.std::basic_string"*)***
  %8 = load void (%"class.std::numpunct"*, %"class.std::basic_string"*)**, void (%"class.std::numpunct"*, %"class.std::basic_string"*)*** %7, align 8
  %9 = getelementptr inbounds void (%"class.std::numpunct"*, %"class.std::basic_string"*)*, void (%"class.std::numpunct"*, %"class.std::basic_string"*)** %8, i64 5
  %10 = load void (%"class.std::numpunct"*, %"class.std::basic_string"*)*, void (%"class.std::numpunct"*, %"class.std::basic_string"*)** %9, align 8
  call void %10(%"class.std::numpunct"* %6, %"class.std::basic_string"* sret align 8 %1)
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBAAEBD_K@Z"(%"class.std::basic_string"* %0, i64 %1) #3 comdat align 2 {
  %3 = alloca i64, align 8
  %4 = alloca %"class.std::basic_string"*, align 8
  store i64 %1, i64* %3, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %4, align 8
  %5 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  %6 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %5, i32 0, i32 0
  %7 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %6, i32 0, i32 0
  %8 = call i8* @"?_Myptr@?$_String_val@U?$_Simple_types@D@std@@@std@@QEBAPEBDXZ"(%"class.std::_String_val"* %7) #5
  %9 = load i64, i64* %3, align 8
  %10 = getelementptr inbounds i8, i8* %8, i64 %9
  ret i8* %10
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i8 @"?thousands_sep@?$numpunct@D@std@@QEBADXZ"(%"class.std::numpunct"* %0) #1 comdat align 2 {
  %2 = alloca %"class.std::numpunct"*, align 8
  store %"class.std::numpunct"* %0, %"class.std::numpunct"** %2, align 8
  %3 = load %"class.std::numpunct"*, %"class.std::numpunct"** %2, align 8
  %4 = bitcast %"class.std::numpunct"* %3 to i8 (%"class.std::numpunct"*)***
  %5 = load i8 (%"class.std::numpunct"*)**, i8 (%"class.std::numpunct"*)*** %4, align 8
  %6 = getelementptr inbounds i8 (%"class.std::numpunct"*)*, i8 (%"class.std::numpunct"*)** %5, i64 4
  %7 = load i8 (%"class.std::numpunct"*)*, i8 (%"class.std::numpunct"*)** %6, align 8
  %8 = call i8 %7(%"class.std::numpunct"* %3)
  ret i8 %8
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?insert@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@_K0D@Z"(%"class.std::basic_string"* %0, i64 %1, i64 %2, i8 %3) #1 comdat align 2 {
  %5 = alloca %"class.std::basic_string"*, align 8
  %6 = alloca i8, align 1
  %7 = alloca i64, align 8
  %8 = alloca i64, align 8
  %9 = alloca %"class.std::basic_string"*, align 8
  %10 = alloca i64, align 8
  %11 = alloca i8*, align 8
  %12 = alloca i8*, align 8
  %13 = alloca %class.anon.6, align 1
  store i8 %3, i8* %6, align 1
  store i64 %2, i64* %7, align 8
  store i64 %1, i64* %8, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %9, align 8
  %14 = load %"class.std::basic_string"*, %"class.std::basic_string"** %9, align 8
  %15 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %16 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %15, i32 0, i32 0
  %17 = load i64, i64* %8, align 8
  call void @"?_Check_offset@?$_String_val@U?$_Simple_types@D@std@@@std@@QEBAX_K@Z"(%"class.std::_String_val"* %16, i64 %17)
  %18 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %19 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %18, i32 0, i32 0
  %20 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %19, i32 0, i32 1
  %21 = load i64, i64* %20, align 8
  store i64 %21, i64* %10, align 8
  %22 = load i64, i64* %7, align 8
  %23 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %24 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %23, i32 0, i32 0
  %25 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %24, i32 0, i32 2
  %26 = load i64, i64* %25, align 8
  %27 = load i64, i64* %10, align 8
  %28 = sub i64 %26, %27
  %29 = icmp ule i64 %22, %28
  br i1 %29, label %30, label %56

30:                                               ; preds = %4
  %31 = load i64, i64* %10, align 8
  %32 = load i64, i64* %7, align 8
  %33 = add i64 %31, %32
  %34 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %35 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %34, i32 0, i32 0
  %36 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %35, i32 0, i32 1
  store i64 %33, i64* %36, align 8
  %37 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %38 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %37, i32 0, i32 0
  %39 = call i8* @"?_Myptr@?$_String_val@U?$_Simple_types@D@std@@@std@@QEAAPEADXZ"(%"class.std::_String_val"* %38) #5
  store i8* %39, i8** %11, align 8
  %40 = load i8*, i8** %11, align 8
  %41 = load i64, i64* %8, align 8
  %42 = getelementptr inbounds i8, i8* %40, i64 %41
  store i8* %42, i8** %12, align 8
  %43 = load i64, i64* %10, align 8
  %44 = load i64, i64* %8, align 8
  %45 = sub i64 %43, %44
  %46 = add i64 %45, 1
  %47 = load i8*, i8** %12, align 8
  %48 = load i8*, i8** %12, align 8
  %49 = load i64, i64* %7, align 8
  %50 = getelementptr inbounds i8, i8* %48, i64 %49
  %51 = call i8* @"?move@?$_Char_traits@DH@std@@SAPEADQEADQEBD_K@Z"(i8* %50, i8* %47, i64 %46) #5
  %52 = load i8, i8* %6, align 1
  %53 = load i64, i64* %7, align 8
  %54 = load i8*, i8** %12, align 8
  %55 = call i8* @"?assign@?$_Narrow_char_traits@DH@std@@SAPEADQEAD_KD@Z"(i8* %54, i64 %53, i8 %52) #5
  store %"class.std::basic_string"* %14, %"class.std::basic_string"** %5, align 8
  br label %64

56:                                               ; preds = %4
  %57 = load i8, i8* %6, align 1
  %58 = load i64, i64* %7, align 8
  %59 = load i64, i64* %8, align 8
  %60 = load i64, i64* %7, align 8
  %61 = getelementptr inbounds %class.anon.6, %class.anon.6* %13, i32 0, i32 0
  %62 = load i8, i8* %61, align 1
  %63 = call nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"??$_Reallocate_grow_by@V<lambda_1>@?0??insert@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV34@_K0D@Z@_K_KD@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV01@_KV<lambda_1>@?0??insert@01@QEAAAEAV01@00D@Z@_K2D@Z"(%"class.std::basic_string"* %14, i64 %60, i8 %62, i64 %59, i64 %58, i8 %57)
  store %"class.std::basic_string"* %63, %"class.std::basic_string"** %5, align 8
  br label %64

64:                                               ; preds = %56, %30
  %65 = load %"class.std::basic_string"*, %"class.std::basic_string"** %5, align 8
  ret %"class.std::basic_string"* %65
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?_Rep@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@D_K@Z"(%"class.std::num_put"* %0, %"class.std::ostreambuf_iterator"* noalias sret align 8 %1, %"class.std::ostreambuf_iterator"* %2, i8 %3, i64 %4) #1 comdat align 2 {
  %6 = alloca i8*, align 8
  %7 = alloca i64, align 8
  %8 = alloca i8, align 1
  %9 = alloca %"class.std::num_put"*, align 8
  %10 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  store i8* %10, i8** %6, align 8
  store i64 %4, i64* %7, align 8
  store i8 %3, i8* %8, align 1
  store %"class.std::num_put"* %0, %"class.std::num_put"** %9, align 8
  %11 = load %"class.std::num_put"*, %"class.std::num_put"** %9, align 8
  br label %12

12:                                               ; preds = %19, %5
  %13 = load i64, i64* %7, align 8
  %14 = icmp ult i64 0, %13
  br i1 %14, label %15, label %23

15:                                               ; preds = %12
  %16 = load i8, i8* %8, align 1
  %17 = call nonnull align 8 dereferenceable(16) %"class.std::ostreambuf_iterator"* @"??D?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAAAEAV01@XZ"(%"class.std::ostreambuf_iterator"* %2) #5
  %18 = call nonnull align 8 dereferenceable(16) %"class.std::ostreambuf_iterator"* @"??4?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAAAEAV01@D@Z"(%"class.std::ostreambuf_iterator"* %17, i8 %16)
  br label %19

19:                                               ; preds = %15
  %20 = load i64, i64* %7, align 8
  %21 = add i64 %20, -1
  store i64 %21, i64* %7, align 8
  %22 = call nonnull align 8 dereferenceable(16) %"class.std::ostreambuf_iterator"* @"??E?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAAAEAV01@XZ"(%"class.std::ostreambuf_iterator"* %2) #5
  br label %12

23:                                               ; preds = %12
  %24 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  %25 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %24, i8* align 8 %25, i64 16, i1 false)
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?_Put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@PEBD_K@Z"(%"class.std::num_put"* %0, %"class.std::ostreambuf_iterator"* noalias sret align 8 %1, %"class.std::ostreambuf_iterator"* %2, i8* %3, i64 %4) #1 comdat align 2 {
  %6 = alloca i8*, align 8
  %7 = alloca i64, align 8
  %8 = alloca i8*, align 8
  %9 = alloca %"class.std::num_put"*, align 8
  %10 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  store i8* %10, i8** %6, align 8
  store i64 %4, i64* %7, align 8
  store i8* %3, i8** %8, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %9, align 8
  %11 = load %"class.std::num_put"*, %"class.std::num_put"** %9, align 8
  br label %12

12:                                               ; preds = %20, %5
  %13 = load i64, i64* %7, align 8
  %14 = icmp ult i64 0, %13
  br i1 %14, label %15, label %26

15:                                               ; preds = %12
  %16 = load i8*, i8** %8, align 8
  %17 = load i8, i8* %16, align 1
  %18 = call nonnull align 8 dereferenceable(16) %"class.std::ostreambuf_iterator"* @"??D?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAAAEAV01@XZ"(%"class.std::ostreambuf_iterator"* %2) #5
  %19 = call nonnull align 8 dereferenceable(16) %"class.std::ostreambuf_iterator"* @"??4?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAAAEAV01@D@Z"(%"class.std::ostreambuf_iterator"* %18, i8 %17)
  br label %20

20:                                               ; preds = %15
  %21 = load i64, i64* %7, align 8
  %22 = add i64 %21, -1
  store i64 %22, i64* %7, align 8
  %23 = call nonnull align 8 dereferenceable(16) %"class.std::ostreambuf_iterator"* @"??E?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAAAEAV01@XZ"(%"class.std::ostreambuf_iterator"* %2) #5
  %24 = load i8*, i8** %8, align 8
  %25 = getelementptr inbounds i8, i8* %24, i32 1
  store i8* %25, i8** %8, align 8
  br label %12

26:                                               ; preds = %12
  %27 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  %28 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %27, i8* align 8 %28, i64 16, i1 false)
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i64 @"?_Getcat@?$ctype@D@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z"(%"class.std::locale::facet"** %0, %"class.std::locale"* %1) #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %3 = alloca %"class.std::locale"*, align 8
  %4 = alloca %"class.std::locale::facet"**, align 8
  %5 = alloca %"class.std::_Locinfo", align 8
  %6 = alloca i1, align 1
  store %"class.std::locale"* %1, %"class.std::locale"** %3, align 8
  store %"class.std::locale::facet"** %0, %"class.std::locale::facet"*** %4, align 8
  %7 = load %"class.std::locale::facet"**, %"class.std::locale::facet"*** %4, align 8
  %8 = icmp ne %"class.std::locale::facet"** %7, null
  br i1 %8, label %9, label %32

9:                                                ; preds = %2
  %10 = load %"class.std::locale::facet"**, %"class.std::locale::facet"*** %4, align 8
  %11 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %10, align 8
  %12 = icmp ne %"class.std::locale::facet"* %11, null
  br i1 %12, label %32, label %13

13:                                               ; preds = %9
  %14 = call noalias nonnull i8* @"??2@YAPEAX_K@Z"(i64 48) #22
  store i1 true, i1* %6, align 1
  %15 = bitcast i8* %14 to %"class.std::ctype"*
  %16 = load %"class.std::locale"*, %"class.std::locale"** %3, align 8
  %17 = invoke i8* @"?c_str@locale@std@@QEBAPEBDXZ"(%"class.std::locale"* %16)
          to label %18 unwind label %27

18:                                               ; preds = %13
  %19 = invoke %"class.std::_Locinfo"* @"??0_Locinfo@std@@QEAA@PEBD@Z"(%"class.std::_Locinfo"* %5, i8* %17)
          to label %20 unwind label %27

20:                                               ; preds = %18
  %21 = invoke %"class.std::ctype"* @"??0?$ctype@D@std@@QEAA@AEBV_Locinfo@1@_K@Z"(%"class.std::ctype"* %15, %"class.std::_Locinfo"* nonnull align 8 dereferenceable(104) %5, i64 0)
          to label %22 unwind label %25

22:                                               ; preds = %20
  store i1 false, i1* %6, align 1
  %23 = bitcast %"class.std::ctype"* %15 to %"class.std::locale::facet"*
  %24 = load %"class.std::locale::facet"**, %"class.std::locale::facet"*** %4, align 8
  store %"class.std::locale::facet"* %23, %"class.std::locale::facet"** %24, align 8
  call void @"??1_Locinfo@std@@QEAA@XZ"(%"class.std::_Locinfo"* %5) #5
  br label %32

25:                                               ; preds = %20
  %26 = cleanuppad within none []
  call void @"??1_Locinfo@std@@QEAA@XZ"(%"class.std::_Locinfo"* %5) #5 [ "funclet"(token %26) ]
  cleanupret from %26 unwind label %27

27:                                               ; preds = %25, %18, %13
  %28 = cleanuppad within none []
  %29 = load i1, i1* %6, align 1
  br i1 %29, label %30, label %31

30:                                               ; preds = %27
  call void @"??3@YAXPEAX@Z"(i8* %14) #20 [ "funclet"(token %28) ]
  br label %31

31:                                               ; preds = %30, %27
  cleanupret from %28 unwind to caller

32:                                               ; preds = %22, %9, %2
  ret i64 2
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local %"class.std::ctype"* @"??0?$ctype@D@std@@QEAA@AEBV_Locinfo@1@_K@Z"(%"class.std::ctype"* returned %0, %"class.std::_Locinfo"* nonnull align 8 dereferenceable(104) %1, i64 %2) unnamed_addr #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %4 = alloca i64, align 8
  %5 = alloca %"class.std::_Locinfo"*, align 8
  %6 = alloca %"class.std::ctype"*, align 8
  store i64 %2, i64* %4, align 8
  store %"class.std::_Locinfo"* %1, %"class.std::_Locinfo"** %5, align 8
  store %"class.std::ctype"* %0, %"class.std::ctype"** %6, align 8
  %7 = load %"class.std::ctype"*, %"class.std::ctype"** %6, align 8
  %8 = bitcast %"class.std::ctype"* %7 to %"struct.std::ctype_base"*
  %9 = load i64, i64* %4, align 8
  %10 = call %"struct.std::ctype_base"* @"??0ctype_base@std@@QEAA@_K@Z"(%"struct.std::ctype_base"* %8, i64 %9)
  %11 = bitcast %"class.std::ctype"* %7 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7?$ctype@D@std@@6B@" to i32 (...)**), i32 (...)*** %11, align 8
  %12 = getelementptr inbounds %"class.std::ctype", %"class.std::ctype"* %7, i32 0, i32 1
  %13 = load %"class.std::_Locinfo"*, %"class.std::_Locinfo"** %5, align 8
  invoke void @"?_Init@?$ctype@D@std@@IEAAXAEBV_Locinfo@2@@Z"(%"class.std::ctype"* %7, %"class.std::_Locinfo"* nonnull align 8 dereferenceable(104) %13)
          to label %14 unwind label %15

14:                                               ; preds = %3
  ret %"class.std::ctype"* %7

15:                                               ; preds = %3
  %16 = cleanuppad within none []
  %17 = bitcast %"class.std::ctype"* %7 to %"struct.std::ctype_base"*
  call void @"??1ctype_base@std@@UEAA@XZ"(%"struct.std::ctype_base"* %17) #5 [ "funclet"(token %16) ]
  cleanupret from %16 unwind to caller
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"struct.std::ctype_base"* @"??0ctype_base@std@@QEAA@_K@Z"(%"struct.std::ctype_base"* returned %0, i64 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i64, align 8
  %4 = alloca %"struct.std::ctype_base"*, align 8
  store i64 %1, i64* %3, align 8
  store %"struct.std::ctype_base"* %0, %"struct.std::ctype_base"** %4, align 8
  %5 = load %"struct.std::ctype_base"*, %"struct.std::ctype_base"** %4, align 8
  %6 = bitcast %"struct.std::ctype_base"* %5 to %"class.std::locale::facet"*
  %7 = load i64, i64* %3, align 8
  %8 = call %"class.std::locale::facet"* @"??0facet@locale@std@@IEAA@_K@Z"(%"class.std::locale::facet"* %6, i64 %7)
  %9 = bitcast %"struct.std::ctype_base"* %5 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7ctype_base@std@@6B@" to i32 (...)**), i32 (...)*** %9, align 8
  ret %"struct.std::ctype_base"* %5
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?_Init@?$ctype@D@std@@IEAAXAEBV_Locinfo@2@@Z"(%"class.std::ctype"* %0, %"class.std::_Locinfo"* nonnull align 8 dereferenceable(104) %1) #1 comdat align 2 {
  %3 = alloca %"class.std::_Locinfo"*, align 8
  %4 = alloca %"class.std::ctype"*, align 8
  %5 = alloca %struct._Ctypevec, align 8
  store %"class.std::_Locinfo"* %1, %"class.std::_Locinfo"** %3, align 8
  store %"class.std::ctype"* %0, %"class.std::ctype"** %4, align 8
  %6 = load %"class.std::ctype"*, %"class.std::ctype"** %4, align 8
  %7 = load %"class.std::_Locinfo"*, %"class.std::_Locinfo"** %3, align 8
  call void @"?_Getctype@_Locinfo@std@@QEBA?AU_Ctypevec@@XZ"(%"class.std::_Locinfo"* %7, %struct._Ctypevec* sret align 8 %5)
  %8 = getelementptr inbounds %"class.std::ctype", %"class.std::ctype"* %6, i32 0, i32 1
  %9 = bitcast %struct._Ctypevec* %8 to i8*
  %10 = bitcast %struct._Ctypevec* %5 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %9, i8* align 8 %10, i64 32, i1 false)
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1ctype_base@std@@UEAA@XZ"(%"struct.std::ctype_base"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"struct.std::ctype_base"*, align 8
  store %"struct.std::ctype_base"* %0, %"struct.std::ctype_base"** %2, align 8
  %3 = load %"struct.std::ctype_base"*, %"struct.std::ctype_base"** %2, align 8
  %4 = bitcast %"struct.std::ctype_base"* %3 to %"class.std::locale::facet"*
  call void @"??1facet@locale@std@@MEAA@XZ"(%"class.std::locale::facet"* %4) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??_G?$ctype@D@std@@MEAAPEAXI@Z"(%"class.std::ctype"* %0, i32 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca i32, align 4
  %5 = alloca %"class.std::ctype"*, align 8
  store i32 %1, i32* %4, align 4
  store %"class.std::ctype"* %0, %"class.std::ctype"** %5, align 8
  %6 = load %"class.std::ctype"*, %"class.std::ctype"** %5, align 8
  %7 = bitcast %"class.std::ctype"* %6 to i8*
  store i8* %7, i8** %3, align 8
  %8 = load i32, i32* %4, align 4
  call void @"??1?$ctype@D@std@@MEAA@XZ"(%"class.std::ctype"* %6) #5
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %12, label %10

10:                                               ; preds = %2
  %11 = bitcast %"class.std::ctype"* %6 to i8*
  call void @"??3@YAXPEAX@Z"(i8* %11) #20
  br label %12

12:                                               ; preds = %10, %2
  %13 = load i8*, i8** %3, align 8
  ret i8* %13
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i8* @"?do_tolower@?$ctype@D@std@@MEBAPEBDPEADPEBD@Z"(%"class.std::ctype"* %0, i8* %1, i8* %2) unnamed_addr #1 comdat align 2 {
  %4 = alloca i8*, align 8
  %5 = alloca i8*, align 8
  %6 = alloca %"class.std::ctype"*, align 8
  store i8* %2, i8** %4, align 8
  store i8* %1, i8** %5, align 8
  store %"class.std::ctype"* %0, %"class.std::ctype"** %6, align 8
  %7 = load %"class.std::ctype"*, %"class.std::ctype"** %6, align 8
  call void @"??$_Adl_verify_range@PEADPEBD@std@@YAXAEBQEADAEBQEBD@Z"(i8** nonnull align 8 dereferenceable(8) %5, i8** nonnull align 8 dereferenceable(8) %4)
  br label %8

8:                                                ; preds = %20, %3
  %9 = load i8*, i8** %5, align 8
  %10 = load i8*, i8** %4, align 8
  %11 = icmp ne i8* %9, %10
  br i1 %11, label %12, label %23

12:                                               ; preds = %8
  %13 = getelementptr inbounds %"class.std::ctype", %"class.std::ctype"* %7, i32 0, i32 1
  %14 = load i8*, i8** %5, align 8
  %15 = load i8, i8* %14, align 1
  %16 = zext i8 %15 to i32
  %17 = call i32 @_Tolower(i32 %16, %struct._Ctypevec* %13)
  %18 = trunc i32 %17 to i8
  %19 = load i8*, i8** %5, align 8
  store i8 %18, i8* %19, align 1
  br label %20

20:                                               ; preds = %12
  %21 = load i8*, i8** %5, align 8
  %22 = getelementptr inbounds i8, i8* %21, i32 1
  store i8* %22, i8** %5, align 8
  br label %8

23:                                               ; preds = %8
  %24 = load i8*, i8** %5, align 8
  ret i8* %24
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i8 @"?do_tolower@?$ctype@D@std@@MEBADD@Z"(%"class.std::ctype"* %0, i8 %1) unnamed_addr #1 comdat align 2 {
  %3 = alloca i8, align 1
  %4 = alloca %"class.std::ctype"*, align 8
  store i8 %1, i8* %3, align 1
  store %"class.std::ctype"* %0, %"class.std::ctype"** %4, align 8
  %5 = load %"class.std::ctype"*, %"class.std::ctype"** %4, align 8
  %6 = getelementptr inbounds %"class.std::ctype", %"class.std::ctype"* %5, i32 0, i32 1
  %7 = load i8, i8* %3, align 1
  %8 = zext i8 %7 to i32
  %9 = call i32 @_Tolower(i32 %8, %struct._Ctypevec* %6)
  %10 = trunc i32 %9 to i8
  ret i8 %10
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i8* @"?do_toupper@?$ctype@D@std@@MEBAPEBDPEADPEBD@Z"(%"class.std::ctype"* %0, i8* %1, i8* %2) unnamed_addr #1 comdat align 2 {
  %4 = alloca i8*, align 8
  %5 = alloca i8*, align 8
  %6 = alloca %"class.std::ctype"*, align 8
  store i8* %2, i8** %4, align 8
  store i8* %1, i8** %5, align 8
  store %"class.std::ctype"* %0, %"class.std::ctype"** %6, align 8
  %7 = load %"class.std::ctype"*, %"class.std::ctype"** %6, align 8
  call void @"??$_Adl_verify_range@PEADPEBD@std@@YAXAEBQEADAEBQEBD@Z"(i8** nonnull align 8 dereferenceable(8) %5, i8** nonnull align 8 dereferenceable(8) %4)
  br label %8

8:                                                ; preds = %20, %3
  %9 = load i8*, i8** %5, align 8
  %10 = load i8*, i8** %4, align 8
  %11 = icmp ne i8* %9, %10
  br i1 %11, label %12, label %23

12:                                               ; preds = %8
  %13 = getelementptr inbounds %"class.std::ctype", %"class.std::ctype"* %7, i32 0, i32 1
  %14 = load i8*, i8** %5, align 8
  %15 = load i8, i8* %14, align 1
  %16 = zext i8 %15 to i32
  %17 = call i32 @_Toupper(i32 %16, %struct._Ctypevec* %13)
  %18 = trunc i32 %17 to i8
  %19 = load i8*, i8** %5, align 8
  store i8 %18, i8* %19, align 1
  br label %20

20:                                               ; preds = %12
  %21 = load i8*, i8** %5, align 8
  %22 = getelementptr inbounds i8, i8* %21, i32 1
  store i8* %22, i8** %5, align 8
  br label %8

23:                                               ; preds = %8
  %24 = load i8*, i8** %5, align 8
  ret i8* %24
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i8 @"?do_toupper@?$ctype@D@std@@MEBADD@Z"(%"class.std::ctype"* %0, i8 %1) unnamed_addr #1 comdat align 2 {
  %3 = alloca i8, align 1
  %4 = alloca %"class.std::ctype"*, align 8
  store i8 %1, i8* %3, align 1
  store %"class.std::ctype"* %0, %"class.std::ctype"** %4, align 8
  %5 = load %"class.std::ctype"*, %"class.std::ctype"** %4, align 8
  %6 = getelementptr inbounds %"class.std::ctype", %"class.std::ctype"* %5, i32 0, i32 1
  %7 = load i8, i8* %3, align 1
  %8 = zext i8 %7 to i32
  %9 = call i32 @_Toupper(i32 %8, %struct._Ctypevec* %6)
  %10 = trunc i32 %9 to i8
  ret i8 %10
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i8* @"?do_widen@?$ctype@D@std@@MEBAPEBDPEBD0PEAD@Z"(%"class.std::ctype"* %0, i8* %1, i8* %2, i8* %3) unnamed_addr #1 comdat align 2 {
  %5 = alloca i8*, align 8
  %6 = alloca i8*, align 8
  %7 = alloca i8*, align 8
  %8 = alloca %"class.std::ctype"*, align 8
  store i8* %3, i8** %5, align 8
  store i8* %2, i8** %6, align 8
  store i8* %1, i8** %7, align 8
  store %"class.std::ctype"* %0, %"class.std::ctype"** %8, align 8
  %9 = load %"class.std::ctype"*, %"class.std::ctype"** %8, align 8
  call void @"??$_Adl_verify_range@PEBDPEBD@std@@YAXAEBQEBD0@Z"(i8** nonnull align 8 dereferenceable(8) %7, i8** nonnull align 8 dereferenceable(8) %6)
  %10 = load i8*, i8** %5, align 8
  %11 = load i8*, i8** %7, align 8
  %12 = load i8*, i8** %6, align 8
  %13 = load i8*, i8** %7, align 8
  %14 = ptrtoint i8* %12 to i64
  %15 = ptrtoint i8* %13 to i64
  %16 = sub i64 %14, %15
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %10, i8* align 1 %11, i64 %16, i1 false)
  %17 = load i8*, i8** %6, align 8
  ret i8* %17
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8 @"?do_widen@?$ctype@D@std@@MEBADD@Z"(%"class.std::ctype"* %0, i8 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8, align 1
  %4 = alloca %"class.std::ctype"*, align 8
  store i8 %1, i8* %3, align 1
  store %"class.std::ctype"* %0, %"class.std::ctype"** %4, align 8
  %5 = load %"class.std::ctype"*, %"class.std::ctype"** %4, align 8
  %6 = load i8, i8* %3, align 1
  ret i8 %6
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"?do_narrow@?$ctype@D@std@@MEBAPEBDPEBD0DPEAD@Z"(%"class.std::ctype"* %0, i8* %1, i8* %2, i8 %3, i8* %4) unnamed_addr #3 comdat align 2 {
  %6 = alloca i8*, align 8
  %7 = alloca i8, align 1
  %8 = alloca i8*, align 8
  %9 = alloca i8*, align 8
  %10 = alloca %"class.std::ctype"*, align 8
  store i8* %4, i8** %6, align 8
  store i8 %3, i8* %7, align 1
  store i8* %2, i8** %8, align 8
  store i8* %1, i8** %9, align 8
  store %"class.std::ctype"* %0, %"class.std::ctype"** %10, align 8
  %11 = load %"class.std::ctype"*, %"class.std::ctype"** %10, align 8
  call void @"??$_Adl_verify_range@PEBDPEBD@std@@YAXAEBQEBD0@Z"(i8** nonnull align 8 dereferenceable(8) %9, i8** nonnull align 8 dereferenceable(8) %8)
  %12 = load i8*, i8** %6, align 8
  %13 = load i8*, i8** %9, align 8
  %14 = load i8*, i8** %8, align 8
  %15 = load i8*, i8** %9, align 8
  %16 = ptrtoint i8* %14 to i64
  %17 = ptrtoint i8* %15 to i64
  %18 = sub i64 %16, %17
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %12, i8* align 1 %13, i64 %18, i1 false)
  %19 = load i8*, i8** %8, align 8
  ret i8* %19
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8 @"?do_narrow@?$ctype@D@std@@MEBADDD@Z"(%"class.std::ctype"* %0, i8 %1, i8 %2) unnamed_addr #3 comdat align 2 {
  %4 = alloca i8, align 1
  %5 = alloca i8, align 1
  %6 = alloca %"class.std::ctype"*, align 8
  store i8 %2, i8* %4, align 1
  store i8 %1, i8* %5, align 1
  store %"class.std::ctype"* %0, %"class.std::ctype"** %6, align 8
  %7 = load %"class.std::ctype"*, %"class.std::ctype"** %6, align 8
  %8 = load i8, i8* %5, align 1
  ret i8 %8
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??_Gctype_base@std@@UEAAPEAXI@Z"(%"struct.std::ctype_base"* %0, i32 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca i32, align 4
  %5 = alloca %"struct.std::ctype_base"*, align 8
  store i32 %1, i32* %4, align 4
  store %"struct.std::ctype_base"* %0, %"struct.std::ctype_base"** %5, align 8
  %6 = load %"struct.std::ctype_base"*, %"struct.std::ctype_base"** %5, align 8
  %7 = bitcast %"struct.std::ctype_base"* %6 to i8*
  store i8* %7, i8** %3, align 8
  %8 = load i32, i32* %4, align 4
  call void @"??1ctype_base@std@@UEAA@XZ"(%"struct.std::ctype_base"* %6) #5
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %12, label %10

10:                                               ; preds = %2
  %11 = bitcast %"struct.std::ctype_base"* %6 to i8*
  call void @"??3@YAXPEAX@Z"(i8* %11) #20
  br label %12

12:                                               ; preds = %10, %2
  %13 = load i8*, i8** %3, align 8
  ret i8* %13
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?_Getctype@_Locinfo@std@@QEBA?AU_Ctypevec@@XZ"(%"class.std::_Locinfo"* %0, %struct._Ctypevec* noalias sret align 8 %1) #1 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::_Locinfo"*, align 8
  %5 = bitcast %struct._Ctypevec* %1 to i8*
  store i8* %5, i8** %3, align 8
  store %"class.std::_Locinfo"* %0, %"class.std::_Locinfo"** %4, align 8
  %6 = load %"class.std::_Locinfo"*, %"class.std::_Locinfo"** %4, align 8
  call void @_Getctype(%struct._Ctypevec* sret align 8 %1)
  ret void
}

declare dso_local void @_Getctype(%struct._Ctypevec* sret align 8) #4

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1?$ctype@D@std@@MEAA@XZ"(%"class.std::ctype"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::ctype"*, align 8
  store %"class.std::ctype"* %0, %"class.std::ctype"** %2, align 8
  %3 = load %"class.std::ctype"*, %"class.std::ctype"** %2, align 8
  %4 = bitcast %"class.std::ctype"* %3 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7?$ctype@D@std@@6B@" to i32 (...)**), i32 (...)*** %4, align 8
  call void @"?_Tidy@?$ctype@D@std@@IEAAXXZ"(%"class.std::ctype"* %3) #5
  %5 = bitcast %"class.std::ctype"* %3 to %"struct.std::ctype_base"*
  call void @"??1ctype_base@std@@UEAA@XZ"(%"struct.std::ctype_base"* %5) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?_Tidy@?$ctype@D@std@@IEAAXXZ"(%"class.std::ctype"* %0) #3 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %2 = alloca %"class.std::ctype"*, align 8
  store %"class.std::ctype"* %0, %"class.std::ctype"** %2, align 8
  %3 = load %"class.std::ctype"*, %"class.std::ctype"** %2, align 8
  %4 = getelementptr inbounds %"class.std::ctype", %"class.std::ctype"* %3, i32 0, i32 1
  %5 = getelementptr inbounds %struct._Ctypevec, %struct._Ctypevec* %4, i32 0, i32 2
  %6 = load i32, i32* %5, align 8
  %7 = icmp slt i32 0, %6
  br i1 %7, label %8, label %14

8:                                                ; preds = %1
  %9 = getelementptr inbounds %"class.std::ctype", %"class.std::ctype"* %3, i32 0, i32 1
  %10 = getelementptr inbounds %struct._Ctypevec, %struct._Ctypevec* %9, i32 0, i32 1
  %11 = load i16*, i16** %10, align 8
  %12 = bitcast i16* %11 to i8*
  invoke void @free(i8* %12)
          to label %13 unwind label %34

13:                                               ; preds = %8
  br label %28

14:                                               ; preds = %1
  %15 = getelementptr inbounds %"class.std::ctype", %"class.std::ctype"* %3, i32 0, i32 1
  %16 = getelementptr inbounds %struct._Ctypevec, %struct._Ctypevec* %15, i32 0, i32 2
  %17 = load i32, i32* %16, align 8
  %18 = icmp slt i32 %17, 0
  br i1 %18, label %19, label %27

19:                                               ; preds = %14
  %20 = getelementptr inbounds %"class.std::ctype", %"class.std::ctype"* %3, i32 0, i32 1
  %21 = getelementptr inbounds %struct._Ctypevec, %struct._Ctypevec* %20, i32 0, i32 1
  %22 = load i16*, i16** %21, align 8
  %23 = icmp eq i16* %22, null
  br i1 %23, label %26, label %24

24:                                               ; preds = %19
  %25 = bitcast i16* %22 to i8*
  call void @"??_V@YAXPEAX@Z"(i8* %25) #20
  br label %26

26:                                               ; preds = %24, %19
  br label %27

27:                                               ; preds = %26, %14
  br label %28

28:                                               ; preds = %27, %13
  %29 = getelementptr inbounds %"class.std::ctype", %"class.std::ctype"* %3, i32 0, i32 1
  %30 = getelementptr inbounds %struct._Ctypevec, %struct._Ctypevec* %29, i32 0, i32 3
  %31 = load i16*, i16** %30, align 8
  %32 = bitcast i16* %31 to i8*
  invoke void @free(i8* %32)
          to label %33 unwind label %34

33:                                               ; preds = %28
  ret void

34:                                               ; preds = %28, %8
  %35 = cleanuppad within none []
  call void @__std_terminate() #18 [ "funclet"(token %35) ]
  unreachable
}

; Function Attrs: nobuiltin nounwind
declare dso_local void @"??_V@YAXPEAX@Z"(i8*) #6

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??$_Adl_verify_range@PEADPEBD@std@@YAXAEBQEADAEBQEBD@Z"(i8** nonnull align 8 dereferenceable(8) %0, i8** nonnull align 8 dereferenceable(8) %1) #3 comdat {
  %3 = alloca i8**, align 8
  %4 = alloca i8**, align 8
  store i8** %1, i8*** %3, align 8
  store i8** %0, i8*** %4, align 8
  ret void
}

declare dso_local i32 @_Tolower(i32, %struct._Ctypevec*) #4

declare dso_local i32 @_Toupper(i32, %struct._Ctypevec*) #4

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??$_Adl_verify_range@PEBDPEBD@std@@YAXAEBQEBD0@Z"(i8** nonnull align 8 dereferenceable(8) %0, i8** nonnull align 8 dereferenceable(8) %1) #3 comdat {
  %3 = alloca i8**, align 8
  %4 = alloca i8**, align 8
  store i8** %1, i8*** %3, align 8
  store i8** %0, i8*** %4, align 8
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@_KD@Z"(%"class.std::basic_string"* %0, i64 %1, i8 %2) #1 comdat align 2 {
  %4 = alloca %"class.std::basic_string"*, align 8
  %5 = alloca i8, align 1
  %6 = alloca i64, align 8
  %7 = alloca %"class.std::basic_string"*, align 8
  %8 = alloca i8*, align 8
  %9 = alloca i8, align 1
  %10 = alloca %class.anon.4, align 1
  store i8 %2, i8* %5, align 1
  store i64 %1, i64* %6, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %7, align 8
  %11 = load %"class.std::basic_string"*, %"class.std::basic_string"** %7, align 8
  %12 = load i64, i64* %6, align 8
  %13 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %11, i32 0, i32 0
  %14 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %13, i32 0, i32 0
  %15 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %14, i32 0, i32 2
  %16 = load i64, i64* %15, align 8
  %17 = icmp ule i64 %12, %16
  br i1 %17, label %18, label %33

18:                                               ; preds = %3
  %19 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %11, i32 0, i32 0
  %20 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %19, i32 0, i32 0
  %21 = call i8* @"?_Myptr@?$_String_val@U?$_Simple_types@D@std@@@std@@QEAAPEADXZ"(%"class.std::_String_val"* %20) #5
  store i8* %21, i8** %8, align 8
  %22 = load i64, i64* %6, align 8
  %23 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %11, i32 0, i32 0
  %24 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %23, i32 0, i32 0
  %25 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %24, i32 0, i32 1
  store i64 %22, i64* %25, align 8
  %26 = load i8, i8* %5, align 1
  %27 = load i64, i64* %6, align 8
  %28 = load i8*, i8** %8, align 8
  %29 = call i8* @"?assign@?$_Narrow_char_traits@DH@std@@SAPEADQEAD_KD@Z"(i8* %28, i64 %27, i8 %26) #5
  store i8 0, i8* %9, align 1
  %30 = load i8*, i8** %8, align 8
  %31 = load i64, i64* %6, align 8
  %32 = getelementptr inbounds i8, i8* %30, i64 %31
  call void @"?assign@?$_Narrow_char_traits@DH@std@@SAXAEADAEBD@Z"(i8* nonnull align 1 dereferenceable(1) %32, i8* nonnull align 1 dereferenceable(1) %9) #5
  store %"class.std::basic_string"* %11, %"class.std::basic_string"** %4, align 8
  br label %39

33:                                               ; preds = %3
  %34 = load i8, i8* %5, align 1
  %35 = load i64, i64* %6, align 8
  %36 = getelementptr inbounds %class.anon.4, %class.anon.4* %10, i32 0, i32 0
  %37 = load i8, i8* %36, align 1
  %38 = call nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"??$_Reallocate_for@V<lambda_1>@?0??assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV34@_KD@Z@D@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV01@_KV<lambda_1>@?0??assign@01@QEAAAEAV01@0D@Z@D@Z"(%"class.std::basic_string"* %11, i64 %35, i8 %37, i8 %34)
  store %"class.std::basic_string"* %38, %"class.std::basic_string"** %4, align 8
  br label %39

39:                                               ; preds = %33, %18
  %40 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  ret %"class.std::basic_string"* %40
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"?assign@?$_Narrow_char_traits@DH@std@@SAPEADQEAD_KD@Z"(i8* %0, i64 %1, i8 %2) #3 comdat align 2 {
  %4 = alloca i8, align 1
  %5 = alloca i64, align 8
  %6 = alloca i8*, align 8
  store i8 %2, i8* %4, align 1
  store i64 %1, i64* %5, align 8
  store i8* %0, i8** %6, align 8
  %7 = load i8*, i8** %6, align 8
  %8 = load i8, i8* %4, align 1
  %9 = sext i8 %8 to i32
  %10 = trunc i32 %9 to i8
  %11 = load i64, i64* %5, align 8
  call void @llvm.memset.p0i8.i64(i8* align 1 %7, i8 %10, i64 %11, i1 false)
  ret i8* %7
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"??$_Reallocate_for@V<lambda_1>@?0??assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV34@_KD@Z@D@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV01@_KV<lambda_1>@?0??assign@01@QEAAAEAV01@0D@Z@D@Z"(%"class.std::basic_string"* %0, i64 %1, i8 %2, i8 %3) #1 comdat align 2 {
  %5 = alloca %class.anon.4, align 1
  %6 = alloca i8, align 1
  %7 = alloca i64, align 8
  %8 = alloca %"class.std::basic_string"*, align 8
  %9 = alloca i64, align 8
  %10 = alloca i64, align 8
  %11 = alloca %"class.std::allocator"*, align 8
  %12 = alloca i8*, align 8
  %13 = getelementptr inbounds %class.anon.4, %class.anon.4* %5, i32 0, i32 0
  store i8 %2, i8* %13, align 1
  store i8 %3, i8* %6, align 1
  store i64 %1, i64* %7, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %8, align 8
  %14 = load %"class.std::basic_string"*, %"class.std::basic_string"** %8, align 8
  %15 = load i64, i64* %7, align 8
  %16 = call i64 @"?max_size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %14) #5
  %17 = icmp ugt i64 %15, %16
  br i1 %17, label %18, label %19

18:                                               ; preds = %4
  call void @"?_Xlen_string@std@@YAXXZ"() #19
  unreachable

19:                                               ; preds = %4
  %20 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %21 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %20, i32 0, i32 0
  %22 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %21, i32 0, i32 2
  %23 = load i64, i64* %22, align 8
  store i64 %23, i64* %9, align 8
  %24 = load i64, i64* %7, align 8
  %25 = call i64 @"?_Calculate_growth@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEBA_K_K@Z"(%"class.std::basic_string"* %14, i64 %24) #5
  store i64 %25, i64* %10, align 8
  %26 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Getal@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV?$allocator@D@2@XZ"(%"class.std::basic_string"* %14) #5
  store %"class.std::allocator"* %26, %"class.std::allocator"** %11, align 8
  %27 = load %"class.std::allocator"*, %"class.std::allocator"** %11, align 8
  %28 = load i64, i64* %10, align 8
  %29 = add i64 %28, 1
  %30 = call i8* @"?allocate@?$allocator@D@std@@QEAAPEAD_K@Z"(%"class.std::allocator"* %27, i64 %29)
  store i8* %30, i8** %12, align 8
  %31 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %32 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %31, i32 0, i32 0
  %33 = bitcast %"class.std::_String_val"* %32 to %"struct.std::_Container_base0"*
  call void @"?_Orphan_all@_Container_base0@std@@QEAAXXZ"(%"struct.std::_Container_base0"* %33) #5
  %34 = load i64, i64* %7, align 8
  %35 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %36 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %35, i32 0, i32 0
  %37 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %36, i32 0, i32 1
  store i64 %34, i64* %37, align 8
  %38 = load i64, i64* %10, align 8
  %39 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %40 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %39, i32 0, i32 0
  %41 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %40, i32 0, i32 2
  store i64 %38, i64* %41, align 8
  %42 = load i8, i8* %6, align 1
  %43 = load i64, i64* %7, align 8
  %44 = load i8*, i8** %12, align 8
  %45 = call i8* @"??$_Unfancy@D@std@@YAPEADPEAD@Z"(i8* %44) #5
  call void @"??R<lambda_1>@?0??assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@_KD@Z@QEBA?A?<auto>@@QEAD0D@Z"(%class.anon.4* %5, i8* %45, i64 %43, i8 %42)
  %46 = load i64, i64* %9, align 8
  %47 = icmp ule i64 16, %46
  br i1 %47, label %48, label %62

48:                                               ; preds = %19
  %49 = load %"class.std::allocator"*, %"class.std::allocator"** %11, align 8
  %50 = load i64, i64* %9, align 8
  %51 = add i64 %50, 1
  %52 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %53 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %52, i32 0, i32 0
  %54 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %53, i32 0, i32 0
  %55 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %54 to i8**
  %56 = load i8*, i8** %55, align 8
  call void @"?deallocate@?$allocator@D@std@@QEAAXQEAD_K@Z"(%"class.std::allocator"* %49, i8* %56, i64 %51)
  %57 = load i8*, i8** %12, align 8
  %58 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %59 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %58, i32 0, i32 0
  %60 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %59, i32 0, i32 0
  %61 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %60 to i8**
  store i8* %57, i8** %61, align 8
  br label %67

62:                                               ; preds = %19
  %63 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %14, i32 0, i32 0
  %64 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %63, i32 0, i32 0
  %65 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %64, i32 0, i32 0
  %66 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %65 to i8**
  call void @"??$_Construct_in_place@PEADAEBQEAD@std@@YAXAEAPEADAEBQEAD@Z"(i8** nonnull align 8 dereferenceable(8) %66, i8** nonnull align 8 dereferenceable(8) %12) #5
  br label %67

67:                                               ; preds = %62, %48
  ret %"class.std::basic_string"* %14
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??R<lambda_1>@?0??assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@_KD@Z@QEBA?A?<auto>@@QEAD0D@Z"(%class.anon.4* %0, i8* %1, i64 %2, i8 %3) #3 comdat align 2 {
  %5 = alloca i8, align 1
  %6 = alloca i64, align 8
  %7 = alloca i8*, align 8
  %8 = alloca %class.anon.4*, align 8
  %9 = alloca i8, align 1
  store i8 %3, i8* %5, align 1
  store i64 %2, i64* %6, align 8
  store i8* %1, i8** %7, align 8
  store %class.anon.4* %0, %class.anon.4** %8, align 8
  %10 = load %class.anon.4*, %class.anon.4** %8, align 8
  %11 = load i8, i8* %5, align 1
  %12 = load i64, i64* %6, align 8
  %13 = load i8*, i8** %7, align 8
  %14 = call i8* @"?assign@?$_Narrow_char_traits@DH@std@@SAPEADQEAD_KD@Z"(i8* %13, i64 %12, i8 %11) #5
  store i8 0, i8* %9, align 1
  %15 = load i8*, i8** %7, align 8
  %16 = load i64, i64* %6, align 8
  %17 = getelementptr inbounds i8, i8* %15, i64 %16
  call void @"?assign@?$_Narrow_char_traits@DH@std@@SAXAEADAEBD@Z"(i8* nonnull align 1 dereferenceable(1) %17, i8* nonnull align 1 dereferenceable(1) %9) #5
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i64 @"?_Getcat@?$numpunct@D@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z"(%"class.std::locale::facet"** %0, %"class.std::locale"* %1) #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %3 = alloca %"class.std::locale"*, align 8
  %4 = alloca %"class.std::locale::facet"**, align 8
  %5 = alloca %"class.std::_Locinfo", align 8
  %6 = alloca i1, align 1
  store %"class.std::locale"* %1, %"class.std::locale"** %3, align 8
  store %"class.std::locale::facet"** %0, %"class.std::locale::facet"*** %4, align 8
  %7 = load %"class.std::locale::facet"**, %"class.std::locale::facet"*** %4, align 8
  %8 = icmp ne %"class.std::locale::facet"** %7, null
  br i1 %8, label %9, label %32

9:                                                ; preds = %2
  %10 = load %"class.std::locale::facet"**, %"class.std::locale::facet"*** %4, align 8
  %11 = load %"class.std::locale::facet"*, %"class.std::locale::facet"** %10, align 8
  %12 = icmp ne %"class.std::locale::facet"* %11, null
  br i1 %12, label %32, label %13

13:                                               ; preds = %9
  %14 = call noalias nonnull i8* @"??2@YAPEAX_K@Z"(i64 48) #22
  store i1 true, i1* %6, align 1
  %15 = bitcast i8* %14 to %"class.std::numpunct"*
  %16 = load %"class.std::locale"*, %"class.std::locale"** %3, align 8
  %17 = invoke i8* @"?c_str@locale@std@@QEBAPEBDXZ"(%"class.std::locale"* %16)
          to label %18 unwind label %27

18:                                               ; preds = %13
  %19 = invoke %"class.std::_Locinfo"* @"??0_Locinfo@std@@QEAA@PEBD@Z"(%"class.std::_Locinfo"* %5, i8* %17)
          to label %20 unwind label %27

20:                                               ; preds = %18
  %21 = invoke %"class.std::numpunct"* @"??0?$numpunct@D@std@@QEAA@AEBV_Locinfo@1@_K_N@Z"(%"class.std::numpunct"* %15, %"class.std::_Locinfo"* nonnull align 8 dereferenceable(104) %5, i64 0, i1 zeroext true)
          to label %22 unwind label %25

22:                                               ; preds = %20
  store i1 false, i1* %6, align 1
  %23 = bitcast %"class.std::numpunct"* %15 to %"class.std::locale::facet"*
  %24 = load %"class.std::locale::facet"**, %"class.std::locale::facet"*** %4, align 8
  store %"class.std::locale::facet"* %23, %"class.std::locale::facet"** %24, align 8
  call void @"??1_Locinfo@std@@QEAA@XZ"(%"class.std::_Locinfo"* %5) #5
  br label %32

25:                                               ; preds = %20
  %26 = cleanuppad within none []
  call void @"??1_Locinfo@std@@QEAA@XZ"(%"class.std::_Locinfo"* %5) #5 [ "funclet"(token %26) ]
  cleanupret from %26 unwind label %27

27:                                               ; preds = %25, %18, %13
  %28 = cleanuppad within none []
  %29 = load i1, i1* %6, align 1
  br i1 %29, label %30, label %31

30:                                               ; preds = %27
  call void @"??3@YAXPEAX@Z"(i8* %14) #20 [ "funclet"(token %28) ]
  br label %31

31:                                               ; preds = %30, %27
  cleanupret from %28 unwind to caller

32:                                               ; preds = %22, %9, %2
  ret i64 4
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local %"class.std::numpunct"* @"??0?$numpunct@D@std@@QEAA@AEBV_Locinfo@1@_K_N@Z"(%"class.std::numpunct"* returned %0, %"class.std::_Locinfo"* nonnull align 8 dereferenceable(104) %1, i64 %2, i1 zeroext %3) unnamed_addr #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %5 = alloca i8, align 1
  %6 = alloca i64, align 8
  %7 = alloca %"class.std::_Locinfo"*, align 8
  %8 = alloca %"class.std::numpunct"*, align 8
  %9 = zext i1 %3 to i8
  store i8 %9, i8* %5, align 1
  store i64 %2, i64* %6, align 8
  store %"class.std::_Locinfo"* %1, %"class.std::_Locinfo"** %7, align 8
  store %"class.std::numpunct"* %0, %"class.std::numpunct"** %8, align 8
  %10 = load %"class.std::numpunct"*, %"class.std::numpunct"** %8, align 8
  %11 = bitcast %"class.std::numpunct"* %10 to %"class.std::locale::facet"*
  %12 = load i64, i64* %6, align 8
  %13 = call %"class.std::locale::facet"* @"??0facet@locale@std@@IEAA@_K@Z"(%"class.std::locale::facet"* %11, i64 %12)
  %14 = bitcast %"class.std::numpunct"* %10 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7?$numpunct@D@std@@6B@" to i32 (...)**), i32 (...)*** %14, align 8
  %15 = load i8, i8* %5, align 1
  %16 = trunc i8 %15 to i1
  %17 = load %"class.std::_Locinfo"*, %"class.std::_Locinfo"** %7, align 8
  invoke void @"?_Init@?$numpunct@D@std@@IEAAXAEBV_Locinfo@2@_N@Z"(%"class.std::numpunct"* %10, %"class.std::_Locinfo"* nonnull align 8 dereferenceable(104) %17, i1 zeroext %16)
          to label %18 unwind label %19

18:                                               ; preds = %4
  ret %"class.std::numpunct"* %10

19:                                               ; preds = %4
  %20 = cleanuppad within none []
  %21 = bitcast %"class.std::numpunct"* %10 to %"class.std::locale::facet"*
  call void @"??1facet@locale@std@@MEAA@XZ"(%"class.std::locale::facet"* %21) #5 [ "funclet"(token %20) ]
  cleanupret from %20 unwind to caller
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?_Init@?$numpunct@D@std@@IEAAXAEBV_Locinfo@2@_N@Z"(%"class.std::numpunct"* %0, %"class.std::_Locinfo"* nonnull align 8 dereferenceable(104) %1, i1 zeroext %2) #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %4 = alloca i8, align 1
  %5 = alloca %"class.std::_Locinfo"*, align 8
  %6 = alloca %"class.std::numpunct"*, align 8
  %7 = alloca %struct.lconv*, align 8
  %8 = alloca %struct._Cvtvec, align 4
  %9 = alloca %"struct.std::_Tidy_guard", align 8
  %10 = alloca %struct._Cvtvec, align 4
  %11 = alloca %struct._Cvtvec, align 4
  %12 = zext i1 %2 to i8
  store i8 %12, i8* %4, align 1
  store %"class.std::_Locinfo"* %1, %"class.std::_Locinfo"** %5, align 8
  store %"class.std::numpunct"* %0, %"class.std::numpunct"** %6, align 8
  %13 = load %"class.std::numpunct"*, %"class.std::numpunct"** %6, align 8
  %14 = load %"class.std::_Locinfo"*, %"class.std::_Locinfo"** %5, align 8
  %15 = call %struct.lconv* @"?_Getlconv@_Locinfo@std@@QEBAPEBUlconv@@XZ"(%"class.std::_Locinfo"* %14)
  store %struct.lconv* %15, %struct.lconv** %7, align 8
  %16 = load %"class.std::_Locinfo"*, %"class.std::_Locinfo"** %5, align 8
  call void @"?_Getcvt@_Locinfo@std@@QEBA?AU_Cvtvec@@XZ"(%"class.std::_Locinfo"* %16, %struct._Cvtvec* sret align 4 %8)
  %17 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %13, i32 0, i32 1
  store i8* null, i8** %17, align 8
  %18 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %13, i32 0, i32 4
  store i8* null, i8** %18, align 8
  %19 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %13, i32 0, i32 5
  store i8* null, i8** %19, align 8
  %20 = getelementptr inbounds %"struct.std::_Tidy_guard", %"struct.std::_Tidy_guard"* %9, i32 0, i32 0
  store %"class.std::numpunct"* %13, %"class.std::numpunct"** %20, align 8
  %21 = load %"class.std::_Locinfo"*, %"class.std::_Locinfo"** %5, align 8
  invoke void @"?_Getcvt@_Locinfo@std@@QEBA?AU_Cvtvec@@XZ"(%"class.std::_Locinfo"* %21, %struct._Cvtvec* sret align 4 %10)
          to label %22 unwind label %63

22:                                               ; preds = %3
  %23 = load i8, i8* %4, align 1
  %24 = trunc i8 %23 to i1
  br i1 %24, label %25, label %26

25:                                               ; preds = %22
  br label %30

26:                                               ; preds = %22
  %27 = load %struct.lconv*, %struct.lconv** %7, align 8
  %28 = getelementptr inbounds %struct.lconv, %struct.lconv* %27, i32 0, i32 2
  %29 = load i8*, i8** %28, align 8
  br label %30

30:                                               ; preds = %26, %25
  %31 = phi i8* [ getelementptr inbounds ([1 x i8], [1 x i8]* @"??_C@_00CNPNBAHC@?$AA@", i64 0, i64 0), %25 ], [ %29, %26 ]
  %32 = invoke i8* @"??$_Maklocstr@D@std@@YAPEADPEBDPEADAEBU_Cvtvec@@@Z"(i8* %31, i8* null, %struct._Cvtvec* nonnull align 4 dereferenceable(44) %10)
          to label %33 unwind label %63

33:                                               ; preds = %30
  %34 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %13, i32 0, i32 1
  store i8* %32, i8** %34, align 8
  %35 = load %"class.std::_Locinfo"*, %"class.std::_Locinfo"** %5, align 8
  %36 = invoke i8* @"?_Getfalse@_Locinfo@std@@QEBAPEBDXZ"(%"class.std::_Locinfo"* %35)
          to label %37 unwind label %63

37:                                               ; preds = %33
  %38 = invoke i8* @"??$_Maklocstr@D@std@@YAPEADPEBDPEADAEBU_Cvtvec@@@Z"(i8* %36, i8* null, %struct._Cvtvec* nonnull align 4 dereferenceable(44) %8)
          to label %39 unwind label %63

39:                                               ; preds = %37
  %40 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %13, i32 0, i32 4
  store i8* %38, i8** %40, align 8
  %41 = load %"class.std::_Locinfo"*, %"class.std::_Locinfo"** %5, align 8
  %42 = invoke i8* @"?_Gettrue@_Locinfo@std@@QEBAPEBDXZ"(%"class.std::_Locinfo"* %41)
          to label %43 unwind label %63

43:                                               ; preds = %39
  %44 = invoke i8* @"??$_Maklocstr@D@std@@YAPEADPEBDPEADAEBU_Cvtvec@@@Z"(i8* %42, i8* null, %struct._Cvtvec* nonnull align 4 dereferenceable(44) %8)
          to label %45 unwind label %63

45:                                               ; preds = %43
  %46 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %13, i32 0, i32 5
  store i8* %44, i8** %46, align 8
  %47 = getelementptr inbounds %"struct.std::_Tidy_guard", %"struct.std::_Tidy_guard"* %9, i32 0, i32 0
  store %"class.std::numpunct"* null, %"class.std::numpunct"** %47, align 8
  %48 = load i8, i8* %4, align 1
  %49 = trunc i8 %48 to i1
  br i1 %49, label %50, label %57

50:                                               ; preds = %45
  %51 = invoke i8 @"??$_Maklocchr@D@std@@YADDPEADAEBU_Cvtvec@@@Z"(i8 46, i8* null, %struct._Cvtvec* nonnull align 4 dereferenceable(44) %8)
          to label %52 unwind label %63

52:                                               ; preds = %50
  %53 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %13, i32 0, i32 2
  store i8 %51, i8* %53, align 8
  %54 = invoke i8 @"??$_Maklocchr@D@std@@YADDPEADAEBU_Cvtvec@@@Z"(i8 44, i8* null, %struct._Cvtvec* nonnull align 4 dereferenceable(44) %8)
          to label %55 unwind label %63

55:                                               ; preds = %52
  %56 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %13, i32 0, i32 3
  store i8 %54, i8* %56, align 1
  br label %62

57:                                               ; preds = %45
  %58 = bitcast %struct._Cvtvec* %11 to i8*
  %59 = bitcast %struct._Cvtvec* %8 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %58, i8* align 4 %59, i64 44, i1 false)
  %60 = load %struct.lconv*, %struct.lconv** %7, align 8
  invoke void @"??$_Getvals@D@?$numpunct@D@std@@IEAAXDPEBUlconv@@U_Cvtvec@@@Z"(%"class.std::numpunct"* %13, i8 0, %struct.lconv* %60, %struct._Cvtvec* %11)
          to label %61 unwind label %63

61:                                               ; preds = %57
  br label %62

62:                                               ; preds = %61, %55
  call void @"??1?$_Tidy_guard@V?$numpunct@D@std@@@std@@QEAA@XZ"(%"struct.std::_Tidy_guard"* %9) #5
  ret void

63:                                               ; preds = %57, %52, %50, %43, %39, %37, %33, %30, %3
  %64 = cleanuppad within none []
  call void @"??1?$_Tidy_guard@V?$numpunct@D@std@@@std@@QEAA@XZ"(%"struct.std::_Tidy_guard"* %9) #5 [ "funclet"(token %64) ]
  cleanupret from %64 unwind to caller
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??_G?$numpunct@D@std@@MEAAPEAXI@Z"(%"class.std::numpunct"* %0, i32 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca i32, align 4
  %5 = alloca %"class.std::numpunct"*, align 8
  store i32 %1, i32* %4, align 4
  store %"class.std::numpunct"* %0, %"class.std::numpunct"** %5, align 8
  %6 = load %"class.std::numpunct"*, %"class.std::numpunct"** %5, align 8
  %7 = bitcast %"class.std::numpunct"* %6 to i8*
  store i8* %7, i8** %3, align 8
  %8 = load i32, i32* %4, align 4
  call void @"??1?$numpunct@D@std@@MEAA@XZ"(%"class.std::numpunct"* %6) #5
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %12, label %10

10:                                               ; preds = %2
  %11 = bitcast %"class.std::numpunct"* %6 to i8*
  call void @"??3@YAXPEAX@Z"(i8* %11) #20
  br label %12

12:                                               ; preds = %10, %2
  %13 = load i8*, i8** %3, align 8
  ret i8* %13
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8 @"?do_decimal_point@?$numpunct@D@std@@MEBADXZ"(%"class.std::numpunct"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::numpunct"*, align 8
  store %"class.std::numpunct"* %0, %"class.std::numpunct"** %2, align 8
  %3 = load %"class.std::numpunct"*, %"class.std::numpunct"** %2, align 8
  %4 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %3, i32 0, i32 2
  %5 = load i8, i8* %4, align 8
  ret i8 %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8 @"?do_thousands_sep@?$numpunct@D@std@@MEBADXZ"(%"class.std::numpunct"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::numpunct"*, align 8
  store %"class.std::numpunct"* %0, %"class.std::numpunct"** %2, align 8
  %3 = load %"class.std::numpunct"*, %"class.std::numpunct"** %2, align 8
  %4 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %3, i32 0, i32 3
  %5 = load i8, i8* %4, align 1
  ret i8 %5
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?do_grouping@?$numpunct@D@std@@MEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ"(%"class.std::numpunct"* %0, %"class.std::basic_string"* noalias sret align 8 %1) unnamed_addr #1 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::numpunct"*, align 8
  %5 = bitcast %"class.std::basic_string"* %1 to i8*
  store i8* %5, i8** %3, align 8
  store %"class.std::numpunct"* %0, %"class.std::numpunct"** %4, align 8
  %6 = load %"class.std::numpunct"*, %"class.std::numpunct"** %4, align 8
  %7 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %6, i32 0, i32 1
  %8 = load i8*, i8** %7, align 8
  %9 = call %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@QEBD@Z"(%"class.std::basic_string"* %1, i8* %8)
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?do_falsename@?$numpunct@D@std@@MEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ"(%"class.std::numpunct"* %0, %"class.std::basic_string"* noalias sret align 8 %1) unnamed_addr #1 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::numpunct"*, align 8
  %5 = bitcast %"class.std::basic_string"* %1 to i8*
  store i8* %5, i8** %3, align 8
  store %"class.std::numpunct"* %0, %"class.std::numpunct"** %4, align 8
  %6 = load %"class.std::numpunct"*, %"class.std::numpunct"** %4, align 8
  %7 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %6, i32 0, i32 4
  %8 = load i8*, i8** %7, align 8
  %9 = call %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@QEBD@Z"(%"class.std::basic_string"* %1, i8* %8)
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?do_truename@?$numpunct@D@std@@MEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ"(%"class.std::numpunct"* %0, %"class.std::basic_string"* noalias sret align 8 %1) unnamed_addr #1 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::numpunct"*, align 8
  %5 = bitcast %"class.std::basic_string"* %1 to i8*
  store i8* %5, i8** %3, align 8
  store %"class.std::numpunct"* %0, %"class.std::numpunct"** %4, align 8
  %6 = load %"class.std::numpunct"*, %"class.std::numpunct"** %4, align 8
  %7 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %6, i32 0, i32 5
  %8 = load i8*, i8** %7, align 8
  %9 = call %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@QEBD@Z"(%"class.std::basic_string"* %1, i8* %8)
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local %struct.lconv* @"?_Getlconv@_Locinfo@std@@QEBAPEBUlconv@@XZ"(%"class.std::_Locinfo"* %0) #1 comdat align 2 {
  %2 = alloca %"class.std::_Locinfo"*, align 8
  store %"class.std::_Locinfo"* %0, %"class.std::_Locinfo"** %2, align 8
  %3 = load %"class.std::_Locinfo"*, %"class.std::_Locinfo"** %2, align 8
  %4 = call %struct.lconv* @localeconv()
  ret %struct.lconv* %4
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?_Getcvt@_Locinfo@std@@QEBA?AU_Cvtvec@@XZ"(%"class.std::_Locinfo"* %0, %struct._Cvtvec* noalias sret align 4 %1) #1 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::_Locinfo"*, align 8
  %5 = bitcast %struct._Cvtvec* %1 to i8*
  store i8* %5, i8** %3, align 8
  store %"class.std::_Locinfo"* %0, %"class.std::_Locinfo"** %4, align 8
  %6 = load %"class.std::_Locinfo"*, %"class.std::_Locinfo"** %4, align 8
  call void @_Getcvt(%struct._Cvtvec* sret align 4 %1)
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i8* @"??$_Maklocstr@D@std@@YAPEADPEBDPEADAEBU_Cvtvec@@@Z"(i8* %0, i8* %1, %struct._Cvtvec* nonnull align 4 dereferenceable(44) %2) #1 comdat {
  %4 = alloca %struct._Cvtvec*, align 8
  %5 = alloca i8*, align 8
  %6 = alloca i8*, align 8
  %7 = alloca i64, align 8
  %8 = alloca i8*, align 8
  %9 = alloca i8*, align 8
  store %struct._Cvtvec* %2, %struct._Cvtvec** %4, align 8
  store i8* %1, i8** %5, align 8
  store i8* %0, i8** %6, align 8
  %10 = load i8*, i8** %6, align 8
  %11 = call i64 @strlen(i8* %10)
  %12 = add i64 %11, 1
  store i64 %12, i64* %7, align 8
  %13 = load i64, i64* %7, align 8
  %14 = call noalias i8* @calloc(i64 %13, i64 1)
  store i8* %14, i8** %8, align 8
  %15 = load i8*, i8** %8, align 8
  %16 = icmp ne i8* %15, null
  br i1 %16, label %18, label %17

17:                                               ; preds = %3
  call void @"?_Xbad_alloc@std@@YAXXZ"() #19
  unreachable

18:                                               ; preds = %3
  %19 = load i8*, i8** %8, align 8
  store i8* %19, i8** %9, align 8
  br label %20

20:                                               ; preds = %27, %18
  %21 = load i64, i64* %7, align 8
  %22 = icmp ult i64 0, %21
  br i1 %22, label %23, label %34

23:                                               ; preds = %20
  %24 = load i8*, i8** %6, align 8
  %25 = load i8, i8* %24, align 1
  %26 = load i8*, i8** %9, align 8
  store i8 %25, i8* %26, align 1
  br label %27

27:                                               ; preds = %23
  %28 = load i64, i64* %7, align 8
  %29 = add i64 %28, -1
  store i64 %29, i64* %7, align 8
  %30 = load i8*, i8** %9, align 8
  %31 = getelementptr inbounds i8, i8* %30, i32 1
  store i8* %31, i8** %9, align 8
  %32 = load i8*, i8** %6, align 8
  %33 = getelementptr inbounds i8, i8* %32, i32 1
  store i8* %33, i8** %6, align 8
  br label %20

34:                                               ; preds = %20
  %35 = load i8*, i8** %8, align 8
  ret i8* %35
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"?_Getfalse@_Locinfo@std@@QEBAPEBDXZ"(%"class.std::_Locinfo"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::_Locinfo"*, align 8
  store %"class.std::_Locinfo"* %0, %"class.std::_Locinfo"** %2, align 8
  %3 = load %"class.std::_Locinfo"*, %"class.std::_Locinfo"** %2, align 8
  ret i8* getelementptr inbounds ([6 x i8], [6 x i8]* @"??_C@_05LAPONLG@false?$AA@", i64 0, i64 0)
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"?_Gettrue@_Locinfo@std@@QEBAPEBDXZ"(%"class.std::_Locinfo"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::_Locinfo"*, align 8
  store %"class.std::_Locinfo"* %0, %"class.std::_Locinfo"** %2, align 8
  %3 = load %"class.std::_Locinfo"*, %"class.std::_Locinfo"** %2, align 8
  ret i8* getelementptr inbounds ([5 x i8], [5 x i8]* @"??_C@_04LOAJBDKD@true?$AA@", i64 0, i64 0)
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8 @"??$_Maklocchr@D@std@@YADDPEADAEBU_Cvtvec@@@Z"(i8 %0, i8* %1, %struct._Cvtvec* nonnull align 4 dereferenceable(44) %2) #3 comdat {
  %4 = alloca %struct._Cvtvec*, align 8
  %5 = alloca i8*, align 8
  %6 = alloca i8, align 1
  store %struct._Cvtvec* %2, %struct._Cvtvec** %4, align 8
  store i8* %1, i8** %5, align 8
  store i8 %0, i8* %6, align 1
  %7 = load i8, i8* %6, align 1
  ret i8 %7
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??$_Getvals@D@?$numpunct@D@std@@IEAAXDPEBUlconv@@U_Cvtvec@@@Z"(%"class.std::numpunct"* %0, i8 %1, %struct.lconv* %2, %struct._Cvtvec* %3) #3 comdat align 2 {
  %5 = alloca %struct.lconv*, align 8
  %6 = alloca i8, align 1
  %7 = alloca %"class.std::numpunct"*, align 8
  store %struct.lconv* %2, %struct.lconv** %5, align 8
  store i8 %1, i8* %6, align 1
  store %"class.std::numpunct"* %0, %"class.std::numpunct"** %7, align 8
  %8 = load %"class.std::numpunct"*, %"class.std::numpunct"** %7, align 8
  %9 = load %struct.lconv*, %struct.lconv** %5, align 8
  %10 = getelementptr inbounds %struct.lconv, %struct.lconv* %9, i32 0, i32 0
  %11 = load i8*, i8** %10, align 8
  %12 = getelementptr inbounds i8, i8* %11, i64 0
  %13 = load i8, i8* %12, align 1
  %14 = call i8 @"??$_Maklocchr@D@std@@YADDPEADAEBU_Cvtvec@@@Z"(i8 %13, i8* null, %struct._Cvtvec* nonnull align 4 dereferenceable(44) %3)
  %15 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %8, i32 0, i32 2
  store i8 %14, i8* %15, align 8
  %16 = load %struct.lconv*, %struct.lconv** %5, align 8
  %17 = getelementptr inbounds %struct.lconv, %struct.lconv* %16, i32 0, i32 1
  %18 = load i8*, i8** %17, align 8
  %19 = getelementptr inbounds i8, i8* %18, i64 0
  %20 = load i8, i8* %19, align 1
  %21 = call i8 @"??$_Maklocchr@D@std@@YADDPEADAEBU_Cvtvec@@@Z"(i8 %20, i8* null, %struct._Cvtvec* nonnull align 4 dereferenceable(44) %3)
  %22 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %8, i32 0, i32 3
  store i8 %21, i8* %22, align 1
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1?$_Tidy_guard@V?$numpunct@D@std@@@std@@QEAA@XZ"(%"struct.std::_Tidy_guard"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"struct.std::_Tidy_guard"*, align 8
  store %"struct.std::_Tidy_guard"* %0, %"struct.std::_Tidy_guard"** %2, align 8
  %3 = load %"struct.std::_Tidy_guard"*, %"struct.std::_Tidy_guard"** %2, align 8
  %4 = getelementptr inbounds %"struct.std::_Tidy_guard", %"struct.std::_Tidy_guard"* %3, i32 0, i32 0
  %5 = load %"class.std::numpunct"*, %"class.std::numpunct"** %4, align 8
  %6 = icmp ne %"class.std::numpunct"* %5, null
  br i1 %6, label %7, label %10

7:                                                ; preds = %1
  %8 = getelementptr inbounds %"struct.std::_Tidy_guard", %"struct.std::_Tidy_guard"* %3, i32 0, i32 0
  %9 = load %"class.std::numpunct"*, %"class.std::numpunct"** %8, align 8
  call void @"?_Tidy@?$numpunct@D@std@@AEAAXXZ"(%"class.std::numpunct"* %9) #5
  br label %10

10:                                               ; preds = %7, %1
  ret void
}

declare dso_local %struct.lconv* @localeconv() #4

declare dso_local void @_Getcvt(%struct._Cvtvec* sret align 4) #4

declare dso_local noalias i8* @calloc(i64, i64) #4

; Function Attrs: noreturn
declare dso_local void @"?_Xbad_alloc@std@@YAXXZ"() #9

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?_Tidy@?$numpunct@D@std@@AEAAXXZ"(%"class.std::numpunct"* %0) #3 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %2 = alloca %"class.std::numpunct"*, align 8
  store %"class.std::numpunct"* %0, %"class.std::numpunct"** %2, align 8
  %3 = load %"class.std::numpunct"*, %"class.std::numpunct"** %2, align 8
  %4 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %3, i32 0, i32 1
  %5 = load i8*, i8** %4, align 8
  invoke void @free(i8* %5)
          to label %6 unwind label %13

6:                                                ; preds = %1
  %7 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %3, i32 0, i32 4
  %8 = load i8*, i8** %7, align 8
  invoke void @free(i8* %8)
          to label %9 unwind label %13

9:                                                ; preds = %6
  %10 = getelementptr inbounds %"class.std::numpunct", %"class.std::numpunct"* %3, i32 0, i32 5
  %11 = load i8*, i8** %10, align 8
  invoke void @free(i8* %11)
          to label %12 unwind label %13

12:                                               ; preds = %9
  ret void

13:                                               ; preds = %9, %6, %1
  %14 = cleanuppad within none []
  call void @__std_terminate() #18 [ "funclet"(token %14) ]
  unreachable
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1?$numpunct@D@std@@MEAA@XZ"(%"class.std::numpunct"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::numpunct"*, align 8
  store %"class.std::numpunct"* %0, %"class.std::numpunct"** %2, align 8
  %3 = load %"class.std::numpunct"*, %"class.std::numpunct"** %2, align 8
  %4 = bitcast %"class.std::numpunct"* %3 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7?$numpunct@D@std@@6B@" to i32 (...)**), i32 (...)*** %4, align 8
  call void @"?_Tidy@?$numpunct@D@std@@AEAAXXZ"(%"class.std::numpunct"* %3) #5
  %5 = bitcast %"class.std::numpunct"* %3 to %"class.std::locale::facet"*
  call void @"??1facet@locale@std@@MEAA@XZ"(%"class.std::locale::facet"* %5) #5
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?_Check_offset@?$_String_val@U?$_Simple_types@D@std@@@std@@QEBAX_K@Z"(%"class.std::_String_val"* %0, i64 %1) #1 comdat align 2 {
  %3 = alloca i64, align 8
  %4 = alloca %"class.std::_String_val"*, align 8
  store i64 %1, i64* %3, align 8
  store %"class.std::_String_val"* %0, %"class.std::_String_val"** %4, align 8
  %5 = load %"class.std::_String_val"*, %"class.std::_String_val"** %4, align 8
  %6 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %5, i32 0, i32 1
  %7 = load i64, i64* %6, align 8
  %8 = load i64, i64* %3, align 8
  %9 = icmp ult i64 %7, %8
  br i1 %9, label %10, label %11

10:                                               ; preds = %2
  call void @"?_Xran@?$_String_val@U?$_Simple_types@D@std@@@std@@SAXXZ"() #19
  unreachable

11:                                               ; preds = %2
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"??$_Reallocate_grow_by@V<lambda_1>@?0??insert@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV34@_K0D@Z@_K_KD@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV01@_KV<lambda_1>@?0??insert@01@QEAAAEAV01@00D@Z@_K2D@Z"(%"class.std::basic_string"* %0, i64 %1, i8 %2, i64 %3, i64 %4, i8 %5) #1 comdat align 2 {
  %7 = alloca %class.anon.6, align 1
  %8 = alloca i8, align 1
  %9 = alloca i64, align 8
  %10 = alloca i64, align 8
  %11 = alloca i64, align 8
  %12 = alloca %"class.std::basic_string"*, align 8
  %13 = alloca %"class.std::_String_val"*, align 8
  %14 = alloca i64, align 8
  %15 = alloca i64, align 8
  %16 = alloca i64, align 8
  %17 = alloca i64, align 8
  %18 = alloca %"class.std::allocator"*, align 8
  %19 = alloca i8*, align 8
  %20 = alloca i8*, align 8
  %21 = alloca i8*, align 8
  %22 = getelementptr inbounds %class.anon.6, %class.anon.6* %7, i32 0, i32 0
  store i8 %2, i8* %22, align 1
  store i8 %5, i8* %8, align 1
  store i64 %4, i64* %9, align 8
  store i64 %3, i64* %10, align 8
  store i64 %1, i64* %11, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %12, align 8
  %23 = load %"class.std::basic_string"*, %"class.std::basic_string"** %12, align 8
  %24 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %23, i32 0, i32 0
  %25 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %24, i32 0, i32 0
  store %"class.std::_String_val"* %25, %"class.std::_String_val"** %13, align 8
  %26 = load %"class.std::_String_val"*, %"class.std::_String_val"** %13, align 8
  %27 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %26, i32 0, i32 1
  %28 = load i64, i64* %27, align 8
  store i64 %28, i64* %14, align 8
  %29 = call i64 @"?max_size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %23) #5
  %30 = load i64, i64* %14, align 8
  %31 = sub i64 %29, %30
  %32 = load i64, i64* %11, align 8
  %33 = icmp ult i64 %31, %32
  br i1 %33, label %34, label %35

34:                                               ; preds = %6
  call void @"?_Xlen_string@std@@YAXXZ"() #19
  unreachable

35:                                               ; preds = %6
  %36 = load i64, i64* %14, align 8
  %37 = load i64, i64* %11, align 8
  %38 = add i64 %36, %37
  store i64 %38, i64* %15, align 8
  %39 = load %"class.std::_String_val"*, %"class.std::_String_val"** %13, align 8
  %40 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %39, i32 0, i32 2
  %41 = load i64, i64* %40, align 8
  store i64 %41, i64* %16, align 8
  %42 = load i64, i64* %15, align 8
  %43 = call i64 @"?_Calculate_growth@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEBA_K_K@Z"(%"class.std::basic_string"* %23, i64 %42) #5
  store i64 %43, i64* %17, align 8
  %44 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Getal@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV?$allocator@D@2@XZ"(%"class.std::basic_string"* %23) #5
  store %"class.std::allocator"* %44, %"class.std::allocator"** %18, align 8
  %45 = load %"class.std::allocator"*, %"class.std::allocator"** %18, align 8
  %46 = load i64, i64* %17, align 8
  %47 = add i64 %46, 1
  %48 = call i8* @"?allocate@?$allocator@D@std@@QEAAPEAD_K@Z"(%"class.std::allocator"* %45, i64 %47)
  store i8* %48, i8** %19, align 8
  %49 = load %"class.std::_String_val"*, %"class.std::_String_val"** %13, align 8
  %50 = bitcast %"class.std::_String_val"* %49 to %"struct.std::_Container_base0"*
  call void @"?_Orphan_all@_Container_base0@std@@QEAAXXZ"(%"struct.std::_Container_base0"* %50) #5
  %51 = load i64, i64* %15, align 8
  %52 = load %"class.std::_String_val"*, %"class.std::_String_val"** %13, align 8
  %53 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %52, i32 0, i32 1
  store i64 %51, i64* %53, align 8
  %54 = load i64, i64* %17, align 8
  %55 = load %"class.std::_String_val"*, %"class.std::_String_val"** %13, align 8
  %56 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %55, i32 0, i32 2
  store i64 %54, i64* %56, align 8
  %57 = load i8*, i8** %19, align 8
  %58 = call i8* @"??$_Unfancy@D@std@@YAPEADPEAD@Z"(i8* %57) #5
  store i8* %58, i8** %20, align 8
  %59 = load i64, i64* %16, align 8
  %60 = icmp ule i64 16, %59
  br i1 %60, label %61, label %81

61:                                               ; preds = %35
  %62 = load %"class.std::_String_val"*, %"class.std::_String_val"** %13, align 8
  %63 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %62, i32 0, i32 0
  %64 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %63 to i8**
  %65 = load i8*, i8** %64, align 8
  store i8* %65, i8** %21, align 8
  %66 = load i8, i8* %8, align 1
  %67 = load i64, i64* %9, align 8
  %68 = load i64, i64* %10, align 8
  %69 = load i64, i64* %14, align 8
  %70 = load i8*, i8** %21, align 8
  %71 = call i8* @"??$_Unfancy@D@std@@YAPEADPEAD@Z"(i8* %70) #5
  %72 = load i8*, i8** %20, align 8
  call void @"??R<lambda_1>@?0??insert@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@_K0D@Z@QEBA?A?<auto>@@QEADQEBD000D@Z"(%class.anon.6* %7, i8* %72, i8* %71, i64 %69, i64 %68, i64 %67, i8 %66)
  %73 = load %"class.std::allocator"*, %"class.std::allocator"** %18, align 8
  %74 = load i64, i64* %16, align 8
  %75 = add i64 %74, 1
  %76 = load i8*, i8** %21, align 8
  call void @"?deallocate@?$allocator@D@std@@QEAAXQEAD_K@Z"(%"class.std::allocator"* %73, i8* %76, i64 %75)
  %77 = load i8*, i8** %19, align 8
  %78 = load %"class.std::_String_val"*, %"class.std::_String_val"** %13, align 8
  %79 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %78, i32 0, i32 0
  %80 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %79 to i8**
  store i8* %77, i8** %80, align 8
  br label %94

81:                                               ; preds = %35
  %82 = load i8, i8* %8, align 1
  %83 = load i64, i64* %9, align 8
  %84 = load i64, i64* %10, align 8
  %85 = load i64, i64* %14, align 8
  %86 = load %"class.std::_String_val"*, %"class.std::_String_val"** %13, align 8
  %87 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %86, i32 0, i32 0
  %88 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %87 to [16 x i8]*
  %89 = getelementptr inbounds [16 x i8], [16 x i8]* %88, i64 0, i64 0
  %90 = load i8*, i8** %20, align 8
  call void @"??R<lambda_1>@?0??insert@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@_K0D@Z@QEBA?A?<auto>@@QEADQEBD000D@Z"(%class.anon.6* %7, i8* %90, i8* %89, i64 %85, i64 %84, i64 %83, i8 %82)
  %91 = load %"class.std::_String_val"*, %"class.std::_String_val"** %13, align 8
  %92 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %91, i32 0, i32 0
  %93 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %92 to i8**
  call void @"??$_Construct_in_place@PEADAEBQEAD@std@@YAXAEAPEADAEBQEAD@Z"(i8** nonnull align 8 dereferenceable(8) %93, i8** nonnull align 8 dereferenceable(8) %19) #5
  br label %94

94:                                               ; preds = %81, %61
  ret %"class.std::basic_string"* %23
}

; Function Attrs: noinline noreturn optnone uwtable
define linkonce_odr dso_local void @"?_Xran@?$_String_val@U?$_Simple_types@D@std@@@std@@SAXXZ"() #8 comdat align 2 {
  call void @"?_Xout_of_range@std@@YAXPEBD@Z"(i8* getelementptr inbounds ([24 x i8], [24 x i8]* @"??_C@_0BI@CFPLBAOH@invalid?5string?5position?$AA@", i64 0, i64 0)) #19
  unreachable
}

; Function Attrs: noreturn
declare dso_local void @"?_Xout_of_range@std@@YAXPEBD@Z"(i8*) #9

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??R<lambda_1>@?0??insert@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@_K0D@Z@QEBA?A?<auto>@@QEADQEBD000D@Z"(%class.anon.6* %0, i8* %1, i8* %2, i64 %3, i64 %4, i64 %5, i8 %6) #3 comdat align 2 {
  %8 = alloca i8, align 1
  %9 = alloca i64, align 8
  %10 = alloca i64, align 8
  %11 = alloca i64, align 8
  %12 = alloca i8*, align 8
  %13 = alloca i8*, align 8
  %14 = alloca %class.anon.6*, align 8
  store i8 %6, i8* %8, align 1
  store i64 %5, i64* %9, align 8
  store i64 %4, i64* %10, align 8
  store i64 %3, i64* %11, align 8
  store i8* %2, i8** %12, align 8
  store i8* %1, i8** %13, align 8
  store %class.anon.6* %0, %class.anon.6** %14, align 8
  %15 = load %class.anon.6*, %class.anon.6** %14, align 8
  %16 = load i64, i64* %10, align 8
  %17 = load i8*, i8** %12, align 8
  %18 = load i8*, i8** %13, align 8
  %19 = call i8* @"?copy@?$_Char_traits@DH@std@@SAPEADQEADQEBD_K@Z"(i8* %18, i8* %17, i64 %16) #5
  %20 = load i8, i8* %8, align 1
  %21 = load i64, i64* %9, align 8
  %22 = load i8*, i8** %13, align 8
  %23 = load i64, i64* %10, align 8
  %24 = getelementptr inbounds i8, i8* %22, i64 %23
  %25 = call i8* @"?assign@?$_Narrow_char_traits@DH@std@@SAPEADQEAD_KD@Z"(i8* %24, i64 %21, i8 %20) #5
  %26 = load i64, i64* %11, align 8
  %27 = load i64, i64* %10, align 8
  %28 = sub i64 %26, %27
  %29 = add i64 %28, 1
  %30 = load i8*, i8** %12, align 8
  %31 = load i64, i64* %10, align 8
  %32 = getelementptr inbounds i8, i8* %30, i64 %31
  %33 = load i8*, i8** %13, align 8
  %34 = load i64, i64* %10, align 8
  %35 = getelementptr inbounds i8, i8* %33, i64 %34
  %36 = load i64, i64* %9, align 8
  %37 = getelementptr inbounds i8, i8* %35, i64 %36
  %38 = call i8* @"?copy@?$_Char_traits@DH@std@@SAPEADQEADQEBD_K@Z"(i8* %37, i8* %32, i64 %29) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(16) %"class.std::ostreambuf_iterator"* @"??D?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAAAEAV01@XZ"(%"class.std::ostreambuf_iterator"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::ostreambuf_iterator"*, align 8
  store %"class.std::ostreambuf_iterator"* %0, %"class.std::ostreambuf_iterator"** %2, align 8
  %3 = load %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"** %2, align 8
  ret %"class.std::ostreambuf_iterator"* %3
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(16) %"class.std::ostreambuf_iterator"* @"??4?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAAAEAV01@D@Z"(%"class.std::ostreambuf_iterator"* %0, i8 %1) #1 comdat align 2 {
  %3 = alloca i8, align 1
  %4 = alloca %"class.std::ostreambuf_iterator"*, align 8
  %5 = alloca i32, align 4
  %6 = alloca i32, align 4
  store i8 %1, i8* %3, align 1
  store %"class.std::ostreambuf_iterator"* %0, %"class.std::ostreambuf_iterator"** %4, align 8
  %7 = load %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"** %4, align 8
  %8 = getelementptr inbounds %"class.std::ostreambuf_iterator", %"class.std::ostreambuf_iterator"* %7, i32 0, i32 1
  %9 = load %"class.std::basic_streambuf"*, %"class.std::basic_streambuf"** %8, align 8
  %10 = icmp ne %"class.std::basic_streambuf"* %9, null
  br i1 %10, label %11, label %18

11:                                               ; preds = %2
  %12 = getelementptr inbounds %"class.std::ostreambuf_iterator", %"class.std::ostreambuf_iterator"* %7, i32 0, i32 1
  %13 = load %"class.std::basic_streambuf"*, %"class.std::basic_streambuf"** %12, align 8
  %14 = load i8, i8* %3, align 1
  %15 = call i32 @"?sputc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHD@Z"(%"class.std::basic_streambuf"* %13, i8 %14)
  store i32 %15, i32* %5, align 4
  %16 = call i32 @"?eof@?$_Narrow_char_traits@DH@std@@SAHXZ"() #5
  store i32 %16, i32* %6, align 4
  %17 = call zeroext i1 @"?eq_int_type@?$_Narrow_char_traits@DH@std@@SA_NAEBH0@Z"(i32* nonnull align 4 dereferenceable(4) %6, i32* nonnull align 4 dereferenceable(4) %5) #5
  br label %18

18:                                               ; preds = %11, %2
  %19 = phi i1 [ true, %2 ], [ %17, %11 ]
  br i1 %19, label %20, label %22

20:                                               ; preds = %18
  %21 = getelementptr inbounds %"class.std::ostreambuf_iterator", %"class.std::ostreambuf_iterator"* %7, i32 0, i32 0
  store i8 1, i8* %21, align 8
  br label %22

22:                                               ; preds = %20, %18
  ret %"class.std::ostreambuf_iterator"* %7
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(16) %"class.std::ostreambuf_iterator"* @"??E?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@QEAAAEAV01@XZ"(%"class.std::ostreambuf_iterator"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::ostreambuf_iterator"*, align 8
  store %"class.std::ostreambuf_iterator"* %0, %"class.std::ostreambuf_iterator"** %2, align 8
  %3 = load %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"** %2, align 8
  ret %"class.std::ostreambuf_iterator"* %3
}

; Function Attrs: nounwind
declare void @llvm.va_start(i8*) #5

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i32 @_vsprintf_s_l(i8* %0, i64 %1, i8* %2, %struct.__crt_locale_pointers* %3, i8* %4) #1 comdat {
  %6 = alloca i8*, align 8
  %7 = alloca %struct.__crt_locale_pointers*, align 8
  %8 = alloca i8*, align 8
  %9 = alloca i64, align 8
  %10 = alloca i8*, align 8
  %11 = alloca i32, align 4
  store i8* %4, i8** %6, align 8
  store %struct.__crt_locale_pointers* %3, %struct.__crt_locale_pointers** %7, align 8
  store i8* %2, i8** %8, align 8
  store i64 %1, i64* %9, align 8
  store i8* %0, i8** %10, align 8
  %12 = load i8*, i8** %6, align 8
  %13 = load %struct.__crt_locale_pointers*, %struct.__crt_locale_pointers** %7, align 8
  %14 = load i8*, i8** %8, align 8
  %15 = load i64, i64* %9, align 8
  %16 = load i8*, i8** %10, align 8
  %17 = call i64* @__local_stdio_printf_options()
  %18 = load i64, i64* %17, align 8
  %19 = call i32 @__stdio_common_vsprintf_s(i64 %18, i8* %16, i64 %15, i8* %14, %struct.__crt_locale_pointers* %13, i8* %12)
  store i32 %19, i32* %11, align 4
  %20 = load i32, i32* %11, align 4
  %21 = icmp slt i32 %20, 0
  br i1 %21, label %22, label %23

22:                                               ; preds = %5
  br label %25

23:                                               ; preds = %5
  %24 = load i32, i32* %11, align 4
  br label %25

25:                                               ; preds = %23, %22
  %26 = phi i32 [ -1, %22 ], [ %24, %23 ]
  ret i32 %26
}

; Function Attrs: nounwind
declare void @llvm.va_end(i8*) #5

declare dso_local i32 @__stdio_common_vsprintf_s(i64, i8*, i64, i8*, %struct.__crt_locale_pointers*, i8*) #4

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i64* @__local_stdio_printf_options() #3 comdat {
  ret i64* @"?_OptionsStorage@?1??__local_stdio_printf_options@@9@4_KA"
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* returned %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::basic_string"*, align 8
  %3 = alloca %"struct.std::_Zero_then_variadic_args_t", align 1
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %2, align 8
  %4 = load %"class.std::basic_string"*, %"class.std::basic_string"** %2, align 8
  %5 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %4, i32 0, i32 0
  %6 = getelementptr inbounds %"struct.std::_Zero_then_variadic_args_t", %"struct.std::_Zero_then_variadic_args_t"* %3, i32 0, i32 0
  %7 = load i8, i8* %6, align 1
  %8 = call %"class.std::_Compressed_pair"* @"??$?0$$V@?$_Compressed_pair@V?$allocator@D@std@@V?$_String_val@U?$_Simple_types@D@std@@@2@$00@std@@QEAA@U_Zero_then_variadic_args_t@1@@Z"(%"class.std::_Compressed_pair"* %5, i8 %7) #5
  %9 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %4, i32 0, i32 0
  %10 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %9, i32 0, i32 0
  %11 = bitcast %"class.std::_String_val"* %10 to %"struct.std::_Container_base0"*
  call void @"?_Alloc_proxy@_Container_base0@std@@QEAAXAEBU_Fake_allocator@2@@Z"(%"struct.std::_Container_base0"* %11, %"struct.std::_Fake_allocator"* nonnull align 1 dereferenceable(1) @"?_Fake_alloc@std@@3U_Fake_allocator@1@B") #5
  call void @"?_Tidy_init@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXXZ"(%"class.std::basic_string"* %4) #5
  ret %"class.std::basic_string"* %4
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i64 @"?precision@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::ios_base"*, align 8
  store %"class.std::ios_base"* %0, %"class.std::ios_base"** %2, align 8
  %3 = load %"class.std::ios_base"*, %"class.std::ios_base"** %2, align 8
  %4 = getelementptr inbounds %"class.std::ios_base", %"class.std::ios_base"* %3, i32 0, i32 5
  %5 = load i64, i64* %4, align 8
  ret i64 %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i32 @"??$_Float_put_desired_precision@O@std@@YAH_JH@Z"(i64 %0, i32 %1) #3 comdat {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = alloca i64, align 8
  %6 = alloca i8, align 1
  %7 = alloca i8, align 1
  %8 = alloca i32, align 4
  store i32 %1, i32* %4, align 4
  store i64 %0, i64* %5, align 8
  %9 = load i32, i32* %4, align 4
  %10 = icmp eq i32 %9, 12288
  %11 = zext i1 %10 to i8
  store i8 %11, i8* %6, align 1
  %12 = load i8, i8* %6, align 1
  %13 = trunc i8 %12 to i1
  br i1 %13, label %14, label %15

14:                                               ; preds = %2
  store i32 13, i32* %3, align 4
  br label %33

15:                                               ; preds = %2
  %16 = load i64, i64* %5, align 8
  %17 = icmp sgt i64 %16, 0
  br i1 %17, label %18, label %21

18:                                               ; preds = %15
  %19 = load i64, i64* %5, align 8
  %20 = trunc i64 %19 to i32
  store i32 %20, i32* %3, align 4
  br label %33

21:                                               ; preds = %15
  %22 = load i64, i64* %5, align 8
  %23 = icmp eq i64 %22, 0
  br i1 %23, label %24, label %32

24:                                               ; preds = %21
  %25 = load i32, i32* %4, align 4
  %26 = icmp eq i32 %25, 0
  %27 = zext i1 %26 to i8
  store i8 %27, i8* %7, align 1
  %28 = load i8, i8* %7, align 1
  %29 = trunc i8 %28 to i1
  br i1 %29, label %30, label %31

30:                                               ; preds = %24
  store i32 1, i32* %3, align 4
  br label %33

31:                                               ; preds = %24
  store i32 0, i32* %3, align 4
  br label %33

32:                                               ; preds = %21
  store i32 6, i32* %8, align 4
  store i32 6, i32* %3, align 4
  br label %33

33:                                               ; preds = %32, %31, %30, %18, %14
  %34 = load i32, i32* %3, align 4
  ret i32 %34
}

; Function Attrs: nobuiltin noinline nounwind optnone readnone uwtable
define linkonce_odr dso_local double @fabsl(double %0) #14 comdat {
  %2 = alloca double, align 8
  store double %0, double* %2, align 8
  %3 = load double, double* %2, align 8
  %4 = call double @llvm.fabs.f64(double %3)
  ret double %4
}

; Function Attrs: nobuiltin noinline nounwind optnone uwtable
define linkonce_odr dso_local double @frexpl(double %0, i32* %1) #15 comdat {
  %3 = alloca i32*, align 8
  %4 = alloca double, align 8
  store i32* %1, i32** %3, align 8
  store double %0, double* %4, align 8
  %5 = load i32*, i32** %3, align 8
  %6 = load double, double* %4, align 8
  %7 = call double @frexp(double %6, i32* %5) #5
  ret double %7
}

; Function Attrs: nounwind readnone
declare dso_local i32 @abs(i32) #16

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?resize@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAX_KD@Z"(%"class.std::basic_string"* %0, i64 %1, i8 %2) #1 comdat align 2 {
  %4 = alloca i8, align 1
  %5 = alloca i64, align 8
  %6 = alloca %"class.std::basic_string"*, align 8
  %7 = alloca i64, align 8
  store i8 %2, i8* %4, align 1
  store i64 %1, i64* %5, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %6, align 8
  %8 = load %"class.std::basic_string"*, %"class.std::basic_string"** %6, align 8
  %9 = call i64 @"?size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %8) #5
  store i64 %9, i64* %7, align 8
  %10 = load i64, i64* %5, align 8
  %11 = load i64, i64* %7, align 8
  %12 = icmp ule i64 %10, %11
  br i1 %12, label %13, label %15

13:                                               ; preds = %3
  %14 = load i64, i64* %5, align 8
  call void @"?_Eos@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAX_K@Z"(%"class.std::basic_string"* %8, i64 %14)
  br label %21

15:                                               ; preds = %3
  %16 = load i8, i8* %4, align 1
  %17 = load i64, i64* %5, align 8
  %18 = load i64, i64* %7, align 8
  %19 = sub i64 %17, %18
  %20 = call nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@_KD@Z"(%"class.std::basic_string"* %8, i64 %19, i8 %16)
  br label %21

21:                                               ; preds = %15, %13
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"?_Ffmt@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAPEADPEADDH@Z"(%"class.std::num_put"* %0, i8* %1, i8 %2, i32 %3) #3 comdat align 2 {
  %5 = alloca i32, align 4
  %6 = alloca i8, align 1
  %7 = alloca i8*, align 8
  %8 = alloca %"class.std::num_put"*, align 8
  %9 = alloca i8*, align 8
  %10 = alloca i8, align 1
  %11 = alloca i32, align 4
  store i32 %3, i32* %5, align 4
  store i8 %2, i8* %6, align 1
  store i8* %1, i8** %7, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %8, align 8
  %12 = load %"class.std::num_put"*, %"class.std::num_put"** %8, align 8
  %13 = load i8*, i8** %7, align 8
  store i8* %13, i8** %9, align 8
  %14 = load i8*, i8** %9, align 8
  %15 = getelementptr inbounds i8, i8* %14, i32 1
  store i8* %15, i8** %9, align 8
  store i8 37, i8* %14, align 1
  %16 = load i32, i32* %5, align 4
  %17 = and i32 %16, 32
  %18 = icmp ne i32 %17, 0
  br i1 %18, label %19, label %22

19:                                               ; preds = %4
  %20 = load i8*, i8** %9, align 8
  %21 = getelementptr inbounds i8, i8* %20, i32 1
  store i8* %21, i8** %9, align 8
  store i8 43, i8* %20, align 1
  br label %22

22:                                               ; preds = %19, %4
  %23 = load i32, i32* %5, align 4
  %24 = and i32 %23, 16
  %25 = icmp ne i32 %24, 0
  br i1 %25, label %26, label %29

26:                                               ; preds = %22
  %27 = load i8*, i8** %9, align 8
  %28 = getelementptr inbounds i8, i8* %27, i32 1
  store i8* %28, i8** %9, align 8
  store i8 35, i8* %27, align 1
  br label %29

29:                                               ; preds = %26, %22
  %30 = load i8*, i8** %9, align 8
  %31 = getelementptr inbounds i8, i8* %30, i32 1
  store i8* %31, i8** %9, align 8
  store i8 46, i8* %30, align 1
  %32 = load i8*, i8** %9, align 8
  %33 = getelementptr inbounds i8, i8* %32, i32 1
  store i8* %33, i8** %9, align 8
  store i8 42, i8* %32, align 1
  %34 = load i8, i8* %6, align 1
  %35 = sext i8 %34 to i32
  %36 = icmp ne i32 %35, 0
  br i1 %36, label %37, label %41

37:                                               ; preds = %29
  %38 = load i8, i8* %6, align 1
  %39 = load i8*, i8** %9, align 8
  %40 = getelementptr inbounds i8, i8* %39, i32 1
  store i8* %40, i8** %9, align 8
  store i8 %38, i8* %39, align 1
  br label %41

41:                                               ; preds = %37, %29
  %42 = load i32, i32* %5, align 4
  %43 = and i32 %42, 12288
  store i32 %43, i32* %11, align 4
  %44 = load i32, i32* %5, align 4
  %45 = and i32 %44, 4
  %46 = icmp ne i32 %45, 0
  br i1 %46, label %47, label %63

47:                                               ; preds = %41
  %48 = load i32, i32* %11, align 4
  %49 = icmp eq i32 %48, 8192
  br i1 %49, label %50, label %51

50:                                               ; preds = %47
  store i8 102, i8* %10, align 1
  br label %62

51:                                               ; preds = %47
  %52 = load i32, i32* %11, align 4
  %53 = icmp eq i32 %52, 12288
  br i1 %53, label %54, label %55

54:                                               ; preds = %51
  store i8 65, i8* %10, align 1
  br label %61

55:                                               ; preds = %51
  %56 = load i32, i32* %11, align 4
  %57 = icmp eq i32 %56, 4096
  br i1 %57, label %58, label %59

58:                                               ; preds = %55
  store i8 69, i8* %10, align 1
  br label %60

59:                                               ; preds = %55
  store i8 71, i8* %10, align 1
  br label %60

60:                                               ; preds = %59, %58
  br label %61

61:                                               ; preds = %60, %54
  br label %62

62:                                               ; preds = %61, %50
  br label %79

63:                                               ; preds = %41
  %64 = load i32, i32* %11, align 4
  %65 = icmp eq i32 %64, 8192
  br i1 %65, label %66, label %67

66:                                               ; preds = %63
  store i8 102, i8* %10, align 1
  br label %78

67:                                               ; preds = %63
  %68 = load i32, i32* %11, align 4
  %69 = icmp eq i32 %68, 12288
  br i1 %69, label %70, label %71

70:                                               ; preds = %67
  store i8 97, i8* %10, align 1
  br label %77

71:                                               ; preds = %67
  %72 = load i32, i32* %11, align 4
  %73 = icmp eq i32 %72, 4096
  br i1 %73, label %74, label %75

74:                                               ; preds = %71
  store i8 101, i8* %10, align 1
  br label %76

75:                                               ; preds = %71
  store i8 103, i8* %10, align 1
  br label %76

76:                                               ; preds = %75, %74
  br label %77

77:                                               ; preds = %76, %70
  br label %78

78:                                               ; preds = %77, %66
  br label %79

79:                                               ; preds = %78, %62
  %80 = load i8, i8* %10, align 1
  %81 = load i8*, i8** %9, align 8
  %82 = getelementptr inbounds i8, i8* %81, i32 1
  store i8* %82, i8** %9, align 8
  store i8 %80, i8* %81, align 1
  %83 = load i8*, i8** %9, align 8
  store i8 0, i8* %83, align 1
  %84 = load i8*, i8** %7, align 8
  ret i8* %84
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?_Fput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEBD_K@Z"(%"class.std::num_put"* %0, %"class.std::ostreambuf_iterator"* noalias sret align 8 %1, %"class.std::ostreambuf_iterator"* %2, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %3, i8 %4, i8* %5, i64 %6) #1 comdat align 2 personality i8* bitcast (i32 (...)* @__CxxFrameHandler3 to i8*) {
  %8 = alloca i8*, align 8
  %9 = alloca i64, align 8
  %10 = alloca i8*, align 8
  %11 = alloca i8, align 1
  %12 = alloca %"class.std::ios_base"*, align 8
  %13 = alloca %"class.std::num_put"*, align 8
  %14 = alloca i64, align 8
  %15 = alloca i8*, align 8
  %16 = alloca i64, align 8
  %17 = alloca [2 x i8], align 1
  %18 = alloca i64, align 8
  %19 = alloca %"class.std::ctype"*, align 8
  %20 = alloca %"class.std::locale", align 8
  %21 = alloca %"class.std::basic_string", align 8
  %22 = alloca %"class.std::numpunct"*, align 8
  %23 = alloca %"class.std::locale", align 8
  %24 = alloca %"class.std::basic_string", align 8
  %25 = alloca i8, align 1
  %26 = alloca i64, align 8
  %27 = alloca i8*, align 8
  %28 = alloca i64, align 8
  %29 = alloca i32, align 4
  %30 = alloca %"class.std::ostreambuf_iterator", align 8
  %31 = alloca %"class.std::ostreambuf_iterator", align 8
  %32 = alloca %"class.std::ostreambuf_iterator", align 8
  %33 = alloca %"class.std::ostreambuf_iterator", align 8
  %34 = alloca %"class.std::ostreambuf_iterator", align 8
  %35 = alloca %"class.std::ostreambuf_iterator", align 8
  %36 = alloca %"class.std::ostreambuf_iterator", align 8
  %37 = alloca %"class.std::ostreambuf_iterator", align 8
  %38 = alloca %"class.std::ostreambuf_iterator", align 8
  %39 = alloca %"class.std::ostreambuf_iterator", align 8
  %40 = alloca %"class.std::ostreambuf_iterator", align 8
  %41 = alloca %"class.std::ostreambuf_iterator", align 8
  %42 = alloca %"class.std::ostreambuf_iterator", align 8
  %43 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  store i8* %43, i8** %8, align 8
  store i64 %6, i64* %9, align 8
  store i8* %5, i8** %10, align 8
  store i8 %4, i8* %11, align 1
  store %"class.std::ios_base"* %3, %"class.std::ios_base"** %12, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %13, align 8
  %44 = load %"class.std::num_put"*, %"class.std::num_put"** %13, align 8
  %45 = load i64, i64* %9, align 8
  %46 = icmp ult i64 0, %45
  br i1 %46, label %47, label %59

47:                                               ; preds = %7
  %48 = load i8*, i8** %10, align 8
  %49 = load i8, i8* %48, align 1
  %50 = sext i8 %49 to i32
  %51 = icmp eq i32 %50, 43
  br i1 %51, label %57, label %52

52:                                               ; preds = %47
  %53 = load i8*, i8** %10, align 8
  %54 = load i8, i8* %53, align 1
  %55 = sext i8 %54 to i32
  %56 = icmp eq i32 %55, 45
  br label %57

57:                                               ; preds = %52, %47
  %58 = phi i1 [ true, %47 ], [ %56, %52 ]
  br label %59

59:                                               ; preds = %57, %7
  %60 = phi i1 [ false, %7 ], [ %58, %57 ]
  %61 = zext i1 %60 to i64
  store i64 %61, i64* %14, align 8
  %62 = load %"class.std::ios_base"*, %"class.std::ios_base"** %12, align 8
  %63 = call i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %62)
  %64 = and i32 %63, 12288
  %65 = icmp ne i32 %64, 12288
  br i1 %65, label %66, label %67

66:                                               ; preds = %59
  store i8* getelementptr inbounds ([3 x i8], [3 x i8]* @"??_C@_02MDKMJEGG@eE?$AA@", i64 0, i64 0), i8** %15, align 8
  br label %99

67:                                               ; preds = %59
  store i8* getelementptr inbounds ([3 x i8], [3 x i8]* @"??_C@_02OOPEBDOJ@pP?$AA@", i64 0, i64 0), i8** %15, align 8
  %68 = load i64, i64* %14, align 8
  %69 = add i64 %68, 2
  %70 = load i64, i64* %9, align 8
  %71 = icmp ule i64 %69, %70
  br i1 %71, label %72, label %98

72:                                               ; preds = %67
  %73 = load i8*, i8** %10, align 8
  %74 = load i64, i64* %14, align 8
  %75 = getelementptr inbounds i8, i8* %73, i64 %74
  %76 = load i8, i8* %75, align 1
  %77 = sext i8 %76 to i32
  %78 = icmp eq i32 %77, 48
  br i1 %78, label %79, label %98

79:                                               ; preds = %72
  %80 = load i8*, i8** %10, align 8
  %81 = load i64, i64* %14, align 8
  %82 = add i64 %81, 1
  %83 = getelementptr inbounds i8, i8* %80, i64 %82
  %84 = load i8, i8* %83, align 1
  %85 = sext i8 %84 to i32
  %86 = icmp eq i32 %85, 120
  br i1 %86, label %95, label %87

87:                                               ; preds = %79
  %88 = load i8*, i8** %10, align 8
  %89 = load i64, i64* %14, align 8
  %90 = add i64 %89, 1
  %91 = getelementptr inbounds i8, i8* %88, i64 %90
  %92 = load i8, i8* %91, align 1
  %93 = sext i8 %92 to i32
  %94 = icmp eq i32 %93, 88
  br i1 %94, label %95, label %98

95:                                               ; preds = %87, %79
  %96 = load i64, i64* %14, align 8
  %97 = add i64 %96, 2
  store i64 %97, i64* %14, align 8
  br label %98

98:                                               ; preds = %95, %87, %72, %67
  br label %99

99:                                               ; preds = %98, %66
  %100 = load i8*, i8** %15, align 8
  %101 = load i8*, i8** %10, align 8
  %102 = getelementptr inbounds i8, i8* %101, i64 0
  %103 = call i64 @strcspn(i8* %102, i8* %100)
  store i64 %103, i64* %16, align 8
  %104 = bitcast [2 x i8]* %17 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %104, i8* align 1 getelementptr inbounds ([2 x i8], [2 x i8]* @"__const.?_Fput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEBD_K@Z._Dp", i32 0, i32 0), i64 2, i1 false)
  %105 = call %struct.lconv* @localeconv()
  %106 = getelementptr inbounds %struct.lconv, %struct.lconv* %105, i32 0, i32 0
  %107 = load i8*, i8** %106, align 8
  %108 = getelementptr inbounds i8, i8* %107, i64 0
  %109 = load i8, i8* %108, align 1
  %110 = getelementptr inbounds [2 x i8], [2 x i8]* %17, i64 0, i64 0
  store i8 %109, i8* %110, align 1
  %111 = getelementptr inbounds [2 x i8], [2 x i8]* %17, i64 0, i64 0
  %112 = load i8*, i8** %10, align 8
  %113 = getelementptr inbounds i8, i8* %112, i64 0
  %114 = call i64 @strcspn(i8* %113, i8* %111)
  store i64 %114, i64* %18, align 8
  %115 = load %"class.std::ios_base"*, %"class.std::ios_base"** %12, align 8
  call void @"?getloc@ios_base@std@@QEBA?AVlocale@2@XZ"(%"class.std::ios_base"* %115, %"class.std::locale"* sret align 8 %20)
  %116 = invoke nonnull align 8 dereferenceable(48) %"class.std::ctype"* @"??$use_facet@V?$ctype@D@std@@@std@@YAAEBV?$ctype@D@0@AEBVlocale@0@@Z"(%"class.std::locale"* nonnull align 8 dereferenceable(16) %20)
          to label %117 unwind label %146

117:                                              ; preds = %99
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %20) #5
  store %"class.std::ctype"* %116, %"class.std::ctype"** %19, align 8
  %118 = load i64, i64* %9, align 8
  %119 = call %"class.std::basic_string"* @"??0?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@_KD@Z"(%"class.std::basic_string"* %21, i64 %118, i8 0)
  %120 = load %"class.std::ctype"*, %"class.std::ctype"** %19, align 8
  %121 = call nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAD_K@Z"(%"class.std::basic_string"* %21, i64 0) #5
  %122 = load i8*, i8** %10, align 8
  %123 = load i64, i64* %9, align 8
  %124 = getelementptr inbounds i8, i8* %122, i64 %123
  %125 = load i8*, i8** %10, align 8
  %126 = invoke i8* @"?widen@?$ctype@D@std@@QEBAPEBDPEBD0PEAD@Z"(%"class.std::ctype"* %120, i8* %125, i8* %124, i8* %121)
          to label %127 unwind label %291

127:                                              ; preds = %117
  %128 = load %"class.std::ios_base"*, %"class.std::ios_base"** %12, align 8
  invoke void @"?getloc@ios_base@std@@QEBA?AVlocale@2@XZ"(%"class.std::ios_base"* %128, %"class.std::locale"* sret align 8 %23)
          to label %129 unwind label %291

129:                                              ; preds = %127
  %130 = invoke nonnull align 8 dereferenceable(48) %"class.std::numpunct"* @"??$use_facet@V?$numpunct@D@std@@@std@@YAAEBV?$numpunct@D@0@AEBVlocale@0@@Z"(%"class.std::locale"* nonnull align 8 dereferenceable(16) %23)
          to label %131 unwind label %148

131:                                              ; preds = %129
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %23) #5
  store %"class.std::numpunct"* %130, %"class.std::numpunct"** %22, align 8
  %132 = load %"class.std::numpunct"*, %"class.std::numpunct"** %22, align 8
  invoke void @"?grouping@?$numpunct@D@std@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ"(%"class.std::numpunct"* %132, %"class.std::basic_string"* sret align 8 %24)
          to label %133 unwind label %291

133:                                              ; preds = %131
  %134 = load %"class.std::numpunct"*, %"class.std::numpunct"** %22, align 8
  %135 = invoke i8 @"?thousands_sep@?$numpunct@D@std@@QEBADXZ"(%"class.std::numpunct"* %134)
          to label %136 unwind label %289

136:                                              ; preds = %133
  store i8 %135, i8* %25, align 1
  %137 = load i64, i64* %18, align 8
  %138 = load i64, i64* %9, align 8
  %139 = icmp ne i64 %137, %138
  br i1 %139, label %140, label %150

140:                                              ; preds = %136
  %141 = load %"class.std::numpunct"*, %"class.std::numpunct"** %22, align 8
  %142 = invoke i8 @"?decimal_point@?$numpunct@D@std@@QEBADXZ"(%"class.std::numpunct"* %141)
          to label %143 unwind label %289

143:                                              ; preds = %140
  %144 = load i64, i64* %18, align 8
  %145 = call nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAD_K@Z"(%"class.std::basic_string"* %21, i64 %144) #5
  store i8 %142, i8* %145, align 1
  br label %150

146:                                              ; preds = %99
  %147 = cleanuppad within none []
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %20) #5 [ "funclet"(token %147) ]
  cleanupret from %147 unwind to caller

148:                                              ; preds = %129
  %149 = cleanuppad within none []
  call void @"??1locale@std@@QEAA@XZ"(%"class.std::locale"* %23) #5 [ "funclet"(token %149) ]
  cleanupret from %149 unwind label %291

150:                                              ; preds = %143, %136
  %151 = load i64, i64* %18, align 8
  %152 = load i64, i64* %9, align 8
  %153 = icmp eq i64 %151, %152
  br i1 %153, label %154, label %156

154:                                              ; preds = %150
  %155 = load i64, i64* %16, align 8
  br label %158

156:                                              ; preds = %150
  %157 = load i64, i64* %18, align 8
  br label %158

158:                                              ; preds = %156, %154
  %159 = phi i64 [ %155, %154 ], [ %157, %156 ]
  store i64 %159, i64* %26, align 8
  %160 = call nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBAAEBD_K@Z"(%"class.std::basic_string"* %24, i64 0) #5
  store i8* %160, i8** %27, align 8
  br label %161

161:                                              ; preds = %198, %158
  %162 = load i8*, i8** %27, align 8
  %163 = load i8, i8* %162, align 1
  %164 = sext i8 %163 to i32
  %165 = icmp ne i32 %164, 127
  br i1 %165, label %166, label %179

166:                                              ; preds = %161
  %167 = load i8*, i8** %27, align 8
  %168 = load i8, i8* %167, align 1
  %169 = sext i8 %168 to i32
  %170 = icmp slt i32 0, %169
  br i1 %170, label %171, label %179

171:                                              ; preds = %166
  %172 = load i8*, i8** %27, align 8
  %173 = load i8, i8* %172, align 1
  %174 = sext i8 %173 to i64
  %175 = load i64, i64* %26, align 8
  %176 = load i64, i64* %14, align 8
  %177 = sub i64 %175, %176
  %178 = icmp ult i64 %174, %177
  br label %179

179:                                              ; preds = %171, %166, %161
  %180 = phi i1 [ false, %166 ], [ false, %161 ], [ %178, %171 ]
  br i1 %180, label %181, label %199

181:                                              ; preds = %179
  %182 = load i8, i8* %25, align 1
  %183 = load i8*, i8** %27, align 8
  %184 = load i8, i8* %183, align 1
  %185 = sext i8 %184 to i64
  %186 = load i64, i64* %26, align 8
  %187 = sub i64 %186, %185
  store i64 %187, i64* %26, align 8
  %188 = invoke nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?insert@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@_K0D@Z"(%"class.std::basic_string"* %21, i64 %187, i64 1, i8 %182)
          to label %189 unwind label %289

189:                                              ; preds = %181
  %190 = load i8*, i8** %27, align 8
  %191 = getelementptr inbounds i8, i8* %190, i64 1
  %192 = load i8, i8* %191, align 1
  %193 = sext i8 %192 to i32
  %194 = icmp slt i32 0, %193
  br i1 %194, label %195, label %198

195:                                              ; preds = %189
  %196 = load i8*, i8** %27, align 8
  %197 = getelementptr inbounds i8, i8* %196, i32 1
  store i8* %197, i8** %27, align 8
  br label %198

198:                                              ; preds = %195, %189
  br label %161

199:                                              ; preds = %179
  %200 = call i64 @"?size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %21) #5
  store i64 %200, i64* %9, align 8
  %201 = load %"class.std::ios_base"*, %"class.std::ios_base"** %12, align 8
  %202 = invoke i64 @"?width@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %201)
          to label %203 unwind label %289

203:                                              ; preds = %199
  %204 = icmp sle i64 %202, 0
  br i1 %204, label %211, label %205

205:                                              ; preds = %203
  %206 = load %"class.std::ios_base"*, %"class.std::ios_base"** %12, align 8
  %207 = invoke i64 @"?width@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %206)
          to label %208 unwind label %289

208:                                              ; preds = %205
  %209 = load i64, i64* %9, align 8
  %210 = icmp ule i64 %207, %209
  br i1 %210, label %211, label %212

211:                                              ; preds = %208, %203
  store i64 0, i64* %28, align 8
  br label %218

212:                                              ; preds = %208
  %213 = load %"class.std::ios_base"*, %"class.std::ios_base"** %12, align 8
  %214 = invoke i64 @"?width@ios_base@std@@QEBA_JXZ"(%"class.std::ios_base"* %213)
          to label %215 unwind label %289

215:                                              ; preds = %212
  %216 = load i64, i64* %9, align 8
  %217 = sub i64 %214, %216
  store i64 %217, i64* %28, align 8
  br label %218

218:                                              ; preds = %215, %211
  %219 = load %"class.std::ios_base"*, %"class.std::ios_base"** %12, align 8
  %220 = invoke i32 @"?flags@ios_base@std@@QEBAHXZ"(%"class.std::ios_base"* %219)
          to label %221 unwind label %289

221:                                              ; preds = %218
  %222 = and i32 %220, 448
  store i32 %222, i32* %29, align 4
  %223 = load i32, i32* %29, align 4
  %224 = icmp ne i32 %223, 64
  br i1 %224, label %225, label %243

225:                                              ; preds = %221
  %226 = load i32, i32* %29, align 4
  %227 = icmp ne i32 %226, 256
  br i1 %227, label %228, label %243

228:                                              ; preds = %225
  %229 = load i64, i64* %28, align 8
  %230 = load i8, i8* %11, align 1
  %231 = bitcast %"class.std::ostreambuf_iterator"* %31 to i8*
  %232 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %231, i8* align 8 %232, i64 16, i1 false)
  invoke void @"?_Rep@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@D_K@Z"(%"class.std::num_put"* %44, %"class.std::ostreambuf_iterator"* sret align 8 %30, %"class.std::ostreambuf_iterator"* %31, i8 %230, i64 %229)
          to label %233 unwind label %289

233:                                              ; preds = %228
  %234 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  %235 = bitcast %"class.std::ostreambuf_iterator"* %30 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %234, i8* align 8 %235, i64 16, i1 false)
  store i64 0, i64* %28, align 8
  %236 = load i64, i64* %14, align 8
  %237 = call nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAD_K@Z"(%"class.std::basic_string"* %21, i64 0) #5
  %238 = bitcast %"class.std::ostreambuf_iterator"* %33 to i8*
  %239 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %238, i8* align 8 %239, i64 16, i1 false)
  invoke void @"?_Put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@PEBD_K@Z"(%"class.std::num_put"* %44, %"class.std::ostreambuf_iterator"* sret align 8 %32, %"class.std::ostreambuf_iterator"* %33, i8* %237, i64 %236)
          to label %240 unwind label %289

240:                                              ; preds = %233
  %241 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  %242 = bitcast %"class.std::ostreambuf_iterator"* %32 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %241, i8* align 8 %242, i64 16, i1 false)
  br label %270

243:                                              ; preds = %225, %221
  %244 = load i32, i32* %29, align 4
  %245 = icmp eq i32 %244, 256
  br i1 %245, label %246, label %261

246:                                              ; preds = %243
  %247 = load i64, i64* %14, align 8
  %248 = call nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAD_K@Z"(%"class.std::basic_string"* %21, i64 0) #5
  %249 = bitcast %"class.std::ostreambuf_iterator"* %35 to i8*
  %250 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %249, i8* align 8 %250, i64 16, i1 false)
  invoke void @"?_Put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@PEBD_K@Z"(%"class.std::num_put"* %44, %"class.std::ostreambuf_iterator"* sret align 8 %34, %"class.std::ostreambuf_iterator"* %35, i8* %248, i64 %247)
          to label %251 unwind label %289

251:                                              ; preds = %246
  %252 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  %253 = bitcast %"class.std::ostreambuf_iterator"* %34 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %252, i8* align 8 %253, i64 16, i1 false)
  %254 = load i64, i64* %28, align 8
  %255 = load i8, i8* %11, align 1
  %256 = bitcast %"class.std::ostreambuf_iterator"* %37 to i8*
  %257 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %256, i8* align 8 %257, i64 16, i1 false)
  invoke void @"?_Rep@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@D_K@Z"(%"class.std::num_put"* %44, %"class.std::ostreambuf_iterator"* sret align 8 %36, %"class.std::ostreambuf_iterator"* %37, i8 %255, i64 %254)
          to label %258 unwind label %289

258:                                              ; preds = %251
  %259 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  %260 = bitcast %"class.std::ostreambuf_iterator"* %36 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %259, i8* align 8 %260, i64 16, i1 false)
  store i64 0, i64* %28, align 8
  br label %269

261:                                              ; preds = %243
  %262 = load i64, i64* %14, align 8
  %263 = call nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAD_K@Z"(%"class.std::basic_string"* %21, i64 0) #5
  %264 = bitcast %"class.std::ostreambuf_iterator"* %39 to i8*
  %265 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %264, i8* align 8 %265, i64 16, i1 false)
  invoke void @"?_Put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@PEBD_K@Z"(%"class.std::num_put"* %44, %"class.std::ostreambuf_iterator"* sret align 8 %38, %"class.std::ostreambuf_iterator"* %39, i8* %263, i64 %262)
          to label %266 unwind label %289

266:                                              ; preds = %261
  %267 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  %268 = bitcast %"class.std::ostreambuf_iterator"* %38 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %267, i8* align 8 %268, i64 16, i1 false)
  br label %269

269:                                              ; preds = %266, %258
  br label %270

270:                                              ; preds = %269, %240
  %271 = load i64, i64* %9, align 8
  %272 = load i64, i64* %14, align 8
  %273 = sub i64 %271, %272
  %274 = load i64, i64* %14, align 8
  %275 = call nonnull align 1 dereferenceable(1) i8* @"??A?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAD_K@Z"(%"class.std::basic_string"* %21, i64 %274) #5
  %276 = bitcast %"class.std::ostreambuf_iterator"* %41 to i8*
  %277 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %276, i8* align 8 %277, i64 16, i1 false)
  invoke void @"?_Put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@PEBD_K@Z"(%"class.std::num_put"* %44, %"class.std::ostreambuf_iterator"* sret align 8 %40, %"class.std::ostreambuf_iterator"* %41, i8* %275, i64 %273)
          to label %278 unwind label %289

278:                                              ; preds = %270
  %279 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  %280 = bitcast %"class.std::ostreambuf_iterator"* %40 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %279, i8* align 8 %280, i64 16, i1 false)
  %281 = load %"class.std::ios_base"*, %"class.std::ios_base"** %12, align 8
  %282 = invoke i64 @"?width@ios_base@std@@QEAA_J_J@Z"(%"class.std::ios_base"* %281, i64 0)
          to label %283 unwind label %289

283:                                              ; preds = %278
  %284 = load i64, i64* %28, align 8
  %285 = load i8, i8* %11, align 1
  %286 = bitcast %"class.std::ostreambuf_iterator"* %42 to i8*
  %287 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %286, i8* align 8 %287, i64 16, i1 false)
  invoke void @"?_Rep@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@D_K@Z"(%"class.std::num_put"* %44, %"class.std::ostreambuf_iterator"* sret align 8 %1, %"class.std::ostreambuf_iterator"* %42, i8 %285, i64 %284)
          to label %288 unwind label %289

288:                                              ; preds = %283
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %24) #5
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %21) #5
  ret void

289:                                              ; preds = %283, %278, %270, %261, %251, %246, %233, %228, %218, %212, %205, %199, %181, %140, %133
  %290 = cleanuppad within none []
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %24) #5 [ "funclet"(token %290) ]
  cleanupret from %290 unwind label %291

291:                                              ; preds = %289, %131, %148, %127, %117
  %292 = cleanuppad within none []
  call void @"??1?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAA@XZ"(%"class.std::basic_string"* %21) #5 [ "funclet"(token %292) ]
  cleanupret from %292 unwind to caller
}

; Function Attrs: nounwind readnone speculatable willreturn
declare double @llvm.fabs.f64(double) #17

; Function Attrs: nounwind
declare dso_local double @frexp(double, i32*) #12

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?_Eos@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAX_K@Z"(%"class.std::basic_string"* %0, i64 %1) #3 comdat align 2 {
  %3 = alloca i64, align 8
  %4 = alloca %"class.std::basic_string"*, align 8
  %5 = alloca i8, align 1
  store i64 %1, i64* %3, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %4, align 8
  %6 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  store i8 0, i8* %5, align 1
  %7 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %6, i32 0, i32 0
  %8 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %7, i32 0, i32 0
  %9 = call i8* @"?_Myptr@?$_String_val@U?$_Simple_types@D@std@@@std@@QEAAPEADXZ"(%"class.std::_String_val"* %8) #5
  %10 = load i64, i64* %3, align 8
  %11 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %6, i32 0, i32 0
  %12 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %11, i32 0, i32 0
  %13 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %12, i32 0, i32 1
  store i64 %10, i64* %13, align 8
  %14 = getelementptr inbounds i8, i8* %9, i64 %10
  call void @"?assign@?$_Narrow_char_traits@DH@std@@SAXAEADAEBD@Z"(i8* nonnull align 1 dereferenceable(1) %14, i8* nonnull align 1 dereferenceable(1) %5) #5
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@_KD@Z"(%"class.std::basic_string"* %0, i64 %1, i8 %2) #1 comdat align 2 {
  %4 = alloca %"class.std::basic_string"*, align 8
  %5 = alloca i8, align 1
  %6 = alloca i64, align 8
  %7 = alloca %"class.std::basic_string"*, align 8
  %8 = alloca i64, align 8
  %9 = alloca i8*, align 8
  %10 = alloca i8, align 1
  %11 = alloca %class.anon.8, align 1
  store i8 %2, i8* %5, align 1
  store i64 %1, i64* %6, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %7, align 8
  %12 = load %"class.std::basic_string"*, %"class.std::basic_string"** %7, align 8
  %13 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %12, i32 0, i32 0
  %14 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %13, i32 0, i32 0
  %15 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %14, i32 0, i32 1
  %16 = load i64, i64* %15, align 8
  store i64 %16, i64* %8, align 8
  %17 = load i64, i64* %6, align 8
  %18 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %12, i32 0, i32 0
  %19 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %18, i32 0, i32 0
  %20 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %19, i32 0, i32 2
  %21 = load i64, i64* %20, align 8
  %22 = load i64, i64* %8, align 8
  %23 = sub i64 %21, %22
  %24 = icmp ule i64 %17, %23
  br i1 %24, label %25, label %46

25:                                               ; preds = %3
  %26 = load i64, i64* %8, align 8
  %27 = load i64, i64* %6, align 8
  %28 = add i64 %26, %27
  %29 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %12, i32 0, i32 0
  %30 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %29, i32 0, i32 0
  %31 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %30, i32 0, i32 1
  store i64 %28, i64* %31, align 8
  %32 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %12, i32 0, i32 0
  %33 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %32, i32 0, i32 0
  %34 = call i8* @"?_Myptr@?$_String_val@U?$_Simple_types@D@std@@@std@@QEAAPEADXZ"(%"class.std::_String_val"* %33) #5
  store i8* %34, i8** %9, align 8
  %35 = load i8, i8* %5, align 1
  %36 = load i64, i64* %6, align 8
  %37 = load i8*, i8** %9, align 8
  %38 = load i64, i64* %8, align 8
  %39 = getelementptr inbounds i8, i8* %37, i64 %38
  %40 = call i8* @"?assign@?$_Narrow_char_traits@DH@std@@SAPEADQEAD_KD@Z"(i8* %39, i64 %36, i8 %35) #5
  store i8 0, i8* %10, align 1
  %41 = load i8*, i8** %9, align 8
  %42 = load i64, i64* %8, align 8
  %43 = load i64, i64* %6, align 8
  %44 = add i64 %42, %43
  %45 = getelementptr inbounds i8, i8* %41, i64 %44
  call void @"?assign@?$_Narrow_char_traits@DH@std@@SAXAEADAEBD@Z"(i8* nonnull align 1 dereferenceable(1) %45, i8* nonnull align 1 dereferenceable(1) %10) #5
  store %"class.std::basic_string"* %12, %"class.std::basic_string"** %4, align 8
  br label %53

46:                                               ; preds = %3
  %47 = load i8, i8* %5, align 1
  %48 = load i64, i64* %6, align 8
  %49 = load i64, i64* %6, align 8
  %50 = getelementptr inbounds %class.anon.8, %class.anon.8* %11, i32 0, i32 0
  %51 = load i8, i8* %50, align 1
  %52 = call nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"??$_Reallocate_grow_by@V<lambda_1>@?0??append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV34@_KD@Z@_KD@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV01@_KV<lambda_1>@?0??append@01@QEAAAEAV01@0D@Z@_KD@Z"(%"class.std::basic_string"* %12, i64 %49, i8 %51, i64 %48, i8 %47)
  store %"class.std::basic_string"* %52, %"class.std::basic_string"** %4, align 8
  br label %53

53:                                               ; preds = %46, %25
  %54 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  ret %"class.std::basic_string"* %54
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"??$_Reallocate_grow_by@V<lambda_1>@?0??append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV34@_KD@Z@_KD@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV01@_KV<lambda_1>@?0??append@01@QEAAAEAV01@0D@Z@_KD@Z"(%"class.std::basic_string"* %0, i64 %1, i8 %2, i64 %3, i8 %4) #1 comdat align 2 {
  %6 = alloca %class.anon.8, align 1
  %7 = alloca i8, align 1
  %8 = alloca i64, align 8
  %9 = alloca i64, align 8
  %10 = alloca %"class.std::basic_string"*, align 8
  %11 = alloca %"class.std::_String_val"*, align 8
  %12 = alloca i64, align 8
  %13 = alloca i64, align 8
  %14 = alloca i64, align 8
  %15 = alloca i64, align 8
  %16 = alloca %"class.std::allocator"*, align 8
  %17 = alloca i8*, align 8
  %18 = alloca i8*, align 8
  %19 = alloca i8*, align 8
  %20 = getelementptr inbounds %class.anon.8, %class.anon.8* %6, i32 0, i32 0
  store i8 %2, i8* %20, align 1
  store i8 %4, i8* %7, align 1
  store i64 %3, i64* %8, align 8
  store i64 %1, i64* %9, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %10, align 8
  %21 = load %"class.std::basic_string"*, %"class.std::basic_string"** %10, align 8
  %22 = getelementptr inbounds %"class.std::basic_string", %"class.std::basic_string"* %21, i32 0, i32 0
  %23 = getelementptr inbounds %"class.std::_Compressed_pair", %"class.std::_Compressed_pair"* %22, i32 0, i32 0
  store %"class.std::_String_val"* %23, %"class.std::_String_val"** %11, align 8
  %24 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %25 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %24, i32 0, i32 1
  %26 = load i64, i64* %25, align 8
  store i64 %26, i64* %12, align 8
  %27 = call i64 @"?max_size@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEBA_KXZ"(%"class.std::basic_string"* %21) #5
  %28 = load i64, i64* %12, align 8
  %29 = sub i64 %27, %28
  %30 = load i64, i64* %9, align 8
  %31 = icmp ult i64 %29, %30
  br i1 %31, label %32, label %33

32:                                               ; preds = %5
  call void @"?_Xlen_string@std@@YAXXZ"() #19
  unreachable

33:                                               ; preds = %5
  %34 = load i64, i64* %12, align 8
  %35 = load i64, i64* %9, align 8
  %36 = add i64 %34, %35
  store i64 %36, i64* %13, align 8
  %37 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %38 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %37, i32 0, i32 2
  %39 = load i64, i64* %38, align 8
  store i64 %39, i64* %14, align 8
  %40 = load i64, i64* %13, align 8
  %41 = call i64 @"?_Calculate_growth@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEBA_K_K@Z"(%"class.std::basic_string"* %21, i64 %40) #5
  store i64 %41, i64* %15, align 8
  %42 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Getal@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV?$allocator@D@2@XZ"(%"class.std::basic_string"* %21) #5
  store %"class.std::allocator"* %42, %"class.std::allocator"** %16, align 8
  %43 = load %"class.std::allocator"*, %"class.std::allocator"** %16, align 8
  %44 = load i64, i64* %15, align 8
  %45 = add i64 %44, 1
  %46 = call i8* @"?allocate@?$allocator@D@std@@QEAAPEAD_K@Z"(%"class.std::allocator"* %43, i64 %45)
  store i8* %46, i8** %17, align 8
  %47 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %48 = bitcast %"class.std::_String_val"* %47 to %"struct.std::_Container_base0"*
  call void @"?_Orphan_all@_Container_base0@std@@QEAAXXZ"(%"struct.std::_Container_base0"* %48) #5
  %49 = load i64, i64* %13, align 8
  %50 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %51 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %50, i32 0, i32 1
  store i64 %49, i64* %51, align 8
  %52 = load i64, i64* %15, align 8
  %53 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %54 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %53, i32 0, i32 2
  store i64 %52, i64* %54, align 8
  %55 = load i8*, i8** %17, align 8
  %56 = call i8* @"??$_Unfancy@D@std@@YAPEADPEAD@Z"(i8* %55) #5
  store i8* %56, i8** %18, align 8
  %57 = load i64, i64* %14, align 8
  %58 = icmp ule i64 16, %57
  br i1 %58, label %59, label %78

59:                                               ; preds = %33
  %60 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %61 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %60, i32 0, i32 0
  %62 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %61 to i8**
  %63 = load i8*, i8** %62, align 8
  store i8* %63, i8** %19, align 8
  %64 = load i8, i8* %7, align 1
  %65 = load i64, i64* %8, align 8
  %66 = load i64, i64* %12, align 8
  %67 = load i8*, i8** %19, align 8
  %68 = call i8* @"??$_Unfancy@D@std@@YAPEADPEAD@Z"(i8* %67) #5
  %69 = load i8*, i8** %18, align 8
  call void @"??R<lambda_1>@?0??append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@_KD@Z@QEBA?A?<auto>@@QEADQEBD00D@Z"(%class.anon.8* %6, i8* %69, i8* %68, i64 %66, i64 %65, i8 %64)
  %70 = load %"class.std::allocator"*, %"class.std::allocator"** %16, align 8
  %71 = load i64, i64* %14, align 8
  %72 = add i64 %71, 1
  %73 = load i8*, i8** %19, align 8
  call void @"?deallocate@?$allocator@D@std@@QEAAXQEAD_K@Z"(%"class.std::allocator"* %70, i8* %73, i64 %72)
  %74 = load i8*, i8** %17, align 8
  %75 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %76 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %75, i32 0, i32 0
  %77 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %76 to i8**
  store i8* %74, i8** %77, align 8
  br label %90

78:                                               ; preds = %33
  %79 = load i8, i8* %7, align 1
  %80 = load i64, i64* %8, align 8
  %81 = load i64, i64* %12, align 8
  %82 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %83 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %82, i32 0, i32 0
  %84 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %83 to [16 x i8]*
  %85 = getelementptr inbounds [16 x i8], [16 x i8]* %84, i64 0, i64 0
  %86 = load i8*, i8** %18, align 8
  call void @"??R<lambda_1>@?0??append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@_KD@Z@QEBA?A?<auto>@@QEADQEBD00D@Z"(%class.anon.8* %6, i8* %86, i8* %85, i64 %81, i64 %80, i8 %79)
  %87 = load %"class.std::_String_val"*, %"class.std::_String_val"** %11, align 8
  %88 = getelementptr inbounds %"class.std::_String_val", %"class.std::_String_val"* %87, i32 0, i32 0
  %89 = bitcast %"union.std::_String_val<std::_Simple_types<char>>::_Bxty"* %88 to i8**
  call void @"??$_Construct_in_place@PEADAEBQEAD@std@@YAXAEAPEADAEBQEAD@Z"(i8** nonnull align 8 dereferenceable(8) %89, i8** nonnull align 8 dereferenceable(8) %17) #5
  br label %90

90:                                               ; preds = %78, %59
  ret %"class.std::basic_string"* %21
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??R<lambda_1>@?0??append@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV23@_KD@Z@QEBA?A?<auto>@@QEADQEBD00D@Z"(%class.anon.8* %0, i8* %1, i8* %2, i64 %3, i64 %4, i8 %5) #3 comdat align 2 {
  %7 = alloca i8, align 1
  %8 = alloca i64, align 8
  %9 = alloca i64, align 8
  %10 = alloca i8*, align 8
  %11 = alloca i8*, align 8
  %12 = alloca %class.anon.8*, align 8
  %13 = alloca i8, align 1
  store i8 %5, i8* %7, align 1
  store i64 %4, i64* %8, align 8
  store i64 %3, i64* %9, align 8
  store i8* %2, i8** %10, align 8
  store i8* %1, i8** %11, align 8
  store %class.anon.8* %0, %class.anon.8** %12, align 8
  %14 = load %class.anon.8*, %class.anon.8** %12, align 8
  %15 = load i64, i64* %9, align 8
  %16 = load i8*, i8** %10, align 8
  %17 = load i8*, i8** %11, align 8
  %18 = call i8* @"?copy@?$_Char_traits@DH@std@@SAPEADQEADQEBD_K@Z"(i8* %17, i8* %16, i64 %15) #5
  %19 = load i8, i8* %7, align 1
  %20 = load i64, i64* %8, align 8
  %21 = load i8*, i8** %11, align 8
  %22 = load i64, i64* %9, align 8
  %23 = getelementptr inbounds i8, i8* %21, i64 %22
  %24 = call i8* @"?assign@?$_Narrow_char_traits@DH@std@@SAPEADQEAD_KD@Z"(i8* %23, i64 %20, i8 %19) #5
  store i8 0, i8* %13, align 1
  %25 = load i8*, i8** %11, align 8
  %26 = load i64, i64* %9, align 8
  %27 = load i64, i64* %8, align 8
  %28 = add i64 %26, %27
  %29 = getelementptr inbounds i8, i8* %25, i64 %28
  call void @"?assign@?$_Narrow_char_traits@DH@std@@SAXAEADAEBD@Z"(i8* nonnull align 1 dereferenceable(1) %29, i8* nonnull align 1 dereferenceable(1) %13) #5
  ret void
}

declare dso_local i64 @strcspn(i8*, i8*) #4

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local i8 @"?decimal_point@?$numpunct@D@std@@QEBADXZ"(%"class.std::numpunct"* %0) #1 comdat align 2 {
  %2 = alloca %"class.std::numpunct"*, align 8
  store %"class.std::numpunct"* %0, %"class.std::numpunct"** %2, align 8
  %3 = load %"class.std::numpunct"*, %"class.std::numpunct"** %2, align 8
  %4 = bitcast %"class.std::numpunct"* %3 to i8 (%"class.std::numpunct"*)***
  %5 = load i8 (%"class.std::numpunct"*)**, i8 (%"class.std::numpunct"*)*** %4, align 8
  %6 = getelementptr inbounds i8 (%"class.std::numpunct"*)*, i8 (%"class.std::numpunct"*)** %5, i64 3
  %7 = load i8 (%"class.std::numpunct"*)*, i8 (%"class.std::numpunct"*)** %6, align 8
  %8 = call i8 %7(%"class.std::numpunct"* %3)
  ret i8 %8
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i32 @"??$_Float_put_desired_precision@N@std@@YAH_JH@Z"(i64 %0, i32 %1) #3 comdat {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = alloca i64, align 8
  %6 = alloca i8, align 1
  %7 = alloca i8, align 1
  %8 = alloca i32, align 4
  store i32 %1, i32* %4, align 4
  store i64 %0, i64* %5, align 8
  %9 = load i32, i32* %4, align 4
  %10 = icmp eq i32 %9, 12288
  %11 = zext i1 %10 to i8
  store i8 %11, i8* %6, align 1
  %12 = load i8, i8* %6, align 1
  %13 = trunc i8 %12 to i1
  br i1 %13, label %14, label %15

14:                                               ; preds = %2
  store i32 13, i32* %3, align 4
  br label %33

15:                                               ; preds = %2
  %16 = load i64, i64* %5, align 8
  %17 = icmp sgt i64 %16, 0
  br i1 %17, label %18, label %21

18:                                               ; preds = %15
  %19 = load i64, i64* %5, align 8
  %20 = trunc i64 %19 to i32
  store i32 %20, i32* %3, align 4
  br label %33

21:                                               ; preds = %15
  %22 = load i64, i64* %5, align 8
  %23 = icmp eq i64 %22, 0
  br i1 %23, label %24, label %32

24:                                               ; preds = %21
  %25 = load i32, i32* %4, align 4
  %26 = icmp eq i32 %25, 0
  %27 = zext i1 %26 to i8
  store i8 %27, i8* %7, align 1
  %28 = load i8, i8* %7, align 1
  %29 = trunc i8 %28 to i1
  br i1 %29, label %30, label %31

30:                                               ; preds = %24
  store i32 1, i32* %3, align 4
  br label %33

31:                                               ; preds = %24
  store i32 0, i32* %3, align 4
  br label %33

32:                                               ; preds = %21
  store i32 6, i32* %8, align 4
  store i32 6, i32* %3, align 4
  br label %33

33:                                               ; preds = %32, %31, %30, %18, %14
  %34 = load i32, i32* %3, align 4
  ret i32 %34
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"?_Ifmt@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAPEADPEADPEBDH@Z"(%"class.std::num_put"* %0, i8* %1, i8* %2, i32 %3) #3 comdat align 2 {
  %5 = alloca i32, align 4
  %6 = alloca i8*, align 8
  %7 = alloca i8*, align 8
  %8 = alloca %"class.std::num_put"*, align 8
  %9 = alloca i8*, align 8
  %10 = alloca i32, align 4
  store i32 %3, i32* %5, align 4
  store i8* %2, i8** %6, align 8
  store i8* %1, i8** %7, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %8, align 8
  %11 = load %"class.std::num_put"*, %"class.std::num_put"** %8, align 8
  %12 = load i8*, i8** %7, align 8
  store i8* %12, i8** %9, align 8
  %13 = load i8*, i8** %9, align 8
  %14 = getelementptr inbounds i8, i8* %13, i32 1
  store i8* %14, i8** %9, align 8
  store i8 37, i8* %13, align 1
  %15 = load i32, i32* %5, align 4
  %16 = and i32 %15, 32
  %17 = icmp ne i32 %16, 0
  br i1 %17, label %18, label %21

18:                                               ; preds = %4
  %19 = load i8*, i8** %9, align 8
  %20 = getelementptr inbounds i8, i8* %19, i32 1
  store i8* %20, i8** %9, align 8
  store i8 43, i8* %19, align 1
  br label %21

21:                                               ; preds = %18, %4
  %22 = load i32, i32* %5, align 4
  %23 = and i32 %22, 8
  %24 = icmp ne i32 %23, 0
  br i1 %24, label %25, label %28

25:                                               ; preds = %21
  %26 = load i8*, i8** %9, align 8
  %27 = getelementptr inbounds i8, i8* %26, i32 1
  store i8* %27, i8** %9, align 8
  store i8 35, i8* %26, align 1
  br label %28

28:                                               ; preds = %25, %21
  %29 = load i8*, i8** %6, align 8
  %30 = getelementptr inbounds i8, i8* %29, i64 0
  %31 = load i8, i8* %30, align 1
  %32 = sext i8 %31 to i32
  %33 = icmp ne i32 %32, 76
  br i1 %33, label %34, label %40

34:                                               ; preds = %28
  %35 = load i8*, i8** %6, align 8
  %36 = getelementptr inbounds i8, i8* %35, i64 0
  %37 = load i8, i8* %36, align 1
  %38 = load i8*, i8** %9, align 8
  %39 = getelementptr inbounds i8, i8* %38, i32 1
  store i8* %39, i8** %9, align 8
  store i8 %37, i8* %38, align 1
  br label %47

40:                                               ; preds = %28
  %41 = load i8*, i8** %9, align 8
  %42 = getelementptr inbounds i8, i8* %41, i32 1
  store i8* %42, i8** %9, align 8
  store i8 73, i8* %41, align 1
  %43 = load i8*, i8** %9, align 8
  %44 = getelementptr inbounds i8, i8* %43, i32 1
  store i8* %44, i8** %9, align 8
  store i8 54, i8* %43, align 1
  %45 = load i8*, i8** %9, align 8
  %46 = getelementptr inbounds i8, i8* %45, i32 1
  store i8* %46, i8** %9, align 8
  store i8 52, i8* %45, align 1
  br label %47

47:                                               ; preds = %40, %34
  %48 = load i32, i32* %5, align 4
  %49 = and i32 %48, 3584
  store i32 %49, i32* %10, align 4
  %50 = load i32, i32* %10, align 4
  %51 = icmp eq i32 %50, 1024
  br i1 %51, label %52, label %53

52:                                               ; preds = %47
  br label %68

53:                                               ; preds = %47
  %54 = load i32, i32* %10, align 4
  %55 = icmp ne i32 %54, 2048
  br i1 %55, label %56, label %60

56:                                               ; preds = %53
  %57 = load i8*, i8** %6, align 8
  %58 = getelementptr inbounds i8, i8* %57, i64 1
  %59 = load i8, i8* %58, align 1
  br label %66

60:                                               ; preds = %53
  %61 = load i32, i32* %5, align 4
  %62 = and i32 %61, 4
  %63 = icmp ne i32 %62, 0
  %64 = zext i1 %63 to i64
  %65 = select i1 %63, i8 88, i8 120
  br label %66

66:                                               ; preds = %60, %56
  %67 = phi i8 [ %59, %56 ], [ %65, %60 ]
  br label %68

68:                                               ; preds = %66, %52
  %69 = phi i8 [ 111, %52 ], [ %67, %66 ]
  %70 = load i8*, i8** %9, align 8
  %71 = getelementptr inbounds i8, i8* %70, i32 1
  store i8* %71, i8** %9, align 8
  store i8 %69, i8* %70, align 1
  %72 = load i8*, i8** %9, align 8
  store i8 0, i8* %72, align 1
  %73 = load i8*, i8** %7, align 8
  ret i8* %73
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"?assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV12@$$QEAV12@@Z"(%"class.std::basic_string"* %0, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %1) #3 comdat align 2 {
  %3 = alloca %"class.std::basic_string"*, align 8
  %4 = alloca %"class.std::basic_string"*, align 8
  store %"class.std::basic_string"* %1, %"class.std::basic_string"** %3, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %4, align 8
  %5 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  %6 = load %"class.std::basic_string"*, %"class.std::basic_string"** %3, align 8
  %7 = call nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"??$move@AEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@YA$$QEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@0@AEAV10@@Z"(%"class.std::basic_string"* nonnull align 8 dereferenceable(32) %6) #5
  %8 = call nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"??4?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV01@$$QEAV01@@Z"(%"class.std::basic_string"* %5, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %7) #5
  ret %"class.std::basic_string"* %5
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?truename@?$numpunct@D@std@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ"(%"class.std::numpunct"* %0, %"class.std::basic_string"* noalias sret align 8 %1) #1 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::numpunct"*, align 8
  %5 = bitcast %"class.std::basic_string"* %1 to i8*
  store i8* %5, i8** %3, align 8
  store %"class.std::numpunct"* %0, %"class.std::numpunct"** %4, align 8
  %6 = load %"class.std::numpunct"*, %"class.std::numpunct"** %4, align 8
  %7 = bitcast %"class.std::numpunct"* %6 to void (%"class.std::numpunct"*, %"class.std::basic_string"*)***
  %8 = load void (%"class.std::numpunct"*, %"class.std::basic_string"*)**, void (%"class.std::numpunct"*, %"class.std::basic_string"*)*** %7, align 8
  %9 = getelementptr inbounds void (%"class.std::numpunct"*, %"class.std::basic_string"*)*, void (%"class.std::numpunct"*, %"class.std::basic_string"*)** %8, i64 7
  %10 = load void (%"class.std::numpunct"*, %"class.std::basic_string"*)*, void (%"class.std::numpunct"*, %"class.std::basic_string"*)** %9, align 8
  call void %10(%"class.std::numpunct"* %6, %"class.std::basic_string"* sret align 8 %1)
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?falsename@?$numpunct@D@std@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@XZ"(%"class.std::numpunct"* %0, %"class.std::basic_string"* noalias sret align 8 %1) #1 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca %"class.std::numpunct"*, align 8
  %5 = bitcast %"class.std::basic_string"* %1 to i8*
  store i8* %5, i8** %3, align 8
  store %"class.std::numpunct"* %0, %"class.std::numpunct"** %4, align 8
  %6 = load %"class.std::numpunct"*, %"class.std::numpunct"** %4, align 8
  %7 = bitcast %"class.std::numpunct"* %6 to void (%"class.std::numpunct"*, %"class.std::basic_string"*)***
  %8 = load void (%"class.std::numpunct"*, %"class.std::basic_string"*)**, void (%"class.std::numpunct"*, %"class.std::basic_string"*)*** %7, align 8
  %9 = getelementptr inbounds void (%"class.std::numpunct"*, %"class.std::basic_string"*)*, void (%"class.std::numpunct"*, %"class.std::basic_string"*)** %8, i64 6
  %10 = load void (%"class.std::numpunct"*, %"class.std::basic_string"*)*, void (%"class.std::numpunct"*, %"class.std::basic_string"*)** %9, align 8
  call void %10(%"class.std::numpunct"* %6, %"class.std::basic_string"* sret align 8 %1)
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"??$move@AEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@YA$$QEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@0@AEAV10@@Z"(%"class.std::basic_string"* nonnull align 8 dereferenceable(32) %0) #3 comdat {
  %2 = alloca %"class.std::basic_string"*, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %2, align 8
  %3 = load %"class.std::basic_string"*, %"class.std::basic_string"** %2, align 8
  ret %"class.std::basic_string"* %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(32) %"class.std::basic_string"* @"??4?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QEAAAEAV01@$$QEAV01@@Z"(%"class.std::basic_string"* %0, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %1) #3 comdat align 2 {
  %3 = alloca %"class.std::basic_string"*, align 8
  %4 = alloca %"class.std::basic_string"*, align 8
  %5 = alloca %"struct.std::_Equal_allocators", align 1
  store %"class.std::basic_string"* %1, %"class.std::basic_string"** %3, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %4, align 8
  %6 = load %"class.std::basic_string"*, %"class.std::basic_string"** %4, align 8
  %7 = load %"class.std::basic_string"*, %"class.std::basic_string"** %3, align 8
  %8 = call %"class.std::basic_string"* @"??$addressof@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@YAPEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@0@AEAV10@@Z"(%"class.std::basic_string"* nonnull align 8 dereferenceable(32) %7) #5
  %9 = icmp ne %"class.std::basic_string"* %6, %8
  br i1 %9, label %10, label %14

10:                                               ; preds = %2
  %11 = load %"class.std::basic_string"*, %"class.std::basic_string"** %3, align 8
  %12 = getelementptr inbounds %"struct.std::_Equal_allocators", %"struct.std::_Equal_allocators"* %5, i32 0, i32 0
  %13 = load i8, i8* %12, align 1
  call void @"?_Move_assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXAEAV12@U_Equal_allocators@2@@Z"(%"class.std::basic_string"* %6, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %11, i8 %13) #5
  br label %14

14:                                               ; preds = %10, %2
  ret %"class.std::basic_string"* %6
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::basic_string"* @"??$addressof@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@std@@YAPEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@0@AEAV10@@Z"(%"class.std::basic_string"* nonnull align 8 dereferenceable(32) %0) #3 comdat {
  %2 = alloca %"class.std::basic_string"*, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %2, align 8
  %3 = load %"class.std::basic_string"*, %"class.std::basic_string"** %2, align 8
  ret %"class.std::basic_string"* %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"?_Move_assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXAEAV12@U_Equal_allocators@2@@Z"(%"class.std::basic_string"* %0, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %1, i8 %2) #3 comdat align 2 {
  %4 = alloca %"struct.std::_Equal_allocators", align 1
  %5 = alloca %"class.std::basic_string"*, align 8
  %6 = alloca %"class.std::basic_string"*, align 8
  %7 = getelementptr inbounds %"struct.std::_Equal_allocators", %"struct.std::_Equal_allocators"* %4, i32 0, i32 0
  store i8 %2, i8* %7, align 1
  store %"class.std::basic_string"* %1, %"class.std::basic_string"** %5, align 8
  store %"class.std::basic_string"* %0, %"class.std::basic_string"** %6, align 8
  %8 = load %"class.std::basic_string"*, %"class.std::basic_string"** %6, align 8
  call void @"?_Tidy_deallocate@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXXZ"(%"class.std::basic_string"* %8) #5
  %9 = load %"class.std::basic_string"*, %"class.std::basic_string"** %5, align 8
  %10 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Getal@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV?$allocator@D@2@XZ"(%"class.std::basic_string"* %9) #5
  %11 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"?_Getal@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAAEAV?$allocator@D@2@XZ"(%"class.std::basic_string"* %8) #5
  call void @"??$_Pocma@V?$allocator@D@std@@@std@@YAXAEAV?$allocator@D@0@0@Z"(%"class.std::allocator"* nonnull align 1 dereferenceable(1) %11, %"class.std::allocator"* nonnull align 1 dereferenceable(1) %10) #5
  %12 = load %"class.std::basic_string"*, %"class.std::basic_string"** %5, align 8
  call void @"?_Take_contents@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEAAXAEAV12@@Z"(%"class.std::basic_string"* %8, %"class.std::basic_string"* nonnull align 8 dereferenceable(32) %12) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??$_Pocma@V?$allocator@D@std@@@std@@YAXAEAV?$allocator@D@0@0@Z"(%"class.std::allocator"* nonnull align 1 dereferenceable(1) %0, %"class.std::allocator"* nonnull align 1 dereferenceable(1) %1) #3 comdat {
  %3 = alloca %"class.std::allocator"*, align 8
  %4 = alloca %"class.std::allocator"*, align 8
  store %"class.std::allocator"* %1, %"class.std::allocator"** %3, align 8
  store %"class.std::allocator"* %0, %"class.std::allocator"** %4, align 8
  %5 = load %"class.std::allocator"*, %"class.std::allocator"** %3, align 8
  %6 = call nonnull align 1 dereferenceable(1) %"class.std::allocator"* @"??$move@AEAV?$allocator@D@std@@@std@@YA$$QEAV?$allocator@D@0@AEAV10@@Z"(%"class.std::allocator"* nonnull align 1 dereferenceable(1) %5) #5
  %7 = load %"class.std::allocator"*, %"class.std::allocator"** %4, align 8
  ret void
}

declare dso_local void @"?_Locinfo_dtor@_Locinfo@std@@SAXPEAV12@@Z"(%"class.std::_Locinfo"*) #4

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::bad_cast"* @"??0bad_cast@std@@QEAA@XZ"(%"class.std::bad_cast"* returned %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::bad_cast"*, align 8
  store %"class.std::bad_cast"* %0, %"class.std::bad_cast"** %2, align 8
  %3 = load %"class.std::bad_cast"*, %"class.std::bad_cast"** %2, align 8
  %4 = bitcast %"class.std::bad_cast"* %3 to %"class.std::exception"*
  %5 = call %"class.std::exception"* @"??0exception@std@@QEAA@QEBDH@Z"(%"class.std::exception"* %4, i8* getelementptr inbounds ([9 x i8], [9 x i8]* @"??_C@_08EPJLHIJG@bad?5cast?$AA@", i64 0, i64 0), i32 1) #5
  %6 = bitcast %"class.std::bad_cast"* %3 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7bad_cast@std@@6B@" to i32 (...)**), i32 (...)*** %6, align 8
  ret %"class.std::bad_cast"* %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::bad_cast"* @"??0bad_cast@std@@QEAA@AEBV01@@Z"(%"class.std::bad_cast"* returned %0, %"class.std::bad_cast"* nonnull align 8 dereferenceable(24) %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca %"class.std::bad_cast"*, align 8
  %4 = alloca %"class.std::bad_cast"*, align 8
  store %"class.std::bad_cast"* %1, %"class.std::bad_cast"** %3, align 8
  store %"class.std::bad_cast"* %0, %"class.std::bad_cast"** %4, align 8
  %5 = load %"class.std::bad_cast"*, %"class.std::bad_cast"** %4, align 8
  %6 = bitcast %"class.std::bad_cast"* %5 to %"class.std::exception"*
  %7 = load %"class.std::bad_cast"*, %"class.std::bad_cast"** %3, align 8
  %8 = bitcast %"class.std::bad_cast"* %7 to %"class.std::exception"*
  %9 = call %"class.std::exception"* @"??0exception@std@@QEAA@AEBV01@@Z"(%"class.std::exception"* %6, %"class.std::exception"* nonnull align 8 dereferenceable(24) %8) #5
  %10 = bitcast %"class.std::bad_cast"* %5 to i32 (...)***
  store i32 (...)** bitcast (i8** @"??_7bad_cast@std@@6B@" to i32 (...)**), i32 (...)*** %10, align 8
  ret %"class.std::bad_cast"* %5
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??1bad_cast@std@@UEAA@XZ"(%"class.std::bad_cast"* %0) unnamed_addr #3 comdat align 2 {
  %2 = alloca %"class.std::bad_cast"*, align 8
  store %"class.std::bad_cast"* %0, %"class.std::bad_cast"** %2, align 8
  %3 = load %"class.std::bad_cast"*, %"class.std::bad_cast"** %2, align 8
  %4 = bitcast %"class.std::bad_cast"* %3 to %"class.std::exception"*
  call void @"??1exception@std@@UEAA@XZ"(%"class.std::exception"* %4) #5
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local i8* @"??_Gbad_cast@std@@UEAAPEAXI@Z"(%"class.std::bad_cast"* %0, i32 %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca i8*, align 8
  %4 = alloca i32, align 4
  %5 = alloca %"class.std::bad_cast"*, align 8
  store i32 %1, i32* %4, align 4
  store %"class.std::bad_cast"* %0, %"class.std::bad_cast"** %5, align 8
  %6 = load %"class.std::bad_cast"*, %"class.std::bad_cast"** %5, align 8
  %7 = bitcast %"class.std::bad_cast"* %6 to i8*
  store i8* %7, i8** %3, align 8
  %8 = load i32, i32* %4, align 4
  call void @"??1bad_cast@std@@UEAA@XZ"(%"class.std::bad_cast"* %6) #5
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %12, label %10

10:                                               ; preds = %2
  %11 = bitcast %"class.std::bad_cast"* %6 to i8*
  call void @"??3@YAXPEAX@Z"(i8* %11) #20
  br label %12

12:                                               ; preds = %10, %2
  %13 = load i8*, i8** %3, align 8
  ret i8* %13
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::_Compressed_pair.2"* @"??$?0AEAPEAV_Facet_base@std@@@?$_Compressed_pair@U?$default_delete@V_Facet_base@std@@@std@@PEAV_Facet_base@2@$00@std@@QEAA@U_Zero_then_variadic_args_t@1@AEAPEAV_Facet_base@1@@Z"(%"class.std::_Compressed_pair.2"* returned %0, i8 %1, %"class.std::_Facet_base"** nonnull align 8 dereferenceable(8) %2) unnamed_addr #3 comdat align 2 {
  %4 = alloca %"struct.std::_Zero_then_variadic_args_t", align 1
  %5 = alloca %"class.std::_Facet_base"**, align 8
  %6 = alloca %"class.std::_Compressed_pair.2"*, align 8
  %7 = getelementptr inbounds %"struct.std::_Zero_then_variadic_args_t", %"struct.std::_Zero_then_variadic_args_t"* %4, i32 0, i32 0
  store i8 %1, i8* %7, align 1
  store %"class.std::_Facet_base"** %2, %"class.std::_Facet_base"*** %5, align 8
  store %"class.std::_Compressed_pair.2"* %0, %"class.std::_Compressed_pair.2"** %6, align 8
  %8 = load %"class.std::_Compressed_pair.2"*, %"class.std::_Compressed_pair.2"** %6, align 8
  %9 = bitcast %"class.std::_Compressed_pair.2"* %8 to %"struct.std::default_delete"*
  %10 = getelementptr inbounds %"class.std::_Compressed_pair.2", %"class.std::_Compressed_pair.2"* %8, i32 0, i32 0
  %11 = load %"class.std::_Facet_base"**, %"class.std::_Facet_base"*** %5, align 8
  %12 = call nonnull align 8 dereferenceable(8) %"class.std::_Facet_base"** @"??$forward@AEAPEAV_Facet_base@std@@@std@@YAAEAPEAV_Facet_base@0@AEAPEAV10@@Z"(%"class.std::_Facet_base"** nonnull align 8 dereferenceable(8) %11) #5
  %13 = load %"class.std::_Facet_base"*, %"class.std::_Facet_base"** %12, align 8
  store %"class.std::_Facet_base"* %13, %"class.std::_Facet_base"** %10, align 8
  ret %"class.std::_Compressed_pair.2"* %8
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 8 dereferenceable(8) %"class.std::_Facet_base"** @"??$forward@AEAPEAV_Facet_base@std@@@std@@YAAEAPEAV_Facet_base@0@AEAPEAV10@@Z"(%"class.std::_Facet_base"** nonnull align 8 dereferenceable(8) %0) #3 comdat {
  %2 = alloca %"class.std::_Facet_base"**, align 8
  store %"class.std::_Facet_base"** %0, %"class.std::_Facet_base"*** %2, align 8
  %3 = load %"class.std::_Facet_base"**, %"class.std::_Facet_base"*** %2, align 8
  ret %"class.std::_Facet_base"** %3
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::_Facet_base"* @"??$exchange@PEAV_Facet_base@std@@$$T@std@@YAPEAV_Facet_base@0@AEAPEAV10@$$QEA$$T@Z"(%"class.std::_Facet_base"** nonnull align 8 dereferenceable(8) %0, i8** nonnull align 8 dereferenceable(8) %1) #3 comdat {
  %3 = alloca i8**, align 8
  %4 = alloca %"class.std::_Facet_base"**, align 8
  %5 = alloca %"class.std::_Facet_base"*, align 8
  store i8** %1, i8*** %3, align 8
  store %"class.std::_Facet_base"** %0, %"class.std::_Facet_base"*** %4, align 8
  %6 = load %"class.std::_Facet_base"**, %"class.std::_Facet_base"*** %4, align 8
  %7 = load %"class.std::_Facet_base"*, %"class.std::_Facet_base"** %6, align 8
  store %"class.std::_Facet_base"* %7, %"class.std::_Facet_base"** %5, align 8
  %8 = load i8**, i8*** %3, align 8
  %9 = load %"class.std::_Facet_base"**, %"class.std::_Facet_base"*** %4, align 8
  store %"class.std::_Facet_base"* null, %"class.std::_Facet_base"** %9, align 8
  %10 = load %"class.std::_Facet_base"*, %"class.std::_Facet_base"** %5, align 8
  ret %"class.std::_Facet_base"* %10
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local nonnull align 1 dereferenceable(1) %"struct.std::default_delete"* @"?_Get_first@?$_Compressed_pair@U?$default_delete@V_Facet_base@std@@@std@@PEAV_Facet_base@2@$00@std@@QEAAAEAU?$default_delete@V_Facet_base@std@@@2@XZ"(%"class.std::_Compressed_pair.2"* %0) #3 comdat align 2 {
  %2 = alloca %"class.std::_Compressed_pair.2"*, align 8
  store %"class.std::_Compressed_pair.2"* %0, %"class.std::_Compressed_pair.2"** %2, align 8
  %3 = load %"class.std::_Compressed_pair.2"*, %"class.std::_Compressed_pair.2"** %2, align 8
  %4 = bitcast %"class.std::_Compressed_pair.2"* %3 to %"struct.std::default_delete"*
  ret %"struct.std::default_delete"* %4
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local void @"??R?$default_delete@V_Facet_base@std@@@std@@QEBAXPEAV_Facet_base@1@@Z"(%"struct.std::default_delete"* %0, %"class.std::_Facet_base"* %1) #3 comdat align 2 {
  %3 = alloca %"class.std::_Facet_base"*, align 8
  %4 = alloca %"struct.std::default_delete"*, align 8
  store %"class.std::_Facet_base"* %1, %"class.std::_Facet_base"** %3, align 8
  store %"struct.std::default_delete"* %0, %"struct.std::default_delete"** %4, align 8
  %5 = load %"struct.std::default_delete"*, %"struct.std::default_delete"** %4, align 8
  %6 = load %"class.std::_Facet_base"*, %"class.std::_Facet_base"** %3, align 8
  %7 = icmp eq %"class.std::_Facet_base"* %6, null
  br i1 %7, label %14, label %8

8:                                                ; preds = %2
  %9 = bitcast %"class.std::_Facet_base"* %6 to i8* (%"class.std::_Facet_base"*, i32)***
  %10 = load i8* (%"class.std::_Facet_base"*, i32)**, i8* (%"class.std::_Facet_base"*, i32)*** %9, align 8
  %11 = getelementptr inbounds i8* (%"class.std::_Facet_base"*, i32)*, i8* (%"class.std::_Facet_base"*, i32)** %10, i64 0
  %12 = load i8* (%"class.std::_Facet_base"*, i32)*, i8* (%"class.std::_Facet_base"*, i32)** %11, align 8
  %13 = call i8* %12(%"class.std::_Facet_base"* %6, i32 1) #5
  br label %14

14:                                               ; preds = %8, %2
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define linkonce_odr dso_local %"class.std::locale"* @"??0locale@std@@QEAA@AEBV01@@Z"(%"class.std::locale"* returned %0, %"class.std::locale"* nonnull align 8 dereferenceable(16) %1) unnamed_addr #3 comdat align 2 {
  %3 = alloca %"class.std::locale"*, align 8
  %4 = alloca %"class.std::locale"*, align 8
  store %"class.std::locale"* %1, %"class.std::locale"** %3, align 8
  store %"class.std::locale"* %0, %"class.std::locale"** %4, align 8
  %5 = load %"class.std::locale"*, %"class.std::locale"** %4, align 8
  %6 = bitcast %"class.std::locale"* %5 to %"class.std::_Locbase"*
  %7 = bitcast %"class.std::locale"* %5 to i8*
  %8 = getelementptr inbounds i8, i8* %7, i64 1
  %9 = bitcast i8* %8 to %"struct.std::_Crt_new_delete"*
  %10 = getelementptr inbounds %"class.std::locale", %"class.std::locale"* %5, i32 0, i32 1
  %11 = load %"class.std::locale"*, %"class.std::locale"** %3, align 8
  %12 = getelementptr inbounds %"class.std::locale", %"class.std::locale"* %11, i32 0, i32 1
  %13 = load %"class.std::locale::_Locimp"*, %"class.std::locale::_Locimp"** %12, align 8
  store %"class.std::locale::_Locimp"* %13, %"class.std::locale::_Locimp"** %10, align 8
  %14 = getelementptr inbounds %"class.std::locale", %"class.std::locale"* %5, i32 0, i32 1
  %15 = load %"class.std::locale::_Locimp"*, %"class.std::locale::_Locimp"** %14, align 8
  %16 = bitcast %"class.std::locale::_Locimp"* %15 to %"class.std::locale::facet"*
  %17 = bitcast %"class.std::locale::facet"* %16 to void (%"class.std::locale::facet"*)***
  %18 = load void (%"class.std::locale::facet"*)**, void (%"class.std::locale::facet"*)*** %17, align 8
  %19 = getelementptr inbounds void (%"class.std::locale::facet"*)*, void (%"class.std::locale::facet"*)** %18, i64 1
  %20 = load void (%"class.std::locale::facet"*)*, void (%"class.std::locale::facet"*)** %19, align 8
  call void %20(%"class.std::locale::facet"* %16) #5
  ret %"class.std::locale"* %5
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DJ@Z"(%"class.std::num_put"* %0, %"class.std::ostreambuf_iterator"* noalias sret align 8 %1, %"class.std::ostreambuf_iterator"* %2, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %3, i8 %4, i32 %5) #1 comdat align 2 {
  %7 = alloca i8*, align 8
  %8 = alloca i32, align 4
  %9 = alloca i8, align 1
  %10 = alloca %"class.std::ios_base"*, align 8
  %11 = alloca %"class.std::num_put"*, align 8
  %12 = alloca %"class.std::ostreambuf_iterator", align 8
  %13 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  store i8* %13, i8** %7, align 8
  store i32 %5, i32* %8, align 4
  store i8 %4, i8* %9, align 1
  store %"class.std::ios_base"* %3, %"class.std::ios_base"** %10, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %11, align 8
  %14 = load %"class.std::num_put"*, %"class.std::num_put"** %11, align 8
  %15 = load i32, i32* %8, align 4
  %16 = load i8, i8* %9, align 1
  %17 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %18 = bitcast %"class.std::ostreambuf_iterator"* %12 to i8*
  %19 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %18, i8* align 8 %19, i64 16, i1 false)
  %20 = bitcast %"class.std::num_put"* %14 to void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i32)***
  %21 = load void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i32)**, void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i32)*** %20, align 8
  %22 = getelementptr inbounds void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i32)*, void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i32)** %21, i64 9
  %23 = load void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i32)*, void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i32)** %22, align 8
  call void %23(%"class.std::num_put"* %14, %"class.std::ostreambuf_iterator"* sret align 8 %1, %"class.std::ostreambuf_iterator"* %12, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %17, i8 %16, i32 %15)
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@D_J@Z"(%"class.std::num_put"* %0, %"class.std::ostreambuf_iterator"* noalias sret align 8 %1, %"class.std::ostreambuf_iterator"* %2, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %3, i8 %4, i64 %5) #1 comdat align 2 {
  %7 = alloca i8*, align 8
  %8 = alloca i64, align 8
  %9 = alloca i8, align 1
  %10 = alloca %"class.std::ios_base"*, align 8
  %11 = alloca %"class.std::num_put"*, align 8
  %12 = alloca %"class.std::ostreambuf_iterator", align 8
  %13 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  store i8* %13, i8** %7, align 8
  store i64 %5, i64* %8, align 8
  store i8 %4, i8* %9, align 1
  store %"class.std::ios_base"* %3, %"class.std::ios_base"** %10, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %11, align 8
  %14 = load %"class.std::num_put"*, %"class.std::num_put"** %11, align 8
  %15 = load i64, i64* %8, align 8
  %16 = load i8, i8* %9, align 1
  %17 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %18 = bitcast %"class.std::ostreambuf_iterator"* %12 to i8*
  %19 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %18, i8* align 8 %19, i64 16, i1 false)
  %20 = bitcast %"class.std::num_put"* %14 to void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i64)***
  %21 = load void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i64)**, void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i64)*** %20, align 8
  %22 = getelementptr inbounds void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i64)*, void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i64)** %21, i64 7
  %23 = load void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i64)*, void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, i64)** %22, align 8
  call void %23(%"class.std::num_put"* %14, %"class.std::ostreambuf_iterator"* sret align 8 %1, %"class.std::ostreambuf_iterator"* %12, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %17, i8 %16, i64 %15)
  ret void
}

; Function Attrs: noinline optnone uwtable
define linkonce_odr dso_local void @"?put@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DO@Z"(%"class.std::num_put"* %0, %"class.std::ostreambuf_iterator"* noalias sret align 8 %1, %"class.std::ostreambuf_iterator"* %2, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %3, i8 %4, double %5) #1 comdat align 2 {
  %7 = alloca i8*, align 8
  %8 = alloca double, align 8
  %9 = alloca i8, align 1
  %10 = alloca %"class.std::ios_base"*, align 8
  %11 = alloca %"class.std::num_put"*, align 8
  %12 = alloca %"class.std::ostreambuf_iterator", align 8
  %13 = bitcast %"class.std::ostreambuf_iterator"* %1 to i8*
  store i8* %13, i8** %7, align 8
  store double %5, double* %8, align 8
  store i8 %4, i8* %9, align 1
  store %"class.std::ios_base"* %3, %"class.std::ios_base"** %10, align 8
  store %"class.std::num_put"* %0, %"class.std::num_put"** %11, align 8
  %14 = load %"class.std::num_put"*, %"class.std::num_put"** %11, align 8
  %15 = load double, double* %8, align 8
  %16 = load i8, i8* %9, align 1
  %17 = load %"class.std::ios_base"*, %"class.std::ios_base"** %10, align 8
  %18 = bitcast %"class.std::ostreambuf_iterator"* %12 to i8*
  %19 = bitcast %"class.std::ostreambuf_iterator"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %18, i8* align 8 %19, i64 16, i1 false)
  %20 = bitcast %"class.std::num_put"* %14 to void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, double)***
  %21 = load void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, double)**, void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, double)*** %20, align 8
  %22 = getelementptr inbounds void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, double)*, void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, double)** %21, i64 4
  %23 = load void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, double)*, void (%"class.std::num_put"*, %"class.std::ostreambuf_iterator"*, %"class.std::ostreambuf_iterator"*, %"class.std::ios_base"*, i8, double)** %22, align 8
  call void %23(%"class.std::num_put"* %14, %"class.std::ostreambuf_iterator"* sret align 8 %1, %"class.std::ostreambuf_iterator"* %12, %"class.std::ios_base"* nonnull align 8 dereferenceable(72) %17, i8 %16, double %15)
  ret void
}

attributes #0 = { noinline norecurse optnone uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { noinline optnone uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #2 = { noinline uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #3 = { noinline nounwind optnone uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #4 = { "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #5 = { nounwind }
attributes #6 = { nobuiltin nounwind "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #7 = { argmemonly nounwind willreturn }
attributes #8 = { noinline noreturn optnone uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #9 = { noreturn "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #10 = { argmemonly nounwind willreturn writeonly }
attributes #11 = { nobuiltin allocsize(0) "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #12 = { nounwind "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #13 = { cold noreturn nounwind }
attributes #14 = { nobuiltin noinline nounwind optnone readnone uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #15 = { nobuiltin noinline nounwind optnone uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #16 = { nounwind readnone "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #17 = { nounwind readnone speculatable willreturn }
attributes #18 = { noreturn nounwind }
attributes #19 = { noreturn }
attributes #20 = { builtin nounwind }
attributes #21 = { allocsize(0) }
attributes #22 = { builtin allocsize(0) }
attributes #23 = { nounwind readnone }

!llvm.linker.options = !{!0, !1, !2, !3, !4}
!llvm.module.flags = !{!5, !6}
!llvm.ident = !{!7}

!0 = !{!"/FAILIFMISMATCH:\22_MSC_VER=1900\22"}
!1 = !{!"/FAILIFMISMATCH:\22_ITERATOR_DEBUG_LEVEL=0\22"}
!2 = !{!"/FAILIFMISMATCH:\22RuntimeLibrary=MT_StaticRelease\22"}
!3 = !{!"/DEFAULTLIB:libcpmt.lib"}
!4 = !{!"/FAILIFMISMATCH:\22_CRT_STDIO_ISO_WIDE_SPECIFIERS=0\22"}
!5 = !{i32 1, !"wchar_size", i32 2}
!6 = !{i32 7, !"PIC Level", i32 2}
!7 = !{!"clang version 11.0.1 (https://github.com/llvm/llvm-project 43ff75f2c3feef64f9d73328230d34dac8832a91)"}
!8 = !{!"branch_weights", i32 1, i32 1048575}
