// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.31.

// Section: imports

use super::*;
use crate::api::types::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate_io!();

// Section: dart2rust

impl CstDecode<_Joined> for usize {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> _Joined {
        CstDecode::<RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<_Joined>>>::cst_decode(self).rust_auto_opaque_decode_owned()
    }
}
impl CstDecode<_Split> for usize {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> _Split {
        CstDecode::<RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<_Split>>>::cst_decode(self).rust_auto_opaque_decode_owned()
    }
}
impl CstDecode<RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<_Joined>>>
    for usize
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(
        self,
    ) -> RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<_Joined>> {
        unsafe { decode_rust_opaque_nom(self as _) }
    }
}
impl CstDecode<RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<_Split>>>
    for usize
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(
        self,
    ) -> RustOpaqueNom<flutter_rust_bridge::for_generated::rust_async::RwLock<_Split>> {
        unsafe { decode_rust_opaque_nom(self as _) }
    }
}
impl CstDecode<String> for *mut wire_cst_list_prim_u_8_strict {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> String {
        let vec: Vec<u8> = self.cst_decode();
        String::from_utf8(vec).unwrap()
    }
}
impl CstDecode<crate::api::error::DecodeError> for *mut wire_cst_decode_error {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::error::DecodeError {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::api::error::DecodeError>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::api::error::EncodeError> for *mut wire_cst_encode_error {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::error::EncodeError {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::api::error::EncodeError>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::api::error::HeaderParseError> for *mut wire_cst_header_parse_error {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::error::HeaderParseError {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::api::error::HeaderParseError>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::api::types::SplitOptions> for *mut wire_cst_split_options {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::SplitOptions {
        let wrap = unsafe { flutter_rust_bridge::for_generated::box_from_leak_ptr(self) };
        CstDecode::<crate::api::types::SplitOptions>::cst_decode(*wrap).into()
    }
}
impl CstDecode<crate::api::error::DecodeError> for wire_cst_decode_error {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::error::DecodeError {
        match self.tag {
            0 => {
                let ans = unsafe { self.kind.UnableToDecodeHex };
                crate::api::error::DecodeError::UnableToDecodeHex(
                    ans.field0.cst_decode(),
                    ans.field1.cst_decode(),
                )
            }
            1 => {
                let ans = unsafe { self.kind.UnableToDecodeBase32 };
                crate::api::error::DecodeError::UnableToDecodeBase32(
                    ans.field0.cst_decode(),
                    ans.field1.cst_decode(),
                )
            }
            2 => {
                let ans = unsafe { self.kind.UnableToInflateZlib };
                crate::api::error::DecodeError::UnableToInflateZlib(ans.field0.cst_decode())
            }
            _ => unreachable!(),
        }
    }
}
impl CstDecode<crate::api::error::EncodeError> for wire_cst_encode_error {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::error::EncodeError {
        match self.tag {
            0 => crate::api::error::EncodeError::Empty,
            1 => {
                let ans = unsafe { self.kind.CompressionError };
                crate::api::error::EncodeError::CompressionError(ans.field0.cst_decode())
            }
            _ => unreachable!(),
        }
    }
}
impl CstDecode<crate::api::error::HeaderParseError> for wire_cst_header_parse_error {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::error::HeaderParseError {
        match self.tag {
            0 => crate::api::error::HeaderParseError::Empty,
            1 => {
                let ans = unsafe { self.kind.InvalidEncoding };
                crate::api::error::HeaderParseError::InvalidEncoding(ans.field0.cst_decode())
            }
            2 => {
                let ans = unsafe { self.kind.InvalidFileType };
                crate::api::error::HeaderParseError::InvalidFileType(ans.field0.cst_decode())
            }
            3 => crate::api::error::HeaderParseError::InvalidFixedHeader,
            4 => {
                let ans = unsafe { self.kind.InvalidHeaderSize };
                crate::api::error::HeaderParseError::InvalidHeaderSize(ans.field0.cst_decode())
            }
            5 => {
                let ans = unsafe { self.kind.InvalidHeaderParts };
                crate::api::error::HeaderParseError::InvalidHeaderParts(ans.field0.cst_decode())
            }
            _ => unreachable!(),
        }
    }
}
impl CstDecode<crate::api::error::JoinError> for wire_cst_join_error {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::error::JoinError {
        match self.tag {
            0 => crate::api::error::JoinError::Empty,
            1 => crate::api::error::JoinError::ConflictingHeaders,
            2 => {
                let ans = unsafe { self.kind.TooManyParts };
                crate::api::error::JoinError::TooManyParts(
                    ans.field0.cst_decode(),
                    ans.field1.cst_decode(),
                )
            }
            3 => {
                let ans = unsafe { self.kind.DuplicatePartWrongContent };
                crate::api::error::JoinError::DuplicatePartWrongContent(ans.field0.cst_decode())
            }
            4 => {
                let ans = unsafe { self.kind.PartWithNoData };
                crate::api::error::JoinError::PartWithNoData(ans.field0.cst_decode())
            }
            5 => {
                let ans = unsafe { self.kind.MissingPart };
                crate::api::error::JoinError::MissingPart(ans.field0.cst_decode())
            }
            6 => {
                let ans = unsafe { self.kind.HeaderParseError };
                crate::api::error::JoinError::HeaderParseError(ans.field0.cst_decode())
            }
            7 => {
                let ans = unsafe { self.kind.DecodeError };
                crate::api::error::JoinError::DecodeError(ans.field0.cst_decode())
            }
            _ => unreachable!(),
        }
    }
}
impl CstDecode<Vec<String>> for *mut wire_cst_list_String {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<String> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(CstDecode::cst_decode).collect()
    }
}
impl CstDecode<Vec<u8>> for *mut wire_cst_list_prim_u_8_loose {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<u8> {
        unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl CstDecode<Vec<u8>> for *mut wire_cst_list_prim_u_8_strict {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<u8> {
        unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl CstDecode<crate::api::error::SplitError> for wire_cst_split_error {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::error::SplitError {
        match self.tag {
            0 => crate::api::error::SplitError::Empty,
            1 => crate::api::error::SplitError::CannotFit,
            2 => {
                let ans = unsafe { self.kind.MaxSplitSizeTooLarge };
                crate::api::error::SplitError::MaxSplitSizeTooLarge(ans.field0.cst_decode())
            }
            3 => crate::api::error::SplitError::MinSplitTooSmall,
            4 => crate::api::error::SplitError::InvalidSplitRange,
            5 => crate::api::error::SplitError::InvalidVersionRange,
            6 => {
                let ans = unsafe { self.kind.EncodeError };
                crate::api::error::SplitError::EncodeError(ans.field0.cst_decode())
            }
            _ => unreachable!(),
        }
    }
}
impl CstDecode<crate::api::types::SplitOptions> for wire_cst_split_options {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::SplitOptions {
        crate::api::types::SplitOptions {
            encoding: self.encoding.cst_decode(),
            min_split_number: self.min_split_number.cst_decode(),
            max_split_number: self.max_split_number.cst_decode(),
            min_version: self.min_version.cst_decode(),
            max_version: self.max_version.cst_decode(),
        }
    }
}
impl NewWithNullPtr for wire_cst_decode_error {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: DecodeErrorKind { nil__: () },
        }
    }
}
impl Default for wire_cst_decode_error {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_encode_error {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: EncodeErrorKind { nil__: () },
        }
    }
}
impl Default for wire_cst_encode_error {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_header_parse_error {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: HeaderParseErrorKind { nil__: () },
        }
    }
}
impl Default for wire_cst_header_parse_error {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_join_error {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: JoinErrorKind { nil__: () },
        }
    }
}
impl Default for wire_cst_join_error {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_split_error {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: SplitErrorKind { nil__: () },
        }
    }
}
impl Default for wire_cst_split_error {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_split_options {
    fn new_with_null_ptr() -> Self {
        Self {
            encoding: Default::default(),
            min_split_number: Default::default(),
            max_split_number: Default::default(),
            min_version: Default::default(),
            max_version: Default::default(),
        }
    }
}
impl Default for wire_cst_split_options {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

#[no_mangle]
pub extern "C" fn frbgen_bbqr_dart_wire_Joined_try_new_from_parts(
    port_: i64,
    parts: *mut wire_cst_list_String,
) {
    wire_Joined_try_new_from_parts_impl(port_, parts)
}

#[no_mangle]
pub extern "C" fn frbgen_bbqr_dart_wire_Split_try_from_data(
    port_: i64,
    data: *mut wire_cst_list_prim_u_8_loose,
    file_type: i32,
    options: *mut wire_cst_split_options,
) {
    wire_Split_try_from_data_impl(port_, data, file_type, options)
}

#[no_mangle]
pub extern "C" fn frbgen_bbqr_dart_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLock_Joined(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<_Joined>>::increment_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_bbqr_dart_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLock_Joined(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<_Joined>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_bbqr_dart_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLock_Split(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<_Split>>::increment_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_bbqr_dart_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLock_Split(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        StdArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<_Split>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_bbqr_dart_cst_new_box_autoadd_decode_error() -> *mut wire_cst_decode_error
{
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wire_cst_decode_error::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn frbgen_bbqr_dart_cst_new_box_autoadd_encode_error() -> *mut wire_cst_encode_error
{
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wire_cst_encode_error::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn frbgen_bbqr_dart_cst_new_box_autoadd_header_parse_error(
) -> *mut wire_cst_header_parse_error {
    flutter_rust_bridge::for_generated::new_leak_box_ptr(
        wire_cst_header_parse_error::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn frbgen_bbqr_dart_cst_new_box_autoadd_split_options() -> *mut wire_cst_split_options
{
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wire_cst_split_options::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn frbgen_bbqr_dart_cst_new_list_String(len: i32) -> *mut wire_cst_list_String {
    let wrap = wire_cst_list_String {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(
            <*mut wire_cst_list_prim_u_8_strict>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn frbgen_bbqr_dart_cst_new_list_prim_u_8_loose(
    len: i32,
) -> *mut wire_cst_list_prim_u_8_loose {
    let ans = wire_cst_list_prim_u_8_loose {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn frbgen_bbqr_dart_cst_new_list_prim_u_8_strict(
    len: i32,
) -> *mut wire_cst_list_prim_u_8_strict {
    let ans = wire_cst_list_prim_u_8_strict {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(ans)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_decode_error {
    tag: i32,
    kind: DecodeErrorKind,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DecodeErrorKind {
    UnableToDecodeHex: wire_cst_DecodeError_UnableToDecodeHex,
    UnableToDecodeBase32: wire_cst_DecodeError_UnableToDecodeBase32,
    UnableToInflateZlib: wire_cst_DecodeError_UnableToInflateZlib,
    nil__: (),
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_DecodeError_UnableToDecodeHex {
    field0: usize,
    field1: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_DecodeError_UnableToDecodeBase32 {
    field0: usize,
    field1: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_DecodeError_UnableToInflateZlib {
    field0: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_encode_error {
    tag: i32,
    kind: EncodeErrorKind,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EncodeErrorKind {
    CompressionError: wire_cst_EncodeError_CompressionError,
    nil__: (),
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_EncodeError_CompressionError {
    field0: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_header_parse_error {
    tag: i32,
    kind: HeaderParseErrorKind,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HeaderParseErrorKind {
    InvalidEncoding: wire_cst_HeaderParseError_InvalidEncoding,
    InvalidFileType: wire_cst_HeaderParseError_InvalidFileType,
    InvalidHeaderSize: wire_cst_HeaderParseError_InvalidHeaderSize,
    InvalidHeaderParts: wire_cst_HeaderParseError_InvalidHeaderParts,
    nil__: (),
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_HeaderParseError_InvalidEncoding {
    field0: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_HeaderParseError_InvalidFileType {
    field0: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_HeaderParseError_InvalidHeaderSize {
    field0: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_HeaderParseError_InvalidHeaderParts {
    field0: *mut wire_cst_list_prim_u_8_strict,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_join_error {
    tag: i32,
    kind: JoinErrorKind,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JoinErrorKind {
    TooManyParts: wire_cst_JoinError_TooManyParts,
    DuplicatePartWrongContent: wire_cst_JoinError_DuplicatePartWrongContent,
    PartWithNoData: wire_cst_JoinError_PartWithNoData,
    MissingPart: wire_cst_JoinError_MissingPart,
    HeaderParseError: wire_cst_JoinError_HeaderParseError,
    DecodeError: wire_cst_JoinError_DecodeError,
    nil__: (),
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_JoinError_TooManyParts {
    field0: usize,
    field1: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_JoinError_DuplicatePartWrongContent {
    field0: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_JoinError_PartWithNoData {
    field0: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_JoinError_MissingPart {
    field0: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_JoinError_HeaderParseError {
    field0: *mut wire_cst_header_parse_error,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_JoinError_DecodeError {
    field0: *mut wire_cst_decode_error,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_String {
    ptr: *mut *mut wire_cst_list_prim_u_8_strict,
    len: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_prim_u_8_loose {
    ptr: *mut u8,
    len: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_prim_u_8_strict {
    ptr: *mut u8,
    len: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_split_error {
    tag: i32,
    kind: SplitErrorKind,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SplitErrorKind {
    MaxSplitSizeTooLarge: wire_cst_SplitError_MaxSplitSizeTooLarge,
    EncodeError: wire_cst_SplitError_EncodeError,
    nil__: (),
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_SplitError_MaxSplitSizeTooLarge {
    field0: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_SplitError_EncodeError {
    field0: *mut wire_cst_encode_error,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_split_options {
    encoding: i32,
    min_split_number: usize,
    max_split_number: usize,
    min_version: i32,
    max_version: i32,
}
