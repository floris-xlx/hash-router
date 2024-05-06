# Hash Router

!! STILL A WIP !!
!! LIMITED FUNCTIONALITY !!

The Hash Router crate provides a robust solution for routing Discord messages using unique identifiers (trade hashes) and integrating with databases for efficient data management and retrieval. This crate is designed to facilitate the mapping of Discord IDs (message, channel, and guild IDs) and trade hashes to specific services, enhancing the automation and organization of Discord message signals.

## Features

- **Discord Message Routing**: Efficiently route Discord messages based on trade hashes.
- **Database Integration**: Seamlessly integrates with databases, including local caching and Supabase for persistent storage.
- **Error Handling**: Comprehensive error handling module to manage and report routing errors.
- **Success Responses**: Abstracts success or OK responses for consistent API behavior.
- **Supabase Client**: Utilizes the `supabase_rs` crate to interact with Supabase, providing a Rust SDK client for database operations.

## Usage

This crate is structured into several modules, each responsible for a specific aspect of the hash routing process:

- **Router**: Handles the routing logic for Discord messages.
- **Database**: Manages database interactions, including local caching and Supabase integration.
- **Models**: Defines the data models used for routing, such as `DiscordMessage`.
- **Errors**: Centralizes error handling across the crate.
- **Success**: Defines success response abstractions.

To get started with the Hash Router, add it to your `Cargo.toml`:
```toml
[dependencies]
hash_router = "0.1.2"
```


