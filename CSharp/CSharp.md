# C# Overview
Version: C# 12 / .NET 8 (current stable as of 2025)

---

## What C# Is
C# is a modern, statically typed, multi-paradigm programming language primarily used with the .NET ecosystem. It supports object-oriented, functional, asynchronous, and component-based programming styles. C# powers Unity game development, enterprise backend systems, cloud applications, and cross-platform desktop/mobile apps.

---

## Core Strengths
- Large, mature, stable ecosystem (.NET)
- Excellent tooling (Visual Studio, Rider, VS Code)
- Strong static typing with modern features (records, spans, pattern matching)
- Best-in-class async/await model
- Highly productive for enterprise and business applications
- Unity engine uses C#, making it extremely popular in game development
- Cross-platform through .NET 8 (Windows, macOS, Linux)
- Strong support for web APIs (ASP.NET Core)

## Weaknesses
- Heavier runtime compared to Go or Rust
- More complex than simpler scripting languages (Python)
- Requires build and runtime environment setup
- Not ideal for extremely low-level systems programming

---

## Common Use Cases
- Unity game development
- Enterprise web APIs (ASP.NET Core)
- Cloud-native apps (Azure Functions, AWS Lambda via .NET)
- Cross-platform desktop/mobile (MAUI)
- Data processing backends
- Windows development (WPF, WinUI)
- High-productivity business applications

---

## Syntax Highlights

### Variables
    int score = 42;
    string username = "player1";

### Methods
    int Add(int a, int b) {
        return a + b;
    }

### Classes
    class Player {
        public string Name { get; set; }
        public int HP { get; set; }

        public void TakeDamage(int dmg) {
            HP -= dmg;
        }
    }

### Async / Await
    async Task<string> FetchDataAsync() {
        var response = await http.GetStringAsync(url);
        return response;
    }

### Pattern Matching
    object value = 42;
    if (value is int x && x > 10) {
        Console.WriteLine("Large number");
    }

---

## Naming Conventions
| Type       | Style          | Example               |
|------------|----------------|------------------------|
| variables  | camelCase      | playerSpeed            |
| methods    | PascalCase     | CalculateDamage        |
| classes    | PascalCase     | PlayerStats            |
| interfaces | I + PascalCase | IEnemy                 |
| constants  | PascalCase     | MaxPlayers             |
| namespaces | PascalCase     | GameServer.Core        |

---

## Best Practices
- Use async/await for all I/O-bound tasks
- Favor dependency injection (built-in support in .NET 8)
- Use records for immutable data models
- Write unit tests with xUnit or NUnit
- Keep classes small and focused (SOLID principles)
- Use LINQ for clear and expressive data transformations
- Use nullable reference types (`#nullable enable`) to avoid null issues

---

## Tooling Ecosystem
- Build system: dotnet CLI
- Editors: Visual Studio, VS Code, JetBrains Rider
- Testing: xUnit, NUnit, MSTest
- Web frameworks: ASP.NET Core, Minimal APIs
- Game engines: Unity
- Deployment: Docker, Azure, AWS, on-prem

---

## Mini Reference Tables

### Common Types
| Type     | Description             |
|----------|--------------------------|
| int      | 32-bit integer          |
| long     | 64-bit integer          |
| float    | 32-bit floating point   |
| double   | 64-bit floating point   |
| bool     | true/false              |
| string   | UTF-16 text             |
| List<T>  | Generic list            |
| Dictionary<K,V> | Map structure    |
| Task     | Asynchronous operation  |

### Useful Features
| Feature         | Purpose                     |
|------------------|-----------------------------|
| async/await      | Concurrency and I/O tasks   |
| LINQ             | Functional data querying    |
| Records          | Immutable objects           |
| Span<T>          | High-performance memory     |
| Pattern Matching | Elegant control flow        |

---

## Real-World Examples

### Game Development Example: Enemy Behavior (Unity-like)
    class Enemy {
        public string Name { get; set; }
        public int HP { get; set; }

        public void TakeDamage(int dmg) {
            HP -= dmg;
            if (HP <= 0)
                Console.WriteLine($"{Name} defeated!");
            else
                Console.WriteLine($"{Name} HP left: {HP}");
        }
    }

    var goblin = new Enemy { Name = "Goblin", HP = 30 };
    goblin.TakeDamage(12);
    goblin.TakeDamage(25);

---

### Home Automation Example: Polling Smart Sensors
    async Task MonitorTemperatureAsync() {
        while (true) {
            double temp = await ReadTemperatureAsync();
            Console.WriteLine($"Temperature: {temp}");
            await Task.Delay(1000);
        }
    }

    await MonitorTemperatureAsync();

---

## When to Choose C#
Choose C# when:
- You want the best tooling experience
- You're building enterprise-level backend systems
- You're developing games in Unity
- You want modern language features with strong type safety
- You need cross-platform desktop/mobile apps

Avoid C# if:
- You need a tiny runtime or extremely low-level control
- You prefer lightweight scripting languages
- You require near-zero memory footprint

---
