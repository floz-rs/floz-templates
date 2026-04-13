# floz-templates

Starter project templates for the `floz` framework. These templates are embedded directly into the `floz-cli` and extracted when a developer creates a new project.

Each subdirectory represents a complete, runnable `floz` application demonstrating different architectural patterns and capabilities.

## Templates

### [minimal](./minimal)
A bare minimum `floz` application. Just a main function with `App::new().run().await`. Excellent for learning the absolute basics of the framework.

### [api](./api)
A full CRUD API demonstrating modern `floz` features:
- `Note` model generated using the `#[model("notes")]` macro
- Declarative routing with `#[route]` macros
- Integrated Health check endpoint (`/health`)
- Auto-generated Swagger UI available at `/docs`
- Configured for `sqlite` by default (`postgres` available via feature flags).

### [saas](./saas)
A robust multi-tenant SaaS starter template demonstrating advanced framework capabilities:
- `Organization` and `User` models with one-to-many relationship mapping
- JWT-based authentication middleware
- Abstracted login and registration endpoints
- Native tenant-scoped database queries
- Structured application context passing

## Development Notes

When running locally within the workspace, these templates map to the local `floz` crate path. When `floz-cli` scaffolds a new project from these templates, it automatically sanitizes the `Cargo.toml` by stripping local path mappings and retaining only the framework version specifier for published crates.
