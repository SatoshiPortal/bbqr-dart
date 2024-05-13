use bbqr::{
    error::{JoinError, SplitError},
    Encoding, FileType, Split, SplitOptions, Version,
};
use flutter_rust_bridge::frb;

#[derive(Debug, Clone)]
#[frb(mirror(Split))]
pub struct _Split {
    pub version: Version,
    pub parts: Vec<String>,
    pub encoding: Encoding,
}

#[derive(Debug, Clone)]
#[frb(mirror(SplitOptions))]
pub struct _SplitOptions {
    pub encoding: Encoding,
    pub min_split_number: usize,
    pub max_split_number: usize,
    pub min_version: Version,
    pub max_version: Version,
}

#[derive(Debug, Clone)]
#[frb(mirror(Version))]
pub enum _Version {
    /// Version n°01
    V01 = 0,
    /// Version n°02
    V02 = 1,
    /// Version n°03
    V03 = 2,
    /// Version n°04
    V04 = 3,
    /// Version n°05
    V05 = 4,
    /// Version n°06
    V06 = 5,
    /// Version n°07
    V07 = 6,
    /// Version n°08
    V08 = 7,
    /// Version n°09
    V09 = 8,
    /// Version n°10
    V10 = 9,
    /// Version n°11
    V11 = 10,
    /// Version n°12
    V12 = 11,
    /// Version n°13
    V13 = 12,
    /// Version n°14
    V14 = 13,
    /// Version n°15
    V15 = 14,
    /// Version n°16
    V16 = 15,
    /// Version n°17
    V17 = 16,
    /// Version n°18
    V18 = 17,
    /// Version n°19
    V19 = 18,
    /// Version n°20
    V20 = 19,
    /// Version n°21
    V21 = 20,
    /// Version n°22
    V22 = 21,
    /// Version n°23
    V23 = 22,
    /// Version n°24
    V24 = 23,
    /// Version n°25
    V25 = 24,
    /// Version n°26
    V26 = 25,
    /// Version n°27
    V27 = 26,
    /// Version n°28
    V28 = 27,
    /// Version n°29
    V29 = 28,
    /// Version n°30
    V30 = 29,
    /// Version n°31
    V31 = 30,
    /// Version n°32
    V32 = 31,
    /// Version n°33
    V33 = 32,
    /// Version n°34
    V34 = 33,
    /// Version n°35
    V35 = 34,
    /// Version n°36
    V36 = 35,
    /// Version n°37
    V37 = 36,
    /// Version n°38
    V38 = 37,
    /// Version n°39
    V39 = 38,
    /// Version n°40
    V40 = 39,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[frb(mirror(Encoding))]
pub enum _Encoding {
    Hex,
    Base32,
    Zlib,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[frb(mirror(FileType))]
pub enum _FileType {
    Psbt,
    Transaction,
    Json,
    Cbor,
    UnicodeText,
}

pub enum Error {
    SplitError(SplitError),
    JoinError(JoinError),
}

impl _Split {
    pub fn try_new_from_data(
        data: &[u8],
        file_type: FileType,
        options: SplitOptions,
    ) -> Result<Self, Error> {
        let split = Split::try_from_data(data, file_type, options).map_err(Error::SplitError)?;

        Ok(Self {
            version: split.version,
            parts: split.parts,
            encoding: split.encoding,
        })
    }
}
