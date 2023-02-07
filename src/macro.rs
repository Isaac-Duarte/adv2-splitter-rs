/// Returns a LuaValue::String given a splice
///
/// # Example
/// ```
/// let value = LuaValue::String(String::from("test"));
/// assert_eq!(value, lua_string!("test"));
/// ```
#[macro_export]
macro_rules! lua_string {
    ($x:expr) => {
        LuaValue::String($x.to_string())
    };
}

#[macro_export]
macro_rules! lua_array {
    ($x:expr) => {
        LuaValue::Array($x)
    };
}

#[macro_export]
macro_rules! lua_table {
    ($x:expr) => {
        LuaValue::Table($x)
    };
}

#[macro_export]
macro_rules! lua_vector {
    ($x:expr) => {
        LuaValue::Vector($x)
    };
}

#[macro_export]
macro_rules! lua_angle {
    ($x:expr) => {
        LuaValue::Angle($x)
    };
}

#[macro_export]
macro_rules! lua_bool {
    ($x:expr) => {
        LuaValue::Bool($x)
    };
}

#[macro_export]
macro_rules! lua_double {
    ($x:expr) => {
        LuaValue::Double($x)
    };
}

/// Returns an optional of a `LuaValue` given the enum type
///
/// # Example
/// ```
/// let value = LuaValue::Double(3.14);
///
/// if let Some(double) = get_lua_value!(Double, value) {
///     println!("Value was a double!");
/// }
/// ```
#[macro_export]
macro_rules! get_lua_value {
    (String, $x: expr) => {
        match $x {
            LuaValue::String(val) => Some(val),
            _ => None,
        }
    };
    (Array, $x: expr) => {
        match $x {
            LuaValue::Array(val) => Some(val),
            _ => None,
        }
    };
    (Table, $x: expr) => {
        match $x {
            LuaValue::Table(val) => Some(val),
            _ => None,
        }
    };
    (Double, $x: expr) => {
        match $x {
            LuaValue::Double(val) => Some(val),
            _ => None,
        }
    };
    (Vector, $x: expr) => {
        match $x {
            LuaValue::Vector(val) => Some(val),
            _ => None,
        }
    };
    (Angle, $x: expr) => {
        match $x {
            LuaValue::Angle(val) => Some(val),
            _ => None,
        }
    };
    (Bool, $x: expr) => {
        match $x {
            LuaValue::Bool(val) => Some(val),
            _ => None,
        }
    };
}
