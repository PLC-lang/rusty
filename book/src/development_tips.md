## Improving Compile Times

By default Rust uses the GNU Linker on Linux which compared to [lld](https://lld.llvm.org/) is slower by a margin of [~2x - 4x](https://llvm.org/devmtg/2016-10/slides/Ueyama-lld.pdf).
To improve compile times we can therefore use `lld`.
To do so you will need to run the `rusty/scripts/lld.sh` script inside the `rusty` root folder, i.e. by executing `./scripts/lld.sh`.
**Note** that the script was only tested on Ubuntu based distributions thus far.