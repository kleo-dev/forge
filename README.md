# forge

A flash emulator with my own stuff

## Opcodes

| OP  | INST              | Action                                 |
| --- | ----------------- | -------------------------------------- |
| 0   | \n                | Goes to the next instruction           |
| 1   | HLT               | Stops further emulation of the program |
| 2   | REG (r), (n)      | Sets a registry to a number            |
| 3   | ADD (r), (r), (r) | Adds                                   |
| 4   | SUB (r), (r), (r) | Subtracts                              |
| 5   | MUL (r), (r), (r) | Multiplies                             |
| 6   | DIV (r), (r), (r) | Divides                                |
| 7   | LABEL (s)         | Creates a label                        |
| 8   | JMP (s)           | Jumps into a label                     |
