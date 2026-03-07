# justpostthings

A command-line tool for posting content to multiple social media platforms simultaneously via the [Buffer](https://buffer.com) API. Publish text posts with optional images to X (formerly Twitter) and LinkedIn in a single command.

## Features

- Post text to multiple social media channels at once
- Attach multiple images via URLs
- Schedule posts for a specific date/time
- Override default channels per post
- Configurable default channels

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021)
- A [Buffer](https://buffer.com) account and API key

## Setup

1. Clone the repository:

   ```bash
   git clone <repo-url>
   cd justpostthings
   ```

2. Create a `.env` file in the project root:

   ```
   BUFFER_API_KEY=your_api_key_here
   ```

3. Create a `config.json` with your channel configuration:

   ```json
   {
     "channels": [
       { "name": "x", "id": "your_x_channel_id" },
       { "name": "linkedin", "id": "your_linkedin_channel_id" }
     ],
     "default_post_channels": ["x", "linkedin"]
   }
   ```

   Channel IDs can be found in your Buffer account settings.

4. Build the project:

   ```bash
   cargo build --release
   ```

## Usage

```bash
cargo run -- "Your post text here" [OPTIONS]
```

Or using the built binary:

```bash
./target/release/justpostthings "Your post text here" [OPTIONS]
```

### Options

| Option | Description |
|---|---|
| `<TEXT>` | The text content of the post (positional, required) |
| `--image <URL>` | Image URL to attach (can be repeated for multiple images) |
| `--schedule <DATETIME>` | Schedule the post for a future time (ISO 8601 format) |
| `--channels <NAMES>` | Comma-separated list of channels to post to (overrides defaults) |
| `--config <PATH>` | Path to config file (default: `./config.json`) |

### Examples

Post to all default channels:

```bash
cargo run -- "Check out my new project!"
```

Post with images:

```bash
cargo run -- "New product launch!" --image https://example.com/img1.jpg --image https://example.com/img2.jpg
```

Schedule a post:

```bash
cargo run -- "Coming soon!" --schedule "2025-03-15T10:00:00Z"
```

Post to specific channels only:

```bash
cargo run -- "X-only post" --channels x
```

Use a custom config file:

```bash
cargo run -- "My post" --config ./other_config.json
```

## Environment Variables

| Variable | Required | Description |
|---|---|---|
| `BUFFER_API_KEY` | Yes | Your Buffer API authentication token |
