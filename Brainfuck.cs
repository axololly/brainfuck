using System.Text.RegularExpressions;

class Brainfuck
{
    const string RED   = "\u001b[31m";
    const string GREEN = "\u001b[32m";
    const string CYAN  = "\u001b[38;2;145;231;255m";
    const string WHITE = "\u001b[37m";

    static void ThrowException(string errorName, string message)
    {
        Console.WriteLine($"{RED}{errorName}: {message}{WHITE}");
        Environment.Exit(0);
    }
    
    static void ThrowException(string errorName, int errorLocation, string message)
    {
        Console.WriteLine($"{RED}{errorName}: at position {errorLocation + 1} - {message}{WHITE}");
        Environment.Exit(0);
    }

    static string SanitiseCode(string code)
    {
        code = new Regex(@"\/\/.+")     .Replace(code, m => "");
        code = new Regex(@"\n|\r| |\t") .Replace(code, m => "");
        code = new Regex(@"\/\*.+\*\/") .Replace(code, m => "");

        if (new Regex(@"(\/\*)|(\*\/)").IsMatch(code))
        {
            int errorIndex = code.Contains("/*") ? code.IndexOf("/*") : code.IndexOf("*/");

            ThrowException("SyntaxError", errorIndex, "cannot import code with unended comments.");
        }

        int  leftSquareBrackets = code.Count(c => c == '[');
        int rightSquareBrackets = code.Count(c => c == ']');

        if (leftSquareBrackets != rightSquareBrackets)
        {
            int errorIndex = leftSquareBrackets < rightSquareBrackets ? code.LastIndexOf(']') : code.IndexOf('[');

            ThrowException("SyntaxError", errorIndex, "cannot import code with unended comments.");
        }
        
        return code;
    }

    static void Execute(string brainfuckCode, bool showMemoryAfter = false)
    {
        brainfuckCode = SanitiseCode(brainfuckCode);

        Console.WriteLine();

        int codeIndex = 0;

        Stack<int> whileLoopStartIndexes = [];
        
        bool hasConsoleOutput = false;

        int[] memory = new int[30_000];
        int ptr = 0;
        int furthestPtr = 0;

        while (codeIndex < brainfuckCode.Length)
        {
            char current = brainfuckCode[codeIndex];

            switch (current)
            {
                case '>':
                {
                    if (ptr++ == 30_000 - 1) ThrowException("OutOfBounds", codeIndex, "cannot move pointer outside of rightward bounds.");

                    furthestPtr = Math.Max(furthestPtr, ptr);
                    
                    break;
                }

                case '<':
                {
                    if (ptr-- == 0) ThrowException("OutOfBoundsError", codeIndex, "cannot move pointer outside of leftward bounds.");
                    
                    break;
                }

                case '+':
                {
                    if (memory[ptr]++ == 255) ThrowException("OverflowError", codeIndex, "cannot increment memory block past integer limit of 255.");
                    
                    break;
                }

                case '-':
                {
                    if (memory[ptr]-- == 0) ThrowException("SubZeroError", codeIndex, "cannot decrement memory block past 0.");

                    break;
                }

                case '[':
                {
                    whileLoopStartIndexes.Push(codeIndex);
                    break;
                }

                case ']':
                {
                    // If the block of memory if above 0, jump back to the start
                    // of the while loop and keep repeating the code.
                    if (memory[ptr] > 0) codeIndex = whileLoopStartIndexes.Peek();

                    // Otherwise, since we have left the while loop, remove the start
                    // index to not cause an infinite loop past when it was supposed
                    // to stop.
                    else whileLoopStartIndexes.Pop();

                    break;
                }

                case ',':
                {
                    int input = Console.Read();

                    if (input > 255) ThrowException("InputError", codeIndex, "cannot store character with value that exceeds ASCII range of 0 to 255.");

                    memory[ptr] = input;

                    break;
                }

                case '.':
                {
                    // The integer limit is always 255, so we don't need to
                    // factor in any values above 255.
                    Console.Write((char)memory[ptr]);

                    hasConsoleOutput = true;
                    
                    break;
                }

                default:
                {
                    ThrowException("SyntaxError", codeIndex, $"char '{current}' could not be interpreted.{CYAN}\n\nIf this is meant to be a comment, precede the line with two forward slashes (//), or enclose text in /* and */ for a multi-line comment.");

                    break;
                }
            }

            codeIndex++;
        }

        if (!hasConsoleOutput) Console.Write($"{RED}No output provided.{WHITE}");

        Console.WriteLine();

        if (showMemoryAfter)
        {
            string locationsToValues = string.Join(
                "\n",

                Enumerable
                    .Range(0, furthestPtr)
                    .Where(i => memory[i] > 0)
                    .Select(n => $"{new string(' ', 5 - n.ToString().Length)}{CYAN}{n}{WHITE} - {GREEN}[{memory[n]}]{WHITE}")
            );

            Console.WriteLine($"\n Memory Breakdown\n------------------\n" + locationsToValues);
        }
    }

    static void ExecuteFromFile(string filePath, bool debugMode = false)
    {
        if (!filePath.EndsWith(".bf")) ThrowException("FileLoadError", $"cannot run code from a file that does not have the extension {CYAN}.bf");
        
        string codeExtracted = File.ReadAllText(filePath);

        if (codeExtracted.Length == 0) ThrowException("FileLoadError", "file does not contain any code to execute.");

        Execute(codeExtracted, debugMode);
    }

    static void DisplayHelp()
    {
        Console.WriteLine($"""

        Brainfuck Interpreter
        ---------------------
        
        This is an executable that can run brainfuck files, created by axololly on GitHub.
        
        To use this, navigate to the directory with the brainfuck file (marked with the .bf
        extension) and run a command that looks like this in terminal:

            {CYAN}brainfuck your-file.bf{WHITE}
        

        Extra Details
        -------------

        - Brainfuck files can be documented with C#-style comments that get stripped out before execution.
          Any special characters that are usually used in brainfuck execution, when commented, are ignored.
        
        - Whitespace is also stripped before execution, meaning whitespace is {RED}irrelevant{WHITE} to code execution.

        - The memory is {CYAN}30,000 blocks{WHITE}, and each block {CYAN}cannot{WHITE} exceed the inclusive range of {CYAN}0-255{WHITE}.
        
        - All 8 instructions are the same.
        """);
        
        // Give the user an opportunity to close the terminal.
        Console.Read();

        Environment.Exit(0);
    }

    static void Main(string[] args)
    {
        if (args.Length > 2) ThrowException("ArgumentError", $"too many arguments were given.\n\n{CYAN}If this is meant to be a file path, wrap it in \"quotation marks\"");

        // If the user runs the .exe with no arguments, or with the help
        // argument, display a help page the user can use.
        if (args.Length == 0) DisplayHelp();
        if (args[0] == "-h" || args[0] == "--help") DisplayHelp();

        string filePath = args[0];

        if (filePath.StartsWith('"') || filePath.StartsWith('\'')) filePath = filePath[1..^1];

        if (!Path.Exists(filePath)) ThrowException("FileLoadError", "file path does not exist.");

        bool debugMode = false;

        if (args.Length == 2)
        {
            if (args[1] == "-d" || args[1] == "--debug")
            {
                debugMode = true;
            }
            else
            {
                ThrowException("ArgumentError", $"unrecognised second argument.\n\n{CYAN}This argument is usually for debug mode, which displays the contents of memory. Try it out by adding \"-d\" or \"--debug\" to the end of your arguments.");
            }
        }

        ExecuteFromFile(filePath, debugMode);
    }
}