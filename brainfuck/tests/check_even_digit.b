[
    Take a digit. Output it if it is even, else output 0.
]

Move to cell 3
>>

Take an input
,

Convert from raw byte to text
------------------------------------------------

Copy from cell 3 to 1 and 2
[
    -<+<+>>
]

Cell 1 is for reprinting

Cell 2 is mutated

Set counter in cell 3
+++++

Set check in cell 4
>+<

[
    Back to cell 2
    <

    If not zero, set the check to 0
    [
        >>-<<
    ]

    Go to check
    >>

    If check is not 0
    [
        Move to the first value
        <<<

        Increment to human readable
        ++++++++++++++++++++++++++++++++++++++++++++++++
        .

        Go far away
        >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>+>
    ]
    +
    If check is 0
    [
        Subtract 2 from user input
        <<--

        Subtract 1 from counter
        >-
        
        Move to empty cell
        >>
    ]

    Back to counter
    <<
]