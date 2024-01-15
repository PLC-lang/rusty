#!/usr/bin/python3

"""
Useful for decoding function names in generated IR data.

WARNING: This script assumes all LEB128 values are one byte only!
"""

import zlib
import argparse

from parse import parse_llvm_bytestring

# This should be the encoded data under `__llvm_coverage_mapping`
parser = argparse.ArgumentParser()
parser.add_argument("encoded", help="encoded data under `__llvm_coverage_mapping`")
args = parser.parse_args()

encoded = args.encoded
decoded = parse_llvm_bytestring(encoded)

# Take off the headers
len_uncompressed = decoded.pop(0)
len_compressed = decoded.pop(0)
assert(len_compressed == len(decoded))

# Decompress and separate the function names
decompressed_function_names = zlib.decompress(bytes(decoded))
assert(len_uncompressed == len(decompressed_function_names))
function_names = decompressed_function_names.split(b"\x01")

# Display
print(f'Len(uncompressed): {len_uncompressed}')
print(f'Len(compressed): {len_compressed}')
print(f'Function names: {", ".join([function_name.decode() for function_name in function_names])}')
