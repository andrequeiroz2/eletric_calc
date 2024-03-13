# Eletric Calc

**eletric_calc** is a calculator for Ohm's law.

You can calculate Watts, Amps, Ohms, and Volts.

P = Watts, I = Amps, R = Ohms, V = Volts.

## Fields (values)
- `operation_type`: Accepted values P, I, R, V.
- `unknown_1`: Accepted values P, I, R, V.
- `unknown_2`: Accepted values P, I, R, V.
- `value_unknown_1`: Accepted Float.
- `value_unknown_2`: Accepted Float.

## Example
To calculate Volts, you will provide two other values which can be I, R, or P.

```rust
use std::io;
use eletric_calc::EletricCalc;

let eletric_calc: Result<String, io::Error> = EletricCalc{
         operation_type: String::from("V"),
         unknown_1: String::from("R"),
         unknown_2: String::from("P"),
         value_unknown_1: 2.0,
         value_unknown_2: 2.0
     }.calc();
```

## Result 

2V


## Installation

Add this to your Cargo.toml:

```
[dependencies]
eletric_calc = "0.1.0"
```

## License
This project is licensed under the MIT License - see the LICENSE file for details.
