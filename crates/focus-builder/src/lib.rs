use std::marker::PhantomData;
use thiserror::Error;

/// Error type for builder operations.
#[derive(Debug, Error)]
pub enum BuilderError {
    #[error("missing required field: {0}")]
    MissingField(String),
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("build failed: {0}")]
    Build(String),
}

/// Core builder trait for all focus builders.
pub trait Builder<T> {
    /// Build the target type, consuming the builder.
    fn build(self) -> Result<T, BuilderError>;
}

/// Trait for resource builders that can be validated.
pub trait ResourceBuilder<T>: Builder<T> {
    /// Validate the builder state without consuming it.
    fn validate(&self) -> Result<(), BuilderError>;
}

/// Extension trait for builder chaining.
pub trait BuilderExt<T>: Builder<T> {
    /// Build or panic on error.
    fn build_or_panic(self) -> T
    where
        Self: Sized,
    {
        self.build().expect("builder should succeed")
    }

    /// Build with a fallback function.
    fn build_or_else<F>(self, f: F) -> T
    where
        Self: Sized,
        F: FnOnce(BuilderError) -> T,
    {
        self.build().unwrap_or_else(f)
    }
}

impl<T, B: Builder<T>> BuilderExt<T> for B {}

/// Marker trait for types that can be built incrementally.
pub trait Buildable<T>: Sized {
    /// Associated builder type.
    type Builder: Builder<T>;

    /// Create a new builder for this type.
    fn builder() -> Self::Builder;
}

/// A generic builder for types with a simple `with_` pattern.
pub struct GenericBuilder<T> {
    _phantom: PhantomData<T>,
}

impl<T> GenericBuilder<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for GenericBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestTarget {
        value: i32,
    }

    impl std::fmt::Debug for TestTarget {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TestTarget").field("value", &self.value).finish()
        }
    }

    struct TestBuilder {
        value: Option<i32>,
    }

    impl Builder<TestTarget> for TestBuilder {
        fn build(self) -> Result<TestTarget, BuilderError> {
            let value = self.value.ok_or_else(|| BuilderError::MissingField("value".into()))?;
            if value < 0 {
                return Err(BuilderError::Validation("value must be >= 0".into()));
            }
            Ok(TestTarget { value })
        }
    }

    impl ResourceBuilder<TestTarget> for TestBuilder {
        fn validate(&self) -> Result<(), BuilderError> {
            self.value.ok_or_else(|| BuilderError::MissingField("value".into()))?;
            Ok(())
        }
    }

    #[test]
    fn test_builder_success() {
        let builder = TestBuilder { value: Some(42) };
        let target = builder.build().unwrap();
        assert_eq!(target.value, 42);
    }

    #[test]
    fn test_builder_missing_field() {
        let builder = TestBuilder { value: None };
        let err = builder.build().unwrap_err();
        assert!(matches!(err, BuilderError::MissingField(_)));
    }

    #[test]
    fn test_builder_validation() {
        let builder = TestBuilder { value: Some(-1) };
        let err = builder.build().unwrap_err();
        assert!(matches!(err, BuilderError::Validation(_)));
    }

    #[test]
    fn test_builder_ext_build_or_panic() {
        let builder = TestBuilder { value: Some(42) };
        let target = builder.build_or_panic();
        assert_eq!(target.value, 42);
    }

    #[test]
    fn test_resource_builder_validate() {
        let builder = TestBuilder { value: Some(42) };
        assert!(builder.validate().is_ok());
    }

    #[test]
    fn test_generic_builder_new() {
        let _builder = GenericBuilder::<TestTarget>::new();
    }
}
