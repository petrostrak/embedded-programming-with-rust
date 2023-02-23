### Make sure the F3 is connected. In `/tmp` directory run openocd
```
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

### In `/tmp` run
```
touch itm.txt
itmdump -F -f itm.txt
```


### Inside the project's directory
#### To build 
```
cargo build --target thumbv7em-none-eabihf
```

#### To verify that the produced executable is actually an ARM binary (Inside the project's directory)
```
readelf -h target/thumbv7em-none-eabihf/debug/hello-world
```

#### To connect to debugger

*   Either run: 
```
arm-none-eabi-gdb -q -ex "target extended-remote :3333" target/thumbv7em-none-eabihf/debug/hello-world
```

*   Or: 
```
cargo run -- -q -ex 'target remote :3333' -ex 'load' -ex 'set print asm-demangle on' -ex 'set style sources off' -ex 'b main' -ex 'c' target/thumbv7em-none-eabihf/debug/hello-world
```

Once in the debugger via (gdb)
*   step
*   next

To let it run uninterrupted using the continue command.
*   contunue

To show the arguments of a function, we can use the `info args` command.
*   info args