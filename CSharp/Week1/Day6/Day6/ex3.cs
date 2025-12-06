using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Day6
{
    internal class ex3
    {
        static void Main()
        {
            var party = new (string name, int hp, int max)[]
            {
                ("Astarion", 32, 40),
                ("Shadowheart", 20, 45),
                ("Karlach", 55, 60),
                ("Wyll", 0, 50),
                ("Lae'zel", 47, 50)
            };

            foreach ((string name, int hp, int max) character in party)
            {
                float healthPercent = (100.0f * character.hp / character.max);
                String msg = String.Format("{0}: {1}/{2} ({3}/%) -> ", character.name, character.hp, character.max, healthPercent);
                switch (healthPercent)
                {
                    case >= 75:
                        {
                            msg += "Healthy";
                            break;
                        }
                    case >= 50:
                        {
                            msg += "Hurt";
                            break;
                        }
                    case >= 25:
                        {
                            msg += "Wounded";
                            break;
                        }
                    case > 0:
                        {
                            msg += "Critical";
                            break;
                        }
                    default:
                        {
                            msg += "Dead";
                            break;
                        }
                }
                Console.WriteLine(msg);
            };

            Console.WriteLine("Wounded characters:");
            var woundedPartyMembers = party.Where(c => c.hp <= c.max * 0.25f);
            foreach ((string name, int hp, int max) character in woundedPartyMembers)
            {
                Console.WriteLine("-{0}",character.name);
            }
        }
    }
}
