using System;
using System.IO;
using System.Linq;

namespace AdventOfCode03_1
{
    public class Program
    {
        public static void Main(string[] args)
        {
            int count = 0;
            using(var fileStream = File.OpenRead("Input.txt"))
                using(var streamReader = new StreamReader(fileStream))
                {
                    string line;
                    int[] sides;
                    while((line = streamReader.ReadLine()) != null)
                    {
                        sides = line.Split(new[] {' '}, StringSplitOptions.RemoveEmptyEntries).Select(p => int.Parse(p)).ToArray();
                        if(IsValidTriangle(sides[0], sides[1], sides[2]))
                            count++;
                    }
                }
            Console.WriteLine(count);
        }

        public static bool IsValidTriangle(int x, int y, int z)
        {
            return (x + y > z) && (x + z > y) & (y + z > x);
        }
    }
}
