SET and CS are normally HIGH
|      | SET               | CS          |
|------|-------------------|-------------|
| HIGH | Transmission Mode | GPIO Mode   |
| LOW  | AT mode           | Serial Mode |

# Serial Mode
CS must be LOW for Serial mode\
SET and CS pin must both be LOW to be able to change configuration

both transcievers must have CLSS = A0 (default value)

# GPIO Mode
GPIO mode will not work if CS pin is LOW\
SET pin is irrelevant for GPIO mode

## Transmitter
should be CLSS C0 or C1
- C0: GPIO8 acts as (LED) output (untested)
- C1: all 8 GPIO pins act as input

an input must be pulled to LOW


## Reciever
should be set to CLSS C2 to C5\
- C2: GPIO is normally LOW, goes HIGH for 30ms after recieving signal. Does not detect button release. (useful for momentary button presses)

- C3: inverted version of C2\

- C4: GPIO is normally LOW, goes HIGH after receiving pressed signal, goes LOW after being let go (useful when you need to be able to hold buttons. Might include button bounce signals since it basically mirrors the input, untested)

- C5: inverted version of C4