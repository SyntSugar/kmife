use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct MoveModule {
    pub r#address: String,
    pub r#name: String,
    pub r#friends: Vec<String>,
    pub r#exposed_functions: Vec<MoveExposedFunctions>,
    pub r#structs: Vec<MoveStruct>,
}

#[derive(Debug, Deserialize)]
pub struct MoveExposedFunctions {
    pub r#name: String,
    pub r#visibility: String,
    pub r#is_entry: bool,
    pub r#generic_type_params: Vec<Constraints>,
    pub r#params: Vec<String>,
    pub r#return: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Constraints {
    pub r#constraints: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveStruct {
    pub r#name: String,
    pub r#is_native: bool,
    pub r#abilities: Vec<String>,
    pub r#generic_type_params: Vec<Constraints>,
    pub r#fields: Vec<Fields>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fields {
    pub r#name: String,
    pub r#type: String,
}

// pub enum FieldType {
//     Address(0),
//     Uint8(1),
//     Uint64(2)
//     Uint128(3)
//     OtherType(4),
//     Bool(5),
//     Vector(6),
//     String(7),
//     Events(8),
//     Coin(9),
// }
