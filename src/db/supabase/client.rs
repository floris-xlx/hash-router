//! This module contains the Supabase client for the xylex hash router.
//!
//! ## Table of contents
//! - We initialize the Supabase SDK client here
//! - All our methods will be routed thru here via base structs
//! - We will have a `Supabase` struct which will have a `Client` struct inside of it
//! - This is the main struct that will be used to interact with the database
//!
#![allow(unused_imports)]
#![allow(unused_variables)]

use supabase_rs::SupabaseClient;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use serde_json::{
    Value,
    json
};

use crate::model::DiscordMessage;
use crate::errors::{HashRoutingError, Errors};



/// Initializes the Supabase client.
///
/// ### `.env` dependencies
/// - `SUPABASE_PUBLIC_URL` - The public URL of the Supabase project
/// - `SUPABASE_ANON_KEY` - The anonymous key of the Supabase project
///
/// With the `.env` dependencies, the client can be initialized, as well as the `.env` file
/// should be located in the root of the project.
///
/// ### Returns
/// - `Ok(supabase_client)` - The Supabase client
/// - `Err(Box<dyn Error>)` - An error occurred
///
pub fn initialize_supabase_client(
)
    -> Result<SupabaseClient, Box<dyn Error>> {
    dotenv().ok();

    let supabase_url: String = env::var(
        "SUPABASE_PUBLIC_URL" // env var name
    )?;

    let supabase_key: String = env::var(
        "SUPABASE_ANON_KEY" // env var name
    )?;

    let supabase_client: SupabaseClient = SupabaseClient::new(
        supabase_url,
        supabase_key
    );

    Ok(supabase_client)
}

/// Implements the `DiscordMessage` struct.
///
/// ### Usage
/// ```rust
/// use xylex_hash_router::db::supabase::client::add_new_message;
/// use xylex_hash_router::model::DiscordMessage;
///
///
/// let message = DiscordMessage::new(
///     message_id,
///     channel_id,
///     guild_id,
///     trade_hash
/// );
///
/// let trade_hash = message.add_new_message().await?;
///
/// assert_eq!(trade_hash, "trade_hash");
///
/// ```
impl DiscordMessage {
    /// Adds a new message to the Supabase database.
    ///
    /// ### Data
    /// - `message_id` - The ID of the message
    /// - `channel_id` - The ID of the channel
    /// - `guild_id` - The ID of the guild
    /// - `trade_hash` - The trade hash
    ///
    /// ### Returns
    /// - `Ok(trade_hash)` - The trade hash
    /// - `Err(Errors::FailedToUploadToSupabase)` - Failed to upload to Supabase
    ///
    /// ### Usage
    /// ```rust
    /// use xylex_hash_router::db::supabase::client::add_new_message;
    /// use xylex_hash_router::model::DiscordMessage;
    ///
    ///
    /// let message = DiscordMessage::new(
    ///     message_id,
    ///     channel_id,
    ///     guild_id,
    ///     trade_hash
    /// );
    ///
    /// let trade_hash = message.add_new_message().await?;
    ///
    /// assert_eq!(trade_hash, "trade_hash");
    ///
    ///
    /// ```
    ///
    /// ### Errors & Exceptions
    /// - `Err(Errors::FailedToUploadToSupabase)` - Failed to upload to Supabase
    /// - `Err(Errors::FailedToInitializeSupabaseClient)` - Failed to initialize Supabase client
    ///
    pub async fn add_new_message(
        &self
    ) -> Result<String, Errors> {

        let supabase_client: SupabaseClient = initialize_supabase_client().expect(
            "Failed to initialize Supabase client"
        );

        let response: Result<String, String> = supabase_client
            .insert_if_unique(
                "hash_router",
                json!({
                    "message_id": self.message_id,
                    "channel_id": self.channel_id,
                    "guild_id": self.guild_id,
                    "trade_hash": self.trade_hash
                })
            )
            .await;

        match response {
            Ok(_) => Ok(self.trade_hash.to_string()),

            Err(e) => Err(Errors::FailedToUploadToSupabase),
        }
    }


    pub async fn get_message_by_trade_hash(
        &self,
        trade_hash: String
    ) -> Result<Vec<Value>, Errors> {
        let supabase_client: SupabaseClient = initialize_supabase_client().expect(
            "Failed to initialize Supabase client"
        );

        let data: Result<Vec<Value>, String> = supabase_client
            .select("hash_router")
            .eq("trade_hash", trade_hash.as_str())
            .execute()
        .await;


        match data {
            Ok(data) => Ok(data),
            Err(e) => Err(Errors::FailedToGetMessageByTradeHash),
        }
    }
}
