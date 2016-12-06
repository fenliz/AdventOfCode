using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode04_1
{
    public class Program
    {
        public static void Main(string[] args)
        {
        }

        public IOrderedEnumerable<KeyValuePair<char, int>> GetCharCount(string value)
        {
            return value.GroupBy(c => c)
                        .ToDictionary(grp => grp.Key, grp => grp.Count())
                        .OrderBy(c => c.Value);
        }

        public static bool IsValidChecksum(string checksum)
        {
            char previousChar = checksum[0];
            for(int i = 1; i < checksum.Length; i++)
            {
                if(checksum[i] < previousChar)
                    return false;
                previousChar = checksum[i];
            }
            return true;
        }
    }
}
