# forge

A flash emulator with my own stuff

## Opcodes

| OP   | INST              | Action                                 |
| ---- | ----------------- | -------------------------------------- |
| 0x01 | HLT               | Stops further emulation of the program |
| 0x02 | REG (r), (n)      | Sets a registry to a number            |
| 0x03 | ADD (r), (r), (r) | Adds                                   |
| 0x04 | SUB (r), (r), (r) | Subtracts                              |
| 0x05 | MUL (r), (r), (r) | Multiplies                             |
| 0x06 | DIV (r), (r), (r) | Divides                                |
| 0x07 | LABEL (s)         | Creates a label                        |
| 0x08 | JMP (s)           | Jumps into a label                     |
| 0x09 | RA                | Registry                               |
| 0x0A | RB                | Registry                               |
| 0x0B | RC                | Registry                               |
| 0x0C | RD                | Registry                               |
| 0x0D | \n                | Goes to the next instruction           |
