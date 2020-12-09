# date_iter: a simple date iterator

## Overview

This provides an incredibly simple way to iterate over calendar date ranges. Although it's easy few lines to add in place as needed, it can be preferable to have a reliable standalone module for such functionality when used frequently. The heavy lifting under the hood is handled by the legendary chrono library.

Basically, provided the start/stop bounds of a date range, iterate from start to end, both inclusive.

## Usage

```rust
    use date_iter::{Date, date_iter};
    for date in date_iter(Date::from_ymd(2020, 12, 4), Date::from_ymd(2020, 12, 8)) {
        println!("{}", date);
    }
```
