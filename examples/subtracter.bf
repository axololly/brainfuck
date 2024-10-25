// Initiate values - I have chosen 57 and 19 for this.
// The plus signs are in groups of 10.
++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ +++++++ > ++++++++++ +++++++++ <

// Move block 1 to block 5 as an intermediary.
// The pointer ends up at block 1.
[>>>>+<<<<-]

// Shift pointer to block 5.
>>>>

// Move block 5 to blocks 1 and 3.
// The pointer ends up at block 5.
[<<<<+>>+>>-]

// Move pointer back to block 2.
<<<

// Move block 2 to block 5 as an intermediary.
// The pointer ends up at block 2.
[>>>+<<<-]

// Shift pointer to block 5.
>>>

// Move block 5 to blocks 2 and 4.
// The pointer ends up at block 5.
[<<<+>>+>-]

// Shift the pointer to block 4.
// This is the number we subtract.
<

// Decrease block 4 and block 3 until block 4 becomes 0.
[<->-]

// The result is stored in block 3.
// Start: [0][0][0][0][0][0]...
// End:   [57][19][38][0][0]...