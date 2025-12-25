# milkv-mars-os

[Overview of the Milk-V Mars](https://milkv.io/docs/mars/overview)

My goal with this hobby os is to learn Rust and the RISC-V architecture. I purchased the Mars 
awhile back when it came out so I could figure out how it works and be prepared to use it in 
the future. An open standard risc architecture is really appealing. Not having to pay license fees to ARM is a big deal.

It has been 20 years since I took operating systems theory in college, so this is a fun intellectual exercise to re-learn these concepts.

Rust has a pre-defined target for riscv 64 bit that matches what we need for the Milk-V Mars.
````
rustup target add riscv64gc-unknown-none-elf
````
The target specifies the architecture, the operating system, and the binary format.
The project can be built with the following command:
````
cargo build --target riscv64gc-unknown-none-elf
````

The project defines a [.cargo/config.toml](.cargo/config.toml) file that helps define 
the target and linker script. The linker script is necessary to set the start address 
of the kernel and in general specify the memory layout. Without the linker script, the
milkv mars or qemu bootloader won't be able to find the kernel start in the compiled binary. 
You'll see qemu or OpenSBI hang without the linker.

Don't run the debug build unless you have a debugger attached. The debug build is bigger 
and very slow and causes problems with the Milk-V Mars bootloader and qemu. 
Use the release flag, `--release`

I haven't tried to attach a a debugger yet, so that is TBD.

Qemu is great for fast iteration and e2e testing. I haven't run it on the Milk-V Mars yet, 
but I think I know how to do it. That is the next TODO.

I'm using Fedora 43 in WSL so all the commands make that assumption, unless otherwise stated.

Install qemu
````
sudo dnf update
````
````
sudo dnf install qemu-system-riscv
````
````
qemu-system-riscv64 --version
````

The kernel for now only works in Machine mode and simply prints out a Hello World message. You can run the kernel with or without OpenSBI.
A TODO for me is to figure out how to build an OpenSBI + Uboot package with the kernel. The command below skips OpenSBI `-bios none`

````
qemu-system-riscv64 \
  -machine virt \
  -smp 4 \
  -m 4G \
  -nographic \
  -bios none \
  -kernel target/riscv64gc-unknown-none-elf/release/milkv_mars_os
````

Expected output:
<pre>
Hello from my rust kernel on JH7110
</pre>

Exit qemu with `ctrl + a` then `x`

#### Running on hardware
We need build and development tools to `make` opensbi.
```
sudo dnf group install development-tools gawk
```

We also need to setup the cross compile toolchain.
```
sudo dnf install gcc-riscv64-linux-gnu binutils-riscv64-linux-gnu
```

Clone [opensbi](https://github.com/riscv-software-src/opensbi)
```
git clone git@github.com:riscv-software-src/opensbi.git
```

```
cd opensbi
export CROSS_COMPILE=riscv64-linux-gnu-
make PLATFORM=generic FW_PAYLOAD_PATH=../milkv-mars-os/target/riscv64gc-unknown-none-elf/release/milkv_mars_os
```
If everything builds correctly, you should see a file named `build/platform/generic/firmware/fw_payload.elf`

Copy this `fw_payload.elf` to the SD card and from uboot, try
```
fatload mmc 1:1 $kernel_addr_r fw_payload.elf
go $kernel_addr_r
```

#### Notes
On Fedora 43 in wsl, I see this issue where WSL interop stops working after a reboot. You can fix it by running the following commands:
````
sudo sh -c 'echo :WSLInterop:M::MZ::/init:PF > /usr/lib/binfmt.d/WSLInterop.conf'
````
````
sudo systemctl restart systemd-binfmt
````
