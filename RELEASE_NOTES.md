## HarUI v1.0.0 - Rust Native UI Component Library

### Features
- **60+ Components**: Full Element Plus equivalent in Rust ecosystem
- **Theme System**: Light/Dark dual theme, Element Plus design tokens
- **Virtual Scrolling**: Table supports 1000+ rows at 60 FPS
- **IME Support**: Input/Textarea with CJK IME, no flicker
- **Zero Web Deps**: Pure Rust rendering (iced + winit + wgpu)
- **2035 Tests**: Unit + integration + fuzz + snapshot tests
- **14 Demos**: Including showcase comprehensive demo
- **POS Specialized**: Keypad/Payment/HangOrder/CustomerDisplay/StatusBar

### Quick Start
```toml
[dependencies]
har-ui-core = { git = "https://github.com/szzmj1980/HarUI", tag = "v1.0.0" }
har-ui-components = { git = "https://github.com/szzmj1980/HarUI", tag = "v1.0.0" }
iced = "0.13"
```

### Stats
- 2035 tests, 0 failure, 0 warning, 0 error
- 60 components with view() rendering methods
- 14 demo applications + showcase
- Cargo build --workspace: 0 warning 0 error
