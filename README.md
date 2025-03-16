## Brainfuck for QBE

Brainfuck compiler written in Rust that produces QBE IR output.
This naïve implementation takes around 3ms to execute the 
"Hello, World!" program found in the repo. 

Uses libc instead of direct syscalls.
