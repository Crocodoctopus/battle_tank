#[allow(dead_code)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Lmb,
    Rmb,
}

#[allow(dead_code)]
pub enum Event {
    KeyDown(Key),
    KeyUp(Key),
    MouseMove(f32, f32),
    Invalid,
}
