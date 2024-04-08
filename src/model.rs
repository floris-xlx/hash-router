//! Models for hash router
//!
//! We implement several structs to faciliate the tracking of discord messages or other ways trades have been sent out by their
//! `trade_hash`
//!
//! ###
//!
//!

/// ### `DiscordMessage` is the struct that represents a discord message
/// Usually a trade is attached to this message which also routes back to it's own `trade_hash`
///
pub struct DiscordMessage {
    pub channel_id: u64,
    pub guild_id: u64,
    pub message_id: u64,
    pub trade_hash: String
}


/// ## `DiscordMessage` Implementation
///
///
impl DiscordMessage {
    /// Constructs a new `DiscordMessage`
    ///
    /// ## Usage
    /// ```
    /// let message = DiscordMessage::new(channel_id, guild_id, message_id, trade_hash);
    ///
    /// assert_eq!(message.channel_id, channel_id);
    /// assert_eq!(message.guild_id, guild_id);
    /// assert_eq!(message.message_id, message_id);
    /// assert_eq!(message.trade_hash, trade_hash);
    /// ```
    pub fn new(
        channel_id: u64,
        guild_id: u64,
        message_id: u64,
        trade_hash: String
    ) -> Self {

        DiscordMessage {
            channel_id,
            guild_id,
            message_id,
            trade_hash
        }
    }

    /// Constructs a link to the discord message
    ///
    /// ## Usage
    /// ```
    /// let message = DiscordMessage::new(channel_id, guild_id, message_id, trade_hash);
    /// assert_eq!(message.construct_message_link(), "https://discord.com/channels/{}/{}/{}", self.guild_id, self.channel_id, self.message_id);
    /// ```
    ///
    /// This is incredibily useful for discord embeds linking to a specific trade or message
    ///
    pub fn construct_message_link(
        &self
    ) -> String {
        format!("https://discord.com/channels/{}/{}/{}", self.guild_id, self.channel_id, self.message_id)
    }
}

