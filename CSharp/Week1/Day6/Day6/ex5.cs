using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Day6
{
    internal class ex5
    {   
        enum EventType { Damage = 0, Heal, Buff, Debuff, Death, Count}
        record EventLog{
            public required EventType Type;
            public required string Target;
            public required string Origin;
            public int Ammount;
            public required string Name;
        }

        struct Character
        {
            public String name;
            public int hp;
            public int maxHp;
            public Character(String name, int hp, int maxHp)
            {
                this.name = name;
                this.hp = hp;
                this.maxHp = maxHp;
            }
        }

        static void PrintLogs(ref List<EventLog> logs, bool stopAtFirstDeath = false)
        {
            var messages = new String[]
            {
                "{0} used {1} to attack {2} dealing {3} damage!",
                "{0} used {1} to heal {2} for {3} health points!",
                "{0} used {1} on {2}.",
                "{0} used {1} on {2}!",
                "{0} has killed {2}!"
            };
            foreach (EventLog log in logs)
            {
                Console.WriteLine(messages[(int)log.Type], log.Origin, log.Name, log.Target, log.Ammount);
            }
        }

        static void SimulateBattle(ref Random rand, ref List<EventLog> logs, ref List<Character> party, ref List<Character> enemyParty)
        {
            for (int i = 0; i < 10; i++)
            {
                EventType type = (EventType)rand.Next((int)EventType.Count);
                String target = party[rand.Next(party.Count)].name;
                String origin = enemyParty[rand.Next(enemyParty.Count)].name;
                int amount = rand.Next(10);

                String name = "Placeholder Skill Name";

                EventLog log = new EventLog { Type = type, Origin = origin, Target = target, Ammount = amount, Name = name };
                logs.Add(log);
            }
        }

        static void Main()
        {
            Random rand = new Random();

            List<Character> party = new List<Character>()
            {
                new Character("Astarion", 32, 40),
                new Character("Shadowheart", 20, 45),
                new Character("Karlach", 55, 60),
                new Character("Wyll", 1, 50),
                new Character("Lae'zel", 47, 50)
            };
            List<Character> enemyParty = new List<Character>()
            {
                new Character("Goblin", 32, 40),
                new Character("Gnol", 20, 45),
                new Character("Troll", 55, 60),
                new Character("Dire Wolf", 10, 50),
                new Character("Beholder", 47, 50)
            };

            var logs = new List<EventLog>();
            SimulateBattle(ref rand, ref logs, ref party, ref enemyParty);
            PrintLogs(ref logs, true);
        }
    }
}
