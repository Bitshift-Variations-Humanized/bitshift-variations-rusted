pub mod output {
    /// Marker trait to generalize over sample types
    pub trait SampleKind: Sized + std::fmt::Debug + Clone + Copy {
        type OutputKind;
        fn combine(input: [Self;4]) -> Self::OutputKind;
    }

    impl SampleKind for u8 {
        type OutputKind = u8;
        fn combine([a,b,c,d]: [u8;4]) -> u8 {
            a+b+c+d
        }
    }

    /// Describes the scaled_frequency and volume of a note. The frequency is scaled by an unknown factor as of now.
    #[derive(Debug,Clone,Copy)]
    pub struct NoteSlice {
        pub frequency: u32,
        pub volume: u8
    }
    impl SampleKind for NoteSlice {
        type OutputKind = NoteChordSlice;
        fn combine([a,b,c,d]: [Self;4]) -> Self::OutputKind {
            NoteChordSlice([a,b,c,d])
        }
    }

    /// Duration in this case counts the number of 1/8000'ths of a second the note is played for
    /// Volume is never zero
    #[derive(Debug,Clone,Copy)]
    pub struct Note {
        pub frequency: u32,
        pub volume: u8,
        pub duration: u32
    }

    /// Describes a duration of time that may or may not contain a note
    #[derive(Debug,Clone,Copy)]
    pub enum MaybeNote {
        Note(Note),
        Silence {
            duration: u32
        }
    }

    impl MaybeNote {
        pub fn same_note(&self,rhs: &Self) -> bool {
            match (self,rhs) {
                (
                    MaybeNote::Note(Note {
                        frequency: frequency_a,
                        volume: volume_a,
                        ..
                    }),
                    MaybeNote::Note(Note {
                        frequency: frequency_b,
                        volume: volume_b,
                        ..
                    })
                ) => (frequency_a == frequency_b)&&(volume_a == volume_b),
                (MaybeNote::Silence { .. }, MaybeNote::Silence { .. }) => true,
                _ => false
            }
        }
    }

    impl std::ops::Add for MaybeNote {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            match (&self,&rhs) {
                (
                    MaybeNote::Note(Note {
                        frequency: frequency_a,
                        volume: volume_a,
                        duration: duration_a
                    }),
                    MaybeNote::Note(Note {
                        frequency: frequency_b,
                        volume: volume_b,
                        duration: duration_b
                    })
                ) => if (frequency_a == frequency_b) && (volume_a == volume_b) {
                    MaybeNote::Note(Note {
                        frequency: *frequency_a,
                        volume: *volume_a,
                        duration: *duration_a + *duration_b
                    })
                } else {
                    panic!("Tried to add two different MaybeNotes")
                },
                (MaybeNote::Silence { duration: duration_a }, MaybeNote::Silence { duration: duration_b }) => MaybeNote::Silence { duration: *duration_a + *duration_b },
                _ => panic!("Tried to add two different MaybeNotes")
            }
        }
    }

    impl From<NoteSlice> for MaybeNote {
        fn from(NoteSlice { frequency, volume }: NoteSlice) -> Self {
            if volume == 0 {
                MaybeNote::Silence { duration: 1 }
            } else {
                MaybeNote::Note(Note { frequency, volume, duration: 1 })
            }
        }
    }

    /// Describes the four frequencies played by all channels in a single 1/8000'ths of a second
    #[derive(Debug,Clone,Copy)]
    pub struct NoteChordSlice(pub [NoteSlice;4]);
}

use output::*;

/// In 1/8000'ths of a second
const VARIATIONS_LENGTH: u32 = 7864319;

#[derive(Debug,Clone,Copy)]
pub struct Variations<T: SampleKind> {
    time: u32,
    output_kind: std::marker::PhantomData<T>
}

impl<T: SampleKind> Default for Variations<T> {
    fn default() -> Self {
        Variations { time: Default::default(), output_kind: std::marker::PhantomData }
    }
}

fn last_byte(input: u32) -> u8 {
    (input & 255).try_into().unwrap()
}

/// The note generation trait
trait G {
    /// This function takes various arguments to generate a sample at a given time
    /// 
    /// # Panics
    /// Will panic if `note_chord` is not in the range `0..8`
    /// Will panic if `volume_bitmask` is not in the range `0..3`
    fn g(time: u32, note_chord: u8, volume_bitmask: u8, octave: u8) -> Self;
}

impl G for u8 {
    fn g(time: u32, volume_bitmask: u8, note_index: u8, octave_shift_down: u8) -> u8 {
    
    if volume_bitmask & 3 != volume_bitmask {
        panic!("Invalid volume bitmask passed: {volume_bitmask}");
    }

    if note_index & 7 != note_index {
        panic!("Invalid note index passed: {note_index}");
    }

    // Extracts bits index 16,17,18 of time
    let time_middle_bits = 3 & (time>>16);

    // Decides chord based on that
    let chord = if time_middle_bits != 0 {
        "BY}6YB6%"
    } else {
        "Qj}6jQ6%"
    };

    // Picks a chord based on the note_index. Unsafe code.
    let picked_note = (chord.as_bytes())[note_index as usize];

    // The picked note is turned into a base frequency (which translates into xxx Hz) by adding
    let frequency = picked_note + 51;

    // Picks a sample by multiplying time by frequency and pitch shifting it octave_shit_down number of octaves down
    let sample = (time * (frequency as u32)) >> octave_shift_down;

    // Picks the first two bits of the sample, masks it with the bitmask (possibly to set the volume), then multiplies the sample height by 16 (2**4 or 1 << 4)
    let amplified_sample = (sample & (volume_bitmask as u32) & 3) << 4;
    
    (amplified_sample & 255).try_into().unwrap()
    }
}

impl G for NoteSlice {
    fn g(time: u32, volume_bitmask: u8, note_index: u8, octave_shift_down: u8) -> NoteSlice {

        if volume_bitmask & 3 != volume_bitmask {
            panic!("Invalid volume bitmask passed: {volume_bitmask}");
        }
    
        if note_index & 7 != note_index {
            panic!("Invalid note index passed: {note_index}");
        }

        // Extracts bits index 16,17,18 of time
        let time_middle_bits = 3 & (time>>16);

        // Decides chord based on that
        let chord = if time_middle_bits != 0 {
            "BY}6YB6%"
        } else {
            "Qj}6jQ6%"
        };

        // Picks a chord based on the note_index. Unsafe code.
        let picked_note = (chord.as_bytes())[note_index as usize];

        // The picked note is turned into a base frequency (which translates into xxx Hz) by adding
        let frequency = picked_note + 51;

        // Finds the overall scaled frequency
        let scaled_frequency = (frequency as u32) << (16 - octave_shift_down);

        NoteSlice {
            frequency: scaled_frequency,
            volume: volume_bitmask
        }
    }
}

impl<T: SampleKind + G> Iterator for Variations<T> where
{
    type Item = T::OutputKind;
    fn next(&mut self) -> Option<Self::Item> {
        if self.time >= VARIATIONS_LENGTH {
            None
        } else {
            // These are mutations that happen on every iteration in the original code
            let n: u32 = self.time >> 14;
            let s: u32 = self.time>>17;
            // These are parameters specified by Rob Miles, define the....
            let out = Some(T::combine([
            // Speed of time/ Volume           / Note from chord                       / Octave shift
              T::g(self.time, 1,                 last_byte(n) & 7 ,                      12),
              T::g(self.time, last_byte(s)  & 3, last_byte(n^self.time>>13) & 7,         10),
              T::g(self.time, last_byte(s/3)& 3, last_byte(n+((self.time>>11)%3)) & 7,   10),
              T::g(self.time, last_byte(s/5)& 3, last_byte(8+n-((self.time>>10)%3)) & 7, 9 )
            ]));

            // Time is incremented on every pass
            self.time += 1;
            out
        }
    }
}

#[cfg(test)]
#[test]
fn matches_original() {
    let original = std::fs::read("./bitshift-variations-extracted.bin").unwrap();
    let produced = Variations::<u8>::default().collect::<Vec<_>>();
    assert_eq!(original,produced);
}