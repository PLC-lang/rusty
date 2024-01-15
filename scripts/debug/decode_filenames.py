#!/usr/bin/python3

"""
Useful for decoding filenames in generated IR data.

WARNING: This script assumes all LEB128 values are one byte only!
"""

import zlib
import argparse

from parse import parse_llvm_bytestring, parse_llvm_string_to_list, parse_hex_string

# This should be the encoded data under `__llvm_prf_nm`
parser = argparse.ArgumentParser()
parser.add_argument("encoded", help="encoded data under `__llvm_prf_nm`")
args = parser.parse_args()

encoded = args.encoded
decoded = parse_llvm_bytestring(encoded)

# Take off the headers
num_files = decoded.pop(0)
len_uncompressed = decoded.pop(0)
len_compressed = decoded.pop(0)
assert(len_compressed == len(decoded))

# Decompress and separate the filenames
decoded_filenames = zlib.decompress(bytes(decoded))
assert(len(decoded_filenames) == len_uncompressed)
filenames = parse_llvm_string_to_list(decoded_filenames)

# Display
print(f'Files: {num_files}')
print(f'Len(uncompressed): {len_uncompressed}')
print(f'Len(compressed): {len_compressed}')
print(f'Filenames: {", ".join([filename.decode() for filename in filenames])}')
