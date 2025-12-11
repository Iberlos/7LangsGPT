C# Language Overview
1. What C# Is

C# is a modern, high-level, object-oriented language created by Microsoft.
It runs on .NET, supports many programming paradigms, and is widely used in:

Game development (Unity)

Desktop applications

Web development (ASP.NET)

Cloud apps / enterprise systems

Mobile apps (Xamarin / .NET MAUI)

APIs and microservices

C# provides:

strong static typing

automatic memory management (GC)

async-await concurrency

excellent tooling (Visual Studio, Rider, VS Code)

2. Key Features

Garbage-collected → no manual memory management

Rich OOP model → classes, interfaces, inheritance

LINQ → SQL-style queries inside code

Async/Await → best-in-class async support

Generics

Reflection

Attributes (metadata)

3. Tooling
Visual Studio / Rider

Full IDE with debugging, breakpoints, profiling, refactoring tools.

.NET SDK

Command line tools:

dotnet new → create project

dotnet build → compile

dotnet run → run

dotnet test → run tests

NuGet

Package manager for C# libraries.

4. Basic Syntax
Variables
int hp = 50;
var name = "Astarion";

Functions (Methods)
static int Add(int a, int b)
{
    return a + b;
}

Classes
class Player
{
    public string Name { get; set; }
    public int HP { get; set; }

    public void Heal(int amount)
    {
        HP += amount;
    }
}

Constructors
public Player(string name, int hp)
{
    Name = name;
    HP = hp;
}

5. Namespaces

Used for logical grouping:

namespace Game.Combat
{
    class Attack { }
}

6. Interfaces
interface IAttack
{
    int Damage();
}

class Goblin : IAttack
{
    public int Damage() => 5;
}

7. Enums
enum Status
{
    Healthy,
    Wounded,
    Critical,
    Dead
}

8. Properties

C# encapsulates fields with elegant syntax:

public int HP { get; private set; }

9. Collections
List
List<int> values = new() { 1, 2, 3 };

Dictionary
var stats = new Dictionary<string, int>
{
    ["Strength"] = 18,
    ["Dexterity"] = 14
};

Arrays
int[] nums = { 1, 2, 3 };

10. LINQ

Powerful query syntax built into C#:

var lowHP = players
    .Where(p => p.HP < 20)
    .Select(p => p.Name);

11. Async / Await

C# is excellent for asynchronous programming:

async Task<int> LoadScoreAsync()
{
    await Task.Delay(500);
    return 42;
}

12. Error Handling

Uses try/catch:

try
{
    int result = 10 / 0;
}
catch (DivideByZeroException ex)
{
    Console.WriteLine(ex.Message);
}

13. File I/O
string data = File.ReadAllText("save.json");
File.WriteAllText("save.json", data);

14. Best Practices

Use PascalCase for classes and methods

Use camelCase for variables

Prefer properties over public fields

Keep classes focused (Single Responsibility Principle)

Use async/await instead of manual threading

Prefer dependency injection for large projects

15. When to Use C#

You should choose C# when building:

Unity game projects

Desktop applications

ASP.NET web APIs

Large enterprise applications

Tools & utilities

Cross-platform apps (.NET MAUI)

16. Learning Advice

Practice writing small classes and structuring projects with namespaces

Learn LINQ early — it’s one of C#’s best features

Build small utilities using async/await

Use Visual Studio’s debugger — it’s extremely powerful

Try building a Unity side project

Quick Reference Snippets
Random number
var rng = new Random();
int n = rng.Next(0, 10);

String interpolation
Console.WriteLine($"{name} has {hp} HP");

Switch expression
var status = hp switch
{
    > 75 => "Healthy",
    > 50 => "Hurt",
    > 25 => "Wounded",
    > 0  => "Critical",
    _    => "Dead"
};