# 🚀 Rust MySQL Full-Stack

**Rust MySQL Full-Stack** is a modern, production-ready full-stack web application template built entirely in **Rust**, featuring:

- A **Yew**-powered SPA frontend compiled to WebAssembly
- A fast, asynchronous **Actix Web** backend
- A robust **MySQL** database layer via **SQLx**

This project serves as a complete example of building scalable, type-safe web applications in Rust without relying on JavaScript or external runtimes.

---

## 🧩 Frontend (Yew + WebAssembly)

The frontend uses the [Yew](https://github.com/yewstack/yew) framework, enabling you to write reactive web interfaces in Rust. Features include:

- **Client-Side Rendering (CSR)** with `"csr"` feature enabled
- Asynchronous API calls using `gloo-net` and `wasm-bindgen-futures`
- JSON data serialization with `serde` and `serde_json`
- Clean state management and component-based architecture
- Integration with browser APIs through `gloo`

**Frontend dependencies:**

```toml
yew = { git = "https://github.com/yewstack/yew/", features = ["csr"] }
gloo = "0.11.0"
gloo-net = "0.6.0"
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4.50"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.12.20", features = ["json"] }
