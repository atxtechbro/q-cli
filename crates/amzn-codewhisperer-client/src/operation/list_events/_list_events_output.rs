// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListEventsOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub conversation_id: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub events: ::std::vec::Vec<crate::types::Event>,
    #[allow(missing_docs)] // documentation missing in model
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListEventsOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn conversation_id(&self) -> &str {
        use std::ops::Deref;
        self.conversation_id.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn events(&self) -> &[crate::types::Event] {
        use std::ops::Deref;
        self.events.deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for ListEventsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListEventsOutput {
    /// Creates a new builder-style object to manufacture
    /// [`ListEventsOutput`](crate::operation::list_events::ListEventsOutput).
    pub fn builder() -> crate::operation::list_events::builders::ListEventsOutputBuilder {
        crate::operation::list_events::builders::ListEventsOutputBuilder::default()
    }
}

/// A builder for [`ListEventsOutput`](crate::operation::list_events::ListEventsOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListEventsOutputBuilder {
    pub(crate) conversation_id: ::std::option::Option<::std::string::String>,
    pub(crate) events: ::std::option::Option<::std::vec::Vec<crate::types::Event>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListEventsOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn conversation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.conversation_id = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_conversation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.conversation_id = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_conversation_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.conversation_id
    }

    /// Appends an item to `events`.
    ///
    /// To override the contents of this collection use [`set_events`](Self::set_events).
    pub fn events(mut self, input: crate::types::Event) -> Self {
        let mut v = self.events.unwrap_or_default();
        v.push(input);
        self.events = ::std::option::Option::Some(v);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_events(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Event>>) -> Self {
        self.events = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_events(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Event>> {
        &self.events
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }

    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }

    /// Consumes the builder and constructs a
    /// [`ListEventsOutput`](crate::operation::list_events::ListEventsOutput). This method will
    /// fail if any of the following fields are not set:
    /// - [`conversation_id`](crate::operation::list_events::builders::ListEventsOutputBuilder::conversation_id)
    /// - [`events`](crate::operation::list_events::builders::ListEventsOutputBuilder::events)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_events::ListEventsOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_events::ListEventsOutput {
            conversation_id: self.conversation_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "conversation_id",
                    "conversation_id was not specified but it is required when building ListEventsOutput",
                )
            })?,
            events: self.events.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "events",
                    "events was not specified but it is required when building ListEventsOutput",
                )
            })?,
            next_token: self.next_token,
            _request_id: self._request_id,
        })
    }
}
