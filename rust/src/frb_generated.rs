// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.33.

#![allow(
    non_camel_case_types,
    unused,
    non_snake_case,
    clippy::needless_return,
    clippy::redundant_closure_call,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::unused_unit,
    clippy::double_parens,
    clippy::let_and_return,
    clippy::too_many_arguments,
    clippy::match_single_binding
)]

// Section: imports

use crate::api::types::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate!(
    default_stream_sink_codec = DcoCodec,
    default_rust_opaque = RustOpaqueNom,
    default_rust_auto_opaque = RustAutoOpaqueNom,
);
pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_VERSION: &str = "2.0.0-dev.33";
pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_CONTENT_HASH: i32 = 186225969;

// Section: executor

flutter_rust_bridge::frb_generated_default_handler!();

// Section: wire_funcs

fn wire_Split_try_new_from_data_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    data: impl CstDecode<Vec<u8>>,
    file_type: impl CstDecode<crate::api::types::FileType>,
    options: impl CstDecode<crate::api::types::SplitOptions>,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "Split_try_new_from_data",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_data = data.cst_decode();
            let api_file_type = file_type.cst_decode();
            let api_options = options.cst_decode();
            move |context| {
                transform_result_dco((move || {
                    crate::api::types::_Split::try_new_from_data(
                        api_data,
                        api_file_type,
                        api_options,
                    )
                })())
            }
        },
    )
}

// Section: static_checks

#[allow(clippy::unnecessary_literal_unwrap)]
const _: fn() = || {
    match None::<crate::api::types::EncodeError>.unwrap() {
        crate::api::types::EncodeError::Empty => {}
        crate::api::types::EncodeError::CompressionError(field0) => {
            let _: String = field0;
        }
    }
    match None::<crate::api::types::SplitError>.unwrap() {
        crate::api::types::SplitError::Empty => {}
        crate::api::types::SplitError::CannotFit => {}
        crate::api::types::SplitError::MaxSplitSizeTooLarge(field0) => {
            let _: usize = field0;
        }
        crate::api::types::SplitError::MinSplitTooSmall => {}
        crate::api::types::SplitError::InvalidSplitRange => {}
        crate::api::types::SplitError::InvalidVersionRange => {}
        crate::api::types::SplitError::EncodeError(field0) => {
            let _: crate::api::types::EncodeError = field0;
        }
    }
    {
        let SplitOptions = None::<crate::api::types::SplitOptions>.unwrap();
        let _: crate::api::types::Encoding = SplitOptions.encoding;
        let _: usize = SplitOptions.min_split_number;
        let _: usize = SplitOptions.max_split_number;
        let _: crate::api::types::Version = SplitOptions.min_version;
        let _: crate::api::types::Version = SplitOptions.max_version;
    }
};

// Section: dart2rust

impl CstDecode<crate::api::types::Encoding> for i32 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::Encoding {
        match self {
            0 => crate::api::types::Encoding::Hex,
            1 => crate::api::types::Encoding::Base32,
            2 => crate::api::types::Encoding::Zlib,
            _ => unreachable!("Invalid variant for Encoding: {}", self),
        }
    }
}
impl CstDecode<crate::api::types::FileType> for i32 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::FileType {
        match self {
            0 => crate::api::types::FileType::Psbt,
            1 => crate::api::types::FileType::Transaction,
            2 => crate::api::types::FileType::Json,
            3 => crate::api::types::FileType::Cbor,
            4 => crate::api::types::FileType::UnicodeText,
            _ => unreachable!("Invalid variant for FileType: {}", self),
        }
    }
}
impl CstDecode<i32> for i32 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> i32 {
        self
    }
}
impl CstDecode<u8> for u8 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> u8 {
        self
    }
}
impl CstDecode<usize> for usize {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> usize {
        self
    }
}
impl CstDecode<crate::api::types::Version> for i32 {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::Version {
        match self {
            0 => crate::api::types::Version::V01,
            1 => crate::api::types::Version::V02,
            2 => crate::api::types::Version::V03,
            3 => crate::api::types::Version::V04,
            4 => crate::api::types::Version::V05,
            5 => crate::api::types::Version::V06,
            6 => crate::api::types::Version::V07,
            7 => crate::api::types::Version::V08,
            8 => crate::api::types::Version::V09,
            9 => crate::api::types::Version::V10,
            10 => crate::api::types::Version::V11,
            11 => crate::api::types::Version::V12,
            12 => crate::api::types::Version::V13,
            13 => crate::api::types::Version::V14,
            14 => crate::api::types::Version::V15,
            15 => crate::api::types::Version::V16,
            16 => crate::api::types::Version::V17,
            17 => crate::api::types::Version::V18,
            18 => crate::api::types::Version::V19,
            19 => crate::api::types::Version::V20,
            20 => crate::api::types::Version::V21,
            21 => crate::api::types::Version::V22,
            22 => crate::api::types::Version::V23,
            23 => crate::api::types::Version::V24,
            24 => crate::api::types::Version::V25,
            25 => crate::api::types::Version::V26,
            26 => crate::api::types::Version::V27,
            27 => crate::api::types::Version::V28,
            28 => crate::api::types::Version::V29,
            29 => crate::api::types::Version::V30,
            30 => crate::api::types::Version::V31,
            31 => crate::api::types::Version::V32,
            32 => crate::api::types::Version::V33,
            33 => crate::api::types::Version::V34,
            34 => crate::api::types::Version::V35,
            35 => crate::api::types::Version::V36,
            36 => crate::api::types::Version::V37,
            37 => crate::api::types::Version::V38,
            38 => crate::api::types::Version::V39,
            39 => crate::api::types::Version::V40,
            _ => unreachable!("Invalid variant for Version: {}", self),
        }
    }
}
impl SseDecode for _Split {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <RustOpaqueNom<
            flutter_rust_bridge::for_generated::RustAutoOpaqueInner<_Split>,
        >>::sse_decode(deserializer);
        return inner.rust_auto_opaque_decode_owned();
    }
}

impl SseDecode for RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<_Split>> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <usize>::sse_decode(deserializer);
        return unsafe { decode_rust_opaque_nom(inner) };
    }
}

impl SseDecode for String {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <Vec<u8>>::sse_decode(deserializer);
        return String::from_utf8(inner).unwrap();
    }
}

impl SseDecode for crate::api::types::EncodeError {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut tag_ = <i32>::sse_decode(deserializer);
        match tag_ {
            0 => {
                return crate::api::types::EncodeError::Empty;
            }
            1 => {
                let mut var_field0 = <String>::sse_decode(deserializer);
                return crate::api::types::EncodeError::CompressionError(var_field0);
            }
            _ => {
                unimplemented!("");
            }
        }
    }
}

impl SseDecode for crate::api::types::Encoding {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => crate::api::types::Encoding::Hex,
            1 => crate::api::types::Encoding::Base32,
            2 => crate::api::types::Encoding::Zlib,
            _ => unreachable!("Invalid variant for Encoding: {}", inner),
        };
    }
}

impl SseDecode for crate::api::types::FileType {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => crate::api::types::FileType::Psbt,
            1 => crate::api::types::FileType::Transaction,
            2 => crate::api::types::FileType::Json,
            3 => crate::api::types::FileType::Cbor,
            4 => crate::api::types::FileType::UnicodeText,
            _ => unreachable!("Invalid variant for FileType: {}", inner),
        };
    }
}

impl SseDecode for i32 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_i32::<NativeEndian>().unwrap()
    }
}

impl SseDecode for Vec<u8> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut len_ = <i32>::sse_decode(deserializer);
        let mut ans_ = vec![];
        for idx_ in 0..len_ {
            ans_.push(<u8>::sse_decode(deserializer));
        }
        return ans_;
    }
}

impl SseDecode for crate::api::types::SplitError {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut tag_ = <i32>::sse_decode(deserializer);
        match tag_ {
            0 => {
                return crate::api::types::SplitError::Empty;
            }
            1 => {
                return crate::api::types::SplitError::CannotFit;
            }
            2 => {
                let mut var_field0 = <usize>::sse_decode(deserializer);
                return crate::api::types::SplitError::MaxSplitSizeTooLarge(var_field0);
            }
            3 => {
                return crate::api::types::SplitError::MinSplitTooSmall;
            }
            4 => {
                return crate::api::types::SplitError::InvalidSplitRange;
            }
            5 => {
                return crate::api::types::SplitError::InvalidVersionRange;
            }
            6 => {
                let mut var_field0 = <crate::api::types::EncodeError>::sse_decode(deserializer);
                return crate::api::types::SplitError::EncodeError(var_field0);
            }
            _ => {
                unimplemented!("");
            }
        }
    }
}

impl SseDecode for crate::api::types::SplitOptions {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut var_encoding = <crate::api::types::Encoding>::sse_decode(deserializer);
        let mut var_minSplitNumber = <usize>::sse_decode(deserializer);
        let mut var_maxSplitNumber = <usize>::sse_decode(deserializer);
        let mut var_minVersion = <crate::api::types::Version>::sse_decode(deserializer);
        let mut var_maxVersion = <crate::api::types::Version>::sse_decode(deserializer);
        return crate::api::types::SplitOptions {
            encoding: var_encoding,
            min_split_number: var_minSplitNumber,
            max_split_number: var_maxSplitNumber,
            min_version: var_minVersion,
            max_version: var_maxVersion,
        };
    }
}

impl SseDecode for u8 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u8().unwrap()
    }
}

impl SseDecode for usize {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u64::<NativeEndian>().unwrap() as _
    }
}

impl SseDecode for crate::api::types::Version {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        let mut inner = <i32>::sse_decode(deserializer);
        return match inner {
            0 => crate::api::types::Version::V01,
            1 => crate::api::types::Version::V02,
            2 => crate::api::types::Version::V03,
            3 => crate::api::types::Version::V04,
            4 => crate::api::types::Version::V05,
            5 => crate::api::types::Version::V06,
            6 => crate::api::types::Version::V07,
            7 => crate::api::types::Version::V08,
            8 => crate::api::types::Version::V09,
            9 => crate::api::types::Version::V10,
            10 => crate::api::types::Version::V11,
            11 => crate::api::types::Version::V12,
            12 => crate::api::types::Version::V13,
            13 => crate::api::types::Version::V14,
            14 => crate::api::types::Version::V15,
            15 => crate::api::types::Version::V16,
            16 => crate::api::types::Version::V17,
            17 => crate::api::types::Version::V18,
            18 => crate::api::types::Version::V19,
            19 => crate::api::types::Version::V20,
            20 => crate::api::types::Version::V21,
            21 => crate::api::types::Version::V22,
            22 => crate::api::types::Version::V23,
            23 => crate::api::types::Version::V24,
            24 => crate::api::types::Version::V25,
            25 => crate::api::types::Version::V26,
            26 => crate::api::types::Version::V27,
            27 => crate::api::types::Version::V28,
            28 => crate::api::types::Version::V29,
            29 => crate::api::types::Version::V30,
            30 => crate::api::types::Version::V31,
            31 => crate::api::types::Version::V32,
            32 => crate::api::types::Version::V33,
            33 => crate::api::types::Version::V34,
            34 => crate::api::types::Version::V35,
            35 => crate::api::types::Version::V36,
            36 => crate::api::types::Version::V37,
            37 => crate::api::types::Version::V38,
            38 => crate::api::types::Version::V39,
            39 => crate::api::types::Version::V40,
            _ => unreachable!("Invalid variant for Version: {}", inner),
        };
    }
}

impl SseDecode for bool {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_u8().unwrap() != 0
    }
}

fn pde_ffi_dispatcher_primary_impl(
    func_id: i32,
    port: flutter_rust_bridge::for_generated::MessagePort,
    ptr: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len: i32,
    data_len: i32,
) {
    // Codec=Pde (Serialization + dispatch), see doc to use other codecs
    match func_id {
        _ => unreachable!(),
    }
}

fn pde_ffi_dispatcher_sync_impl(
    func_id: i32,
    ptr: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len: i32,
    data_len: i32,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartSse {
    // Codec=Pde (Serialization + dispatch), see doc to use other codecs
    match func_id {
        _ => unreachable!(),
    }
}

// Section: rust2dart

// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for FrbWrapper<_Split> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<_, StdArc<_>>(self.0)
            .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive for FrbWrapper<_Split> {}

impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<_Split>> for _Split {
    fn into_into_dart(self) -> FrbWrapper<_Split> {
        self.into()
    }
}

// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for FrbWrapper<crate::api::types::EncodeError> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            crate::api::types::EncodeError::Empty => [0.into_dart()].into_dart(),
            crate::api::types::EncodeError::CompressionError(field0) => {
                [1.into_dart(), field0.into_into_dart().into_dart()].into_dart()
            }
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<crate::api::types::EncodeError>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<crate::api::types::EncodeError>>
    for crate::api::types::EncodeError
{
    fn into_into_dart(self) -> FrbWrapper<crate::api::types::EncodeError> {
        self.into()
    }
}
// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for FrbWrapper<crate::api::types::Encoding> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            crate::api::types::Encoding::Hex => 0.into_dart(),
            crate::api::types::Encoding::Base32 => 1.into_dart(),
            crate::api::types::Encoding::Zlib => 2.into_dart(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<crate::api::types::Encoding>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<crate::api::types::Encoding>>
    for crate::api::types::Encoding
{
    fn into_into_dart(self) -> FrbWrapper<crate::api::types::Encoding> {
        self.into()
    }
}
// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for FrbWrapper<crate::api::types::FileType> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            crate::api::types::FileType::Psbt => 0.into_dart(),
            crate::api::types::FileType::Transaction => 1.into_dart(),
            crate::api::types::FileType::Json => 2.into_dart(),
            crate::api::types::FileType::Cbor => 3.into_dart(),
            crate::api::types::FileType::UnicodeText => 4.into_dart(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<crate::api::types::FileType>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<crate::api::types::FileType>>
    for crate::api::types::FileType
{
    fn into_into_dart(self) -> FrbWrapper<crate::api::types::FileType> {
        self.into()
    }
}
// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for FrbWrapper<crate::api::types::SplitError> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            crate::api::types::SplitError::Empty => [0.into_dart()].into_dart(),
            crate::api::types::SplitError::CannotFit => [1.into_dart()].into_dart(),
            crate::api::types::SplitError::MaxSplitSizeTooLarge(field0) => {
                [2.into_dart(), field0.into_into_dart().into_dart()].into_dart()
            }
            crate::api::types::SplitError::MinSplitTooSmall => [3.into_dart()].into_dart(),
            crate::api::types::SplitError::InvalidSplitRange => [4.into_dart()].into_dart(),
            crate::api::types::SplitError::InvalidVersionRange => [5.into_dart()].into_dart(),
            crate::api::types::SplitError::EncodeError(field0) => {
                [6.into_dart(), field0.into_into_dart().into_dart()].into_dart()
            }
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<crate::api::types::SplitError>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<crate::api::types::SplitError>>
    for crate::api::types::SplitError
{
    fn into_into_dart(self) -> FrbWrapper<crate::api::types::SplitError> {
        self.into()
    }
}
// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for FrbWrapper<crate::api::types::SplitOptions> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        [
            self.0.encoding.into_into_dart().into_dart(),
            self.0.min_split_number.into_into_dart().into_dart(),
            self.0.max_split_number.into_into_dart().into_dart(),
            self.0.min_version.into_into_dart().into_dart(),
            self.0.max_version.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<crate::api::types::SplitOptions>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<crate::api::types::SplitOptions>>
    for crate::api::types::SplitOptions
{
    fn into_into_dart(self) -> FrbWrapper<crate::api::types::SplitOptions> {
        self.into()
    }
}
// Codec=Dco (DartCObject based), see doc to use other codecs
impl flutter_rust_bridge::IntoDart for FrbWrapper<crate::api::types::Version> {
    fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {
        match self.0 {
            crate::api::types::Version::V01 => 0.into_dart(),
            crate::api::types::Version::V02 => 1.into_dart(),
            crate::api::types::Version::V03 => 2.into_dart(),
            crate::api::types::Version::V04 => 3.into_dart(),
            crate::api::types::Version::V05 => 4.into_dart(),
            crate::api::types::Version::V06 => 5.into_dart(),
            crate::api::types::Version::V07 => 6.into_dart(),
            crate::api::types::Version::V08 => 7.into_dart(),
            crate::api::types::Version::V09 => 8.into_dart(),
            crate::api::types::Version::V10 => 9.into_dart(),
            crate::api::types::Version::V11 => 10.into_dart(),
            crate::api::types::Version::V12 => 11.into_dart(),
            crate::api::types::Version::V13 => 12.into_dart(),
            crate::api::types::Version::V14 => 13.into_dart(),
            crate::api::types::Version::V15 => 14.into_dart(),
            crate::api::types::Version::V16 => 15.into_dart(),
            crate::api::types::Version::V17 => 16.into_dart(),
            crate::api::types::Version::V18 => 17.into_dart(),
            crate::api::types::Version::V19 => 18.into_dart(),
            crate::api::types::Version::V20 => 19.into_dart(),
            crate::api::types::Version::V21 => 20.into_dart(),
            crate::api::types::Version::V22 => 21.into_dart(),
            crate::api::types::Version::V23 => 22.into_dart(),
            crate::api::types::Version::V24 => 23.into_dart(),
            crate::api::types::Version::V25 => 24.into_dart(),
            crate::api::types::Version::V26 => 25.into_dart(),
            crate::api::types::Version::V27 => 26.into_dart(),
            crate::api::types::Version::V28 => 27.into_dart(),
            crate::api::types::Version::V29 => 28.into_dart(),
            crate::api::types::Version::V30 => 29.into_dart(),
            crate::api::types::Version::V31 => 30.into_dart(),
            crate::api::types::Version::V32 => 31.into_dart(),
            crate::api::types::Version::V33 => 32.into_dart(),
            crate::api::types::Version::V34 => 33.into_dart(),
            crate::api::types::Version::V35 => 34.into_dart(),
            crate::api::types::Version::V36 => 35.into_dart(),
            crate::api::types::Version::V37 => 36.into_dart(),
            crate::api::types::Version::V38 => 37.into_dart(),
            crate::api::types::Version::V39 => 38.into_dart(),
            crate::api::types::Version::V40 => 39.into_dart(),
        }
    }
}
impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive
    for FrbWrapper<crate::api::types::Version>
{
}
impl flutter_rust_bridge::IntoIntoDart<FrbWrapper<crate::api::types::Version>>
    for crate::api::types::Version
{
    fn into_into_dart(self) -> FrbWrapper<crate::api::types::Version> {
        self.into()
    }
}

impl SseEncode for _Split {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<_Split>>>::sse_encode(flutter_rust_bridge::for_generated::rust_auto_opaque_encode::<_, StdArc<_>>(self), serializer);
    }
}

impl SseEncode for RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<_Split>> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        let (ptr, size) = self.sse_encode_raw();
        <usize>::sse_encode(ptr, serializer);
        <i32>::sse_encode(size, serializer);
    }
}

impl SseEncode for String {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <Vec<u8>>::sse_encode(self.into_bytes(), serializer);
    }
}

impl SseEncode for crate::api::types::EncodeError {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        match self {
            crate::api::types::EncodeError::Empty => {
                <i32>::sse_encode(0, serializer);
            }
            crate::api::types::EncodeError::CompressionError(field0) => {
                <i32>::sse_encode(1, serializer);
                <String>::sse_encode(field0, serializer);
            }
        }
    }
}

impl SseEncode for crate::api::types::Encoding {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                crate::api::types::Encoding::Hex => 0,
                crate::api::types::Encoding::Base32 => 1,
                crate::api::types::Encoding::Zlib => 2,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for crate::api::types::FileType {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                crate::api::types::FileType::Psbt => 0,
                crate::api::types::FileType::Transaction => 1,
                crate::api::types::FileType::Json => 2,
                crate::api::types::FileType::Cbor => 3,
                crate::api::types::FileType::UnicodeText => 4,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for i32 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_i32::<NativeEndian>(self).unwrap();
    }
}

impl SseEncode for Vec<u8> {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(self.len() as _, serializer);
        for item in self {
            <u8>::sse_encode(item, serializer);
        }
    }
}

impl SseEncode for crate::api::types::SplitError {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        match self {
            crate::api::types::SplitError::Empty => {
                <i32>::sse_encode(0, serializer);
            }
            crate::api::types::SplitError::CannotFit => {
                <i32>::sse_encode(1, serializer);
            }
            crate::api::types::SplitError::MaxSplitSizeTooLarge(field0) => {
                <i32>::sse_encode(2, serializer);
                <usize>::sse_encode(field0, serializer);
            }
            crate::api::types::SplitError::MinSplitTooSmall => {
                <i32>::sse_encode(3, serializer);
            }
            crate::api::types::SplitError::InvalidSplitRange => {
                <i32>::sse_encode(4, serializer);
            }
            crate::api::types::SplitError::InvalidVersionRange => {
                <i32>::sse_encode(5, serializer);
            }
            crate::api::types::SplitError::EncodeError(field0) => {
                <i32>::sse_encode(6, serializer);
                <crate::api::types::EncodeError>::sse_encode(field0, serializer);
            }
        }
    }
}

impl SseEncode for crate::api::types::SplitOptions {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <crate::api::types::Encoding>::sse_encode(self.encoding, serializer);
        <usize>::sse_encode(self.min_split_number, serializer);
        <usize>::sse_encode(self.max_split_number, serializer);
        <crate::api::types::Version>::sse_encode(self.min_version, serializer);
        <crate::api::types::Version>::sse_encode(self.max_version, serializer);
    }
}

impl SseEncode for u8 {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u8(self).unwrap();
    }
}

impl SseEncode for usize {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer
            .cursor
            .write_u64::<NativeEndian>(self as _)
            .unwrap();
    }
}

impl SseEncode for crate::api::types::Version {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        <i32>::sse_encode(
            match self {
                crate::api::types::Version::V01 => 0,
                crate::api::types::Version::V02 => 1,
                crate::api::types::Version::V03 => 2,
                crate::api::types::Version::V04 => 3,
                crate::api::types::Version::V05 => 4,
                crate::api::types::Version::V06 => 5,
                crate::api::types::Version::V07 => 6,
                crate::api::types::Version::V08 => 7,
                crate::api::types::Version::V09 => 8,
                crate::api::types::Version::V10 => 9,
                crate::api::types::Version::V11 => 10,
                crate::api::types::Version::V12 => 11,
                crate::api::types::Version::V13 => 12,
                crate::api::types::Version::V14 => 13,
                crate::api::types::Version::V15 => 14,
                crate::api::types::Version::V16 => 15,
                crate::api::types::Version::V17 => 16,
                crate::api::types::Version::V18 => 17,
                crate::api::types::Version::V19 => 18,
                crate::api::types::Version::V20 => 19,
                crate::api::types::Version::V21 => 20,
                crate::api::types::Version::V22 => 21,
                crate::api::types::Version::V23 => 22,
                crate::api::types::Version::V24 => 23,
                crate::api::types::Version::V25 => 24,
                crate::api::types::Version::V26 => 25,
                crate::api::types::Version::V27 => 26,
                crate::api::types::Version::V28 => 27,
                crate::api::types::Version::V29 => 28,
                crate::api::types::Version::V30 => 29,
                crate::api::types::Version::V31 => 30,
                crate::api::types::Version::V32 => 31,
                crate::api::types::Version::V33 => 32,
                crate::api::types::Version::V34 => 33,
                crate::api::types::Version::V35 => 34,
                crate::api::types::Version::V36 => 35,
                crate::api::types::Version::V37 => 36,
                crate::api::types::Version::V38 => 37,
                crate::api::types::Version::V39 => 38,
                crate::api::types::Version::V40 => 39,
                _ => {
                    unimplemented!("");
                }
            },
            serializer,
        );
    }
}

impl SseEncode for bool {
    // Codec=Sse (Serialization based), see doc to use other codecs
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_u8(self as _).unwrap();
    }
}

#[cfg(not(target_family = "wasm"))]
#[path = "frb_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "frb_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;
