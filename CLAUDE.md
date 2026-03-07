# justpostthings

A Rust CLI tool for posting content to multiple social media platforms simultaneously via the **Buffer API**. Supports text with optional images, post scheduling, and per-channel translation.

## Tech Stack

- **Rust** (Edition 2021) with **Tokio** async runtime
- **clap** for CLI argument parsing
- **reqwest** for HTTP (JSON + multipart)
- **serde/serde_json** for serialization

## External Services

| Service | Purpose | Auth |
|---------|---------|------|
| Buffer (GraphQL) | Social media posting (X, LinkedIn) | Bearer token |
| OpenAI (gpt-4o-mini) | Text translation | Bearer token |
| Google Gemini (2.0 Flash) | Text translation (alternative) | Query param |
| ImgBB | Image hosting for local files | API key |

API keys are loaded from `.env` (`BUFFER_API_KEY`, `OPENAI_API_KEY`, `GEMINI_API_KEY`, `IMGBB_API_KEY`).

## Project Structure

```
src/
├── main.rs              # CLI entry point, config loading, Buffer GraphQL posting
├── translation.rs       # TranslationService trait + factory function
├── openai_service.rs    # OpenAI translation implementation
├── gemini_service.rs    # Google Gemini translation implementation
└── imgbb.rs             # ImgBB image upload (local file → base64 → public URL)
```

- **config.json** — Channel definitions (name, Buffer ID, translation settings), default channels, and translation service selection.

## Key Patterns

- **Trait-based abstraction**: `TranslationService` trait with OpenAI/Gemini implementations, selected via factory in `translation.rs`
- **Lazy initialization**: Translation service only created if a channel requires it
- **Image resolution**: Local paths are uploaded to ImgBB; URLs are used as-is
- **Per-channel translation**: Each channel can independently enable translation with its own from/to language pair

## CLI Usage

```bash
cargo run -- "Post text here" \
  --image /path/to/local.jpg \
  --image https://example.com/image.png \
  --schedule "2025-03-15T10:00:00Z" \
  --channels x,linkedin \
  --config ./config.json
```

## Build & Run

```bash
cargo build            # Debug build
cargo build --release  # Release build
cargo run -- "text"    # Run directly
```
