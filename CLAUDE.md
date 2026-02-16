# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TyAdmin is a Rust-based admin backend system built with the Loco framework (v0.16), ported from a Java codebase. It provides a complete e-commerce admin API with JWT authentication, Redis session management, and PostgreSQL database.

**Key Technologies:**
- **Framework**: Loco-rs (Rust web framework similar to Rails)
- **Database**: PostgreSQL with SeaORM
- **Cache**: Redis for session/token storage
- **Auth**: JWT tokens stored in Redis with custom header `Authori-zation` (note the hyphen)
- **API Style**: RESTful with query parameters (not path parameters due to Loco limitations)

## Essential Commands

### Development
```bash
# Start the development server (listens on http://0.0.0.0:5150)
cargo loco start

# Watch mode (auto-restart on file changes)
cargo loco watch

# Build the project
cargo build

# Run in release mode
cargo build --release && cargo loco start --environment production
```

### Database Operations
```bash
# Run migrations
cargo loco db migrate

# Reset database (dangerous!)
cargo loco db reset

# Generate a new migration
cargo loco db generate migration <name>
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test <test_name>

# Run tests with output
cargo test -- --nocapture

# Run tests in a specific file
cargo test --test <test_file_name>
```

### Utilities
```bash
# List all routes
cargo loco routes

# List all middleware
cargo loco middleware

# Validate configuration
cargo loco doctor

# Run a custom task
cargo loco task <task_name>
```

## Architecture

### Layer Structure

The codebase follows a clean architecture with distinct responsibilities:

#### 1. **Controllers** (`src/controllers/`) - HTTP Layer
- Parse HTTP requests and validate input
- Call Service layer methods
- Return formatted responses using `CommonResult<T>`
- **Keep thin** - no business logic here
- All routes use query parameters, NOT path parameters (Loco framework limitation)

#### 2. **Services** (`src/services/`) - Business Logic Layer
- **Complex business logic**: Multi-condition queries, data aggregation, statistics
- **Cross-entity operations**: Coordinate multiple models/tables
- **Business workflows**: Order processing, report generation, etc.
- **Data transformation**: Model → Response DTO conversion
- Returns `Result<T>` types

**When to write Service methods:**
- Complex queries with multiple filters, pagination, sorting
- Operations involving multiple entities (e.g., orders + products + users)
- Business calculations and aggregations
- Workflow orchestration

#### 3. **Models** - Two-Part Structure

**A. Entity Models** (`src/models/_entities/`) - Database Schema
- **Auto-generated** by SeaORM from database schema
- Pure data structures: `Model`, `ActiveModel`, `Entity`
- **DO NOT manually edit** - will be overwritten on regeneration
- Direct 1:1 mapping to database tables

**B. Business Models** (`src/models/*.rs`) - Entity Extensions
- **Manually written** to extend entity functionality
- Re-exports entities: `pub use super::_entities::users::{Model, ActiveModel, Entity}`
- **Common query methods**: `find_by_email()`, `find_by_id()`, `find_by_token()`
- **Entity behaviors**: `verify_password()`, `generate_jwt()`
- **Lifecycle hooks**: `before_save()`, `after_save()`
- **Data validation**: Input validation rules
- **DTOs**: Request/Response parameter structs

**When to write Model methods:**
- Simple single-table queries by specific fields
- Entity-specific business logic (password verification, token generation)
- Reusable queries needed by multiple Services
- Data validation and lifecycle hooks

#### Layer Interaction Pattern

```
Controller → Service → Model → Database
                ↓
            (can also)
                ↓
Controller → Model → Database  (for simple CRUD)
```

**Example: User Login Flow**
```rust
// Controller - thin wrapper
async fn login(Json(params): Json<LoginParams>) -> Result<Response> {
    // ✅ Call Model layer for simple entity query
    let user = users::Model::find_by_email(&db, &params.email).await?;

    // ✅ Call Model layer for entity behavior
    if !user.verify_password(&params.password) {
        return unauthorized("Invalid credentials");
    }

    // ✅ Call Model layer for entity operation
    let token = user.generate_jwt(&secret, expiration)?;

    format::json(LoginResponse { token })
}

// Model layer - entity-focused methods
impl Model {
    pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> ModelResult<Self> {
        users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(db).await?
            .ok_or_else(|| ModelError::EntityNotFound)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        hash::verify_password(password, &self.password)
    }
}
```

**Example: Complex Business Logic**
```rust
// Service layer - business-focused methods
impl StoreProductService {
    pub async fn get_admin_list(
        db: &DatabaseConnection,
        request: &StoreProductSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<StoreProductResponse>> {
        // Complex multi-condition query
        let mut query = store_product::Entity::find();

        // Business logic: filter by product type
        match request.product_type {
            1 => query = query.filter(store_product::Column::IsShow.eq(1)),
            2 => query = query.filter(store_product::Column::IsShow.eq(0)),
            // ... more conditions
        }

        // Business logic: keyword search across multiple fields
        if let Some(keywords) = &request.keywords {
            query = query.filter(
                Condition::any()
                    .add(store_product::Column::StoreName.contains(keywords))
                    .add(store_product::Column::Keyword.contains(keywords))
            );
        }

        // Pagination and data transformation
        let products = query.paginate(db, limit).fetch_page(page).await?;
        let list = products.into_iter().map(Self::model_to_response).collect();

        Ok(CommonPage { list, total, page_number, page_size })
    }
}
```

**When to call Model layer from Service:**
- ✅ Simple entity queries: `users::Model::find_by_email()`
- ✅ Entity behaviors: `user.verify_password()`, `user.generate_jwt()`
- ✅ Entity creation with business rules: `users::Model::create_with_password()`
- ❌ Complex business queries (write directly in Service)
- ❌ Cross-entity operations (coordinate in Service)
- ❌ Data aggregation and statistics (handle in Service)

### Key Architectural Patterns

**Boolean Fields in Database:**
- Database uses `i32`/`u32` for boolean fields (0 = false, 1 = true)
- Response DTOs use actual `bool` types
- Service layer converts: `model.is_show != 0` → `bool`
- When updating: use `Set(1)` or `Set(0)`, not `Set(true)`/`Set(false)`

**Authentication Flow:**
1. User logs in via `/api/admin/login`
2. Server generates JWT token and stores `LoginUserVo` in Redis
3. Token returned in response with lowercase key `"token"` (not `"Token"`)
4. Client sends token in header: `Authori-zation: TOKEN:ADMIN:{token}`
5. Middleware validates token and loads user from Redis

**CORS Configuration:**
- Frontend runs on `http://localhost:9527`
- Custom auth header: `Authori-zation` (with hyphen, not `Authorization`)
- Configured in `config/development.yaml`

### Module Organization

```
src/
├── app.rs                 # Application bootstrap, route registration
├── controllers/           # HTTP handlers (thin layer)
│   ├── admin_login.rs    # Admin authentication endpoints
│   ├── store_product.rs  # Product management (14 endpoints)
│   ├── home.rs           # Dashboard statistics
│   └── ...
├── services/             # Business logic (thick layer)
│   ├── store_product_service.rs  # Complex product queries, aggregations
│   ├── home_service.rs           # Cross-entity statistics
│   └── ...
├── models/
│   ├── users.rs          # User entity extensions (find_by_email, verify_password, etc.)
│   ├── _entities/        # SeaORM generated models (DO NOT EDIT)
│   │   ├── users.rs      # User entity schema
│   │   ├── store_product.rs
│   │   └── ...
│   └── ...
├── utils/                # Shared utilities
│   ├── auth.rs          # JWT validation, permission checks
│   ├── redis_client.rs  # Redis connection pool
│   ├── crypto.rs        # DES encryption (Java compatibility)
│   └── permission.rs    # Permission checking logic
├── initializers/         # App initialization
│   └── redis.rs         # Redis connection setup
└── views/               # Response templates (if using SSR)
```

**Note on Models:**
- `models/_entities/*.rs` = Database schema (auto-generated, read-only)
- `models/*.rs` = Business extensions (manually written, add methods here)
- Example: `models/users.rs` extends `models/_entities/users.rs`

## Critical Implementation Details

### Database Type Conversions

When converting from Model to Response DTO:
```rust
StoreProductResponse {
    // String fields → Option<String>
    image: Some(model.image),

    // Integer booleans → bool
    is_show: model.is_show != 0,

    // u32 → i32 for JSON compatibility
    sales: model.sales as i32,
    stock: model.stock as i32,
}
```

When updating boolean fields:
```rust
// CORRECT
product.is_show = Set(1);
product.is_del = Set(0);

// WRONG - will cause type errors
product.is_show = Set(true);
```

### Date/Time Queries with PostgreSQL

PostgreSQL requires explicit type casting for timestamp comparisons:
```rust
// CORRECT - use Expr::cust() for type casting
use sea_orm::sea_query::Expr;
query.filter(Expr::cust(&format!("create_time >= '{} 00:00:00'::timestamp", date)))

// WRONG - will cause "operator does not exist" error
query.filter(store_order::Column::CreateTime.gte(date))
```

### Route Registration

Due to Loco framework limitations with Axum 0.8:
- **Use query parameters**: `GET /api/product/info?id=123`
- **NOT path parameters**: `GET /api/product/info/:id` (causes route registration to stop)

```rust
// CORRECT
#[debug_handler]
async fn get_info(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,  // Query parameters
) -> Result<Response> {
    let id = params.id;
    // ...
}

// Route definition
.add("/info", get(get_info))  // No :id in path
```

### Redis Token Storage

Tokens are stored with a specific key format:
```rust
// Key format: "admin:token:{token_value}"
let redis_key = format!("admin:token:{}", token);

// Store LoginUserVo as JSON
redis.set_ex(&redis_key, &json_string, expiration_seconds).await?;

// Retrieve and deserialize
let user_json = redis.get(&redis_key).await?;
let user: LoginUserVo = serde_json::from_str(&user_json)?;
```

### Menu Structure Requirements

All menu nodes must have a `childList` field, even if empty:
```rust
MenuResponse {
    id: menu.id,
    name: menu.name,
    // REQUIRED: Always include childList
    child_list: vec![],  // Empty if no children
}
```

## Configuration

**Database**: `config/development.yaml`
```yaml
database:
  uri: postgres://postgres:password@localhost:5432/ty_admin_development
  auto_migrate: true
```

**Redis**: `config/development.yaml`
```yaml
redis:
  uri: redis://127.0.0.1:6379
```

**Server**: Runs on `http://0.0.0.0:5150`

**Frontend**: Expected at `http://localhost:9527`

## Common Patterns

### Adding Model Layer Methods

When you need reusable entity queries or behaviors, add them to the Model layer:

**1. Create Model Extension File** (`src/models/store_product.rs`)
```rust
// Re-export the entity
pub use super::_entities::store_product::{self, ActiveModel, Entity, Model};

use loco_rs::prelude::*;
use sea_orm::*;

impl Model {
    /// Find product by ID
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        store_product::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Check if product exists
    pub async fn exists(db: &DatabaseConnection, id: i32) -> ModelResult<bool> {
        Ok(store_product::Entity::find_by_id(id).one(db).await?.is_some())
    }

    /// Find all active products
    pub async fn find_active(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        store_product::Entity::find()
            .filter(store_product::Column::IsShow.eq(1))
            .filter(store_product::Column::IsDel.eq(0))
            .all(db)
            .await
            .map_err(ModelError::from)
    }
}

impl ActiveModel {
    /// Soft delete a product
    pub async fn soft_delete(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.is_del = Set(1);
        self.update(db).await.map_err(ModelError::from)
    }
}
```

**2. Export in `src/models/mod.rs`**
```rust
pub mod _entities;
pub mod users;
pub mod store_product;  // Add this
```

**3. Use in Service Layer**
```rust
impl StoreProductService {
    pub async fn get_info(db: &DatabaseConnection, id: i32) -> Result<StoreProductInfoResponse> {
        // ✅ Call Model layer method
        let product = store_product::Model::find_by_id(db, id).await?;

        Ok(StoreProductInfoResponse {
            product: Self::model_to_response(product),
            content: None,
            attr: None,
        })
    }

    pub async fn delete_product(db: &DatabaseConnection, id: i32) -> Result<bool> {
        // ✅ Call Model layer method
        let product = store_product::Model::find_by_id(db, id).await?;

        // ✅ Call ActiveModel method
        product.into_active_model().soft_delete(db).await?;

        Ok(true)
    }
}
```

### Adding a New Endpoint

1. **Create/Update Service** (`src/services/`)
```rust
impl MyService {
    pub async fn my_method(db: &DatabaseConnection, params: &MyParams) -> Result<MyResponse> {
        // Business logic here
        let results = my_entity::Entity::find()
            .filter(my_entity::Column::IsActive.eq(1))
            .all(db)
            .await?;

        Ok(MyResponse { data: results })
    }
}
```

2. **Create Controller Handler** (`src/controllers/`)
```rust
#[debug_handler]
async fn my_handler(
    State(ctx): State<AppContext>,
    Query(params): Query<MyParams>,
) -> Result<Response> {
    let response = MyService::my_method(&ctx.db, &params).await?;
    format::json(CommonResult::success(response))
}
```

3. **Register Route** (in controller's `routes()` function)
```rust
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/my-module")
        .add("/my-endpoint", get(my_handler))
}
```

4. **Add to App** (`src/app.rs`)
```rust
fn routes(_ctx: &AppContext) -> AppRoutes {
    AppRoutes::with_default_routes()
        .add_route(controllers::my_module::routes())
}
```

### Response Format

All API responses use `CommonResult<T>`:
```rust
// Success with data
format::json(CommonResult::success(data))

// Success without data
format::json(CommonResult::<String>::success_empty())

// Error (let Loco handle it)
return Err(Error::string("Error message"));
```

## Java Compatibility Notes

This project is ported from Java, maintaining compatibility:

1. **DES Encryption**: Uses DES for password encryption (see `utils/crypto.rs`)
2. **Token Format**: `TOKEN:ADMIN:{uuid}` prefix
3. **Header Name**: `Authori-zation` (not standard `Authorization`)
4. **Boolean Storage**: Database uses integers (0/1) not SQL boolean
5. **Timestamp Handling**: Requires explicit PostgreSQL casting

## Troubleshooting

**"operator does not exist: timestamp >= text"**
- Use `Expr::cust()` with explicit `::timestamp` casting

**Routes not registering after certain point**
- Check for path parameters (`:id`) in route definitions
- Convert to query parameters

**Type mismatch: expected `i32`, found `bool`**
- Database boolean fields are integers
- Use `Set(1)` or `Set(0)` when updating

**Redis connection failed**
- Ensure Redis is running: `redis-server`
- Check connection string in `config/development.yaml`

**CORS errors from frontend**
- Verify frontend origin in `config/development.yaml` CORS settings
- Check `Authori-zation` header is in `allow_headers` list
