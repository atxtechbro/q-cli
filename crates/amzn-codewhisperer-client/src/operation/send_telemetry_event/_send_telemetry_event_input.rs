// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SendTelemetryEventInput {
    #[allow(missing_docs)] // documentation missing in model
    pub client_token: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub telemetry_event: ::std::option::Option<crate::types::TelemetryEvent>,
    #[allow(missing_docs)] // documentation missing in model
    pub opt_out_preference: ::std::option::Option<crate::types::OptOutPreference>,
    #[allow(missing_docs)] // documentation missing in model
    pub user_context: ::std::option::Option<crate::types::UserContext>,
    #[allow(missing_docs)] // documentation missing in model
    pub profile_arn: ::std::option::Option<::std::string::String>,
    /// Unique identifier for the model
    pub model_id: ::std::option::Option<::std::string::String>,
}
impl SendTelemetryEventInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn telemetry_event(&self) -> ::std::option::Option<&crate::types::TelemetryEvent> {
        self.telemetry_event.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn opt_out_preference(&self) -> ::std::option::Option<&crate::types::OptOutPreference> {
        self.opt_out_preference.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn user_context(&self) -> ::std::option::Option<&crate::types::UserContext> {
        self.user_context.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn profile_arn(&self) -> ::std::option::Option<&str> {
        self.profile_arn.as_deref()
    }

    /// Unique identifier for the model
    pub fn model_id(&self) -> ::std::option::Option<&str> {
        self.model_id.as_deref()
    }
}
impl SendTelemetryEventInput {
    /// Creates a new builder-style object to manufacture
    /// [`SendTelemetryEventInput`](crate::operation::send_telemetry_event::SendTelemetryEventInput).
    pub fn builder() -> crate::operation::send_telemetry_event::builders::SendTelemetryEventInputBuilder {
        crate::operation::send_telemetry_event::builders::SendTelemetryEventInputBuilder::default()
    }
}

/// A builder for
/// [`SendTelemetryEventInput`](crate::operation::send_telemetry_event::SendTelemetryEventInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct SendTelemetryEventInputBuilder {
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) telemetry_event: ::std::option::Option<crate::types::TelemetryEvent>,
    pub(crate) opt_out_preference: ::std::option::Option<crate::types::OptOutPreference>,
    pub(crate) user_context: ::std::option::Option<crate::types::UserContext>,
    pub(crate) profile_arn: ::std::option::Option<::std::string::String>,
    pub(crate) model_id: ::std::option::Option<::std::string::String>,
}
impl SendTelemetryEventInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn telemetry_event(mut self, input: crate::types::TelemetryEvent) -> Self {
        self.telemetry_event = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_telemetry_event(mut self, input: ::std::option::Option<crate::types::TelemetryEvent>) -> Self {
        self.telemetry_event = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_telemetry_event(&self) -> &::std::option::Option<crate::types::TelemetryEvent> {
        &self.telemetry_event
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn opt_out_preference(mut self, input: crate::types::OptOutPreference) -> Self {
        self.opt_out_preference = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_opt_out_preference(mut self, input: ::std::option::Option<crate::types::OptOutPreference>) -> Self {
        self.opt_out_preference = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_opt_out_preference(&self) -> &::std::option::Option<crate::types::OptOutPreference> {
        &self.opt_out_preference
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn user_context(mut self, input: crate::types::UserContext) -> Self {
        self.user_context = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_user_context(mut self, input: ::std::option::Option<crate::types::UserContext>) -> Self {
        self.user_context = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_user_context(&self) -> &::std::option::Option<crate::types::UserContext> {
        &self.user_context
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn profile_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.profile_arn = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_profile_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.profile_arn = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_profile_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.profile_arn
    }

    /// Unique identifier for the model
    pub fn model_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.model_id = ::std::option::Option::Some(input.into());
        self
    }

    /// Unique identifier for the model
    pub fn set_model_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.model_id = input;
        self
    }

    /// Unique identifier for the model
    pub fn get_model_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.model_id
    }

    /// Consumes the builder and constructs a
    /// [`SendTelemetryEventInput`](crate::operation::send_telemetry_event::SendTelemetryEventInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::send_telemetry_event::SendTelemetryEventInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::send_telemetry_event::SendTelemetryEventInput {
            client_token: self.client_token,
            telemetry_event: self.telemetry_event,
            opt_out_preference: self.opt_out_preference,
            user_context: self.user_context,
            profile_arn: self.profile_arn,
            model_id: self.model_id,
        })
    }
}
