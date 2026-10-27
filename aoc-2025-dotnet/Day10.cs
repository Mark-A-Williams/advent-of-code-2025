var lines = File.ReadAllLines("../inputs/10.txt");
Console.WriteLine(lines.Length);

/// <summary>
/// Corresponds to one row of the input - a machine with an intended final state and the info of the buttons to get us there.
/// </summary>
/// <param name="Size"></param>
/// <param name="IntendedFinalState"></param>
/// <param name="ButtonWiringSchematics">A list of buttons representing where the Bitmask indicates which lights the button toggles.</param>
struct Machine(
    int Size,
    LightingBitMask IntendedFinalState,
    List<LightingBitMask> ButtonWiringSchematics);

// This is essentially a wrapper around a binary number where
// e.g. 17 corresponds to #...# (on, off, off, off, on = 2^4 + 2^1 = 17)
// I anticipate this being much more efficient than storing a list of ints, and will
// probably end up adding various helper functions to this.
struct LightingBitMask(int RawValue);
