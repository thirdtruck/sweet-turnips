use serde::{Deserialize, Serialize};

pub trait GameConfig<'de>: PartialEq + Serialize + Deserialize<'de> {}
