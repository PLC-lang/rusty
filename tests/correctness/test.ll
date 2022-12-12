%main = type { [4 x i32] }

@main_instance = global %main zeroinitializer
@__myArray__init = unnamed_addr constant [4 x i32] [i32 1, i32 2, i32 3, i32 4]

define void @target([4 x i32]* %0) {
entry:
  %target = alloca [4 x i32]*, align 8
  store [4 x i32]* %0, [4 x i32]** %target, align 8
  %deref = load [4 x i32]*, [4 x i32]** %target, align 8
  %1 = bitcast [4 x i32]* %deref to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 bitcast ([4 x i32]* @__myArray__init to i8*), i64 ptrtoint ([4 x i32]* getelementptr ([4 x i32], [4 x i32]* null, i32 1) to i64), i1 false)
  %deref1 = load [4 x i32]*, [4 x i32]** %target, align 8
  %tmpVar = getelementptr inbounds [4 x i32], [4 x i32]* %deref1, i32 0, i32 2
  %deref2 = load [4 x i32]*, [4 x i32]** %target, align 8
  %tmpVar3 = getelementptr inbounds [4 x i32], [4 x i32]* %deref2, i32 0, i32 2
  %load_tmpVar = load i32, i32* %tmpVar3, align 4
  %tmpVar4 = add i32 %load_tmpVar, 1
  store i32 %tmpVar4, i32* %tmpVar, align 4
  ret void
}

define void @main(%main* %0) {
entry:
  %arr = getelementptr inbounds %main, %main* %0, i32 0, i32 0
  %1 = alloca [4 x i32], align 4
  call void @target([4 x i32]* %1)
  store [4 x i32]* %1, [4 x i32]* %arr, align 8
  ret void
}

; Function Attrs: argmemonly nofree nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly %0, i8* noalias nocapture readonly %1, i64 %2, i1 immarg %3) #0