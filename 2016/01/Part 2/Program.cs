using System;
using System.Collections.Generic;

namespace AdventOfCode01_2
{
    public class Program
    {
        public static void Main(string[] args)
        {
            string input = "R5, R4, R2, L3, R1, R1, L4, L5, R3, L1, L1, R4, L2, R1, R4, R4, L2, L2, R4, L4, R1, R3, L3, L1, L2, R1, R5, L5, L1, L1, R3, R5, L1, R4, L5, R5, R1, L185, R4, L1, R51, R3, L2, R78, R1, L4, R188, R1, L5, R5, R2, R3, L5, R3, R4, L1, R2, R2, L4, L4, L5, R5, R4, L4, R2, L5, R2, L1, L4, R4, L4, R2, L3, L4, R2, L3, R3, R2, L2, L3, R4, R3, R1, L4, L2, L5, R4, R4, L1, R1, L5, L1, R3, R1, L2, R1, R1, R3, L4, L1, L3, R2, R4, R2, L2, R1, L5, R3, L3, R3, L1, R4, L3, L3, R4, L2, L1, L3, R2, R3, L2, L1, R4, L3, L5, L2, L4, R1, L4, L4, R3, R5, L4, L1, L1, R4, L2, R5, R1, R1, R2, R1, R5, L1, L3, L5, R2";

            string[] instructions = input.Replace(" ", "").Split(',');
            List<Vector2> previousLocations = new List<Vector2>();
            Vector2 location = new Vector2();
            Vector2 direction = new Vector2() { X = 0, Y = 1 };

            foreach (string instruction in instructions)
            {
                double rotation = instruction[0] == 'L' ? -(Math.PI/2) : (Math.PI/2);
                int steps = int.Parse(instruction.Substring(1));

                direction = Vector2.Rotate(direction, rotation);
                for(int i = 0; i < steps; i++)
                {
                    location += direction;
                    if(previousLocations.Contains(location))
                    {
                        Console.WriteLine(Vector2.GetRectilinearDistance(location));
                        Console.ReadLine();
                        return;
                    }
                    previousLocations.Add(location);
                }
            }
        }
    }

    public struct Vector2
    {
        public int X;
        public int Y;

        public static Vector2 Rotate(Vector2 v, double radians)
        {
            var ca = Math.Cos(radians);
            var sa = Math.Sin(radians);

            return new Vector2() { X = (int)(ca * v.X - sa * v.Y), Y = (int)(sa * v.X + ca * v.Y) };
        }

        public static int GetRectilinearDistance(Vector2 v)
        {
            return Math.Abs(v.X) + Math.Abs(v.Y);
        }

        public static Vector2 operator+(Vector2 v1, Vector2 v2)
        {
            return new Vector2() { X = v1.X + v2.X, Y = v1.Y + v2.Y };
        }
    }
}
