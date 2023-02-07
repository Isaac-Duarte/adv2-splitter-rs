use std::collections::{HashMap, LinkedList};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::io::{Cursor, Read, Write};

pub trait AdCodec {
    /// Returns a Duplication Result that will contain the parsed info block and
    ///
    /// # Arguments
    /// * `reader` - A reader to decode and parse into a result struct
    ///
    /// # Examples
    /// ```
    /// use codec;
    /// let codec = AdCodec5::default();
    ///
    /// let input_file = File::open("test.txt").expect("Unable to open file");
    /// let mut reader = BufReader::new(input_file);
    /// let result = codec.decode(input_file)
    /// advanced_dupe::print_result(result);
    /// ```
    fn decode(&self, reader: &mut dyn Read) -> DuplicationResult;

    /// # Arguments
    /// * `dupe_info` - Duplication metadata, note the map must be populated
    /// * `lua_value` - This is the LuaValue as a table
    /// * `stream` - This is the output stream to write to, typically a file
    ///
    /// # Examples
    /// ```
    /// use codec;
    /// let codec = AdCodec5::default();
    ///
    /// let input_file = File::open("test.txt").expect("Unable to open file");
    /// let mut reader = BufReader::new(input_file);
    /// let result = codec.decode(input_file)
    ///
    /// let output_file = File::create("test2.txt").expect("Unable to open output file");
    /// let mut out_stream = BufWriter::new(output_file);
    ///
    /// codec.encode(result.info(), result.value(), out_stream).expect("Unable to encode table");
    /// ```
    fn encode(
        &self,
        dupe_info: &DuplicationInfo,
        lua_value: &LuaValue,
        stream: &mut dyn Write,
    ) -> Result<(), Box<dyn Error>>;

    /// Returns if the signature byte array is valid or not.
    /// # Arguments
    /// * `signature` - Header of the AdvancedDuplicator file
    fn is_valid_signature(&self, signature: &[u8]) -> bool;
}

pub struct DuplicationResult {
    info: DuplicationInfo,
    value: LuaValue,
}

impl DuplicationResult {
    pub fn info(&self) -> &DuplicationInfo {
        &self.info
    }

    pub fn info_mut(&mut self) -> &mut DuplicationInfo {
        &mut self.info
    }

    pub fn value(&self) -> &LuaValue {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut LuaValue {
        &mut self.value
    }
}

#[derive(Clone)]
pub struct DuplicationInfo {
    size: u32,
    time_zone: String,
    date: String,
    time: String,
    player_name: String,
    values: LinkedList<(String, String)>,
}

impl DuplicationInfo {
    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }
    pub fn set_time_zone(&mut self, time_zone: String) {
        self.time_zone = time_zone;
    }
    pub fn set_date(&mut self, date: String) {
        self.date = date;
    }
    pub fn set_time(&mut self, time: String) {
        self.time = time;
    }
    pub fn set_player_name(&mut self, player_name: String) {
        self.player_name = player_name;
    }

    pub fn size(&self) -> u32 {
        self.size
    }
    pub fn time_zone(&self) -> &str {
        &self.time_zone
    }
    pub fn date(&self) -> &str {
        &self.date
    }
    pub fn time(&self) -> &str {
        &self.time
    }
    pub fn player_name(&self) -> &str {
        &self.player_name
    }

    fn add_value(&mut self, key: String, value: String) {
        self.values.push_front((key, value));
    }
}

// Static bytes used in codec decoding and encoding
static HEADER: &'static [u8] = &[65, 68, 50, 70];
static VERSION: &'static [u8] = &[5];
static CRLF: &'static [u8] = &[10];
static INFO_SPLIT: &'static [u8] = &[1];
static INFO_END: &'static [u8] = &[2];

/// The LuaValue enum is used to represent supported values that are
/// serializable and deserializable.
#[derive(Debug, PartialEq, Clone)]
pub enum LuaValue {
    String(String),
    Array(Vec<LuaValue>),
    Table(HashMap<LuaValue, LuaValue>),
    Double(f64),
    Vector(Vec<f64>),
    Angle(Vec<f64>),
    Bool(bool),
}

impl Hash for LuaValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            LuaValue::String(s) => s.hash(state),
            LuaValue::Array(a) => a.hash(state),
            LuaValue::Table(_) => {}
            LuaValue::Double(d) => d.to_bits().hash(state),
            LuaValue::Vector(_) => {}
            LuaValue::Angle(_) => {}
            LuaValue::Bool(b) => b.hash(state),
        }
    }
}

impl LuaValue {
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    pub fn as_array(&self) -> Option<&Vec<LuaValue>> {
        match self {
            LuaValue::Array(array) => Some(array),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<LuaValue>> {
        match self {
            LuaValue::Array(list) => Some(list),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            LuaValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_double(&self) -> bool {
        match *self {
            LuaValue::Double(_) => true,
            _ => false,
        }
    }

    pub fn as_double(&self) -> Option<f64> {
        match *self {
            LuaValue::Double(double) => Some(double),
            _ => None
        }
    }

    pub fn as_double_mut(&mut self) -> Option<&mut f64> {
        match self {
            LuaValue::Double(double) => Some(double),
            _ => None
        }
    }

    pub fn is_table(&self) -> bool {
        match *self {
            LuaValue::Table(_) => true,
            _ => false,
        }
    }

    pub fn as_table(&self) -> Option<&HashMap<LuaValue, LuaValue>> {
        match self {
            LuaValue::Table(table) => Some(table),
            _ => None
        }
    }

    pub fn as_table_mut(&mut self) -> Option<&mut HashMap<LuaValue, LuaValue>> {
        match self {
            LuaValue::Table(table) => Some(table),
            _ => None
        }
    }

    pub fn is_vector(&self) -> bool {
        match *self {
            LuaValue::Vector(_) => true,
            _ => false,
        }
    }

    pub fn as_vector(&self) -> Option<&Vec<f64>> {
        match self {
            LuaValue::Vector(vector) => Some(vector),
            _ => None
        }
    }

    pub fn as_vector_mut(&mut self) -> Option<&mut Vec<f64>> {
        match self {
            LuaValue::Vector(vector) => Some(vector),
            _ => None
        }
    }

    pub fn is_angle(&self) -> bool {
        match *self {
            LuaValue::Angle(_) => true,
            _ => false,
        }
    }

    pub fn as_angle(&self) -> Option<&Vec<f64>> {
        match self {
            LuaValue::Angle(angle) => Some(angle),
            _ => None
        }
    }

    pub fn as_angle_mut(&mut self) -> Option<&mut Vec<f64>> {
        match self {
            LuaValue::Angle(angle) => Some(angle),
            _ => None
        }
    }

    pub fn is_bool(&self) -> bool {
        match *self {
            LuaValue::Bool(_) => true,
            _ => false,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            LuaValue::Bool(boolean) => Some(boolean),
            _ => None
        }
    }
}

impl Eq for LuaValue {}

/// The implementation for the 5th codec version is as follows:
///
/// AdCodec5 Specification:
/// The format will contain two main parts in the file. The header, info block
/// and the data block.
///
/// # Header:
///     - Signature: The header will contain a signature of AD2F in ascii format
///     - Version: The version will be a single byte to determinate which codec encoder was used.
///
/// # Info Block:
///     - Start: The start is representing by a single CRLF byte (0xA)
///     - Body: The body consists of a key value pair delimited by single byte (0x1)
///              For example, say we have a name key and a value of Fozie it would store like this:
///             name[0x1]Fozie[0x1]date[0x1]28 January 2023
///             Note there could be any number of these.
///     - Validation Pair: One special validation pair in the info block is a strain of characters.
///                        Key: check
///                        Value: "\r\n\t\n"
///     - Termination: There is one terminating byte that will represent the end of the Info Block
///                     0x2.
///      - CRLF Flag: Another CRLF byte will be present here (0xA)
///
/// # Data Block:
///     The data block is representing by a Lua Table that could have any number of nested tables/arrays.
///     When encoded the data block is compressed in a LZMA format. This library will use a create called
///     `gmod_lzma` which is a binding of what is used in Garry's Mod.
///
///     Once decoded for each supported Lua type they will be directly serialized into a byte array.
///     - Data Type: This is the first byte will determine the data type
///     - Serialized value: This will be the serialized value of the data type
///     - Termination: Only two data types will have a termination block, and that is tables and arrays
///                 once this byte is hit, it is same to assume this is the end of that particular object.
///
///     Special Use cases:
///         - Strings : For strings if the length is below 246 then two things will be written.
///             - 1 byte length of the string
///             - String value
///           If the string is longer than that then the following will be written:
///             - Data type of string (248)
///             - A 32bit integer representing the length
///             - String value
///
#[derive(Default)]
pub struct AdCodec5 {}

impl AdCodec5 {
    /// Returns the duplication information that was stored in the metadata
    ///
    /// # Arguments
    /// * `data` - The info block from the file
    fn get_duplication_info(data: &[u8]) -> DuplicationInfo {
        let mut duplication_info = DuplicationInfo {
            size: 0,
            time_zone: "".to_string(),
            date: "".to_string(),
            time: "".to_string(),
            player_name: "".to_string(),
            values: Default::default(),
        };

        // Convert the whole info block into a string and split it based off of
        // the separation character
        let data_str = String::from_utf8_lossy(data);
        let split_data: Vec<&str> = data_str.split('\u{1}').collect();

        // For every odd number we will get the key value
        for i in 0..split_data.len() {
            if i % 2 != 0 {
                continue;
            }

            // Ensure there is a value for the key
            if i + 1 >= split_data.len() {
                break;
            }

            let key = split_data[i];
            let value = split_data[i + 1];

            // We will insert the known keys as well as insert it into the value list
            match key {
                "size" => duplication_info.set_size(value.parse().unwrap()),
                "timezone" => duplication_info.set_time_zone(value.to_string()),
                "date" => duplication_info.set_date(value.to_string()),
                "time" => duplication_info.set_time(value.to_string()),
                "name" => duplication_info.set_player_name(value.to_string()),
                "check" => {}
                _ => {}
            }

            // Insert the value into the list
            duplication_info.add_value(key.to_string(), value.to_string());
        }

        duplication_info
    }

    /// Decodes a stream of binary data into a `LuaValue` object.
    ///
    /// # Parameters
    ///
    /// * `cursor`: A mutable reference to a `Cursor` object pointing to the start of the binary
    /// data to be decoded.
    ///
    /// # Returns
    ///
    /// A `Option` object containing a `Box` of a `LuaValue` representing the decoded data, or `None`
    /// if the data is not a valid `LuaValue`.
    fn decode_stream(cursor: &mut Cursor<&[u8]>) -> Option<Box<LuaValue>> {
        let mut data_type = [0u8; 1];

        cursor.read_exact(&mut data_type).expect("Unexpected EOF");
        let data_type = data_type[0];

        match data_type {
            // Lua Table data type
            255 => {
                let mut table: HashMap<LuaValue, LuaValue> = HashMap::new();

                loop {
                    match Self::decode_stream(cursor) {
                        Some(key) => {
                            let value = Self::decode_stream(cursor).unwrap();

                            table.insert(*key, *value);
                        }
                        _ => {
                            return Some(Box::new(LuaValue::Table(table)));
                        }
                    }
                }
            }
            // Lua array data type
            254 => {
                let mut array: Vec<LuaValue> = Vec::new();

                loop {
                    match Self::decode_stream(cursor) {
                        Some(value) => {
                            array.push(*value);
                        }
                        _ => {
                            return Some(Box::new(LuaValue::Array(array)));
                        }
                    }
                }
            }
            // Lua Vector data type
            250 => {
                let mut vector: Vec<f64> = Vec::new();

                vector.push(Self::read_double(cursor));
                vector.push(Self::read_double(cursor));
                vector.push(Self::read_double(cursor));

                Some(Box::new(LuaValue::Vector(vector)))
            }
            // Lua angle data type
            249 => {
                let mut vector: Vec<f64> = Vec::new();

                vector.push(Self::read_double(cursor));
                vector.push(Self::read_double(cursor));
                vector.push(Self::read_double(cursor));

                Some(Box::new(LuaValue::Angle(vector)))
            }
            // Lua double data type
            251 => Some(Box::new(LuaValue::Double(Self::read_double(cursor)))),
            // Lua false data type
            252 => Some(Box::new(LuaValue::Bool(false))),
            // Lua true data type
            253 => Some(Box::new(LuaValue::Bool(true))),
            // Null data type
            246 => None,
            // String under 246 data type
            _ => {
                let length = data_type;
                if length == 0 {
                    return Some(Box::new(LuaValue::String(String::from(""))));
                }

                let mut data = vec![0; length as usize];
                cursor.read_exact(&mut data).expect("Unexpected EOF");
                let data = String::from_utf8_lossy(data.as_slice()).to_string();

                return Some(Box::new(LuaValue::String(data)));
            }
        }
    }

    /// Serializes a `LuaValue` into a binary stream.
    ///
    /// The function takes in a `LuaValue` and a mutable reference to a type `T` that implements the
    /// `Write` trait, and serializes the `LuaValue` into the binary stream.
    ///
    /// # Arguments
    ///
    /// * `value` - The `LuaValue` to be serialized.
    /// * `stream` - A mutable reference to a type that implements the `Write` trait, representing the
    /// binary stream to serialize `value` into.
    ///
    /// # Returns
    ///
    /// Returns a `Result` of type `Result<(), Box<dyn Error>>`. If the serialization is successful,
    /// the `Ok` variant is returned. If an error occurs during serialization, the `Err` variant is
    /// returned with an error message boxed in a `Box<dyn Error>`.
    fn encode_stream<T: Write>(value: &LuaValue, stream: &mut T) -> Result<(), Box<dyn Error>> {
        match value {
            LuaValue::String(str) => {
                if str.len() < 246 {
                    let buff = [str.len() as u8];
                    stream.write(&buff)?;

                    let buff = str.as_bytes();
                    stream.write(buff)?;
                }
            }
            LuaValue::Array(arr) => {
                let buff = [254; 1];
                stream.write(&buff)?;

                for value in arr {
                    Self::encode_stream(value, stream)?;
                }

                let buff: [u8; 1] = [246; 1];
                stream.write(&buff)?;
            }
            LuaValue::Table(tbl) => {
                let buff = [255; 1];
                stream.write(&buff)?;

                for (key, value) in tbl {
                    Self::encode_stream(key, stream)?;
                    Self::encode_stream(value, stream)?;
                }

                let buff: [u8; 1] = [246; 1];
                stream.write(&buff)?;
            }
            LuaValue::Double(double) => {
                let buff = [251; 1];
                stream.write(&buff)?;
                stream.write(&Self::double_as_bytes(&double))?;
            }
            LuaValue::Vector(vector) => {
                let buff = [250; 1];
                stream.write(&buff)?;

                stream.write(&Self::double_as_bytes(vector.get(0).unwrap()))?;
                stream.write(&Self::double_as_bytes(vector.get(1).unwrap()))?;
                stream.write(&Self::double_as_bytes(vector.get(2).unwrap()))?;
            }
            LuaValue::Angle(angle) => {
                let buff = [249; 1];
                stream.write(&buff)?;

                stream.write(&Self::double_as_bytes(angle.get(0).unwrap()))?;
                stream.write(&Self::double_as_bytes(angle.get(1).unwrap()))?;
                stream.write(&Self::double_as_bytes(angle.get(2).unwrap()))?;
            }
            LuaValue::Bool(boolean) => match boolean {
                true => {
                    let buff = [253; 1];
                    stream.write(&buff)?;
                }
                false => {
                    let buff = [252; 1];
                    stream.write(&buff)?;
                }
            },
        }

        Ok(())
    }

    /// Returns deserialized double
    ///
    /// # Arguments
    /// `input` - Stream that will contain the serialized double
    fn read_double(input: &mut dyn Read) -> f64 {
        let mut data = [0u8; 8];
        input.read_exact(&mut data).unwrap();
        let val = f64::from_bits(u64::from_le_bytes(data));
        return val;
    }

    /// Returns serialized double represented as a byte array
    ///
    /// # Arguments
    /// `value` - Double value to be serialized
    fn double_as_bytes(value: &f64) -> [u8; 8] {
        value.to_le_bytes()
    }
}

impl AdCodec for AdCodec5 {
    /// Decodes the `reader` into a `DuplicationResult` by reading the steam into a byte array,
    /// splitting the info and data block, parsing the info block into a `DuplicationInfo` struct,
    /// decompressing the data block with  `gmod_lzma`, and finally decoding the decompressed
    /// data block into a `LuaValue`.
    ///
    /// # Arguments
    ///
    /// * `reader` - The input stream to be decoded.
    ///
    /// # Returns
    ///
    /// A `DuplicationResult` containing the `DuplicationInfo` and the `LuaValue` decoded from the input stream.
    ///
    /// # Errors
    ///
    /// Returns an error if the LZMA data is unable to be decompressed.
    fn decode(&self, reader: &mut dyn Read) -> DuplicationResult {
        // Skip a CRLF byte
        reader.bytes().next();

        let mut b_output: Vec<u8> = Vec::new();
        let mut buff = [0; 1];
        let mut info_block_end: usize = 0;

        // Read the rest of the output into a byte array
        // and note the end of the info block
        while let Ok(_) = reader.read_exact(&mut buff) {
            if info_block_end == 0 && buff[0] == 0x2 {
                info_block_end = b_output.len();
            }

            b_output.push(buff[0]);
        }

        // Split the info and the data block skipping CRLF bytes
        let info = b_output[0..info_block_end].to_vec();
        let data = b_output[(info_block_end + 2)..].to_vec();

        // This will parse the info block into a DuplicationInfo struct
        let dupe_info = AdCodec5::get_duplication_info(info.as_slice());

        let data = gmod_lzma::decompress(data.as_slice()).expect("Unable to decompress LZMA data");

        let mut data = Cursor::new(data.as_slice());

        let value = AdCodec5::decode_stream(&mut data).unwrap();

        DuplicationResult {
            info: dupe_info,
            value: *value,
        }
    }

    /// Encode the `DuplicationInfo` and `LuaValue` to a binary format.
    ///
    /// # Arguments
    /// * `dupe_info` - A struct that contains information about the duplication
    /// * `lua_value` - The Lua value that needs to be encoded
    /// * `stream` - A mutable reference to a type that implements the `Write` trait
    ///
    /// # Returns
    /// * `Result<(), Box<dyn Error>>` - Returns an error if there is any issue with writing to the `stream`
    fn encode(
        &self,
        dupe_info: &DuplicationInfo,
        lua_value: &LuaValue,
        stream: &mut dyn Write,
    ) -> Result<(), Box<dyn Error>> {
        // First thing to do is write the signature and version
        stream.write(&HEADER)?;
        stream.write(&VERSION)?;

        // Write CRLF byte
        stream.write(&CRLF)?;

        for (k, v) in dupe_info.values.iter() {
            stream.write(k.as_bytes())?;
            stream.write(&INFO_SPLIT)?;
            stream.write(v.as_bytes())?;
            stream.write(&INFO_SPLIT)?;
        }

        // Termination of info block
        stream.write(&INFO_END)?;

        // Write CRLF byte
        stream.write(&CRLF)?;

        // Serialize the lua type
        let mut data = Cursor::new(Vec::new());
        AdCodec5::encode_stream(lua_value, &mut data)?;

        // Compress the data block into LZMA format
        let data = gmod_lzma::compress(&data.into_inner(), 9).expect("Unable to compress data");

        stream.write(&data)?;

        Ok(())
    }

    /// Returns true if `signature` is a valid header signature.
    ///
    /// # Arguments
    ///
    /// * `signature` - The byte array to be validated.
    ///
    /// # Example
    ///
    /// ```
    /// use codec5::AdCodec5;
    ///
    /// let codec = AdCodec5::default();
    ///
    /// assert!(codec.is_valid_signature(&codec5::HEADER));
    /// ```
    fn is_valid_signature(&self, signature: &[u8]) -> bool {
        HEADER == signature
    }
}

// Error structs
#[derive(Debug, Clone)]
pub enum CodecError {
    UnsupportedCodec(String),
    InvalidHeader,
}

impl Display for CodecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unsupported Codec was passed")
    }
}

impl Error for CodecError {}

mod test {
    use std::collections::HashMap;
    use std::error::Error;
    use std::io::{Cursor, ErrorKind, Write};
    use std::ptr::write;

    use crate::{get_lua_value, lua_string};
    use crate::codec::{AdCodec, AdCodec5, HEADER, LuaValue};

    #[test]
    fn test_signature_header() -> Result<(), Box<dyn Error>> {
        let codec = AdCodec5::default();
        assert!(codec.is_valid_signature(&HEADER));

        Ok(())
    }

    #[test]
    fn test_read_write_double() -> Result<(), Box<dyn Error>> {
        let double = 50.56;
        let double_bytes = AdCodec5::double_as_bytes(&double);
        let mut cursor = Cursor::new(double_bytes);
        let read_double = AdCodec5::read_double(&mut cursor);

        assert_eq!(read_double, double);

        Ok(())
    }

    #[test]
    fn test_get_duplication_info() -> Result<(), Box<dyn Error>> {
        let duplication_info = String::from("name\u{1}test");
        let duplication_info = duplication_info.as_bytes();

        let result = AdCodec5::get_duplication_info(duplication_info);

        assert_eq!(result.player_name(), "test");
        Ok(())
    }

    #[test]
    fn test_lua_deserialize() -> Result<(), Box<dyn Error>> {
        let mut stream: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        // LuaValue Table
        stream.write(&[255; 1])?;

        // Write a entry for "Key"
        stream.write(&[3; 1])?;
        stream.write("Key".as_bytes())?;

        // Write value for double 20.25
        stream.write(&[251; 1])?;
        stream.write(&AdCodec5::double_as_bytes(&20.25))?;

        // Write the termination block
        stream.write(&[246; 1])?;

        // Deserialize the stream
        let stream = stream.into_inner();
        let mut stream = Cursor::new(stream.as_slice());
        let result = AdCodec5::decode_stream(&mut stream);

        let result = *result.expect("An error occurred while decoding the stream");

        // Validate the result was a table
        let result = get_lua_value!(Table, result).expect("Unable to find table");

        let key = LuaValue::String(String::from("Key"));
        let value = result.get(&key).expect("Key was not found!");

        // Ensure the value is a double
        let value = get_lua_value!(Double, value.clone()).expect("Unable to find double");

        assert_eq!(value, 20.25 as f64);

        Ok(())
    }

    #[test]
    fn test_lua_serialize() -> Result<(), Box<dyn Error>> {
        // Create the artificial lua table
        let mut table: HashMap<LuaValue, LuaValue> = HashMap::default();
        table.insert(
            LuaValue::String(String::from("Key")),
            LuaValue::Double(20.25),
        );

        let mut stream: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        // Serialize the tpye into the stream
        AdCodec5::encode_stream(&LuaValue::Table(table), &mut stream)
            .expect("Unable to serialize table");

        let serialized_data = stream.into_inner();

        let mut data: Vec<u8> = Vec::new();

        // Create the expected byte array
        data.push(255);
        data.push(3);
        data.extend(b"Key");
        data.push(251);
        data.extend(AdCodec5::double_as_bytes(&20.25));
        data.push(246);

        assert_eq!(serialized_data, data);

        Ok(())
    }

    #[test]
    fn test_deserialize_and_serialize() -> Result<(), Box<dyn Error>> {
        let mut table: HashMap<LuaValue, LuaValue> = HashMap::default();
        table.insert(lua_string!("Key"), LuaValue::Double(20.25));

        let mut stream: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        // Serialize the type into the stream
        AdCodec5::encode_stream(&LuaValue::Table(table), &mut stream)
            .expect("Unable to serialize table");

        let stream = stream.into_inner();
        let mut stream = Cursor::new(stream.as_slice());
        let result =
            *AdCodec5::decode_stream(&mut stream).expect("Unable to deserialize lua table");

        // Validate the result was a table
        let result = get_lua_value!(Table, result).expect("Unable to find table");

        let key = LuaValue::String(String::from("Key"));
        let value = result.get(&key).expect("Key was not found!");

        // Ensure the value is a double
        let value = get_lua_value!(Double, value.clone()).expect("Unable to find the double");

        assert_eq!(value, 20.25 as f64);

        Ok(())
    }
}
