#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(clippy::missing_safety_doc)]

#[cfg(docsrs)]
include!("lib_czi_api.rs");

#[cfg(not(docsrs))]
include!(concat!(env!("OUT_DIR"), "/lib_czi_api.rs"));
