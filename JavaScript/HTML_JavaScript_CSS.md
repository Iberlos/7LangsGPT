# HTML + CSS + JavaScript Overview
Runtime: Modern Browser Engines (Chromium, WebKit, Gecko) — 2025 standards

---

## What This Stack Is
HTML, CSS, and JavaScript together form the **front-end triad** of the modern web.

- **HTML** defines the structure and semantics of content.
- **CSS** defines visual styling and layout.
- **JavaScript** adds interactivity, logic, data flow, and API communication.

Every modern website, web app, and web-based game uses these technologies.

---

## Core Strengths
- Runs everywhere — no installation required
- Massive ecosystem and community
- Rich UI capabilities (Canvas, WebGL, WASM)
- Easy to prototype, easy to iterate
- Integrates with modern frameworks (React, Vue, Svelte)
- Strong tooling built directly into browsers

## Weaknesses
- Fragmentation across browsers (less severe today)
- No strong typing without TypeScript
- JavaScript can become hard to maintain without discipline
- Security pitfalls (XSS, CSP requirements)
- DOM performance requires careful handling

---

## Common Use Cases
- Websites & dashboards  
- Browser-based games (Canvas, WebGL)  
- Progressive Web Apps  
- IoT dashboards & home automation control panels  
- Interactive data visualizations  
- Hybrid apps via Electron, Capacitor, or Tauri  

---

## Syntax Highlights

### 1. HTML (Structure)
    <div class="card">
        <h1>Smart Home Dashboard</h1>
        <p>Temperature: <span id="temp">--</span> °C</p>
    </div>

### 2. CSS (Styling)
    .card {
        padding: 1rem;
        background: #1e1e1e;
        color: white;
        border-radius: 8px;
        width: 250px;
        text-align: center;
    }

### 3. JavaScript (Logic)
    async function updateTemp() {
        const response = await fetch("/api/temperature");
        const data = await response.json();
        document.getElementById("temp").textContent = data.value;
    }

    setInterval(updateTemp, 2000);

---

## Naming Conventions

### HTML
- Use `kebab-case` for attributes and classes  
  Example: `data-player-id="23"`, `class="enemy-sprite"`

### CSS
- Use BEM (Block__Element--Modifier) for scalable styling  
  Example: `button__icon--large`

### JavaScript
- camelCase variables: `playerSpeed`
- PascalCase classes: `PlayerCharacter`
- SNAKE_CASE constants: `MAX_ENEMIES`

---

## Best Practices

### HTML
- Use semantic tags (`header`, `section`, `footer`)
- Keep structure clean and minimal

### CSS
- Prefer Flexbox and Grid for layout  
- Use CSS variables for themes  
- Avoid deeply nested selectors

### JavaScript
- Avoid global variables  
- Prefer `const` and `let` over `var`  
- Use async/await for clean asynchronous logic  
- Separate DOM manipulation from business logic  
- Use modules (`import ...`) for larger projects

---

## Tooling Ecosystem
- Browser DevTools (Chrome, Firefox, Safari)
- npm + modern build tools (Vite, esbuild, Webpack)
- Frameworks: React, Vue, Svelte, Solid
- CSS frameworks:
  - Tailwind
  - Bootstrap
  - Material UI
- Testing: Playwright, Jest, Vitest
- Graphics: Canvas, WebGL, WebGPU, Three.js
- Deployment: Netlify, Vercel, GitHub Pages

---

## Mini Reference Tables

### Key HTML Tags
| Tag        | Purpose                   |
|------------|----------------------------|
| div        | Generic container          |
| span       | Inline container           |
| button     | Clickable control          |
| canvas     | Drawing surface (2D/3D)    |
| script     | JavaScript                 |
| link       | External resources (CSS)   |
| img        | Images                     |
| input      | Form control               |

### CSS Layout
| Feature | Description |
|---------|-------------|
| Flexbox | One-dimensional layout |
| Grid    | Two-dimensional layout |
| Positioning | absolute, relative, fixed, sticky |
| Variables | theming with custom properties |

### JavaScript Built-In Objects
| Object | Purpose |
|--------|---------|
| Array | Ordered lists |
| Map | Key-value store |
| Set | Unique values |
| Promise | Async workflow |
| Date | Time handling |
| Math | Math utilities |
| DOM APIs | Browser interaction |

---

## Real-World Examples

### Game Development Example: Simple Canvas Sprite Movement
    const canvas = document.getElementById("game");
    const ctx = canvas.getContext("2d");

    let x = 50;
    let y = 50;

    function loop() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = "red";
        ctx.fillRect(x, y, 20, 20);

        x += 1;
        requestAnimationFrame(loop);
    }

    loop();

### Home Automation Example: Light Toggle UI
HTML:
    <button id="light-btn">Toggle Light</button>

JavaScript:
    document.getElementById("light-btn").onclick = async () => {
        await fetch("/api/light/toggle", { method: "POST" });
        alert("Light toggled!");
    };

---

## When to Choose HTML + CSS + JavaScript

Choose this stack when:
- You want maximum reach with zero installation
- You’re building interactive dashboards or UI-heavy apps
- You want to learn game dev in the browser
- You need to ship fast (prototyping)
- You want to understand modern Web APIs and ecosystems

Avoid if:
- You need deep systems programming (Rust, C++)
- You need extremely strict compile-time guarantees (TypeScript helps though)
- You’re building backend-heavy workloads without UI

---
