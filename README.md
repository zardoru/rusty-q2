Rusty-Q2
=====
A minimal example of a Game DLL/SO for Quake 2 built on Rust based on the work of Paril's minimal DLL/SO for C#.
To use, use `cargo build` and copy the resulting library to its own mod folder, named `gamex86` 
or `gamex86_64` with the appropriate extension for your OS.

Due to the nature of the Quake 2 API, it is mostly unsafe Rust. 