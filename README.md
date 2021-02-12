# Discord Nitro code generator
Discord Nitro code generator written in Rust.  
Build the executable with
```
cargo build --release
```
and get it from `./target/release/discord-nitro-gen`.  
`--release` is important here, since without it the generator will perform like python.  
Run `./discord-nitro-gen -h` to see execution option.
### Performance
Generating & saving 10.000.000 codes takes about 1 second on a `Ryzen 5 3600` and a SSD
