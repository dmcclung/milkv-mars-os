# milkv-mars-os

<pre>https://milkv.io/docs/mars/overview</pre>

Rust has a pre-defined target for riscv 64 bit that matches what we need for the Milk-V Mars.
<pre>rustup target add riscv64gc-unknown-none-elf</pre>
The target specifies the architecture, the operating system, and the binary format.
The project can be built with the following command:
<pre>cargo build --target riscv64gc-unknown-none-elf</pre>

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
<pre>sudo dnf update</pre>
<pre>sudo dnf install qemu-system-riscv qemu-system-riscv-core</pre>
<pre>qemu-system-riscv64 --version</pre>

The kernel for now only works in Machine mode and simply prints out a Hello World message. You can run the kernel with or without OpenSBI.
A TODO for me is to figure out how to build an OpenSBI + Uboot package with the kernel. The command below skips OpenSBI `-bios none`

<pre>
qemu-system-riscv64 \
  -machine virt \
  -smp 4 \
  -m 4G \
  -nographic \
  -bios none \
  -kernel target/riscv64gc-unknown-none-elf/release/milkv_mars_os
</pre>

Expected output:
<pre>
Hello from my rust kernel on JH7110
</pre>

#### Notes
On Fedora 43 in wsl, I see this issue where WSL interop stops working after a reboot. You can fix it by running the following commands:
<pre>
sudo sh -c 'echo :WSLInterop:M::MZ::/init:PF > /usr/lib/binfmt.d/WSLInterop.conf'
sudo systemctl restart systemd-binfmt
</pre>
