#!/usr/bin/bash
if (./interpret-c.sh < main.c | head -c 7864319 | cmp --silent /dev/stdin ../bitshift-variations-extracted.bin) &> /dev/null; then
	echo "Correct!";
else
	echo "Incorrect!";
	exit 1;
fi
