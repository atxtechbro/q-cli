// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TransformEvent {
    /// Identifier for the Transformation Job
    pub job_id: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    #[allow(missing_docs)] // documentation missing in model
    pub ide_category: ::std::option::Option<crate::types::IdeCategory>,
    /// Programming Languages supported by CodeWhisperer
    pub programming_language: ::std::option::Option<crate::types::ProgrammingLanguage>,
    #[allow(missing_docs)] // documentation missing in model
    pub lines_of_code_changed: ::std::option::Option<i32>,
    #[allow(missing_docs)] // documentation missing in model
    pub chars_of_code_changed: ::std::option::Option<i32>,
    #[allow(missing_docs)] // documentation missing in model
    pub lines_of_code_submitted: ::std::option::Option<i32>,
}
impl TransformEvent {
    /// Identifier for the Transformation Job
    pub fn job_id(&self) -> &str {
        use std::ops::Deref;
        self.job_id.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.timestamp.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn ide_category(&self) -> ::std::option::Option<&crate::types::IdeCategory> {
        self.ide_category.as_ref()
    }

    /// Programming Languages supported by CodeWhisperer
    pub fn programming_language(&self) -> ::std::option::Option<&crate::types::ProgrammingLanguage> {
        self.programming_language.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn lines_of_code_changed(&self) -> ::std::option::Option<i32> {
        self.lines_of_code_changed
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn chars_of_code_changed(&self) -> ::std::option::Option<i32> {
        self.chars_of_code_changed
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn lines_of_code_submitted(&self) -> ::std::option::Option<i32> {
        self.lines_of_code_submitted
    }
}
impl TransformEvent {
    /// Creates a new builder-style object to manufacture
    /// [`TransformEvent`](crate::types::TransformEvent).
    pub fn builder() -> crate::types::builders::TransformEventBuilder {
        crate::types::builders::TransformEventBuilder::default()
    }
}

/// A builder for [`TransformEvent`](crate::types::TransformEvent).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct TransformEventBuilder {
    pub(crate) job_id: ::std::option::Option<::std::string::String>,
    pub(crate) timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) ide_category: ::std::option::Option<crate::types::IdeCategory>,
    pub(crate) programming_language: ::std::option::Option<crate::types::ProgrammingLanguage>,
    pub(crate) lines_of_code_changed: ::std::option::Option<i32>,
    pub(crate) chars_of_code_changed: ::std::option::Option<i32>,
    pub(crate) lines_of_code_submitted: ::std::option::Option<i32>,
}
impl TransformEventBuilder {
    /// Identifier for the Transformation Job
    /// This field is required.
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_id = ::std::option::Option::Some(input.into());
        self
    }

    /// Identifier for the Transformation Job
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_id = input;
        self
    }

    /// Identifier for the Transformation Job
    pub fn get_job_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.job_id
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.timestamp = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_timestamp(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.timestamp = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_timestamp(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.timestamp
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn ide_category(mut self, input: crate::types::IdeCategory) -> Self {
        self.ide_category = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_ide_category(mut self, input: ::std::option::Option<crate::types::IdeCategory>) -> Self {
        self.ide_category = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_ide_category(&self) -> &::std::option::Option<crate::types::IdeCategory> {
        &self.ide_category
    }

    /// Programming Languages supported by CodeWhisperer
    pub fn programming_language(mut self, input: crate::types::ProgrammingLanguage) -> Self {
        self.programming_language = ::std::option::Option::Some(input);
        self
    }

    /// Programming Languages supported by CodeWhisperer
    pub fn set_programming_language(mut self, input: ::std::option::Option<crate::types::ProgrammingLanguage>) -> Self {
        self.programming_language = input;
        self
    }

    /// Programming Languages supported by CodeWhisperer
    pub fn get_programming_language(&self) -> &::std::option::Option<crate::types::ProgrammingLanguage> {
        &self.programming_language
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn lines_of_code_changed(mut self, input: i32) -> Self {
        self.lines_of_code_changed = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_lines_of_code_changed(mut self, input: ::std::option::Option<i32>) -> Self {
        self.lines_of_code_changed = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_lines_of_code_changed(&self) -> &::std::option::Option<i32> {
        &self.lines_of_code_changed
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn chars_of_code_changed(mut self, input: i32) -> Self {
        self.chars_of_code_changed = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_chars_of_code_changed(mut self, input: ::std::option::Option<i32>) -> Self {
        self.chars_of_code_changed = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_chars_of_code_changed(&self) -> &::std::option::Option<i32> {
        &self.chars_of_code_changed
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn lines_of_code_submitted(mut self, input: i32) -> Self {
        self.lines_of_code_submitted = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_lines_of_code_submitted(mut self, input: ::std::option::Option<i32>) -> Self {
        self.lines_of_code_submitted = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_lines_of_code_submitted(&self) -> &::std::option::Option<i32> {
        &self.lines_of_code_submitted
    }

    /// Consumes the builder and constructs a [`TransformEvent`](crate::types::TransformEvent).
    /// This method will fail if any of the following fields are not set:
    /// - [`job_id`](crate::types::builders::TransformEventBuilder::job_id)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::TransformEvent, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::TransformEvent {
            job_id: self.job_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "job_id",
                    "job_id was not specified but it is required when building TransformEvent",
                )
            })?,
            timestamp: self.timestamp,
            ide_category: self.ide_category,
            programming_language: self.programming_language,
            lines_of_code_changed: self.lines_of_code_changed,
            chars_of_code_changed: self.chars_of_code_changed,
            lines_of_code_submitted: self.lines_of_code_submitted,
        })
    }
}
