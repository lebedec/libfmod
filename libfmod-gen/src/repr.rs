use pest::iterators::Pair;
use pest::RuleType;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

pub struct JsonConverter {
    pub arrays: Vec<String>,
}

impl JsonConverter {
    pub fn new(arrays: Vec<String>) -> Self {
        JsonConverter { arrays }
    }

    pub fn convert<T, R>(&self, pair: Pair<'_, R>) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
        R: RuleType,
    {
        let value = self.create_value(pair);
        serde_json::from_value(value)
    }

    pub fn create_value<R>(&self, pair: Pair<'_, R>) -> Value
    where
        R: RuleType,
    {
        let rule = format!("{:?}", pair.as_rule());
        let data = pair.as_str();
        let inner = pair.into_inner();
        if inner.peek().is_none() {
            Value::String(data.into())
        } else {
            if self.arrays.contains(&rule) {
                let values = inner.map(|pair| self.create_value(pair)).collect();
                Value::Array(values)
            } else {
                let map = inner.map(|pair| {
                    let key = format!("{:?}", pair.as_rule());
                    let value = self.create_value(pair);
                    (key, value)
                });
                Value::Object(Map::from_iter(map))
            }
        }
    }
}
