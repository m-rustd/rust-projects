mod create;
mod links;
mod link_to;

pub use create::Create;
pub use links::Links;
pub use link_to::LinkTo;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
struct TinyData<T> {
    pub data: T,
    pub ok: bool,
    pub err: Option<String>,
}