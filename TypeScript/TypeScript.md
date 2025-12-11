# TypeScript Overview
Version: TypeScript 5.x (current stable as of 2025)

---

## What TypeScript Is
TypeScript is a statically typed superset of JavaScript that compiles to plain JavaScript. It adds type safety, modern language features, tooling advantages, and compile-time error checking — all while still running on any JavaScript runtime (browsers, Node.js, Deno, Bun, etc.).

---

## Core Strengths
- Adds static typing to JavaScript for safer, more maintainable code
- Excellent editor tooling (auto-complete, refactor tools, error detection)
- Works everywhere JavaScript works
- Great for large codebases and long-term projects
- Integrates seamlessly with frontend and backend environments
- Rich type system (unions, generics, discriminated unions)

## Weaknesses
- Requires a build step (transpilation)
- Type system can be complex for beginners
- Still inherits some quirks from JavaScript
- Runtime errors still possible if types are misused or bypassed

---

## Common Use Cases
- Frontend applications (React, Angular, Svelte, Vue)
- Backend APIs using Node.js, Deno, or Bun
- Infrastructure-as-code (CDK, Pulumi)
- Game and UI tool development in the browser
- Automation scripts (Node-based)
- Cross-platform apps (Electron, Tauri)

---

## Syntax Highlights

### Variables
    let score: number = 42
    const username: string = "player1"
    let isConnected: boolean = true

### Functions
    function greet(name: string): string {
        return `Hello, ${name}!`
    }

### Objects and Interfaces
    interface Player {
        name: string
        hp: number
    }

    const p: Player = { name: "Ava", hp: 100 }

### Enums
    enum Direction {
        Up,
        Down,
        Left,
        Right
    }

### Classes
    class Light {
        isOn: boolean = false

        toggle() {
            this.isOn = !this.isOn
        }
    }

---

## Naming Conventions
| Type      | Style         | Example           |
|-----------|---------------|-------------------|
| variable  | camelCase     | userName          |
| function  | camelCase     | getUserInfo       |
| class     | PascalCase    | PlayerStats       |
| interface | PascalCase I* | IPlayerStats (optional) |
| constant  | UPPER_SNAKE   | MAX_SPEED         |

Interfaces may or may not use the `I` prefix depending on team style.

---

## Best Practices
- Use strict mode (`"strict": true` in tsconfig.json)
- Prefer interfaces over type aliases for object shapes
- Use union types to model real-world state
- Keep functions pure when possible
- Avoid using `any`
- Use `readonly` where applicable
- Organize code into modules
- Enable incremental builds for performance

---

## Tooling Ecosystem
- Build Tools: ts-node, esbuild, Bun, Vite, SWC
- Testing: Vitest, Jest
- Frontend: React, Angular, Vue, Svelte
- Backend: Node.js, Deno, Bun
- Package Manager: npm, pnpm, yarn
- IDEs: VS Code (best-in-class TypeScript support)

---

## Mini Reference Tables

### Basic Types
| Type       | Example              |
|------------|-----------------------|
| number     | 123                  |
| string     | "hello"              |
| boolean    | true                 |
| array      | number[]             |
| tuple      | [string, number]     |
| any        | (avoid!)             |
| unknown    | safer than any        |
| void       | no return value       |
| never      | unreachable code      |

### Useful Keywords
| Keyword    | Meaning                     |
|------------|------------------------------|
| interface  | structural typing            |
| type       | alias or union types         |
| extends    | inheritance                  |
| implements | ensures class conforms       |
| readonly   | prevents reassignment        |
| as         | type assertion               |

---

## Real-World Examples

### Game Development Example: Player State Machine
    type PlayerState =
        | { state: "idle" }
        | { state: "attacking"; damage: number }
        | { state: "dead" }

    function describeState(s: PlayerState) {
        if (s.state === "idle") return "Player is waiting."
        if (s.state === "attacking") return `Player deals ${s.damage} damage!`
        return "Player is dead."
    }

    console.log(describeState({ state: "attacking", damage: 14 }))

---

### Home Automation Example: Smart Light Interface
    interface SmartLight {
        name: string
        isOn: boolean
        toggle(): void
    }

    const kitchenLight: SmartLight = {
        name: "Kitchen",
        isOn: false,
        toggle() {
            this.isOn = !this.isOn
        }
    }

    kitchenLight.toggle()

---

## When to Choose TypeScript
Choose TypeScript when:
- You are building large, long-term JavaScript projects
- You want safer, more maintainable code
- You want excellent tooling and refactoring support
- You work in browser or Node.js environments
- You prefer strong typing or team collaboration

Avoid TypeScript if:
- You need zero build steps and pure scripting
- You are working in extremely constrained embedded environments
- You strongly prefer dynamic languages

---