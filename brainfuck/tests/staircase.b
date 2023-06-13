Set the second bit to 1 (the first is always 0)
>+

[
    Move to the third bit
    >

    Increment all bits until a 0 is found
    [+>]

    Increment the zero
    +

    Go back to the first bit
    [<]

    Go to second bit
    >

    Increment the second bit
    This isn't done earlier so that we don't stop early on the way back
    +
]

Move to top of staircase
>

[
    Print cell
    .

    Move to the right
    >

    If zero: stop
]