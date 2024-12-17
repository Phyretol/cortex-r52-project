### Build
```bash
cargo +nightly build -Zbuild-std=core
```

### QEMU 
Requires QEMU >= 9.0.0
```bash
qemu-system-arm -cpu cortex-r52 -machine mps3-an536 -semihosting -nographic -kernel target/armv8r-none-eabihf/debug/kernel
```

### QEMU (debug mode)
```bash
qemu-system-arm -cpu cortex-r52 -machine mps3-an536 -semihosting -nographic -gdb tcp::3333 -S -kernel target/armv8r-none-eabihf/debug/kernel
```

### GDB Debug (Windows)
```bash
arm-none-eabi-gdb -q -x openocd.gdb target/armv8r-none-eabihf/debug/kernel
```

