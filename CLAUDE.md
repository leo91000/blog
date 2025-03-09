This project is using latest Leptos version.

# Tailwind Icons

This project uses `@egoist/tailwindcss-icons` with all Iconify icons collection. To use icons in your components:

```rust
// Use icons with the i-[collection]-[icon-name] format
<div class="i-mdi-home text-2xl" />  // Material Design Icons
<span class="i-lucide-github" />     // Lucide Icons 
<button class="i-heroicons-user" />  // Heroicons

// You can apply colors, sizing and other Tailwind utilities
<div class="i-mdi-heart text-red-500 dark:text-red-400 text-xl hover:text-red-600" />
```

For a complete list of available icons, visit:
- Browse icons: https://icon-sets.iconify.design/
- Use the collection prefix followed by icon name in kebab-case

# Essential Leptos Guide

## 1. Component Attribute Binding

There are multiple ways to bind attributes to components in Leptos:

### Static and Dynamic Attributes

```rust
<button 
    // Static attribute
    id="submit-button"
    // Reactive class based on condition
    class:red=move || count.get() % 2 == 1
    // Dynamic style
    style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
>
    "Click me"
</button>
```

### Component Attribute Binding

When passing attributes to components (rather than HTML elements), you need to use the `attr:` prefix to distinguish them from component props:

```rust
<MyComponent
    // This is a component prop
    title="My Title"
    // This is an HTML attribute that will be applied to the component's root element
    attr:class="primary-card"
    attr:id="card-1"
    attr:data-testid="card-component"
/>
```

Attributes passed to a component with the `attr:` prefix will be applied to all top-level HTML elements returned from the component's view. If you want attributes to apply to a specific element within your component, you should pass them as props and apply them in your component implementation.

```rust
#[component]
fn Card(
    title: String,
    #[prop(attrs)] // Optional way to collect unrecognized attributes
    attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <div class="card">
            <h2>{title}</h2>
            <div class="card-content" {...attrs}>
                <slot/>
            </div>
        </div>
    }
}
```

### Spreading Attributes

You can "spread" attributes onto components to apply them to all top-level HTML elements returned from the component view:

```rust
// Method 1: Create an attribute list to spread
let spread_attrs = view! {
    <{..} aria-label="component with spreading" title="tooltip text"/>
};

// Method 2: Using spread syntax in a component
<MyComponent
    // Component props
    some_prop="foo"
    // Attributes after {..} are treated as HTML attributes
    {..}
    id="foo"
    // Spread all attributes from the list
    {..spread_attrs}
/>
```

## 2. Server Functions

Server functions allow you to run code on the server and call it easily from the client:

```rust
#[server]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    // Server-only code that runs on the server
    let mut conn = db().await?;
    sqlx::query("INSERT INTO todos (title, completed) VALUES ($1, false)")
        .bind(title)
        .execute(&mut conn)
        .await?;
    Ok(())
}

#[component]
fn TodoForm() -> impl IntoView {
    // Create a server action to call the server function
    let add_todo = ServerAction::<AddTodo>::new();
    
    view! {
        <ActionForm action=add_todo>
            <input type="text" name="title"/>
            <button type="submit">"Add"</button>
        </ActionForm>
    }
}
```

Server functions can use extractors to access server-specific data:

```rust
#[server]
pub async fn get_user_data() -> Result<String, ServerFnError> {
    use axum::extract::Query;
    use leptos_axum::extract;

    let (query, connection_info) = extract().await?;
    // Use server-specific data here
    Ok(format!("Query: {:?}, Connection: {:?}", query, connection_info))
}
```

## 3. Routing

Leptos provides a full-featured client-side router:

```rust
#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                // Regular links work for navigation
                <a href="/">"Home"</a>
                // <A> component has enhancements for nested routes
                <A href="/contacts">"Contacts"</A>
            </nav>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path="/" view=Home/>
                    
                    // Nested routes with parameters
                    <ParentRoute path="/contacts" view=ContactList>
                        <Route path=":id" view=ContactInfo/>
                        <Route path="" view=|| view! { "Select a contact" }/>
                    </ParentRoute>
                    
                    // Wildcard route
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
```

### Route Parameters and Queries

```rust
#[component]
fn ContactInfo() -> impl IntoView {
    // Access route parameters
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();
    
    // Access query parameters
    let query = use_query_map();
    let search = move || query.read().get("q").unwrap_or_default();
    
    // ... rest of component
}
```

### Form-Based Navigation

```rust
#[component]
fn Search() -> impl IntoView {
    let query = use_query_map();
    let search = move || query.read().get("q").unwrap_or_default();
    
    view! {
        <Form method="GET" action="">
            <input 
                type="search" 
                name="q" 
                value=search
                // Auto-submit on input
                oninput="this.form.requestSubmit()"
            />
        </Form>
    }
}
```

## 4. Context

Context allows you to share data throughout a component tree without passing it as props:

```rust
// Provide context at a parent level
#[component]
fn App() -> impl IntoView {
    let (theme, set_theme) = signal("light");
    
    // Make theme available to all child components
    provide_context(theme);
    
    view! {
        <ThemeToggle/>
        <Content/>
    }
}

// Use context in any child component
#[component]
fn ThemeToggle() -> impl IntoView {
    // Access the context provided above
    let theme = use_context::<ReadSignal<&str>>().expect("theme to be provided");
    let set_theme = use_context::<WriteSignal<&str>>().expect("theme setter to be provided");
    
    view! {
        <button on:click=move |_| {
            set_theme.update(|t| *t = if *t == "light" { "dark" } else { "light" });
        }>
            {move || format!("Switch to {} mode", if theme.get() == "light" { "dark" } else { "light" })}
        </button>
    }
}
```

## 5. Stores

Stores provide fine-grained reactivity for structured data:

```rust
use reactive_stores::Store;

// Define your data structure with the Store derive macro
#[derive(Store, Debug, Clone)]
pub struct AppState {
    // For collections, define a key function
    #[store(key: String = |todo| todo.id.clone())]
    todos: Vec<Todo>,
    user: User,
}

#[derive(Store, Debug, Clone)]
struct Todo {
    id: String,
    title: String,
    completed: bool,
}

#[derive(Store, Debug, Clone)]
struct User {
    name: String,
    preferences: Preferences,
}

#[derive(Store, Debug, Clone)]
struct Preferences {
    theme: String,
}

#[component]
fn App() -> impl IntoView {
    // Create a store
    let state = Store::new(AppState {
        todos: vec![/* initial todos */],
        user: User {
            name: "Alice".to_string(),
            preferences: Preferences {
                theme: "light".to_string(),
            }
        }
    });
    
    // Provide the store via context
    provide_context(state);
    
    view! {
        <TodoList/>
        <UserProfile/>
    }
}

#[component]
fn TodoList() -> impl IntoView {
    let state = expect_context::<Store<AppState>>();
    
    // Access a field of the store
    let todos = state.todos();
    
    view! {
        <For
            each=move || todos
            key=|todo| todo.read().id.clone()
            children=|todo| {
                // Reactive access to individual todo fields
                let title = todo.title();
                let completed = todo.completed();
                
                view! {
                    <div>
                        <input 
                            type="checkbox" 
                            prop:checked=move || completed.get()
                            on:change=move |_| *completed.write() = !completed.get()
                        />
                        <span>{move || title.get()}</span>
                    </div>
                }
            }
        />
    }
}

#[component]
fn UserProfile() -> impl IntoView {
    let state = expect_context::<Store<AppState>>();
    
    // Access nested fields directly
    let theme = state.user().preferences().theme();
    
    view! {
        <div>
            <p>"Current theme: " {move || theme.get()}</p>
            <button on:click=move |_| {
                *theme.write() = if theme.get() == "light" { "dark".to_string() } else { "light".to_string() };
            }>
                "Toggle Theme"
            </button>
        </div>
    }
}
```

Stores provide several advantages:
- You can reactively access and update individual fields without causing unnecessary re-renders
- You can work with nested data structures effectively
- Collections like vectors get special methods for efficient manipulation 
- Changes to one field don't notify subscribers to sibling fields

This synthesis covers the most crucial aspects of Leptos for building applications, focusing on the specific areas requested.

# LibSQL Rust Documentation: A Comprehensive Implementation Guide

LibSQL is a modern fork of SQLite designed for extensibility and remote database access, offering a Rust API that maintains compatibility with SQLite while extending its functionality. This documentation provides detailed guidance on using the libSQL Rust crate, with special emphasis on commonly confused aspects of the API.

## Core Concepts and Installation

LibSQL can be added to your Rust project using Cargo:

```rust
cargo add libsql
```

The crate supports various feature flags for conditional compilation depending on your requirements:

```rust
[dependencies]
libsql = { version = "...", features = ["encryption", "remote", "replication"] }
```

The main features include:
- `remote`: Enables HTTP-only client for remote sqld server communication
- `core`: Provides local database functionality
- `replication`: Adds replication capabilities
- `encryption`: Enables encryption at rest support[1]

## Connection Establishment

### Local Database Connection

```rust
use libsql::Builder;

// In-memory database
let db = Builder::new_local(":memory:").build().await?;
let conn = db.connect()?;

// File-based database
let db = Builder::new_local("local.db").build().await?;
let conn = db.connect()?;
```

### Remote Database Connection

```rust
use libsql::Builder;

let url = std::env::var("TURSO_DATABASE_URL").expect("URL must be set");
let token = std::env::var("TURSO_AUTH_TOKEN").expect("Token must be set");

let db = Builder::new_remote(url, token)
    .build()
    .await?;
let conn = db.connect()?;
```

### Embedded Replica (Local + Remote Sync)

```rust
use libsql::Builder;
use std::time::Duration;

let url = std::env::var("TURSO_DATABASE_URL").expect("URL must be set");
let token = std::env::var("TURSO_AUTH_TOKEN").expect("Token must be set");

let db = Builder::new_remote_replica("local.db", url, token)
    .sync_interval(Duration::from_secs(60)) // Optional auto-sync
    .build()
    .await?;
let conn = db.connect()?;
```

## Critical Method Distinction: execute vs query

Understanding the distinction between the `execute` and `query` methods is essential for correct usage of the LibSQL API.

### execute Method

The `execute` method is designed specifically for statements that modify the database (INSERT, UPDATE, DELETE, CREATE TABLE, etc.). Its key characteristics are:

1. Returns a `u64` representing the **number of rows affected** by the operation
2. Should be used for all data modification operations
3. Does not return data rows or query results

```rust
// Example: Insert data
let rows_affected = conn.execute(
    "INSERT INTO users (name, email) VALUES (?1, ?2)",
    params!["John Doe", "john@example.com"]
).await?;

// Example: Create table
conn.execute(
    "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)",
    ()
).await?;

// Example: Update data
let updated_rows = conn.execute(
    "UPDATE users SET name = ?1 WHERE id = ?2",
    params!["Jane Doe", 1]
).await?;

// Example: Delete data
let deleted_rows = conn.execute(
    "DELETE FROM users WHERE id = ?1",
    params![42]
).await?;
```

### query Method

The `query` method is specifically for retrieving data (SELECT statements). Its key characteristics are:

1. Returns `Rows` that can be asynchronously iterated over 
2. Should be used when you need to process returned data
3. Not intended for data modification operations

```rust
// Query data and process results
let rows = conn.query(
    "SELECT id, name, email FROM users WHERE id > ?1",
    params![10]
).await?;

// Process returned rows asynchronously
while let Some(row) = rows.next().await? {
    let id: i64 = row.get(0)?;
    let name: String = row.get(1)?;
    let email: String = row.get(2)?;
    println!("User: {} (ID: {}, Email: {})", name, id, email);
}
```

## Parameter Binding

To prevent SQL injection, LibSQL provides parameter binding instead of string concatenation. Both positional and named parameters are supported:

### Positional Parameters

```rust
// Using indexed parameters with ?1, ?2, etc.
conn.execute(
    "INSERT INTO users (id, name) VALUES (?1, ?2)",
    params![42, "Alice"]
).await?;
```

### Named Parameters

```rust
// Using named parameters with :name, $name, or @name
conn.execute(
    "INSERT INTO users (id, name) VALUES (:id, :name)",
    named_params! {
        ":id": 42,
        ":name": "Alice"
    }
).await?;
```

## Transaction Management

LibSQL supports transactions for grouping operations that should succeed or fail as a single unit:

```rust
// Begin a transaction
let tx = conn.transaction().await?;

// Perform operations within the transaction
tx.execute(
    "INSERT INTO users (name) VALUES (?1)",
    params!["Alice"]
).await?;

tx.execute(
    "UPDATE stats SET user_count = user_count + 1",
    ()
).await?;

// Commit the transaction to apply changes
tx.commit().await?;

// Alternatively, call tx.rollback().await? to cancel changes
```

The Transaction struct provides the same `execute` and `query` methods as Connection, with the same behavior regarding return types[6].

## Batch Operations

For executing multiple SQL statements in a single call:

```rust
// Standard batch execution (non-transactional)
let batch_results = conn.execute_batch(
    "CREATE TABLE temp(id INTEGER); 
     INSERT INTO temp VALUES (1), (2), (3);"
).await?;

// Transactional batch execution (all succeed or all fail)
let batch_results = conn.execute_transactional_batch(
    "INSERT INTO users VALUES (1, 'Alice');
     INSERT INTO profiles VALUES (1, 'Admin');"
).await?;
```

## Synchronization (Embedded Replicas Only)

When using an embedded replica (local database with remote synchronization):

```rust
// Manual sync
db.sync().await?;

// Automatic periodic sync can be configured during setup
let db = Builder::new_remote_replica("local.db", url, token)
    .sync_interval(Duration::from_secs(60))
    .build()
    .await?;
```
