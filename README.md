# milkv-mars-os

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
