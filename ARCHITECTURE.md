# Protlin SDK and Codec Server Architecture

**Official Architecture Document** | **Version 1.0.0** | **Copyright © 2026 Moude AI LLC and Moude Corp**

---

## Overview

Protlin uses a client-server architecture where developers install a lightweight SDK (client) that communicates with the Codec Server for code processing. This ensures intellectual property protection while providing a seamless development experience.

---

## Architecture Components

### 1. Protlin SDK (Client)

**Purpose:** Lightweight development environment installed locally

**Components:**
- Code editor integration
- Syntax highlighting
- Auto-completion
- Error display
- Result visualization
- Network communication layer

**Installation:**
```bash
# Install Protlin SDK
curl -sSL https://install.protlin.dev | sh

# Or via package manager
brew install protlin-sdk        # macOS
apt install protlin-sdk         # Linux
choco install protlin-sdk       # Windows
```

**Size:** ~50MB (vs 1GB+ for full compiler)

### 2. Codec Server (Server)

**Purpose:** Centralized code processing and execution

**Components:**
- Lexical analyzer (lexer)
- Syntax parser
- AST generator
- Interpreter/executor
- Graphics renderer
- Result serializer

**Location:** Cloud-hosted by Moude AI LLC
- Primary: `codec.protlin.dev`
- Backup: `codec-backup.protlin.dev`
- Regional: `codec-{region}.protlin.dev`

---

## Communication Flow

```
┌─────────────────┐         ┌──────────────────┐         ┌─────────────────┐
│   Developer     │         │   Protlin SDK    │         │  Codec Server   │
│   (Local)       │         │   (Client)       │         │   (Cloud)       │
└────────┬────────┘         └────────┬─────────┘         └────────┬────────┘
         │                           │                            │
         │  1. Write Code            │                            │
         ├──────────────────────────>│                            │
         │                           │                            │
         │                           │  2. Send Code + Auth       │
         │                           ├───────────────────────────>│
         │                           │                            │
         │                           │                            │  3. Lex
         │                           │                            ├────────┐
         │                           │                            │        │
         │                           │                            │<───────┘
         │                           │                            │
         │                           │                            │  4. Parse
         │                           │                            ├────────┐
         │                           │                            │        │
         │                           │                            │<───────┘
         │                           │                            │
         │                           │                            │  5. Execute
         │                           │                            ├────────┐
         │                           │                            │        │
         │                           │                            │<───────┘
         │                           │                            │
         │                           │  6. Return Results         │
         │                           │<───────────────────────────┤
         │                           │                            │
         │  7. Display Results       │                            │
         │<──────────────────────────┤                            │
         │                           │                            │
```

---

## Protocol Specification

### Request Format (SDK → Server)

```json
{
  "version": "1.0.0",
  "auth": {
    "api_key": "pk_live_...",
    "user_id": "user_123"
  },
  "request": {
    "type": "execute",
    "code": "println(\"Hello, Protlin!\")",
    "options": {
      "graphics": true,
      "theme": "auto",
      "timeout": 30000
    }
  },
  "metadata": {
    "sdk_version": "1.0.0",
    "platform": "linux",
    "timestamp": 1234567890
  }
}
```

### Response Format (Server → SDK)

```json
{
  "version": "1.0.0",
  "status": "success",
  "result": {
    "output": "Hello, Protlin!\n",
    "errors": [],
    "warnings": [],
    "execution_time": 125,
    "graphics": {
      "windows": [
        {
          "id": "window_0",
          "width": 800,
          "height": 600,
          "buffer": "base64_encoded_image_data"
        }
      ]
    }
  },
  "metadata": {
    "server_version": "1.0.0",
    "region": "us-east-1",
    "timestamp": 1234567890
  }
}
```

---

## SDK Installation Guide

### Step 1: Install SDK

```bash
# Download and install
curl -sSL https://install.protlin.dev | sh

# Verify installation
protlin --version
# Output: Protlin SDK v1.0.0
```

### Step 2: Authenticate

```bash
# Login with API key
protlin auth login

# Or set API key directly
export PROTLIN_API_KEY="pk_live_your_key_here"
```

### Step 3: Run Code

```bash
# Execute a file
protlin run hello.prot

# Interactive mode
protlin repl

# Watch mode (auto-reload)
protlin watch app.prot
```

---

## SDK Commands

### Core Commands

```bash
# Run a Protlin file
protlin run <file.prot>

# Start REPL
protlin repl

# Check syntax
protlin check <file.prot>

# Format code
protlin fmt <file.prot>

# Watch and auto-reload
protlin watch <file.prot>
```

### Authentication

```bash
# Login
protlin auth login

# Logout
protlin auth logout

# Check status
protlin auth status
```

### Configuration

```bash
# Set server endpoint
protlin config set server codec.protlin.dev

# Set theme
protlin config set theme dark

# View config
protlin config list
```

---

## Codec Server API

### Endpoints

**Base URL:** `https://codec.protlin.dev/api/v1`

#### 1. Execute Code
```
POST /execute
Content-Type: application/json
Authorization: Bearer <api_key>

Body: {
  "code": "println(\"Hello\")",
  "options": { "graphics": true }
}

Response: {
  "status": "success",
  "output": "Hello\n",
  "execution_time": 125
}
```

#### 2. Lex Code
```
POST /lex
Content-Type: application/json
Authorization: Bearer <api_key>

Body: {
  "code": "x = 5"
}

Response: {
  "tokens": [
    {"type": "Identifier", "value": "x"},
    {"type": "Assign", "value": "="},
    {"type": "Integer", "value": "5"}
  ]
}
```

#### 3. Parse Code
```
POST /parse
Content-Type: application/json
Authorization: Bearer <api_key>

Body: {
  "code": "x = 5"
}

Response: {
  "ast": {
    "type": "VariableDeclaration",
    "name": "x",
    "value": {"type": "Integer", "value": 5}
  }
}
```

#### 4. Check Syntax
```
POST /check
Content-Type: application/json
Authorization: Bearer <api_key>

Body: {
  "code": "x = "
}

Response: {
  "valid": false,
  "errors": [
    {
      "line": 1,
      "column": 5,
      "message": "Expected expression"
    }
  ]
}
```

---

## Security Features

### 1. Authentication
- API key-based authentication
- JWT tokens for session management
- Rate limiting per user/key

### 2. Code Isolation
- Sandboxed execution environment
- Resource limits (CPU, memory, time)
- Network isolation

### 3. Encryption
- TLS 1.3 for all communications
- End-to-end encryption for code
- Encrypted result transmission

### 4. Intellectual Property Protection
- Code never stored on client
- Server-side compilation only
- No local compiler distribution

---

## Performance Optimization

### 1. Caching
- AST caching for repeated code
- Result caching for pure functions
- CDN for SDK distribution

### 2. Regional Servers
- Auto-routing to nearest server
- Failover to backup servers
- Load balancing

### 3. Compression
- Gzip compression for requests/responses
- Binary protocol for graphics data
- Delta encoding for incremental updates

---

## Pricing Tiers

### Free Tier
- 1,000 executions/month
- 5 second timeout
- Community support
- Public code only

### Pro Tier ($29/month)
- 100,000 executions/month
- 30 second timeout
- Email support
- Private code
- Priority processing

### Enterprise Tier (Custom)
- Unlimited executions
- Custom timeout
- Dedicated support
- On-premise deployment option
- SLA guarantee

---

## SDK Configuration File

**Location:** `~/.protlin/config.toml`

```toml
[auth]
api_key = "pk_live_..."
user_id = "user_123"

[server]
endpoint = "https://codec.protlin.dev"
region = "auto"
timeout = 30000

[editor]
theme = "dark"
font_size = 14
auto_save = true

[execution]
graphics = true
auto_theme = true
max_windows = 10
```

---

## Error Handling

### Network Errors
```json
{
  "status": "error",
  "error": {
    "code": "NETWORK_ERROR",
    "message": "Failed to connect to Codec Server",
    "retry_after": 5000
  }
}
```

### Syntax Errors
```json
{
  "status": "error",
  "error": {
    "code": "SYNTAX_ERROR",
    "message": "Unexpected token",
    "line": 5,
    "column": 12
  }
}
```

### Runtime Errors
```json
{
  "status": "error",
  "error": {
    "code": "RUNTIME_ERROR",
    "message": "Division by zero",
    "stack_trace": ["line 10: x / y"]
  }
}
```

---

## Benefits

### For Developers
✅ Lightweight installation (~50MB vs 1GB+)
✅ No local compilation needed
✅ Always up-to-date compiler
✅ Cross-platform compatibility
✅ Cloud-based execution

### For Moude AI LLC
✅ Intellectual property protection
✅ Centralized updates
✅ Usage analytics
✅ Revenue from API usage
✅ Controlled distribution

---

## Roadmap

### Q2 2026
- [ ] SDK v1.0 release
- [ ] Codec Server deployment
- [ ] VS Code extension
- [ ] Web-based IDE

### Q3 2026
- [ ] Offline mode (cached execution)
- [ ] Collaborative editing
- [ ] Real-time debugging
- [ ] Performance profiler

### Q4 2026
- [ ] On-premise server option
- [ ] Enterprise features
- [ ] Advanced analytics
- [ ] Custom plugins

---

**Copyright © 2026 Moude AI LLC and Moude Corp. All Rights Reserved.**

Protlin™ and Codec Server™ are registered trademarks of Moude AI LLC and Moude Corp.
