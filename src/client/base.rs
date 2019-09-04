use std::borrow::Cow;

use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjRef<'a> {
    #[serde(borrow)]
    id: Cow<'a, str>,
}
