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
                    int[][] sides = new int[3][];
                    bool validInput = true;
                    do
                    {
                        sides[0] = GetNextRow(streamReader);
                        sides[1] = GetNextRow(streamReader);
                        sides[2] = GetNextRow(streamReader);
                        validInput = sides[0] != null && sides[1] != null && sides[2] != null;
                        if(validInput)
                        {
                            for(int i = 0; i < 3; i++)
                            {
                                if(IsValidTriangle(sides[0][i], sides[1][i], sides[2][i]))
                                    count++;
                            }
                        }
                    } while(validInput);
                }
            Console.WriteLine(count);
        }

        public static bool IsValidTriangle(int x, int y, int z)
        {
            return (x + y > z) && (x + z > y) & (y + z > x);
        }

        public static int[] GetNextRow(StreamReader streamReader)
        {
            string line = streamReader.ReadLine();
            if(line != null)
                return line.Split(new[] {' '}, StringSplitOptions.RemoveEmptyEntries).Select(p => int.Parse(p)).ToArray();
            return null;
        }
    }
}
