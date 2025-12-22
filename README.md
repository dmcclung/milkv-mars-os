# milkv-mars-os
### Setup
<pre>rustup target add riscv64gc-unknown-none-elf</pre>

### Test
Update and Install QEMU
<pre>sudo apt update</pre>
<pre>sudo apt install qemu-system-riscv64</pre>

Verify Installation
<pre>qemu-system-riscv64 --version</pre>

Test kernel 
<pre>qemu-system-riscv64 -machine virt -kernel target/riscv64gc-unknown-none-elf/debug/kernel -nographic -smp 4</pre>
- Emulates a generic RISC-V virt board.
- nographic: Uses serial console output.
- smp 4: Simulates 4 cores like the Milk-V Mars.

### Fedora riscv images
- https://images.fedoravforce.org/Mars
- https://fedoraproject.org/wiki/Architectures/RISC-V/QEMU
- https://dl.fedoraproject.org/pub/alt/risc-v/release/42/Server/riscv64/images/

### Working with qemu
<pre>unxz Fedora-Server-Host-Generic-42.20250911-2251ba41cdd3.riscv64.raw.xz</pre>
<pre>sudo apt install opensbi u-boot-qemu qemu-system-riscv64</pre>

<pre>qemu-system-riscv64 \
  -machine virt \
  -smp 4 \
  -m 4G  \
  -nographic \
  -bios /usr/lib/riscv64-linux-gnu/opensbi/generic/fw_jump.elf \
  -kernel /usr/lib/u-boot/qemu-riscv64_smode/uboot.elf \
  -device virtio-net-device,netdev=net0 \
  -netdev user,id=net0,hostfwd=tcp::2222-:22 \
  -drive file=Fedora-Server-Host-Generic-42.20250911-2251ba41cdd3.riscv64.raw,format=raw,if=virtio
</pre>

<pre>Username: fedora</pre>
<pre>Password: linux</pre>
