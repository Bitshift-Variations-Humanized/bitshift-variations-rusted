#include <stdio.h>

const int MAX_TIME = 7864319;

int g(int time,int bitmask,int note_index,int octave_shift_down){
    // Assumes bitmask is 0..3
    // Assumes note_index is 0..8
    char* chord;

    // Extracts bits index 16,17,18 of time
    int time_middle_bits = 3&time>>16;

    // Decides chord based on that
    if (time_middle_bits != 0) {
        chord = "BY}6YB6%";
    } else {
        chord = "Qj}6jQ6%";
    }

    // Picks a chord based on the note_index. Unsafe code.
    int picked_note = chord[note_index];

    // The picked note is turned into a base frequency (which translates into xxx Hz) by adding
    int frequency = picked_note + 51;

    // Picks a sample by multiplying time by frequency and pitch shifting it octave_shit_down number of octaves down
    int sample = (time*frequency)>>octave_shift_down;

    // Picks the first two bits of the sample, masks it with the bitmask (possibly to set the volume), then multiplies the sample height by 16 (2**4 or 1 << 4)
    int amplified_sample = (sample & bitmask & 3) << 4;
    
    return amplified_sample;
};

void main(){
    int time;
    for(time=0;time < MAX_TIME;time++) {
        int n = time>>14;
        int s = time>>17;
        putchar(
            g(time, 1,          n& 7 ,                    12)+
            g(time, s & 3,      (n^time>>13) & 7,         10)+
            g(time, s/3 & 3,    (n+((time>>11)%3)) & 7,   10)+
            g(time, s/5 & 3,    (8+n-((time>>10)%3)) & 7, 9 )
        );
    }
}
