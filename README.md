# bitshift-variations-rusted
Recreates Rob Miles' "Bitshift Variations in C Minor" in Rust, and provides the library and executable to convert the song to midi, as well as a 'humanized' C version of the original.

See the [project readme](https://github.com/Bitshift-Variations-Humanized/.github) for more details.

`cargo run --bin extract-notes` to generate the midi file

`cargo run` to run an executable that works exactly like the original, except it doesn't repeat infinitely

`cargo test` to run a test that ensures the output of the executable is exactly the same, byte-by-byte, as the original up to the repetition point

## File List
- `c`
  - Humanized code for Bitshift Variations, in C. See the folder for more info.
- `src/lib.rs`
  - Contains the song-generating code, both for generating samples, as well as note data
- `src/main.rs`
  - Contains the main executable, works the same as the original except that it doesn't repeat
- `src/extract_notes.rs`
  - Contains the executable to extract the song into `bitshift-variations-midified.mid` currently kinda working (See #1)
- `bitshift-variations-extracted.bin`
  - Raw audio data extracted using [bitshift-variations-extracted](https://github.com/Bitshift-Variations-Humanized/bitshift-variations-extracted), for testing
- `bitshift-variations-midified.mid`
  - Contains the note data for the song. Currently not working fully. (See #1)
