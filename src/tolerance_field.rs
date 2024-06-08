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