# 🏰 WASM Quest: Medieval Platformer

```ascii
                    ⚔️
                   /|
                  / |
    🛡️           /  |
    |||         /   |
    |||        /    |
    |||       /     |
   [:::])====|------|=========>
    |||       \     |
    |||        \    |
    |||         \   |
                  \ |
                   \|
                    ⚔️

    ╔══════════════════════════════════════╗
    ║  WASM QUEST: A Medieval Adventure    ║
    ║  Built with Rust & WebAssembly       ║
    ╚══════════════════════════════════════╝
```

## 🎮 About the Game

A retro 8-bit medieval platformer built entirely in Rust and compiled to WebAssembly. Battle goblins, collect hearts, and journey through infinite procedurally generated levels!

## 🚀 Quick Start

```bash
# Clone the repository
git clone git@github.com:Tortured-Metaphor/WASM-Quest.git
cd WASM-Quest

# Build the WASM module
./build.sh

# Start a local server
python3 -m http.server 8000

# Open in browser
open http://localhost:8000
```

## 🎯 Game Features

- **8-bit Pixel Art**: Hand-crafted pixel sprites with retro aesthetic
- **Sword Combat**: Realistic arc-based sword swinging mechanics
- **Health System**: 7 hearts with quarter-heart precision damage
- **Infinite Levels**: Procedurally generated platforms that never end
- **Collectibles**: Heart pickups to restore health
- **Distance Tracking**: Compete for the longest distance traveled

### Controls
- **A/←** - Move left
- **D/→** - Move right  
- **W/↑** - Jump
- **Space** - Swing sword

## 🛠️ Technical Deep Dive

### Why WebAssembly?

WebAssembly (WASM) allows us to run high-performance Rust code in the browser, giving us:
- **Near-native performance** for game physics and rendering
- **Memory safety** from Rust's ownership system
- **Small binary size** (our game is ~200KB compiled)
- **Direct canvas manipulation** without JavaScript overhead

### Architecture Overview

```
┌─────────────────────────────────────────────────────┐
│                   Web Browser                        │
├─────────────────────────────────────────────────────┤
│            HTML5 Canvas + JavaScript                 │
│                      ↕                               │
│              wasm-bindgen (FFI)                      │
│                      ↕                               │
│         Rust Game Engine (Compiled to WASM)         │
├─────────────────────────────────────────────────────┤
│  • Game Loop (60 FPS)                               │
│  • Physics Engine                                    │
│  • Collision Detection                               │
│  • Sprite Rendering                                  │
│  • Procedural Generation                             │
└─────────────────────────────────────────────────────┘
```

### Core Technologies

#### 1. **Rust → WASM Compilation**
```rust
// Cargo.toml configuration for WASM
[lib]
crate-type = ["cdylib"]  // Dynamic library for WASM

[dependencies]
wasm-bindgen = "0.2"     // Rust/JS interop
web-sys = "0.3"          // Web APIs in Rust
```

#### 2. **Game Loop Architecture**
```rust
// The browser calls our update/render functions via wasm-bindgen
#[wasm_bindgen]
impl GameEngine {
    pub fn update(&self, delta: f64) {
        // Physics, collision, input handling
    }
    
    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        // Draw sprites to canvas
    }
}
```

#### 3. **Pixel Art Rendering System**
Instead of loading image files, we define sprites as code:
```rust
let knight_sprite = vec![
    vec!["", "#C0C0C0", "#C0C0C0", ""],  // Helmet
    vec!["#FDBCB4", "#000", "#000", "#FDBCB4"],  // Face
    vec!["#4169E1", "#4169E1", "#4169E1", "#4169E1"],  // Armor
    // ... more pixels
];
```

This approach:
- Eliminates asset loading
- Keeps everything in a single WASM binary
- Allows dynamic sprite manipulation
- Reduces HTTP requests

#### 4. **Procedural Level Generation**
```rust
fn generate_platforms_ahead(&mut self) {
    // Pseudo-random generation using a seed
    self.seed = (self.seed * 1103515245 + 12345) & 0x7fffffff;
    
    // Generate platforms with varying:
    // - Gap distances (50-200 pixels)
    // - Platform widths (60-150 pixels)  
    // - Heights (±100 pixels variance)
}
```

#### 5. **Collision Detection**
Simple AABB (Axis-Aligned Bounding Box) collision:
```rust
fn check_collision(&self, other: &Entity) -> bool {
    self.x < other.x + other.width &&
    self.x + self.width > other.x &&
    self.y < other.y + other.height &&
    self.y + self.height > other.y
}
```

### Performance Optimizations

1. **Entity Culling**: Remove off-screen entities
2. **Batch Rendering**: Minimize canvas state changes
3. **Fixed Time Step**: Consistent physics at 60 FPS
4. **Memory Pooling**: Reuse entity allocations
5. **WASM Optimization**: `wasm-opt` for smaller binaries

## 📚 Learning Resources

### Getting Started with WASM

1. **Install Rust**: https://rustup.rs/
2. **Install wasm-pack**: 
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

3. **Understanding the Build Pipeline**:
   ```
   Rust Source → rustc → WASM Binary → wasm-bindgen → JS Glue Code
   ```

### Key Concepts to Master

#### FFI (Foreign Function Interface)
```rust
#[wasm_bindgen]
extern "C" {
    // Import JS functions into Rust
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct GameEngine {
    // Exposed to JavaScript
}
```

#### Memory Management
- WASM has linear memory model
- Rust's ownership prevents memory leaks
- Use `Rc<RefCell<>>` for shared mutable state

#### Canvas Rendering
```rust
// Direct canvas manipulation from Rust
ctx.set_fill_style(&JsValue::from_str("#FF0000"));
ctx.fill_rect(x, y, width, height);
```

### Project Structure
```
WASM-Quest/
├── src/
│   └── lib.rs          # Game engine (Rust)
├── pkg/                # Generated WASM + JS bindings
├── index.html          # Game container
├── Cargo.toml          # Rust dependencies
├── build.sh            # Build script
└── manifest.json       # PWA configuration
```

## 🔧 Development Guide

### Adding New Features

1. **New Enemy Type**:
```rust
struct Dragon {
    x: f64,
    y: f64,
    fire_cooldown: f64,
    // ... implement Enemy trait
}
```

2. **Power-ups**:
```rust
enum PowerUp {
    SpeedBoost(duration: f64),
    Invincibility(duration: f64),
    DoubleJump,
}
```

3. **Sound Effects** (Web Audio API):
```rust
// Use web-sys to access Web Audio
let audio_context = web_sys::AudioContext::new()?;
```

### Debugging WASM

1. Use `console_log!` macro for debugging:
```rust
console_log!("Player position: ({}, {})", player.x, player.y);
```

2. Browser DevTools work with WASM:
   - Set breakpoints in Rust source
   - Inspect WASM memory
   - Profile performance

### Building for Production

```bash
# Optimized build
wasm-pack build --release --target web

# Further optimization
wasm-opt -Oz -o optimized.wasm pkg/medieval_platformer_bg.wasm

# Serve with compression
gzip -9 pkg/*.wasm
```

## 🎓 What You'll Learn

Building this project teaches:

1. **Systems Programming**: Low-level memory management in a high-level way
2. **Game Development**: Core concepts like game loops, physics, collision detection
3. **Web Technologies**: Canvas API, requestAnimationFrame, event handling
4. **WebAssembly**: Bridging native code to web browsers
5. **Procedural Generation**: Creating infinite content algorithmically
6. **Performance Optimization**: Making smooth 60 FPS games

## 🤝 Contributing

Contributions welcome! Some ideas:

- [ ] Add sprite animations
- [ ] Implement sound effects
- [ ] Create boss battles
- [ ] Add multiplayer support
- [ ] Design new enemy types
- [ ] Create a level editor
- [ ] Add particle effects
- [ ] Implement a score system

## 📖 Further Reading

- [Rust WASM Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [web-sys Documentation](https://rustwasm.github.io/wasm-bindgen/api/web_sys/)
- [Game Programming Patterns](https://gameprogrammingpatterns.com/)
- [MDN WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly)

## 📝 License

MIT License - See [LICENSE](LICENSE) for details

## 🙏 Acknowledgments

Built with [Claude](https://claude.ai) - An AI coding assistant that helped architect and implement this entire game in a single session!

---

```ascii
    Happy Coding, Brave Knight! ⚔️
    
         🏰
        /║║\
       /║║║║\
      /║║║║║║\
     [========]
      ║  ⚔️  ║
      ║  🛡️  ║
      ╚══════╝
```

**Ready to build your own WASM game? Fork this repo and start your quest!**