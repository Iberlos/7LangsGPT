using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Day6
{
    internal class ex4
    {
        const int WIDTH = 10;
        const int HEIGHT = 10;
        struct Cell
        {
            public String name;
            public int threat;
            public void redef(int threat)
            {
                this.threat = threat;
                this.name = this.threat switch
                {
                    0 => "Nothing",
                    1 => "a Goblin",
                    2 => "a Wolf",
                    3 => "an Orc",
                    4 => "a Troll",
                    _ => "a Dragon"
                };
            }
        };
        static void Main()
        {
            Cell[,] grid = new Cell[WIDTH, HEIGHT];
            Random rand = new Random();
            for(int x = 0; x < WIDTH; x++)
            {
                for(int y = 0; y < HEIGHT; y++)
                {
                    grid[x, y].redef(rand.Next(6));
                }
            }
            for (int x = 0; x < WIDTH; x++)
            {
                for (int y = 0; y < HEIGHT; y++)
                {
                    Console.WriteLine("Scaning ({0},{1})... You see {2}.", x, y, grid[x, y].name);
                    if (grid[x,y].threat == 5)
                    {
                        Console.WriteLine("A boss was found! Ending search!");
                        goto EndLoops;
                    }
                }
            }
        EndLoops:
            return;
        }
    }
}
