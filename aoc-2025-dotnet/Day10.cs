var lines = File.ReadAllLines("../inputs/10.example.txt");
var machines = lines.Select(Machine.Parse).ToList();
foreach (var machine in machines) Console.WriteLine(machine);

/// <summary>
/// Corresponds to one row of the input - a machine with an intended final state and the info of the buttons to get us there.
/// </summary>
/// <param name="Size"></param>
/// <param name="IntendedFinalState"></param>
/// <param name="ButtonWiringSchematics">A list of buttons representing where the Bitmask indicates which lights the button toggles.</param>
struct Machine(
    LightingBitMask IntendedFinalState,
    List<LightingBitMask> ButtonWiringSchematics)
{
    public static Machine Parse(string inputLine)
    {
        var components = inputLine.Split(' ');
        var intendedFinalStateRaw = components.First().Trim('[').Trim(']');
        var buttonWiringSchematicsRaw = components[1..^1];
        var joltageRequirements = components.Last().Trim('{').Trim('}'); // Redundant for part 1

        var intendedFinalState = LightingBitMask.ParseFromFinalStateString(intendedFinalStateRaw);
        var buttonWiringSchematics = buttonWiringSchematicsRaw.Select(LightingBitMask.ParseFromButtonWiringSchematic).ToList();
        return new(intendedFinalState, buttonWiringSchematics);
    }

    public override string ToString()
    {
        return $"{IntendedFinalState} ... {string.Join(',', ButtonWiringSchematics)}";
    }
}

// This is essentially a wrapper around a binary number where
// e.g. 17 corresponds to #...# (on, off, off, off, on = 2^4 + 2^1 = 17)
// I anticipate this being much more efficient than storing a list of ints, and will
// probably end up adding various helper functions to this.
struct LightingBitMask(int RawValue)
{
    /// <summary>
    /// Parses a final state string like '#.#.' (to the number 5 in this example, 2^0 + 2^2, kinda backwards).
    /// </summary>
    public static LightingBitMask ParseFromFinalStateString(string finalState)
    {
        var result = finalState.Select((character, index) => character == '#' ? Math.Pow(2, index) : 0).Sum();
        return new((int)result);
    }

    /// <summary>
    /// Parses a button wiring schematic string like '(0,1,3)' (to the number 11 in this example, 2^0 + 2^1 + 2^3).
    /// </summary>
    public static LightingBitMask ParseFromButtonWiringSchematic(string buttonWiringSchematic)
    {
        var result = buttonWiringSchematic.Trim('(').Trim(')').Split(',').Select(int.Parse).Sum(o => Math.Pow(2, o));
        return new((int)result);
    }

    public override string ToString() => RawValue.ToString();
}
