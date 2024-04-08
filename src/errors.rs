//! This module abstracts the errors encountered within the hash routing process.
//!
//! It provides a structured way to represent and handle errors that occur when routing trade hashes.
//! The main structure used for this purpose is `HashRoutingError`.
//! 
//! We use this to keep a centralised overview of all error handling and display


use std::fmt;


/// Represents an error that can occur during the hash routing process.
///
/// This structure is used to encapsulate errors in a way that they can be easily identified and handled.
/// It contains a single field, `error`, which is a `String` describing the error that occurred.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let error = HashRoutingError { error: "Trade hash not found".to_string() };
/// println!("{}", error);
/// ```
#[derive(Debug, Clone)]
pub struct HashRoutingError {
    /// A human-readable description of the error.
    pub error: Errors,
}


/// Represents the different types of errors that can occur during the hash routing process.
/// 
/// ## Variants
/// * `TradeHashNotFound` - Trade hash not found.
/// * `TradeHashInvalid` - Trade hash is invalid.
/// * `TradeHashAlreadyRouted` - Trade hash has already been routed.
/// * `MessageIdNotFound` - Message ID not found.
/// * `MessageIdInvalid` - Message ID is invalid.
/// * `ChannelIdNotFound` - Channel ID not found.
/// * `ChannelIdInvalid` - Channel ID is invalid.
/// * `GuildIdNotFound` - Guild ID not found.
/// * `GuildIdInvalid` - Guild ID is invalid.
/// * `FailedToInitializeSupabase` - Failed to initialize Supabase client.
/// * `FailedToUploadToSupabase` - Failed to upload to Supabase.
/// * `FailedToGetMessageByTradeHash` - Failed to get message by trade hash.
/// 
/// ## How to add new error types
/// If you want to add a new error type and something is not granular enough yet, simply add it into the `Errors` enum and then implement the `Display` trait for it.
/// 
#[derive(Debug, Clone)]
pub enum Errors {
    TradeHashNotFound,
    TradeHashInvalid,
    TradeHashAlreadyRouted,
    MessageIdNotFound,
    MessageIdInvalid,
    ChannelIdNotFound,
    ChannelIdInvalid,
    GuildIdNotFound,
    GuildIdInvalid,
    FailedToInitializeSupabase,
    FailedToUploadToSupabase,
    FailedToGetMessageByTradeHash
}


/// Implements the `Display` trait for `HashRoutingError`.
///
/// This implementation is used to format the error message for display purposes.
///
/// # Notes
/// - We use `{:?}` to print the error enum as a string. and avoid the collision of the Enum `Errors` not having the `fmt::Display` implementation
/// - If `stdout` isn't propagated to some error logging client, then these errors will only be visible in the CLI when in development mode.
/// 
impl fmt::Display for HashRoutingError {
    fn fmt(
        &self, 
        f: &mut fmt::Formatter<'_>
    ) -> fmt::Result {

        write!(f, "{:?}", self.error) 
    }
}


/// Implements the `Display` trait for `Errors`.
///
/// This implementation is used to format the error message for display purposes.
/// 
/// ## Variants
/// * `TradeHashNotFound` - Trade hash not found.
/// * `TradeHashInvalid` - Trade hash is invalid.
/// * `TradeHashAlreadyRouted` - Trade hash has already been routed.
/// * `MessageIdNotFound` - Message ID not found.
/// * `MessageIdInvalid` - Message ID is invalid.
/// * `ChannelIdNotFound` - Channel ID not found.
/// * `ChannelIdInvalid` - Channel ID is invalid.
/// * `GuildIdNotFound` - Guild ID not found.
/// * `GuildIdInvalid` - Guild ID is invalid.
/// * `FailedToInitializeSupabase` - Failed to initialize Supabase client   
/// * `FailedToUploadToSupabase` - Failed to upload to Supabase
/// * `FailedToGetMessageByTradeHash` - Failed to get message by trade hash
/// 
/// ## Examples
/// 
/// ```
/// let error = Errors::TradeHashNotFound;
/// println!("{}", error);
/// 
/// // console print
/// "Trade hash not found."
/// 
/// ```
/// 
/// ## Notes
/// - These errors can be deferred down to any logging client you pick, they are returned so that the CLI can be used in development mode.
/// Avoid writing or printing just to `stdout` in production but to an appriopriate error outlet
/// 
impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::TradeHashNotFound => write!(f, "Trade hash not found."),
            Errors::TradeHashInvalid => write!(f, "Trade hash is invalid."),
            Errors::TradeHashAlreadyRouted => write!(f, "Trade hash has already been routed."),
            Errors::MessageIdNotFound => write!(f, "Message ID not found."),
            Errors::MessageIdInvalid => write!(f, "Message ID is invalid."),
            Errors::ChannelIdNotFound => write!(f, "Channel ID not found."),
            Errors::ChannelIdInvalid => write!(f, "Channel ID is invalid."),
            Errors::GuildIdNotFound => write!(f, "Guild ID not found."),
            Errors::GuildIdInvalid => write!(f, "Guild ID is invalid."),
            Errors::FailedToInitializeSupabase => write!(f, "Failed to initialize Supabase client."),
            Errors::FailedToUploadToSupabase => write!(f, "Failed to upload to Supabase."),
            Errors::FailedToGetMessageByTradeHash => write!(f, "Failed to get message by trade hash."),
        }
    }
}