#![feature(const_fn_floating_point_arithmetic)]

use rusted_variations as variations;

const TWELFTH_ROOT_OF_TWO: f64 = 1.0594630944;
const NOTE_OFFSET: f64 = 132.53272934414807;
const MIDI_C3: i32 = 48;

fn main() {
    let v: variations::Variations<variations::output::NoteSlice> = Default::default();
    let tracks = v.fold([vec![],vec![],vec![],vec![]],|mut acc, variations::output::NoteChordSlice([a,b,c,d])| {
        acc[0].push(variations::output::MaybeNote::from(a));
        acc[1].push(variations::output::MaybeNote::from(b));
        acc[2].push(variations::output::MaybeNote::from(c));
        acc[3].push(variations::output::MaybeNote::from(d));
        acc
    });

    /// Almost fully sanitized data
    /// Note in semitones relative to C3
    /// Volume in 0..4
    /// Duration in milliseconds
    /// Start in milliseconds from start
    /// Channel index 0..4
    #[derive(Debug)]
    pub struct CompleteNote {
        note: i32,
        volume: u8,
        duration: u32,
        start: u32,
        channel: u8
    }

    let compacted_tracks = tracks.map(|track|
        track.into_iter().enumerate().fold(vec![], |mut acc: Vec<(usize,variations::output::MaybeNote)>, (new_start,new_may_note)| {
                match acc.pop() {
                    Some((start,may_note)) => if may_note.same_note(&new_may_note) {
                        acc.push((start,may_note + new_may_note));
                    } else {
                        acc.push((start,may_note));
                        acc.push((new_start,new_may_note));
                    },
                    None => acc.push((new_start,new_may_note))
                };
                acc
            }
        )
    );
    let correct_tracks = compacted_tracks.into_iter().enumerate().map(|(channel,track)|
        track.into_iter().filter_map(|(start,note)| match note {
            variations::output::MaybeNote::Note(variations::output::Note {
                frequency,
                volume,
                duration
            }) => Some(CompleteNote {
                note: ((f64::from(frequency).log(TWELFTH_ROOT_OF_TWO) - NOTE_OFFSET).round() + 0.2) as i32,
                volume: volume,
                duration: duration / 8,
                start: u32::try_from(start / 8).unwrap(),
                channel: channel.try_into().unwrap()
            }),
            _ => None
        }).collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    let sorted_tracks = correct_tracks.into_iter().map(|mut track| {
        track.sort_by(|lhs,rhs| match lhs.start.cmp(&rhs.start) {
            std::cmp::Ordering::Equal => lhs.channel.cmp(&rhs.channel),
            v => v
        });
        track
    }).collect::<Vec<_>>();
    {
        use midly::*;
        let mut encoder = Smf::new(Header::new(Format::Parallel, Timing::Metrical(num::u15::from(1_000))));
        let midi_tracks = sorted_tracks.into_iter().enumerate().map(|(channel, track)| {
            let mut output_track = vec![
                TrackEvent {
                    delta: num::u28::new(0),
                    kind: TrackEventKind::Meta(MetaMessage::MidiChannel(num::u4::from(u8::try_from(channel).unwrap())))
                },
                TrackEvent {
                    delta: num::u28::new(0),
                    kind: TrackEventKind::Meta(MetaMessage::Tempo(num::u24::new(1_000_000)))
                }
            ];
            let midi_track = track.into_iter().fold(Vec::<(u32,TrackEvent)>::new(),|mut acc,note| {
                let key = num::u7::try_from(u8::try_from(MIDI_C3 + note.note).unwrap()).unwrap();
                let vel = num::u7::try_from(note.volume * 31).unwrap();
                match acc.pop() {
                    Some((prev_start,prev_msg)) => {
                        acc.push((prev_start,prev_msg));
                        acc.push((note.start,TrackEvent {
                            delta: num::u28::try_from(note.start - prev_start).unwrap(),
                            kind: TrackEventKind::Midi {
                                channel: num::u4::try_from(note.channel).unwrap(),
                                message: MidiMessage::NoteOn { key, vel }
                            }
                        }));
                    },
                    None => {
                        acc.push((note.start,TrackEvent {
                            delta: num::u28::new(0),
                            kind: TrackEventKind::Midi {
                                channel: num::u4::try_from(note.channel).unwrap(),
                                message: MidiMessage::NoteOn { key, vel }
                            }
                        }));
                    }
                }
                acc.push((note.start + note.duration, TrackEvent {
                    delta: num::u28::new(note.duration),
                    kind: TrackEventKind::Midi {
                        channel: num::u4::try_from(note.channel).unwrap(),
                        message: MidiMessage::NoteOff { key, vel }
                    }
                }));
                acc
            }).into_iter().map(|(_,ev)| ev).collect::<Vec<_>>();
            output_track.extend(midi_track);
            output_track.push(TrackEvent { delta: num::u28::new(0), kind: TrackEventKind::Meta(MetaMessage::EndOfTrack) });
            output_track
        }).collect::<Vec<_>>();
        encoder.tracks = midi_tracks;
        encoder.save("bitshift-variations-midified.mid").unwrap();
    }
}