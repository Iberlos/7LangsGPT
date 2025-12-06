using System;
using System.Collections.Generic;
using System.Linq;
using System.Reflection.Emit;
using System.Text;
using System.Threading.Tasks;

namespace Day6
{
    internal class ex2
    {
        static void Main()
        {

            float carryWeight = 57.5f;
            float maxWeight = 100f;
            float percent = (100.0f * carryWeight / maxWeight);
            String label = "";
            switch (percent)
            {
                case > 100:
                    {
                        label = "Overloaded";
                        break;
                    }
                case > 75:
                    {
                        label = "Overburdened";
                        break;
                    }
                case > 50:
                    {
                        label = "Encumbered";
                        break;
                    }
                case > 25:
                    {
                        label = "Light Load";
                        break;
                    }
                default:
                    {
                        label = "Unburdened";
                        break;
                    }
            }
            String message = String.Format("Carry Wirght: {0}/{1} ({2}/%) -> {3}", carryWeight, maxWeight, percent, label);
            Console.WriteLine(message);
        }
    }
}
