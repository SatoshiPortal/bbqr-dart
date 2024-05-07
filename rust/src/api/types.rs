
use flutter_rust_bridge::frb;
use bbqr::{Encoding, FileType, Split, SplitOptions, Version};

pub struct Dummy {
    pub field: String,
}
impl Dummy {
    pub fn split(data: Vec<u8>)->Self{
        // split the data using default options
        let split = Split::try_from_data(&data, FileType::UnicodeText, Default::default())
            .expect("Failed to split data");

        // or split the data using zlib encoding, and custom options
        let split = Split::try_from_data(
        &data,
        FileType::UnicodeText,
        SplitOptions {
            encoding: Encoding::Zlib,
            min_split_number: 2,
            max_split_number: 100,
            min_version: Version::V03,
            max_version: Version::V30,
        },
        ).expect("Failed to split data");

        // print out each of the parts
        println!("{:#?}", split.parts);

        // generate the qr codes
        let qr_codes = split.generate_qr_codes();
        Dummy { field: "dummy".to_string() }

    }
}
