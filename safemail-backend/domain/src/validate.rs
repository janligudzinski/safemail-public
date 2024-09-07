use crate::error::ValidationError;

pub trait Validate<T> {
    fn validate(&self, value: &T) -> Result<(), ValidationError>;
}
