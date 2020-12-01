using System;
using System.IO;
using System.Linq;

var numbers = File.ReadAllLines("input.txt")
    .Select((text) => int.Parse(text))
    .ToArray();

Console.WriteLine("Part1: " + Part1(numbers));
Console.WriteLine("Part2: " + Part2(numbers));

int Part1(int[] numbers)
{

    for (var i = 0; i < numbers.Length - 1; i++)
        for (var j = i + 1; j < numbers.Length; j++)
        {
            if (numbers[i] + numbers[j] == 2020)
                return numbers[i] * numbers[j];    
        }
    return 0;
}

int Part2(int[] numbers)
{
    for (var i = 0; i < numbers.Length - 2; i++)
        for (var j = i + 1; j < numbers.Length - 1; j++)
            for (var k = j + 1; k < numbers.Length; k++)
            {
                if (numbers[i] + numbers[j] + numbers[k] == 2020)
                    return numbers[i] * numbers[j] * numbers[k];
            }
    return 0;
}