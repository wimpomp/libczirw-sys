# libCZIrw-sys

Crate linking to [libCZIAPI](https://github.com/ZEISS/libczi).
This crate attempts to provide save wrappers to objects and functions in libCZIAPI.
Direct often unsafe access using pointer is available through the sys module.

By default, libCZIAPI will be statically linked. The feature 'dynamic' will switch it to dynamic linking.

This code is licensed with an MIT license, but Zeiss' libCZI which is included as a submodule has a LGPL license.