#!/usr/bin/bash
declare TEMPLOCATION="$(mktemp)";
echo "Storing Executable at $TEMPLOCATION" &> /dev/stderr;
gcc -xc -pipe -o "$TEMPLOCATION" - &>/dev/stderr && chmod +x "$TEMPLOCATION" &>/dev/stderr && ( "$TEMPLOCATION" $@ ); rm "$TEMPLOCATION" &> /dev/stderr;
