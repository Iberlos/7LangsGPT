using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Day6
{
    internal class ex1
    {
        static void Main()
        {
            int stamina = 90;
            while (stamina >= 20)
            {
                Console.WriteLine("Sprinting");
                //Question: Should I be using write line? Or printf?
                stamina -= 20;
            }
        }
    }
}
