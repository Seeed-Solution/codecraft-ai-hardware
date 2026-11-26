# button_led — Agent Notes

## Purpose

Verify the kit's digital button input and mirror it to the kit LED.

## Hardware facts

- Grove Button is wired to D6. The vendor example uses `pinMode(D6, INPUT)`
  and treats pressed as `HIGH`, so the module provides a pull-down. Source:
  kit wiki default-pin table and Lesson 2.
- Do not use the ATmega328P internal pull-up on this input; it would make an
  idle button read pressed.
- Grove LED is on D4, active high. Source: kit wiki Lesson 2.

## Build

From the repository root, run `codecraft validate --target arduino/grove-beginner-kit --example button_led`. The exact core and libraries are declared in target.json.

## Expected behavior

- At rest: LED off and `Button=RELEASED LED=OFF` appears at startup.
- While held: LED on and `Button=PRESSED LED=ON` is printed once after the
  20 ms debounce. Release prints the inverse transition once.
- Serial baud rate is 9600.

## Physical acceptance

Follow the [Device procedures](../../../AGENTS.md#physical-acceptance). Dated measurements belong to the [Device observations](../../../../../tests/devices/grove-beginner-kit/observations.md); a successful build does not certify physical behavior.

## Limitations

- The sketch implements 20 ms debounce only; it does not detect long press,
  double click, or release timing.
- If the button never releases, inspect for a stuck module and remove any
  accidental `INPUT_PULLUP` change before replacing hardware.
