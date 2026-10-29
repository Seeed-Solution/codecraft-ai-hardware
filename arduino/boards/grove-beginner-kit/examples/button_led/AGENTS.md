<p align="right">
  <strong>English</strong> · <a href="AGENTS.zh_CN.md">简体中文</a>
</p>

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

```bash
arduino-cli compile \
  --fqbn "$(cat arduino/boards/grove-beginner-kit/fqbn.txt)" \
  arduino/boards/grove-beginner-kit/examples/button_led
```

## Expected behavior

- At rest: LED off and `Button=RELEASED LED=OFF` appears at startup.
- While held: LED on and `Button=PRESSED LED=ON` is printed once after the
  20 ms debounce. Release prints the inverse transition once.
- Serial baud rate is 9600.

## Physical test

1. Open the 9600-baud monitor and confirm the initial released line.
2. Press and hold the button; confirm the LED follows immediately and the
   pressed line appears once.
3. Release; confirm the LED turns off and the released line appears once.

## Limitations

- The sketch implements 20 ms debounce only; it does not detect long press,
  double click, or release timing.
- If the button never releases, inspect for a stuck module and remove any
  accidental `INPUT_PULLUP` change before replacing hardware.
