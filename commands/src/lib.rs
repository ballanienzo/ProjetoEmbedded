#![no_std]

#[derive(Debug, PartialEq)]
pub enum Command {
    PlayNote(Note),
    StopSound,
    Sleep(usize),
}

#[derive(Debug, PartialEq)]
pub enum NoteVariation {
    Sharp,
    Flat,
    None,
}

impl NoteVariation {
    fn get_variation(&self, base_frequency: u16) -> u16 {
        match self {
            NoteVariation::Sharp => base_frequency * 107 / 101,
            NoteVariation::Flat => base_frequency * 101 / 107,
            NoteVariation::None => base_frequency,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Note {
    A(NoteVariation, u32),
    B(NoteVariation, u32),
    C(NoteVariation, u32),
    D(NoteVariation, u32),
    E(NoteVariation, u32),
    F(NoteVariation, u32),
    G(NoteVariation, u32),
}

impl Note {
    fn from(n: &str) -> Option<Self> {
        let mut chars = n.chars();
        let note_name = chars.next()?;
        let mut octave = None;
        let mut note_variation = NoteVariation::None;

        if let Some(c) = chars.next() {
            match c {
                '#' => {
                    note_variation = NoteVariation::Sharp;
                }
                'b' => {
                    note_variation = NoteVariation::Flat;
                }
                other => {
                    octave = Some(other.to_digit(10)?);
                }
            }
        }

        if let Some(c) = chars.next() {
            match octave {
                None => octave = Some(c.to_digit(10)?),
                Some(_) => return None,
            };
        }

        match note_name {
            'A' => Some(Note::A(note_variation, octave.unwrap_or(4))),
            'B' => Some(Note::B(note_variation, octave.unwrap_or(4))),
            'C' => Some(Note::C(note_variation, octave.unwrap_or(4))),
            'D' => Some(Note::D(note_variation, octave.unwrap_or(4))),
            'E' => Some(Note::E(note_variation, octave.unwrap_or(4))),
            'F' => Some(Note::F(note_variation, octave.unwrap_or(4))),
            'G' => Some(Note::G(note_variation, octave.unwrap_or(4))),
            _ => None,
        }
    }

    fn frequency(&self) -> u16 {
        match self {
            Note::C(var, octave) => var.get_variation(261) * 2_u16.pow(octave - 4),
            Note::D(var, octave) => var.get_variation(293) * 2_u16.pow(octave - 4),
            Note::E(var, octave) => var.get_variation(329) * 2_u16.pow(octave - 4),
            Note::F(var, octave) => var.get_variation(349) * 2_u16.pow(octave - 4),
            Note::G(var, octave) => var.get_variation(392) * 2_u16.pow(octave - 4),
            Note::A(var, octave) => var.get_variation(440) * 2_u16.pow(octave - 4),
            Note::B(var, octave) => var.get_variation(493) * 2_u16.pow(octave - 4),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note() {
        assert_eq!(Note::from("A4"), Some(Note::A(NoteVariation::None, 4)));
        assert_eq!(Note::from("C#2"), Some(Note::C(NoteVariation::Sharp, 2)));
        assert_eq!(Note::from("A##"), None);
        assert_eq!(Note::from("#4"), None);
        assert_eq!(Note::from("B4b"), None);
        assert_eq!(Note::from("C22"), None);
    }
}
