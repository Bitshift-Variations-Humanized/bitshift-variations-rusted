# Humanized Bitshift Variations in C
## File List
 - `main.c`
   - Contains the humanized code
 - `interpret-c.sh`
   - Used to compile C code from stdin, store it in a temporary file, and clean up afterwards
 - `check-if-correct.sh`
   - Used during the development process to ensure the output of the newly compiled code is the same as that contained in `../bitshift-variations-extracted.bin`
 - `build.sh`
   - Used to simply build the code into `bit-var.out`
 - `bit-var.out`
   - Executable code resulting from compilation of `main.c`
