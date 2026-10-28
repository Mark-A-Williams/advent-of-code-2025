using System.Diagnostics;

Stopwatch stopwatch = new();
stopwatch.Start();

var lines = File.ReadAllLines("../inputs/10.txt");
var machines = lines.Select(Machine.Parse).ToList();

var result = 0;

foreach (var machine in machines)
{
    Console.WriteLine(machine);
    var fewestRequiredPresses = FindFewestButtonPressesToLightMachineCorrectly(machine);
    Console.WriteLine(fewestRequiredPresses);
    result += fewestRequiredPresses;
}

stopwatch.Stop();

Console.WriteLine($"Result: {result}");
Console.WriteLine($"Elapsed time: {stopwatch.ElapsedMilliseconds}ms");

int FindFewestButtonPressesToLightMachineCorrectly(Machine machine, int startingState = 0)
{
    if (machine.IntendedFinalState.RawValue == 0) return 0; // Trivial case

    // Second trivial case - one button will turn on exactly the desired lights
    if (machine.ButtonWiringSchematics.Any(o => o == machine.IntendedFinalState)) return 1;

    var sublists = SubSetsOf(machine.ButtonWiringSchematics);

    for (int n = 2; n <= machine.NumberOfButtons; n++)
    {
        var buttonCombinations = sublists.Where(o => o.Count() == n);

        if (buttonCombinations.Any(o => ButtonsResultInIntendedFinalState(o, machine.IntendedFinalState)))
        {
            return n;
        }
    }

    throw new InvalidOperationException("Unsolvable machine");
}

bool ButtonsResultInIntendedFinalState(IEnumerable<LightingBitMask> buttons, LightingBitMask intendedFinalState)
{
    var result = 0;
    foreach (var button in buttons)
    {
        result ^= button.RawValue;
    }
    return result == intendedFinalState.RawValue;
}

// https://stackoverflow.com/a/999182
static IEnumerable<IEnumerable<T>> SubSetsOf<T>(IEnumerable<T> source)
{
    if (!source.Any())
        return Enumerable.Repeat(Enumerable.Empty<T>(), 1);

    var element = source.Take(1);

    var haveNots = SubSetsOf(source.Skip(1));
    var haves = haveNots.Select(set => element.Concat(set));

    return haves.Concat(haveNots);
}

/// <summary>
/// Corresponds to one row of the input - a machine with an intended final state and the info of the buttons to get us there.
/// </summary>
/// <param name="Size"></param>
/// <param name="IntendedFinalState"></param>
/// <param name="ButtonWiringSchematics">A list of buttons representing where the Bitmask indicates which lights the button toggles.</param>
public record struct Machine(
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

    public int NumberOfButtons => ButtonWiringSchematics.Count;
}

// This is essentially a wrapper around a binary number where
// e.g. 17 corresponds to #...# (on, off, off, off, on = 2^4 + 2^1 = 17)
// I anticipate this being much more efficient than storing a list of ints, and will
// probably end up adding various helper functions to this.
public record struct LightingBitMask(int RawValue)
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