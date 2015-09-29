rust-gpio
=========

**UNDER DEVELOPMENT: For now use [rust-sysfs-gpio](https://github.com/posborne/rust-sysfs-gpio)**

The `gpio` crate provides generic traits for accessing GPIOs on any
system from rust.  Along with this, the project includes canonical
implementations for Linux via sysfs.

This core crate, `gpio`, does not depend on `std` and is intended to
be portable to any system that can be targetted by rust and for which
`core` can be targetted.

License
-------

```
Copyright (c) 2015, Paul Osborne <ospbau@gmail.com>

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/license/LICENSE-2.0> or the MIT license
<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
option.  This file may not be copied, modified, or distributed
except according to those terms.
```
