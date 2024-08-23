/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(rename_all = "PascalCase"))]
pub enum Error {
    Unknown(String)
}