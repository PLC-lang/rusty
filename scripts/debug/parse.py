def parse_llvm_bytestring(encoded: str):
    """
    Parse strings formatted like:
        "\04\0Cx\DA\CBM\CC\CC\03\00\04\1B\01\A6"
    """
    decoded = []
    while(encoded):
        # \ indicates next two chars are hex
        if encoded[0] == '\\':
            decoded.append(int(encoded[1:3], 16))
            encoded = encoded[3:] # skip the / and the two hex letters
        
        # ASCII letter has the value
        else:
            decoded.append(ord(encoded[0]))
            encoded = encoded[1:]

    return decoded

def parse_hex_string(hex_string):
    """
    Parse strings formatted like:
        0011223344..
    """
    return [int(hex_string[i:i+2], 16) for i in range(0, len(hex_string), 2)]

def parse_llvm_string_to_list(packed_string):
    """
    Unpack multiple strings formatted like:
        <length><string><length><string>...
    """
    values = []
    while (packed_string):
        next_string_length = packed_string[0]
        values.append(packed_string[1:next_string_length+1])
        packed_string = packed_string[next_string_length+1:]

    return values
