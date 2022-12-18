# timed-signal

`no_std` Rust helper crate for generating time-dependent signals offering the following features:

- Static on, off and toggle.
- Scheduled state changes.
- Pulse of specific duration with single and retrigger.
- Synchronous blink of multiple signals.

## Usage

- Create an instance of `TimedSignal`.
- Set the signal state either statically or to be changed dynamically dependent on a tick value.
- Call the `update()` function with a tick current value as argument. It will return the state of the signal according to the tick. The tick value can be of any unit, e.g. milliseconds or microseconds. There's no requirement for a continuous value, but it must be increasing monotonically.

## Example

```rust
use timed_signal::TimedSignal;

// Create some instances of to drive LEDs.
let mut led1 = TimedSignal::new();
let mut led2 = TimedSignal::new();
let mut led3 = TimedSignal::new();

// Blink LED1 with a period of 4. LED will then be on for 2 ticks and off for another 2.
led1.blink(4);

// Set LED2 on until a specific tick is reached.
led2.on_until(10);

// Toggle LED3 for a number of ticks.
led3.toggle_for(20);

// Just a dummy loop to iterate over the tick.
for tick in 0..100 {
    let state1 = led1.update(tick);
    let state2 = led2.update(tick);
    let state3 = led3.update(tick);
    println!("{}: {:?} {:?} {:?}", tick, state1, state2, state3);
}
```

## Tests

Run `cargo test` for the unit tests.

## License

Published under the MIT license. Any contribution to this project must be provided under the same license conditions.

Author: Oliver Rockstedt <info@sourcebox.de>
