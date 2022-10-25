# Libraries

RuSTy does not currently have support for source based external libraries.
Source based libraries can be compiled together with the application as a normal files.

Precompiled libraries or system functions can be added using compilation flags or an entry in the `plc.json` file
System functions can also be added using [External Function](libraries/external_functions.md) for each POU in that library.

## Library Structure

A library is defined by : 
- A set of `st` interfaces, each interface represents a function that has been precompiled.
- A binary file for each architecture the library has been built for (`x86_64-linux-gnu`, `aarch64-linux-gnu`, ..)


## Using libraries in the `rustyc` command line

To include a library when using the rustyc command line interface, the include files could be added by the `-i` command
Each `POU`, `Global Variable`


## Libraries Configuration in `plc.json`


