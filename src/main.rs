extern crate core;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, ErrorKind};
use std::panic;
use std::path::PathBuf;

use crate::codec::LuaValue;

mod advanced_dupe;
mod codec;
mod r#macro;

const HELP: &str = "\
Advanced Duplicator Splitter

USAGE:
    adv2-splitter [OPTIONS] [FILE] 

FLAGS:
    -h, --help          Prints help information
    -p, --print         Prints the deserialized dupe

OPTIONS
    --size SIZE         Gives the split size

ARGS:
    <FILE>
";

#[derive(Debug)]
struct AppArgs {
    file: PathBuf,
    size: Option<usize>,
    print: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    perform_split(&args)?;

    Ok(())
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = AppArgs {
        file: pargs.free_from_str()?,
        size: pargs.opt_value_from_str("--size")?,
        print: pargs.contains(["-p", "--print"]),
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

fn perform_split(args: &AppArgs) -> Result<(), Box<dyn Error>> {
    let input_file = File::open(&args.file).expect("Unable to open file");
    let result = advanced_dupe::decode(input_file).expect("Unable to decode file");

    if args.print {
        advanced_dupe::print_result(&result);
    }

    // Find the entities table
    let table = result.value().as_table().expect("Invalid format for AD5");

    let entities = table
        .get(&lua_string!("Entities"))
        .expect("Unable to find entities table");
    let entity_map = entities.as_table().expect("Entity table is not a table");

    // This will be all entities in the map
    let split_lua_values;
    {
        if let Some(size) = args.size {
            let split_maps = split_map(entity_map, size);

            split_lua_values = match split_lua_tables(&result.value(), &split_maps) {
                Ok(val) => val,
                Err(_) => {
                    return Err(Box::new(std::io::Error::new(
                        ErrorKind::InvalidData,
                        "Unable to split the table",
                    )));
                }
            };
        } else {
            split_lua_values = vec![result.value().clone()];
        }
    }

    let file_name = args.file.file_name().unwrap().to_str().unwrap();
    let extension = args.file.extension().unwrap().to_str().unwrap();
    let file_name = file_name.replace(extension, "");
    let file_name = file_name.trim_end_matches(".");

    for i in 0..split_lua_values.len() {
        let lua_value = split_lua_values.get(i).unwrap();
        let file_name = format!("{}-{}.txt", file_name, i);

        let output_file = File::create(file_name)?;
        let mut out_stream = BufWriter::new(output_file);
        advanced_dupe::encode(result.info(), lua_value, &mut out_stream)?;
    }

    Ok(())
}

/// Splits a Lua table into multiple tables using a provided `split_maps` vector.
///
/// # Arguments
///
/// * `base_clone` - A reference to the base Lua table that needs to be split
/// * `split_maps` - A vector of hash maps, each representing a split of the base table.
///
/// # Returns
///
/// A vector of `LuaValue` representing the split tables.
///
/// # Errors
///
/// If the `base_clone` is not a `LuaValue::Table`, this function will panic with the message "".
fn split_lua_tables(
    base_clone: &LuaValue,
    split_maps: &Vec<HashMap<LuaValue, LuaValue>>,
) -> Result<Vec<LuaValue>, Box<dyn Error>> {
    let mut splits: Vec<LuaValue> = Vec::new();

    for split in split_maps {
        let mut split_value =
            get_lua_value!(Table, base_clone.clone()).expect("base_clone must be a table");

        split_value.insert(lua_string!("Entities"), lua_table!(split.clone()));

        // We need to reset the head entity to the first entity in the table
        let val = get_lua_value!(
            Table,
            split_value
                .get(&lua_string!("HeadEnt"))
                .expect("Unable to find HeadEnt")
        )
        .expect("HeadEnt is not a table");

        // We will get the first value and grab the index
        let first = split.keys().next().unwrap();
        let mut val = val.clone();
        val.insert(lua_string!("Index"), first.clone()).unwrap();

        // Reset the head ent
        split_value.insert(lua_string!("HeadEnt"), lua_table!(val));

        // Push the Table into the split value
        splits.push(lua_table!(split_value));
    }

    Ok(splits)
}

/// Splits the given `map` into `n` chunks.
///
/// # Arguments
///
/// * `map` - A reference to a `HashMap<LuaValue, LuaValue>` to be split.
/// * `n` - The number of chunks to split `map` into.
///
/// # Returns
///
/// A `Vec` of `HashMap`s, where each `HashMap` is a chunk of the original `map`.
///
/// # Example
///
/// ```
/// let mut map = HashMap::new();
/// map.insert(LuaValue::String(String::from("Key1")), LuaValue::Number(1.0));
/// map.insert(LuaValue::String(String::from("Key2")), LuaValue::Number(2.0));
/// map.insert(LuaValue::String(String::from("Key3")), LuaValue::Number(3.0));
///
/// let chunks = split_map(&map, 2);
/// assert_eq!(chunks.len(), 2);
/// ```
fn split_map(map: &HashMap<LuaValue, LuaValue>, n: usize) -> Vec<HashMap<LuaValue, LuaValue>> {
    // Calculate the chunk size as the length of the map divided by n
    let chunk_size = map.len() / n;

    // Create a vec to store the chunks
    let mut chunks = Vec::new();

    // Create a HashMap to store the current chunk
    let mut chunk = HashMap::new();

    // Counter to keep track of how many items are in the current chunk
    let mut chunk_count = 0;

    // Iterate through each key-value pair in the map
    for (key, value) in map.iter() {
        // Insert the key-value pair into the current chunk
        chunk.insert(key.clone(), value.clone());

        // Increment the chunk count
        chunk_count += 1;

        // If the chunk count equals the chunk size, add the chunk to the chunks vec and reset the chunk and chunk count
        if chunk_count == chunk_size {
            chunks.push(chunk);
            chunk = HashMap::new();
            chunk_count = 0;
        }
    }

    // If there are items remaining in the chunk, add the chunk to the chunks vec
    if !chunk.is_empty() {
        if chunks.is_empty() {
            chunks.push(chunk);
        } else {
            let last_chunk = chunks.last_mut().unwrap();

            for (key, value) in chunk.iter() {
                last_chunk.insert(key.clone(), value.clone());
            }
        }
    }

    chunks
}
