# Librery - OPAC Frontend Specification (SvelteKit)

> Generated: March 2026
> Purpose: Patron-facing web interface for search, account, holds

---

## 1. Project Structure

```
librery-opac/
├── src/
│   ├── lib/
│   │   ├── components/       # Reusable UI components
│   │   │   ├── SearchBar.svelte
│   │   │   ├── ItemCard.svelte
│   │   │   ├── HoldButton.svelte
│   │   │   └── Header.svelte
│   │   ├── api/             # OpenAPI client (generated)
│   │   │   └── client.ts
│   │   ├── stores/          # Svelte stores
│   │   │   └── auth.ts
│   │   └── types/           # TypeScript types
│   │       └── index.ts
│   ├── routes/
│   │   ├── +layout.svelte   # Global layout
│   │   ├── +page.svelte     # Home / Search
│   │   ├── search/
│   │   │   └── +page.svelte
│   │   ├── item/
│   │   │   └── [id]/
│   │   │       └── +page.svelte
│   │   ├── account/
│   │   │   └── +page.svelte
│   │   └── login/
│   │       └── +page.svelte
│   └── app.html
├── static/
├── justfile
├── package.json
└── svelte.config.js
```

---

## 2. Routing

| Path | Page | Description |
|------|------|-------------|
| `/` | Home | Search bar + featured items |
| `/search` | Search | Search results with filters |
| `/item/{id}` | Item Detail | Item info, availability, place hold |
| `/account` | Account | Patron's loans, holds, fines |
| `/login` | Login | Authentication |

---

## 3. API Client (Generated)

```typescript
// Generated via openapi-generator-cli
import { LibreryClient } from '$lib/api/client';

const client = new LibreryClient({
  baseUrl: 'http://localhost:8080/api/v1',
});

// Use browser's native fetch with HTTP/3 upgrade
// Via Alt-Svc header from server
```

### Client Usage

```typescript
// Search
const results = await client.catalog.search({ q: 'python' });

// Get item
const item = await client.catalog.getItem('item-id');

// Place hold
await client.circulation.placeHold({
  itemId: 'item-id',
  patronId: 'patron-id'
});

// Login (gets Biscuit token in HttpOnly cookie)
await client.actor.login({
  email: 'patron@library.org',
  password: '...'
});
```

---

## 4. Key Components

### SearchBar.svelte

```svelte
<script lang="ts">
  import { goto } from '$app/navigation';
  
  let query = '';
  
  async function handleSearch() {
    goto(`/search?q=${encodeURIComponent(query)}`);
  }
</script>

<form on:submit|preventDefault={handleSearch}>
  <input 
    type="search" 
    bind:value={query}
    placeholder="Search the catalog..."
  />
  <button type="submit">Search</button>
</form>
```

### ItemCard.svelte

```svelte
<script lang="ts">
  export let item: Item;
</script>

<article class="item-card">
  <h3>{item.title}</h3>
  <p>Author: {item.author}</p>
  <span class="status" class:available={item.status === 'Available'}>
    {item.status}
  </span>
  <a href="/item/{item.id}">View Details</a>
</article>
```

---

## 5. Authentication (Biscuit)

### Login Flow

```typescript
// POST to /api/v1/actor/login
// Server sets HttpOnly, Secure cookie with Biscuit token
// Subsequent requests include cookie automatically

// Check auth state
const response = await client.actor.getCurrentActor();
if (response.status === 401) {
  goto('/login');
}
```

### Protected Routes

```typescript
// src/routes/account/+page.svelte
<script lang="ts">
  import { onMount } from 'svelte';
  
  onMount(async () => {
    try {
      const actor = await client.actor.getCurrentActor();
    } catch (e) {
      // Redirect to login if not authenticated
      goto('/login?redirect=/account');
    }
  });
</script>
```

---

## 6. HTTP/3 Upgrade

The browser automatically upgrades to HTTP/3 when the server sends:

```
Alt-Svc: h3=":8080"
```

SvelteKit uses the browser's native `fetch`, which handles this transparently.

---

## 7. Testing

| Test Type | Tool | Scope |
|-----------|------|-------|
| Unit | Vitest | Components, stores |
| Component | @testing-library/svelte | UI rendering |
| E2E | Playwright | Full flows |

---

*Document Version: 1.0*
*Generated: March 2026*