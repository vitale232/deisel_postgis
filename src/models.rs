use crate::types::GeogPoint;
use geojson::{Feature, Geometry, Value};
use serde_json::map::Map;
use serde_json::value::Value as JsonValue;

#[derive(Queryable, Serialize)]
pub struct Location {
    pub id: i32,
    pub loc: GeogPoint,
    pub is_active: bool,
    pub updated_at: std::time::SystemTime,
}

impl Location {
    pub fn to_geojson(&self) -> Feature {
        let id = serde_json::value::Number::from_f64(self.id as f64);
        let id_value = match id {
            Some(id) => JsonValue::Number(id),
            None => JsonValue::Null,
        };
        let mut props = Map::new();
        props.insert(String::from("is_active"), JsonValue::Bool(self.is_active));
        props.insert(String::from("id"), id_value);

        Feature {
            bbox: None,
            geometry: Some(Geometry::new(Value::Point(vec![self.loc.x, self.loc.y]))),
            id: None,
            properties: Some(props),
            foreign_members: None,
        }
    }
}
