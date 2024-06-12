use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TolShaft {
    b,
    c,
    d,
    e,
    f,
    g,
    h,
    js,
    k,
    m,
    n,
    p,
    r,
    s,
    t,
    u,
    x,
    z,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TolHole {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    JS,
    K,
    M,
    N,
    P,
    R,
    S,
    T,
    U,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToleranceField {
    Shaft(TolShaft),
    Hole(TolHole),
}

impl FromStr for ToleranceField {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" => Ok(ToleranceField::Shaft(TolShaft::b)),
            "c" => Ok(ToleranceField::Shaft(TolShaft::c)),
            "d" => Ok(ToleranceField::Shaft(TolShaft::d)),
            "e" => Ok(ToleranceField::Shaft(TolShaft::e)),
            "f" => Ok(ToleranceField::Shaft(TolShaft::f)),
            "g" => Ok(ToleranceField::Shaft(TolShaft::g)),
            "h" => Ok(ToleranceField::Shaft(TolShaft::h)),
            "js" => Ok(ToleranceField::Shaft(TolShaft::js)),
            "k" => Ok(ToleranceField::Shaft(TolShaft::k)),
            "m" => Ok(ToleranceField::Shaft(TolShaft::m)),
            "n" => Ok(ToleranceField::Shaft(TolShaft::n)),
            "p" => Ok(ToleranceField::Shaft(TolShaft::p)),
            "r" => Ok(ToleranceField::Shaft(TolShaft::r)),
            "s" => Ok(ToleranceField::Shaft(TolShaft::s)),
            "t" => Ok(ToleranceField::Shaft(TolShaft::t)),
            "u" => Ok(ToleranceField::Shaft(TolShaft::u)),
            "x" => Ok(ToleranceField::Shaft(TolShaft::x)),
            "z" => Ok(ToleranceField::Shaft(TolShaft::z)),

            "A" => Ok(ToleranceField::Hole(TolHole::A)),
            "B" => Ok(ToleranceField::Hole(TolHole::B)),
            "C" => Ok(ToleranceField::Hole(TolHole::C)),
            "D" => Ok(ToleranceField::Hole(TolHole::D)),
            "E" => Ok(ToleranceField::Hole(TolHole::E)),
            "F" => Ok(ToleranceField::Hole(TolHole::F)),
            "G" => Ok(ToleranceField::Hole(TolHole::G)),
            "H" => Ok(ToleranceField::Hole(TolHole::H)),
            "JS" => Ok(ToleranceField::Hole(TolHole::JS)),
            "K" => Ok(ToleranceField::Hole(TolHole::K)),
            "M" => Ok(ToleranceField::Hole(TolHole::M)),
            "N" => Ok(ToleranceField::Hole(TolHole::N)),
            "P" => Ok(ToleranceField::Hole(TolHole::P)),
            "R" => Ok(ToleranceField::Hole(TolHole::R)),
            "S" => Ok(ToleranceField::Hole(TolHole::S)),
            "T" => Ok(ToleranceField::Hole(TolHole::T)),
            "U" => Ok(ToleranceField::Hole(TolHole::U)),

            _ => Err(()),
        }
    }
}