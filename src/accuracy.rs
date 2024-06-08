#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Accuracy {
    Class4,
    Class5,
    Class6,
    Class7,
    Class8,
    Class9,
    Class10,
    Class11,
    Class12,
    Class13,
    Class14,
    Class15,
    Class16,
}

impl Accuracy {
    pub(crate) fn match_accuracy(value: u8) -> Option<Self> {
        match value {
            4 => Some(Accuracy::Class4),
            5 => Some(Accuracy::Class5),
            6 => Some(Accuracy::Class6),
            7 => Some(Accuracy::Class7),
            8 => Some(Accuracy::Class8),
            9 => Some(Accuracy::Class9),
            10 => Some(Accuracy::Class10),
            11 => Some(Accuracy::Class11),
            12 => Some(Accuracy::Class12),
            13 => Some(Accuracy::Class13),
            14 => Some(Accuracy::Class14),
            15 => Some(Accuracy::Class15),
            16 => Some(Accuracy::Class16),
            _ => None,
        }
    }
}