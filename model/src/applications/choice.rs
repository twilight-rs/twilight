use serde::{Deserialize, Serialize};

/*
| Field  | Type          | Description               |
|--------|---------------|---------------------------|
| name\* | string        | name of the valid choice  |
| value  | string or int | value of the valid choice |
*/

#[serde(untagged)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum CommandOptionChoice {
    String { name: String, value: String },
    Int { name: String, value: i64 },
}
