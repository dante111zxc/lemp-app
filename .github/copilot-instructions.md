# Copilot Instructions for Tauri + Vue + TypeScript Desktop App

## Project Overview

This is a cross-platform desktop application built with **Tauri** (Rust backend), **Vue 3** (TypeScript frontend), and **Vite** (build tool). The app uses a command-based IPC bridge between frontend and backend—Vue components invoke Rust commands via `@tauri-apps/api/core`.

## Architecture

### Frontend (`src/`) - Vue 3 + TypeScript

- **Entry**: [main.ts](main.ts) - Creates and mounts Vue app to `#app`
- **Root Component**: [App.vue](src/App.vue) - Uses `<script setup>` pattern with composition API
- **Communication**: Frontend invokes Tauri commands via `invoke("commandName", args)` from `@tauri-apps/api/core`
- **Build Output**: Compiled to `dist/` folder (consumed by Tauri at runtime)

### Backend (`src-tauri/src/`) - Rust with Tauri

- **Entry**: [main.rs](src-tauri/src/main.rs) - Wrapper that calls `tauri_app_lib::run()`
- **Logic**: [lib.rs](src-tauri/src/lib.rs) - Core Tauri builder setup; commands registered via `#[tauri::command]` macro
- **Example Command**: `greet(name: &str)` invoked from Vue as `invoke("greet", { name })`
- **IPC Handler**: `tauri::generate_handler![greet]` registers all commands for frontend invocation

### Build & Dev Workflow

- **Dev Server**: `pnpm dev` runs Vite on port 1420 (configured in [vite.config.ts](vite.config.ts))
- **Type Checking**: `vue-tsc --noEmit` before production builds
- **Tauri Commands**:
  - `pnpm tauri dev` - Hot-reload dev mode (Vite frontend + Rust compilation)
  - `pnpm build` - Type check + Vite build → Tauri packaging
- **Configuration**: [tauri.conf.json](src-tauri/tauri.conf.json) defines dev URL, build commands, and window properties

## Key Patterns & Conventions

### Frontend-Backend Communication

```typescript
// Vue component invoking Rust command:
import { invoke } from "@tauri-apps/api/core";
const result = await invoke("greet", { name: "Alice" });
```

- Commands must be registered in [lib.rs](src-tauri/src/lib.rs) with `#[tauri::command]` macro
- Arguments are serialized via Serde; use `serde_json` for complex types
- Return values must be serializable

### Vue Component Structure

- **SFC Format**: Single-file components with `<script setup>` pattern (see [App.vue](src/App.vue))
- **Type Safety**: TypeScript with strict mode; use `ref<T>()` for reactive state
- **Styling**: Scoped CSS preferred; avoid global styles unless necessary

### Tauri Plugin Integration

- **Opener Plugin**: Pre-configured in `dependencies` for opening URLs/files
- **New Plugins**: Add to [Cargo.toml](src-tauri/Cargo.toml) `dependencies`, then initialize in `Builder::default()` chain in [lib.rs](src-tauri/src/lib.rs)

### File Organization

- Frontend code lives in `src/`; Tauri backend in `src-tauri/`
- Assets in `public/` (bundled at build) and `src/assets/` (imported in components)
- Keep `.vite/` ignored in development; build outputs go to `dist/`

## Development Workflow

### Starting Development

```bash
pnpm install          # Install frontend + Tauri CLI
pnpm tauri dev        # Hot-reload dev with Vite + Rust watch
```

### Building for Release

```bash
pnpm build            # Type check + Vite build + Tauri package
```

### Common Tasks

| Task                | Command                                                                        | Notes                    |
| ------------------- | ------------------------------------------------------------------------------ | ------------------------ |
| Add Rust command    | Add `#[tauri::command]` in [lib.rs](src-tauri/src/lib.rs), register in handler | Restart `pnpm tauri dev` |
| Add Vue component   | Create `.vue` file in `src/`, import in parent                                 | HMR supported            |
| Type check only     | `vue-tsc --noEmit`                                                             | Run before commits       |
| Update dependencies | `pnpm add` (frontend) or edit [Cargo.toml](src-tauri/Cargo.toml)               | Rebuild as needed        |

## Important Constraints & Gotchas

1. **Port Binding**: Vite dev server strictly uses port 1420; ensure not in use
2. **CSP Disabled**: [tauri.conf.json](src-tauri/tauri.conf.json) sets `csp: null` for development—review for security in production
3. **HMR Ignored**: [vite.config.ts](vite.config.ts) ignores `src-tauri/` changes; Rust changes require server restart
4. **Command Serialization**: Only Serde-serializable types work as command args/returns
5. **Asset Paths**: `public/` assets reference as `/filename.svg`; imported assets use `import`

## Integration Points

- **@tauri-apps/api**: Frontend library for IPC and system access
- **@tauri-apps/plugin-opener**: Pre-configured for opening URLs/files
- **Serde/serde_json**: Rust serialization for IPC and JSON handling
- **Vue/Vite plugins**: [@vitejs/plugin-vue](vite.config.ts) enabled in config

## Testing & Debugging

- **Frontend**: DevTools available in dev mode; check browser console for errors
- **Rust**: Use `println!` macros (output in terminal) or add proper logging with `log` crate
- **IPC Debugging**: Tauri logs available in terminal when running `pnpm tauri dev`
- **Build Issues**: Check [tsconfig.json](tsconfig.json) and [Cargo.toml](src-tauri/Cargo.toml) for version conflicts

## Next Steps for Agents

- When adding features: first define Rust command in `lib.rs`, then invoke from Vue component
- Use `invoke()` consistently with proper error handling (always `.catch()` or `try/catch`)
- Keep command logic in Rust (security, performance); Vue for UI only
- Reference [Tauri docs](https://tauri.app) for advanced features (file system, windows, menus)
