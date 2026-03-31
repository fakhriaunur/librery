# Librery - Staff App Specification (Flutter + FRB)

> Generated: March 2026
> Purpose: Librarian-facing mobile/desktop app with offline capability

---

## 1. Project Structure

```
librery-staff/
├── lib/
│   ├── main.dart                 # App entry
│   ├── app.dart                  # MaterialApp
│   ├── screens/                  # UI screens
│   │   ├── home/
│   │   ├── checkout/
│   │   ├── return/
│   │   ├── catalog/
│   │   ├── patron/
│   │   └── analytics/
│   ├── widgets/                  # Reusable widgets
│   ├── services/                 # Business logic
│   └── theme/
├── test/
├── integration_test/
├── ios/
├── android/
├── linux/
├── windows/
├── macos/
├── justfile                      # Build commands
└── pubspec.yaml
```

---

## 2. Flutter Rust Bridge (FRB)

### pubspec.yaml

```yaml
dependencies:
  flutter_rust_bridge: any

dev_dependencies:
  flutter_rust_bridge_codegen: any
```

### Code Generation

```bash
# Generate Dart bindings from Rust core
flutter_rust_bridge_codegen
```

### Generated API

```dart
// Dart calls Rust directly - no HTTP needed for local operations
import 'package:librery_staff/librery_core.dart';

void checkoutItem(String itemId, String patronId) async {
  final result = await libreryCore.checkout(
    itemId: itemId, 
    patronId: patronId
  );
  
  if (result.isOk()) {
    print('Checkout successful');
  }
}
```

---

## 3. Offline Architecture

### Local Storage (in Rust core)

```rust
// Local SQLite for offline queue
pub struct OfflineStore {
    pool: SqlitePool,
}

impl OfflineStore {
    pub async fn queue_command(&self, cmd: Command) -> Result<(), DbError> {
        // Store command locally when offline
    }
    
    pub async fn sync_when_online(&self) -> Result<(), SyncError> {
        // Sync queue to server when connection restored
    }
}
```

### CRDT Sync (Automerge)

```rust
// Offline bibliographic editing
use automerge::Autodoc;

pub fn merge_changes(local: &Doc, remote: &Doc) -> Doc {
    // CRDT merge - conflict-free
}
```

---

## 4. Screens

### Home Screen

- Dashboard with quick actions
- Today's activity summary
- Offline status indicator

### Checkout Screen

- Barcode scanner input
- Patron lookup
- Confirm checkout

### Return Screen

- Scan item to return
- Check for overdue fines

### Catalog Screen

- Search bibliographic records
- Add/edit items
- View item history

### Patron Screen

- Search patrons
- View patron details
- Manage privileges

### Analytics Screen

- Charts (fl_chart)
- Circulation statistics
- Fine revenue

---

## 5. HTTP/3 via Rust (not Dart)

### Why Rust-driven Networking

Dart's HTTP client lacks native HTTP/3 support. Flutter routes all network calls through embedded Rust core:

```rust
// In librery-core (Rust)
pub async fn http_client_get(url: &str) -> Result<String, HttpError> {
    // Uses reqwest with HTTP/3 QUIC
    let client = reqwest::Client::builder()
        .use_http3(true)
        .build()
        .unwrap();
    
    client.get(url).send().await
}
```

### Fallback

If UDP is blocked (firewall), Rust automatically falls back to HTTP/2.

---

## 6. State Management

### BLoC Pattern

```dart
// checkout_bloc.dart
class CheckoutBloc extends Bloc<CheckoutEvent, CheckoutState> {
  // Events: ScanItem, ScanPatron, ConfirmCheckout
  // States: Initial, ItemScanned, PatronScanned, Success, Error
}
```

### Offline State

```dart
enum NetworkStatus {
  online,
  offline,
  syncing,
}
```

---

## 7. Testing

| Test Type | Tool | Scope |
|-----------|------|-------|
| Widget | Flutter test | UI components |
| Integration | integration_test | Full flows |
| Offline | Flutter test | Queue sync |

---

## 8. Build Targets

| Platform | Command | Output |
|----------|---------|--------|
| iOS | `flutter build ios` | `.ipa` |
| Android | `flutter build apk` | `.apk` |
| Linux | `flutter build linux` | `.deb` |
| Windows | `flutter build windows` | `.exe` |
| macOS | `flutter build macos` | `.dmg` |

---

*Document Version: 1.0*
*Generated: March 2026*