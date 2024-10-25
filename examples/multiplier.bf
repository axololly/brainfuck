// This preserves the numbers in blocks 1 and 2,
// then packs the result to be in block 3.

// Initiate values - I have chosen 4 and 17 for this.
// The pointer starts at block 1.
++++>+++++ +++++ +++++ ++<

// Move block 1 to block 7, then move block 7 back to block 6 and block 1.
[->>>>>>+<<<<<<]
>>>>>>[-<+<<<<<+>>>>>>]
<<<<<<

[
    // Copy block 2 to block 3.
    // The pointer lands on block 4.
    >[>>+<+<-]>>[<<+>>-]
    
    // Reset the pointer to block 1.
    <<<
    
    // Decrement the counter
    -
    
    // Add the contents of block 3 to block 5.
    // The pointer lands at block 3.
    >>[->>+<<]
    
    // Move the pointer back to block 1.
    <<
]

// Move block 6 back to block 1.
// The pointer ends at block 6.
>>>>>[<<<<<+>>>>>-]

// Move block 5 (result) back to block 3.
<[<<+>>-]

// Reset the pointer to block 1.
<<<<