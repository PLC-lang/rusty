searchState.loadedDescShard("bytes", 0, "Provides abstractions for working with bytes.\nA cheaply cloneable and sliceable chunk of contiguous …\nA unique reference to a contiguous slice of memory.\nUtilities for working with buffers.\nReturns the number of bytes the <code>BytesMut</code> can hold without …\nClears the buffer, removing all data.\nClears the buffer, removing all data. Existing capacity is …\nCreates <code>Bytes</code> instance from slice, by copying it.\nAppends given bytes to this <code>BytesMut</code>.\nConverts <code>self</code> into an immutable <code>Bytes</code>.\nReturns the argument unchanged.\nConvert self into <code>BytesMut</code>.\nReturns the argument unchanged.\nCreates a new <code>Bytes</code> from a static slice.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns true if the <code>Bytes</code> has a length of 0.\nReturns true if the <code>BytesMut</code> has a length of 0.\nReturns true if this is the only reference to the data.\nReturns the number of bytes contained in this <code>Bytes</code>.\nReturns the number of bytes contained in this <code>BytesMut</code>.\nCreates a new empty <code>Bytes</code>.\nCreates a new <code>BytesMut</code> with default capacity.\nReserves capacity for at least <code>additional</code> more bytes to be …\nResizes the buffer so that <code>len</code> is equal to <code>new_len</code>.\nSets the length of the buffer.\nReturns a slice of self for the provided range.\nReturns a slice of self that is equivalent to the given …\nReturns the remaining spare capacity of the buffer as a …\nRemoves the bytes from the current view, returning them in …\nSplits the bytes into two at the given index.\nSplits the bytes into two at the given index.\nSplits the bytes into two at the given index.\nSplits the buffer into two at the given index.\nShortens the buffer, keeping the first <code>len</code> bytes and …\nShortens the buffer, keeping the first <code>len</code> bytes and …\nTry to convert self into <code>BytesMut</code>.\nAttempts to cheaply reclaim already allocated capacity for …\nAbsorbs a <code>BytesMut</code> that was previously split off.\nCreates a new <code>BytesMut</code> with the specified capacity.\nCreates a new <code>BytesMut</code> containing <code>len</code> zeros.\nRead bytes from a buffer.\nA trait for values that provide sequential write access to …\nA <code>Chain</code> sequences two buffers.\nIterator over the bytes contained by the buffer.\nA <code>BufMut</code> adapter which limits the amount of bytes that can …\nA <code>Buf</code> adapter which implements <code>io::Read</code> for the inner …\nA <code>Buf</code> adapter which limits the bytes read from an …\nUninitialized byte slice.\nA <code>BufMut</code> adapter which implements <code>io::Write</code> for the inner …\nAdvance the internal cursor of the Buf\nAdvance the internal cursor of the BufMut\nReturn a raw pointer to the slice’s buffer.\nReturn a <code>&amp;mut [MaybeUninit&lt;u8&gt;]</code> to this slice’s buffer.\nCreates an adaptor which will chain this buffer with …\nCreates an adaptor which will chain this buffer with …\nCreates an adapter which will chain this buffer with …\nCreates an adapter which will chain this buffer with …\nReturns a slice starting at the current position and of …\nReturns a mutable slice starting at the current BufMut …\nFills <code>dst</code> with potentially multiple slices starting at <code>self</code>…\nFills <code>dst</code> with potentially multiple slices starting at <code>self</code>…\nCopies bytes  from <code>src</code> into <code>self</code>.\nConsumes <code>len</code> bytes inside self and returns new instance of …\nConsumes <code>len</code> bytes inside self and returns new instance of …\nCopies bytes from <code>self</code> into <code>dst</code>.\nCopies bytes from <code>self</code> into <code>dst</code>.\nGets a mutable reference to the first underlying <code>Buf</code>.\nGets a reference to the first underlying <code>Buf</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreate a <code>&amp;mut UninitSlice</code> from a pointer and a length.\nGets an IEEE754 single-precision (4 bytes) floating point …\nGets an IEEE754 single-precision (4 bytes) floating point …\nGets an IEEE754 single-precision (4 bytes) floating point …\nGets an IEEE754 single-precision (4 bytes) floating point …\nGets an IEEE754 single-precision (4 bytes) floating point …\nGets an IEEE754 single-precision (4 bytes) floating point …\nGets an IEEE754 double-precision (8 bytes) floating point …\nGets an IEEE754 double-precision (8 bytes) floating point …\nGets an IEEE754 double-precision (8 bytes) floating point …\nGets an IEEE754 double-precision (8 bytes) floating point …\nGets an IEEE754 double-precision (8 bytes) floating point …\nGets an IEEE754 double-precision (8 bytes) floating point …\nGets a signed 128 bit integer from <code>self</code> in big-endian byte …\nGets a signed 128 bit integer from <code>self</code> in big-endian byte …\nGets a signed 128 bit integer from <code>self</code> in little-endian …\nGets a signed 128 bit integer from <code>self</code> in little-endian …\nGets a signed 128 bit integer from <code>self</code> in native-endian …\nGets a signed 128 bit integer from <code>self</code> in native-endian …\nGets a signed 16 bit integer from <code>self</code> in big-endian byte …\nGets a signed 16 bit integer from <code>self</code> in big-endian byte …\nGets a signed 16 bit integer from <code>self</code> in little-endian …\nGets a signed 16 bit integer from <code>self</code> in little-endian …\nGets a signed 16 bit integer from <code>self</code> in native-endian …\nGets a signed 16 bit integer from <code>self</code> in native-endian …\nGets a signed 32 bit integer from <code>self</code> in big-endian byte …\nGets a signed 32 bit integer from <code>self</code> in big-endian byte …\nGets a signed 32 bit integer from <code>self</code> in little-endian …\nGets a signed 32 bit integer from <code>self</code> in little-endian …\nGets a signed 32 bit integer from <code>self</code> in native-endian …\nGets a signed 32 bit integer from <code>self</code> in native-endian …\nGets a signed 64 bit integer from <code>self</code> in big-endian byte …\nGets a signed 64 bit integer from <code>self</code> in big-endian byte …\nGets a signed 64 bit integer from <code>self</code> in little-endian …\nGets a signed 64 bit integer from <code>self</code> in little-endian …\nGets a signed 64 bit integer from <code>self</code> in native-endian …\nGets a signed 64 bit integer from <code>self</code> in native-endian …\nGets a signed 8 bit integer from <code>self</code>.\nGets a signed 8 bit integer from <code>self</code>.\nGets a signed n-byte integer from <code>self</code> in big-endian byte …\nGets a signed n-byte integer from <code>self</code> in big-endian byte …\nGets a signed n-byte integer from <code>self</code> in little-endian …\nGets a signed n-byte integer from <code>self</code> in little-endian …\nGets a signed n-byte integer from <code>self</code> in native-endian …\nGets a signed n-byte integer from <code>self</code> in native-endian …\nGets a mutable reference to the underlying <code>Buf</code>.\nGets a mutable reference to the underlying <code>BufMut</code>.\nGets a mutable reference to the underlying <code>Buf</code>.\nGets a mutable reference to the underlying <code>Buf</code>.\nGets a mutable reference to the underlying <code>BufMut</code>.\nGets a reference to the underlying <code>Buf</code>.\nGets a reference to the underlying <code>BufMut</code>.\nGets a reference to the underlying <code>Buf</code>.\nGets a reference to the underlying <code>Buf</code>.\nGets a reference to the underlying <code>BufMut</code>.\nGets an unsigned 128 bit integer from <code>self</code> in big-endian …\nGets an unsigned 128 bit integer from <code>self</code> in big-endian …\nGets an unsigned 128 bit integer from <code>self</code> in …\nGets an unsigned 128 bit integer from <code>self</code> in …\nGets an unsigned 128 bit integer from <code>self</code> in …\nGets an unsigned 128 bit integer from <code>self</code> in …\nGets an unsigned 16 bit integer from <code>self</code> in big-endian …\nGets an unsigned 16 bit integer from <code>self</code> in big-endian …\nGets an unsigned 16 bit integer from <code>self</code> in little-endian …\nGets an unsigned 16 bit integer from <code>self</code> in little-endian …\nGets an unsigned 16 bit integer from <code>self</code> in native-endian …\nGets an unsigned 16 bit integer from <code>self</code> in native-endian …\nGets an unsigned 32 bit integer from <code>self</code> in the …\nGets an unsigned 32 bit integer from <code>self</code> in the …\nGets an unsigned 32 bit integer from <code>self</code> in the …\nGets an unsigned 32 bit integer from <code>self</code> in the …\nGets an unsigned 32 bit integer from <code>self</code> in native-endian …\nGets an unsigned 32 bit integer from <code>self</code> in native-endian …\nGets an unsigned 64 bit integer from <code>self</code> in big-endian …\nGets an unsigned 64 bit integer from <code>self</code> in big-endian …\nGets an unsigned 64 bit integer from <code>self</code> in little-endian …\nGets an unsigned 64 bit integer from <code>self</code> in little-endian …\nGets an unsigned 64 bit integer from <code>self</code> in native-endian …\nGets an unsigned 64 bit integer from <code>self</code> in native-endian …\nGets an unsigned 8 bit integer from <code>self</code>.\nGets an unsigned 8 bit integer from <code>self</code>.\nGets an unsigned n-byte integer from <code>self</code> in big-endian …\nGets an unsigned n-byte integer from <code>self</code> in big-endian …\nGets an unsigned n-byte integer from <code>self</code> in little-endian …\nGets an unsigned n-byte integer from <code>self</code> in little-endian …\nGets an unsigned n-byte integer from <code>self</code> in native-endian …\nGets an unsigned n-byte integer from <code>self</code> in native-endian …\nReturns true if there are any more bytes to consume\nReturns true if there are any more bytes to consume\nReturns true if there is space in <code>self</code> for more bytes.\nReturns true if there is space in <code>self</code> for more bytes.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConsumes this <code>Chain</code>, returning the underlying values.\nConsumes this <code>IntoIter</code>, returning the underlying value.\nConsumes this <code>Limit</code>, returning the underlying value.\nConsumes this <code>Reader</code>, returning the underlying value.\nConsumes this <code>Take</code>, returning the underlying value.\nConsumes this <code>Writer</code>, returning the underlying value.\nGets a mutable reference to the last underlying <code>Buf</code>.\nGets a reference to the last underlying <code>Buf</code>.\nReturns the number of bytes in the slice.\nCreates an adaptor which can write at most <code>limit</code> bytes to …\nCreates an adaptor which can write at most <code>limit</code> bytes to …\nReturns the maximum number of bytes that can be written\nReturns the maximum number of bytes that can be read.\nCreates a <code>&amp;mut UninitSlice</code> wrapping a slice of initialised …\nCreates an iterator over the bytes contained by the buffer.\nTransfer bytes into <code>self</code> from <code>src</code> and advance the cursor …\nTransfer bytes into <code>self</code> from <code>src</code> and advance the cursor …\nPut <code>cnt</code> bytes <code>val</code> into <code>self</code>.\nPut <code>cnt</code> bytes <code>val</code> into <code>self</code>.\nWrites  an IEEE754 single-precision (4 bytes) floating …\nWrites  an IEEE754 single-precision (4 bytes) floating …\nWrites  an IEEE754 single-precision (4 bytes) floating …\nWrites  an IEEE754 single-precision (4 bytes) floating …\nWrites an IEEE754 single-precision (4 bytes) floating …\nWrites an IEEE754 single-precision (4 bytes) floating …\nWrites  an IEEE754 double-precision (8 bytes) floating …\nWrites  an IEEE754 double-precision (8 bytes) floating …\nWrites  an IEEE754 double-precision (8 bytes) floating …\nWrites  an IEEE754 double-precision (8 bytes) floating …\nWrites  an IEEE754 double-precision (8 bytes) floating …\nWrites  an IEEE754 double-precision (8 bytes) floating …\nWrites a signed 128 bit integer to <code>self</code> in the big-endian …\nWrites a signed 128 bit integer to <code>self</code> in the big-endian …\nWrites a signed 128 bit integer to <code>self</code> in little-endian …\nWrites a signed 128 bit integer to <code>self</code> in little-endian …\nWrites a signed 128 bit integer to <code>self</code> in native-endian …\nWrites a signed 128 bit integer to <code>self</code> in native-endian …\nWrites a signed 16 bit integer to <code>self</code> in big-endian byte …\nWrites a signed 16 bit integer to <code>self</code> in big-endian byte …\nWrites a signed 16 bit integer to <code>self</code> in little-endian …\nWrites a signed 16 bit integer to <code>self</code> in little-endian …\nWrites a signed 16 bit integer to <code>self</code> in native-endian …\nWrites a signed 16 bit integer to <code>self</code> in native-endian …\nWrites a signed 32 bit integer to <code>self</code> in big-endian byte …\nWrites a signed 32 bit integer to <code>self</code> in big-endian byte …\nWrites a signed 32 bit integer to <code>self</code> in little-endian …\nWrites a signed 32 bit integer to <code>self</code> in little-endian …\nWrites a signed 32 bit integer to <code>self</code> in native-endian …\nWrites a signed 32 bit integer to <code>self</code> in native-endian …\nWrites a signed 64 bit integer to <code>self</code> in the big-endian …\nWrites a signed 64 bit integer to <code>self</code> in the big-endian …\nWrites a signed 64 bit integer to <code>self</code> in little-endian …\nWrites a signed 64 bit integer to <code>self</code> in little-endian …\nWrites a signed 64 bit integer to <code>self</code> in native-endian …\nWrites a signed 64 bit integer to <code>self</code> in native-endian …\nWrites a signed 8 bit integer to <code>self</code>.\nWrites a signed 8 bit integer to <code>self</code>.\nWrites low <code>nbytes</code> of a signed integer to <code>self</code> in …\nWrites low <code>nbytes</code> of a signed integer to <code>self</code> in …\nWrites low <code>nbytes</code> of a signed integer to <code>self</code> in …\nWrites low <code>nbytes</code> of a signed integer to <code>self</code> in …\nWrites low <code>nbytes</code> of a signed integer to <code>self</code> in …\nWrites low <code>nbytes</code> of a signed integer to <code>self</code> in …\nTransfer bytes into <code>self</code> from <code>src</code> and advance the cursor …\nTransfer bytes into <code>self</code> from <code>src</code> and advance the cursor …\nWrites an unsigned 128 bit integer to <code>self</code> in the …\nWrites an unsigned 128 bit integer to <code>self</code> in the …\nWrites an unsigned 128 bit integer to <code>self</code> in …\nWrites an unsigned 128 bit integer to <code>self</code> in …\nWrites an unsigned 128 bit integer to <code>self</code> in …\nWrites an unsigned 128 bit integer to <code>self</code> in …\nWrites an unsigned 16 bit integer to <code>self</code> in big-endian …\nWrites an unsigned 16 bit integer to <code>self</code> in big-endian …\nWrites an unsigned 16 bit integer to <code>self</code> in little-endian …\nWrites an unsigned 16 bit integer to <code>self</code> in little-endian …\nWrites an unsigned 16 bit integer to <code>self</code> in native-endian …\nWrites an unsigned 16 bit integer to <code>self</code> in native-endian …\nWrites an unsigned 32 bit integer to <code>self</code> in big-endian …\nWrites an unsigned 32 bit integer to <code>self</code> in big-endian …\nWrites an unsigned 32 bit integer to <code>self</code> in little-endian …\nWrites an unsigned 32 bit integer to <code>self</code> in little-endian …\nWrites an unsigned 32 bit integer to <code>self</code> in native-endian …\nWrites an unsigned 32 bit integer to <code>self</code> in native-endian …\nWrites an unsigned 64 bit integer to <code>self</code> in the …\nWrites an unsigned 64 bit integer to <code>self</code> in the …\nWrites an unsigned 64 bit integer to <code>self</code> in little-endian …\nWrites an unsigned 64 bit integer to <code>self</code> in little-endian …\nWrites an unsigned 64 bit integer to <code>self</code> in native-endian …\nWrites an unsigned 64 bit integer to <code>self</code> in native-endian …\nWrites an unsigned 8 bit integer to <code>self</code>.\nWrites an unsigned 8 bit integer to <code>self</code>.\nWrites an unsigned n-byte integer to <code>self</code> in big-endian …\nWrites an unsigned n-byte integer to <code>self</code> in big-endian …\nWrites an unsigned n-byte integer to <code>self</code> in the …\nWrites an unsigned n-byte integer to <code>self</code> in the …\nWrites an unsigned n-byte integer to <code>self</code> in the …\nWrites an unsigned n-byte integer to <code>self</code> in the …\nCreates an adaptor which implements the <code>Read</code> trait for <code>self</code>…\nCreates an adaptor which implements the <code>Read</code> trait for <code>self</code>…\nReturns the number of bytes between the current position …\nReturns the number of bytes that can be written from the …\nSets the maximum number of bytes that can be written.\nSets the maximum number of bytes that can be read.\nCreates an adaptor which will read at most <code>limit</code> bytes …\nCreates an adaptor which will read at most <code>limit</code> bytes …\nCreates a <code>&amp;mut UninitSlice</code> wrapping a slice of …\nWrite a single byte at the specified offset.\nCreates an adaptor which implements the <code>Write</code> trait for …\nCreates an adaptor which implements the <code>Write</code> trait for …")