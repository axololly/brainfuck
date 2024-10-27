from re import sub as replace, match as find
from typing import overload

RED   = "\u001b[31m"
GREEN = "\u001b[32m"
CYAN  = "\u001b[38;2;145;231;255m"
WHITE = "\u001b[37m"

class Interpreter:
    @overload
    def __init__(
        self,
        memory_capacity: int = 30_000
    ) -> None: ...

    @overload
    def __init__(
        self,
        memory_capacity: int = 30_000,
        integer_limit: int = 255
    ) -> None: ...

    
    def __init__(
            self,
            memory_capacity: int = 30_000,
            integer_limit: int = 255
    ) -> None:        
        self.memory = [0 for _ in range(memory_capacity)]
        self.capacity = memory_capacity

        self.pointer = 0
        self.integer_limit = integer_limit


    @overload
    def print_memory(self, up_to: int = 10) -> None: ...

    @overload
    def print_memory(self, up_to: int = 10, offset: int = 0) -> None: ...


    def print_memory(self, up_to: int = 10, offset: int = 0) -> None:
        """
        Print a snippet of the memory in blocks.

        For example, running this brainfuck code:
        ```
        +++++ > +++
        ```

        Would produce this output when memory is printed:ㅤㅤ
        ```
        ㅤ  v (2)
        [5][3][0][0][0][0][0][0][0][0]
        ```

        Parameters
        ----------
        up_to: `int`
            the number of cells that it should print up to.
            Default is 10.
        
        offset: `int`
            how far from the beginning of memory should the snippet display.
            Default is 0.
        """
        
        mem_str = ""
        mem_str_no_colour = ""

        for i in range(up_to):
            i += offset

            # If the cell is not empty, mark it as red.
            # If the cell is empty, mark it as green.
            colour = RED if self.memory[i] else GREEN

            mem_str += f"{colour}[{self.memory[i]}]{WHITE}"

            mem_str_no_colour += f"[{self.memory[i]}]"

        if self.pointer < up_to:
            past = mem_str_no_colour.find('[')

            for _ in range(self.pointer):
                past = mem_str_no_colour.find('[', past + 1)

            past += 1

            pointer_str = f"{' ' * past}{CYAN}v{WHITE} ({self.pointer + 1})"

        else:
            pointer_str = f"{' ' * (up_to + 1)}{CYAN}v{WHITE} ({self.pointer + 1})"

        # Add a white colour code to make sure
        # the rest of the terminal stays white.
        print(pointer_str + '\n' + mem_str + WHITE)
    

    @staticmethod
    def sanitise_code(code: str) -> str:
        # Remove single-line comments
        code = replace(r"\/\/.+", "", code)

        # Remove whitespace characters ("\n", "\r", " " and "\t")
        code = replace("\n|\r| |\t", "", code)

        # Remove multi-line comments (anything between /* and */)
        code = replace(r"\/\*.+\*\/", "", code)

        # Check for either a stray "/*" or a stray "*/"
        # If one is found, this is invalid syntax, and
        # we raise an error.
        if match := find(r"(\/\*)|(\*\/)", code):
            print(f"\n{RED}SyntaxError: at position {match.start()} - cannot import code with unended comments.{WHITE}")
            exit()

        # Check for unterminated while loops
        if code.count('[') != code.count(']'):
            print(f"\n{RED}SyntaxError: cannot import code with unterminated while loops.{WHITE}")
            exit()

        return code


    @overload
    def execute(self, brainfuck_code: str) -> None: ...

    @overload
    def execute(self, brainfuck_code: str, guard_rails: bool = False) -> None: ...


    def execute(self, brainfuck_code: str, guard_rails: bool = False) -> None:
        brainfuck_code = self.sanitise_code(brainfuck_code)
        
        print()
        
        code_index = 0
        while_loop_start_indexes = []

        has_output = False

        while code_index < len(brainfuck_code):
            char = brainfuck_code[code_index]
            sep = '\n\n' if has_output else ''

            match char:
                case '>':
                    self.pointer += 1

                    if self.pointer >= len(self.memory):
                        if guard_rails:
                            self.pointer = len(self.memory) - 1
                            
                        else:
                            print(f"{sep}{RED}OutOfBoundsError: at position {code_index + 1} - cannot move pointer out of rightward bounds.{WHITE}\n")
                            exit()
                
                case '<':
                    self.pointer -= 1

                    if self.pointer < 0:
                        if guard_rails:
                            self.pointer = 0
                            
                        else:
                            print(f"{sep}{RED}OutOfBoundsError: at position {code_index + 1} - cannot move pointer out of leftward bounds.{WHITE}\n")
                            exit()
                
                case '+':
                    self.memory[self.pointer] += 1
                    
                    if self.memory[self.pointer] >= self.integer_limit:
                        if guard_rails:
                            self.memory[self.pointer] %= self.integer_limit
                        else:
                            print(f"{sep}{RED}OverflowError: at position {code_index + 1} - cannot increment memory block past integer limit of {self.integer_limit}.{WHITE}")
                            exit()
                
                case '-':
                    self.memory[self.pointer] -= 1

                    if self.memory[self.pointer] < 0:
                        if guard_rails:
                            self.memory[self.pointer] = 0
                        else:
                            print(f"{sep}{RED}SubZeroError: at position {code_index + 1} - cannot decrement memory block past 0.{WHITE}")
                            exit()
                
                case '[':
                    while_loop_start_indexes.append(code_index)
                
                case ']':
                    # If the block of memory is above 0,
                    # jump back to the start of the while
                    # loop and keep repeating the code.
                    if self.memory[self.pointer]:
                        code_index = while_loop_start_indexes[-1]

                    # Otherwise, nullify the variable representing
                    # where the while loop begins, because we are
                    # not in one anymore.
                    else:
                        while_loop_start_indexes.pop(-1)

                case ',':
                    char = input()

                    if len(char) != 1:
                        print(f"{sep}{RED}InputError: at position {code_index + 1}, received input \"{char}\" - char was not 1 in length.{WHITE}\n")
                        exit()

                    val = ord(char)

                    if val > 255:
                        print(f"{sep}{RED}InputError: at position {code_index + 1} - cannot input char '{char}' with value that exceeds ASCII range of 255.{WHITE}\n")
                        exit()

                    self.memory[self.pointer] = val

                case '.':
                    if self.memory[self.pointer] > 255:
                        print(f"{sep}{RED}InputError: at position {code_index + 1} - cannot output char with value that exceeds printable ASCII range of 32 to 255.{WHITE}\n")
                        exit()

                    print(chr(self.memory[self.pointer]), end = '')

                    has_output = True

                case _:
                    print(f"{sep}{RED}SyntaxError: at position {code_index + 1} - char '{char}' could not be interpreted.{CYAN}\n\nIf this is meant to be a comment, precede the line with two forward slashes (//), or enclose text in /* and */ for a multi-line comment.\n{WHITE}\n")
                    exit()

            # print(code_index, char, while_loop_start_indexes, self.memory[self.pointer])

            code_index += 1
        

        # After we reach the end, check if we are still in a while loop,
        # and if we are, raise an error notifying the user that they have
        # a trailing while loop.
        if while_loop_start_indexes:
            print(f"{sep}{RED}SyntaxError: at position {code_index + 1} - expected while loop terminator ']' but terminator was not found.{WHITE}\n\nSnippet is shown below:{CYAN}\n\n{brainfuck_code[while_loop_start_indexes[-1] : ]}{WHITE}\n")
            exit()
        
        if not has_output:
            print(f"{RED}No output provided.{WHITE}")

        
        print()

    
    @overload
    def execute_from_path(self, file_path: str) -> None: ...

    @overload
    def execute_from_path(self, file_path: str, guard_rails: bool = False) -> None: ...


    def execute_from_path(self, file_path: str, guard_rails: bool = False) -> None:
        if not file_path.endswith(".bf"):
            print(f"\n{RED}FilePathError: cannot run code from a file that does not have the extension {CYAN}.bf{WHITE}")
            exit()
        
        with open(file_path) as f:
            code = f.read()

            if not self.sanitise_code(code):
                print(f"\n{RED}FilePathError: brainfuck file does not contain any code to execute.{WHITE}")
                exit()

            self.execute(code, guard_rails)
