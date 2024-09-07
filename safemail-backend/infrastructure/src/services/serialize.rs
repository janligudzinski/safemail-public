use domain::serde::Serialize;
use domain::serialize::SerializeService;

#[derive(Clone)]
pub struct JsonService;
impl SerializeService for JsonService {
    fn serialize<T: Serialize>(&self, t: &T) -> String {
        serde_json::to_string(t).unwrap()
    }
}
