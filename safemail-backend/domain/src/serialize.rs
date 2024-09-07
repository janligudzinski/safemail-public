pub trait SerializeService {
    fn serialize<T: serde::Serialize>(&self, t: &T) -> String;
}
