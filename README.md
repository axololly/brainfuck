# Brainfuck Interpreter

A Brainfuck interpretor with the first draft being written at home, and a few examples developed while at school.

## Usage

Navigate to a directory that contains the brainfuck file (marked with the `.bf` file extension) that you want to execute, using `cd`, then run the executable with the name of the file you want to run. Alternatively, you can supply an entire path to the executable and the process will still run the same.

Some programs may not display output, in which a message in red text will display notifying the user. Optionally, if your program doesn't display any output, but still affects memory, you can use the debug flag which will show the contents of any cells with non-zero values, as well as their appropriate indexes. This is useful, as for all my examples, I do not output the result of the calculations, and instead I keep it in memory.

> :memo: **Note:** for more help, run the executable with no arguments, or with the help argument that can be either `-h` or `--help`.

## Examples

All four of the examples are fully documented, as to how they work. Here's a quick description of what they all do:

- `adder.bf` - adds the values in the first two blocks and output the result in the third block.

- `copy.bf` - copies the value from the first block to the second block using the third block. This is the basis of the other operations in the brainfuck files of the `examples` folder.

- `multiplier.bf` - multiplies the values in the first two blocks, then packs the result into the third block. This does require a few other blocks - I believe 6 or 7 more - so a lot more space than addition or subtraction.

- `subtracter.bf` - subtracts the values in the first two blocks (`b1` - `b2`) and outputs the result in the third block. This will cause a `SubZeroError` if the value in the second block is more than the block in the first.

ㅤ

> :eyes: ㅤMore examples will be made in the coming days. Brainfuck is pretty addicting once you get into it, but also an absolute pain because it fucks with your brain.
> 
> _Wow, who could've guessed?_
> 
> In terms of the order, it's probably division, then greater than / less than / equal detection, and whatever else comes to mind.