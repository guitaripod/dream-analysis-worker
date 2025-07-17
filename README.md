# Dream Analysis Worker

A Cloudflare Worker that provides AI-powered dream analysis and interpretation. Built with Rust and WebAssembly for high performance and low latency.

## Overview

This service analyzes dream descriptions and provides thoughtful, conversational interpretations. Using advanced AI models, it offers insights into potential meanings, symbols, and psychological perspectives to help users understand their dreams.

## Features

- **AI-Powered Analysis**: Uses Mistral 7B Instruct v0.1 model for intelligent dream interpretation
- **Natural Language Response**: Provides conversational, thoughtful analysis of dream content
- **Symbol and Theme Recognition**: Identifies and interprets common dream symbols and themes
- **Psychological Perspective**: Offers insights based on dream psychology principles
- **Fast Response Times**: Leverages Cloudflare's edge network for low latency
- **CORS Support**: Fully configured for cross-origin requests

## API Usage

### Endpoint

```
POST https://dream-expert-analysis.guitaripod.workers.dev
```

### Request

```json
{
  "dreamPrompt": "Your dream description here..."
}
```

### Response

```json
{
  "analysis": {
    "response": "A detailed interpretation of your dream..."
  }
}
```

### Example

```bash
curl -X POST https://your-worker-name.workers.dev \
  -H "Content-Type: application/json" \
  -d '{"dreamPrompt": "I dreamed about flying over mountains"}' \
  -s | jq
```

## Technical Details

- **Runtime**: Cloudflare Workers with Rust/WASM
- **AI Model**: Cloudflare AI Workers (@cf/mistral/mistral-7b-instruct-v0.1)
- **Max Dream Length**: 5000 characters
- **Performance**: Sub-second response times
- **Scalability**: Automatic global distribution via Cloudflare's edge network

## Development

### Prerequisites

- Node.js 20+
- Rust and wasm-pack
- Cloudflare account with Workers enabled

### Local Development

```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build

# Deploy to Cloudflare
npm run deploy
```

### Environment Variables

Required Cloudflare configuration in `wrangler.toml`:
- Account ID
- AI binding configuration

## Deployment

The project includes automated deployment via GitHub Actions. Pushes to the `master` branch automatically deploy to Cloudflare Workers.

## License

MIT License
