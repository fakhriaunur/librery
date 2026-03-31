# Librery - Terminal Specification (Ratatui)

> Generated: March 2026
> Purpose: Power user terminal interface for librarians

---

## 1. Project Structure

```
librery-tui/
├── src/
│   ├── main.rs              # Entry point
│   ├── app.rs                # TUI app state
│   ├── screens/              # Terminal screens
│   │   ├── home.rs
│   │   ├── checkout.rs
│   │   ├── return.rs
│   │   ├── search.rs
│   │   └── patron.rs
│   ├── components/           # Reusable widgets
│   │   ├── input.rs
│   │   ├── table.rs
│   │   └── status.rs
│   ├── commands/             # Command handlers
│   │   └── mod.rs
│   └── lib.rs
├── justfile                  # Build commands
└── Cargo.toml
```

---

## 2. Command Structure

### Command Palette

| Key | Command | Description |
|-----|---------|-------------|
| `c` | Checkout | Check out item to patron |
| `r` | Return | Return item |
| `h` | Hold | Place hold on item |
| `/` | Search | Search catalog |
| `p` | Patron | Look up patron |
| `q` | Quit | Exit application |

### Input Flow

```
User presses 'c'
    ↓
Checkout screen renders
    ↓
User enters item barcode + patron ID
    ↓
Command sent to Rust core
    ↓
Response displayed in terminal
```

---

## 3. Key Screens

### Home Screen

```
┌─────────────────────────────────────────────────────────────┐
│  LIBRERY v0.1                                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  [C] Checkout    [R] Return    [H] Hold                   │
│  [/] Search      [P] Patron    [Q] Quit                   │
│                                                             │
│  ─────────────────────────────────────────────────────────  │
│                                                             │
│  Today's Activity:                                          │
│  • 12 checkouts                                            │
│  • 8 returns                                               │
│  • 3 holds                                                 │
│                                                             │
│  Status: Online                                            │
└─────────────────────────────────────────────────────────────┘
```

### Checkout Screen

```
┌─────────────────────────────────────────────────────────────┐
│  CHECKOUT                                                  │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Item Barcode: [____________]                              │
│  Patron ID:    [____________]                              │
│                                                             │
│  Press ENTER to confirm                                    │
│  Press ESC to cancel                                       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 4. Rust Core Integration

### Direct Core Embedding

```rust
// TUI directly links to librery-core
use librery_core::domain::circulation::{CheckoutCommand, process_checkout};

// No HTTP needed - direct function call
fn handle_checkout(item_id: String, patron_id: String) {
    let cmd = CheckoutCommand {
        item_id: ItemId::from(item_id),
        patron_id: ActorId::from(patron_id),
        timestamp: Utc::now(),        // Injected by shell
        correlation_id: Uuid::new_v4(), // Injected by shell
    };
    
    let result = process_checkout(cmd);
    // Handle result
}
```

### Network Mode

When connected to remote server:

```rust
// Use reqwest with HTTP/3
let client = reqwest::Client::new();
let response = client.post("http://localhost:8080/api/v1/checkout")
    .json(&checkout_request)
    .send()
    .await;
```

---

## 5. Event Handling

### Key Events

```rust
enum TuiEvent {
    Key(crossterm::event::Key),
    Resize(u16, u16),
    Tick,
}
```

### Command Processing

```rust
pub fn process_command(cmd: &str, state: &AppState) -> CommandResult {
    match cmd {
        "c" => CommandResult::Navigate(Screen::Checkout),
        "r" => CommandResult::Navigate(Screen::Return),
        "/" => CommandResult::Navigate(Screen::Search),
        "q" => CommandResult::Exit,
        _ => CommandResult::Unknown,
    }
}
```

---

## 6. Offline Mode

### Local SQLite

```rust
// Direct SQLite connection for offline admin
let pool = SqlitePool::connect("sqlite:librery.db").await;

// Queue commands when offline
if !is_online() {
    queue_local_command(cmd);
}
```

---

## 7. Build

```bash
# Build for current platform
cargo build --release -p librery-tui

# Cross-compile
cargo build --release -p librery-tui --target x86_64-unknown-linux-gnu
```

---

*Document Version: 1.0*
*Generated: March 2026*