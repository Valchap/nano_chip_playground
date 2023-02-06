# Nano Chip Playground

## Assembler
An assembler that can generate binary output or VHDL code for direct use as a ROM

## Emulator
An emulator for testing the programs before deploying them

## Architecture
This CPU is based on an accumulator architecture, that means that it has only got one register : the accumulator. All instructions(that have a result) will write their result in the accumulator. The only way to write memory is by using the `ST` instruction which will copy the accumulator to the memory at the given address.

The program is stored in a ROM with a width of 14 bits. The first 6 bits are for the opcode, the 8 others are used as an operand(either an address or a constant) or ignored if the instruction doesn't need one.

The CPU uses 4 flags :

Z : the zero flag \
Changes every time the accumulator value changes : set to 1 if accumulator equals 0 and to 0 otherwise

C : the carry flag \
Changes after additions or shifts : takes the value of the bit lost by shift or the value of the 9nth bit of an addition result

V : the overflow flag \
Changes after additions : set to 1 if a signed addition overflows, 0 otherwise

N : negative flag \
Changes every time the accumulator value changes : set to 1 if number is negative and to 0 otherwise (takes the value of the first bit of the number)

## Opcodes / Instructions
0x01 -> ST addr \
Stores accumulator at the given address

0x02 -> LD const \
0x03 -> LD addr \
Load a constant or from memory

0x04 -> AND const \
0x05 -> AND addr \
Logical and with a constant or value at given address

0x06 -> OR const \
0x07 -> OR addr \
Logical or with a constant or value at given address

0x08 -> XOR const \
0x09 -> XOR addr \
Logical xor with a constant or value at given address

0x0A -> SHL acc \
Shift the accumulator one bit to the left

0x0B -> SHR acc \
Shift the accumulator one bit to the right

0x0C -> ADD const \
0x0D -> ADD addr \
Add a constant or value at given address

0x0E -> ADC const \
0x0F -> ADC addr \
Add a constant or value at given address + carry flag

0x10 -> NEG acc \
0x11 -> NEG const \
0x12 -> NEG addr \
Two's complement of accumulator, constant or value at given address

0x13 -> INC acc \
0x14 -> INC addr \
Increment the accumulator or value at given address

0x15 -> DEC acc \
0x16 -> DEC addr \
Decrement the accumulator or value at given address

0x17 -> SETC \
Set C flag to 1

0x18 -> CLRC \
Set C flag to 0

0x19 -> TRFNC \
Set C flag to N flag

0x1A -> BZ0 addr \
0x1B -> BZ1 addr \
0x1C -> BC0 addr \
0x1D -> BC1 addr \
0x1E -> BV0 addr \
0x1F -> BV1 addr \
0x20 -> BN0 addr \
0x21 -> BN1 addr \
Conditional jumps to addresses depending on flags values \
Note that the given address is not a RAM address but the nth instruction in ROM

0x22 -> BRA addr \
Unconditional jump to given address \
Note that the given address is not a RAM address but the nth instruction in ROM

0x3F -> NOP \
Does nothing

