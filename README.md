# `icu-data.rlib`

`icu-data.rlib` is a library that makes available the Unicode Consortium's ICU (International
Components for Unicode) data repository without any C binding to libicu. At present, only the
UCM (UniCode Mapping) files (icu-data/charset) are handled.

Originally, this library was going to store the data uncompressed in Rust data structures.
However, this led to the library being over 1GB in size, and Cargo required 15GB of memory to
compile it. So, we're storing compressed versions in the library which are decompressed on
demand (see documentation of [`ucm::request_mapping_file`]).

Data is © 1991-2021 Unicode, Inc.

For the license to this data, see:
<https://github.com/unicode-org/icu-data/blob/main/LICENSE>

It is an MIT-style license.

This library is Apache 2 licensed and is under the umbrella of the MFEK (Modular Font Editor K)
project.

`icu-data.rlib` source code © 2021 Fredrick R. Brennan & MFEK Authors. See `AUTHORS`.
