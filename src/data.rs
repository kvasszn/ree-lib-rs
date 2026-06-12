use std::collections::HashMap;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone)]
pub enum DataSource {
    /// e.g., msg(...) [ @item_row._RawName ]
    MsgLookup {
        msg_file: String,
        target_query: String, // "item_row"
        target_field: String, // "_RawName"
    },
    /// e.g., rsz(...)._Values[ _ItemId == $self ]
    RszQuery {
        rsz_file: String,
        array_path: String,
        match_field: String,
    }
}

fn extract_between<'a>(s: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let (_, rest) = s.split_once(start)?;
    let (inner, _) = rest.split_once(end)?;
    Some(inner.trim())
}


pub fn deserialize_data_sources<'de, D>(deserializer: D) -> Result<HashMap<String, DataSource>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw_data: HashMap<String, String> = HashMap::deserialize(deserializer)?;
    let mut parsed_data = HashMap::new();

    for (key, val_str) in raw_data {
        if val_str.starts_with("msg(") {
            let msg_file = extract_between(&val_str, "msg(", ")")
                .ok_or_else(|| serde::de::Error::custom(format!("Missing or malformed 'msg(...)' path in: {val_str}")))?
                .to_string();

            let inner_bracket = extract_between(&val_str, "[", "]")
                .ok_or_else(|| serde::de::Error::custom(format!("Missing or malformed '[...]' query brackets in: {val_str}")))?;

            let (query_ref, field) = inner_bracket.split_once('.')
                .ok_or_else(|| serde::de::Error::custom(format!("Expected '@query.field' format inside brackets in: {val_str}")))?;

            let target_query = query_ref.replace("@", "").trim().to_string();

            parsed_data.insert(key, DataSource::MsgLookup {
                msg_file,
                target_query,
                target_field: field.trim().to_string(),
            });

        } else if val_str.starts_with("rsz(") {
            let rsz_file = extract_between(&val_str, "rsz(", ")")
                .ok_or_else(|| serde::de::Error::custom(format!("Missing or malformed 'rsz(...)' path in: {val_str}")))?
                .to_string();

            let (_, tail) = val_str.split_once(").")
                .ok_or_else(|| serde::de::Error::custom(format!("Expected ').' after rsz file path in: {val_str}")))?;

            let (array_path, condition) = tail.split_once('[')
                .ok_or_else(|| serde::de::Error::custom(format!("Missing '[' for array condition in: {val_str}")))?;

            let (match_field, _) = condition.split_once("==")
                .ok_or_else(|| serde::de::Error::custom(format!("Missing '==' condition inside brackets in: {val_str}")))?;

            parsed_data.insert(key, DataSource::RszQuery {
                rsz_file,
                array_path: array_path.trim().to_string(),
                match_field: match_field.trim().to_string(),
            });
            
        } else {
            return Err(serde::de::Error::custom(format!(
                "Unknown data source format for key '{}': {}", key, val_str
            )));
        }
    }

    Ok(parsed_data)
}
