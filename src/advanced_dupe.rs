use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read, Write};

use crate::codec::{AdCodec, AdCodec5, CodecError, DuplicationInfo, DuplicationResult, LuaValue};

pub fn decode(file: File) -> Result<DuplicationResult, Box<dyn Error>> {
    let mut reader = BufReader::new(file);

    // First we will read the header
    let mut header = [0; 4];
    reader.read_exact(&mut header)?;

    // Read the version
    let mut version = [0; 1];
    reader.read_exact(&mut version)?;
    let version = version[0];

    let codec = match version {
        5 => Box::new(AdCodec5 {}) as Box<dyn AdCodec>,
        _ => return Err(Box::new(CodecError::UnsupportedCodec(version.to_string()))),
    };

    if !codec.is_valid_signature(&header) {
        return Err(Box::new(CodecError::InvalidHeader));
    }

    let result = codec.decode(&mut reader);

    Ok(result)
}

pub fn encode(
    dupe_info: &DuplicationInfo,
    lua_value: &LuaValue,
    stream: &mut dyn Write,
) -> Result<(), Box<dyn Error>> {
    let codec = AdCodec5 {};
    codec.encode(dupe_info, lua_value, stream)?;

    Ok(())
}

pub fn print_result(result: &DuplicationResult) {
    // First we will display the info
    let info = result.info();

    println!("Results for duplication:");
    println!("\tSize: {}", info.size());
    println!("\tTimeZone: {}", info.time_zone());
    println!("\tDate: {}", info.date());
    println!("\tTime: {}", info.time());
    println!("\tPlayerName: {}", info.player_name());

    // Then we will display the result
    println!("\tDupe data: ");

    print_value(result.value(), 0);
}

fn print_value(value: &LuaValue, tab: usize) {
    let tab_str = "\t".repeat(tab);

    match value {
        LuaValue::String(str) => {
            println!("{}(String) {}", tab_str, str);
        }
        LuaValue::Array(arr) => {
            println!("{}Array: ", tab_str);

            for value in arr {
                print_value(value, tab + 1)
            }
        }
        LuaValue::Table(table) => {
            for (key, value) in table {
                print_value(key, tab + 1);
                print_value(value, tab + 2);
            }
        }
        LuaValue::Double(double) => {
            println!("{}(Double) {:.4}", tab_str, double);
        }
        LuaValue::Vector(vector) => {
            println!("{}Vector: ", tab_str);
            println!("{}\tX: {:.4}", tab_str, vector.get(0).unwrap());
            println!("{}\tY: {:.4}", tab_str, vector.get(1).unwrap());
            println!("{}\tZ: {:.4}", tab_str, vector.get(2).unwrap());
        }
        LuaValue::Angle(angle) => {
            println!("{}Angle: ", tab_str);
            println!("{}\tPitch: {:.4}", tab_str, angle.get(0).unwrap());
            println!("{}\tYaw: {:.4}", tab_str, angle.get(1).unwrap());
            println!("{}\tRoll: {:.4}", tab_str, angle.get(2).unwrap());
        }
        LuaValue::Bool(boolean) => {
            println!("{}(Boolean) {}", tab_str, boolean);
        }
    }
}
