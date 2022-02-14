use std::fmt;

pub enum Square {
    A1, A2, A3, A4, A5, A6, A7, A8,
    B1, B2, B3, B4, B5, B6, B7, B8,
    C1, C2, C3, C4, C5, C6, C7, C8,
    D1, D2, D3, D4, D5, D6, D7, D8,
    E1, E2, E3, E4, E5, E6, E7, E8,
    F1, F2, F3, F4, F5, F6, F7, F8,
    G1, G2, G3, G4, G5, G6, G7, G8,
    H1, H2, H3, H4, H5, H6, H7, H8,
}

use Square::*;

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            A1 => write!(f, "a1"), A2 => write!(f, "a2"), A3 => write!(f, "a3"), A4 => write!(f, "a4"), 
            A5 => write!(f, "a5"), A6 => write!(f, "a6"), A7 => write!(f, "a7"), A8 => write!(f, "a8"),
            B1 => write!(f, "b1"), B2 => write!(f, "b1"), B3 => write!(f, "b1"), B4 => write!(f, "b1"), 
            B5 => write!(f, "b1"), B6 => write!(f, "b1"), B7 => write!(f, "b1"), B8 => write!(f, "b1"),
            C1 => write!(f, "c1"), C2 => write!(f, "c1"), C3 => write!(f, "c1"), C4 => write!(f, "c1"), 
            C5 => write!(f, "c1"), C6 => write!(f, "c1"), C7 => write!(f, "c1"), C8 => write!(f, "c1"),
            D1 => write!(f, "d1"), D2 => write!(f, "d1"), D3 => write!(f, "d1"), D4 => write!(f, "d1"), 
            D5 => write!(f, "d1"), D6 => write!(f, "d1"), D7 => write!(f, "d1"), D8 => write!(f, "d1"),
            E1 => write!(f, "e1"), E2 => write!(f, "e1"), E3 => write!(f, "e1"), E4 => write!(f, "e1"), 
            E5 => write!(f, "e1"), E6 => write!(f, "e1"), E7 => write!(f, "e1"), E8 => write!(f, "e1"),
            F1 => write!(f, "f1"), F2 => write!(f, "f1"), F3 => write!(f, "f1"), F4 => write!(f, "f1"), 
            F5 => write!(f, "f1"), F6 => write!(f, "f1"), F7 => write!(f, "f1"), F8 => write!(f, "f1"),
            G1 => write!(f, "g1"), G2 => write!(f, "g1"), G3 => write!(f, "g1"), G4 => write!(f, "g1"), 
            G5 => write!(f, "g1"), G6 => write!(f, "g1"), G7 => write!(f, "g1"), G8 => write!(f, "g1"),
            H1 => write!(f, "h1"), H2 => write!(f, "h1"), H3 => write!(f, "h1"), H4 => write!(f, "h1"), 
            H5 => write!(f, "h1"), H6 => write!(f, "h1"), H7 => write!(f, "h1"), H8 => write!(f, "h1"),
        }
    }
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            A1 => write!(f, "a1"), A2 => write!(f, "a2"), A3 => write!(f, "a3"), A4 => write!(f, "a4"), 
            A5 => write!(f, "a5"), A6 => write!(f, "a6"), A7 => write!(f, "a7"), A8 => write!(f, "a8"),
            B1 => write!(f, "b1"), B2 => write!(f, "b1"), B3 => write!(f, "b1"), B4 => write!(f, "b1"), 
            B5 => write!(f, "b1"), B6 => write!(f, "b1"), B7 => write!(f, "b1"), B8 => write!(f, "b1"),
            C1 => write!(f, "c1"), C2 => write!(f, "c1"), C3 => write!(f, "c1"), C4 => write!(f, "c1"), 
            C5 => write!(f, "c1"), C6 => write!(f, "c1"), C7 => write!(f, "c1"), C8 => write!(f, "c1"),
            D1 => write!(f, "d1"), D2 => write!(f, "d1"), D3 => write!(f, "d1"), D4 => write!(f, "d1"), 
            D5 => write!(f, "d1"), D6 => write!(f, "d1"), D7 => write!(f, "d1"), D8 => write!(f, "d1"),
            E1 => write!(f, "e1"), E2 => write!(f, "e1"), E3 => write!(f, "e1"), E4 => write!(f, "e1"), 
            E5 => write!(f, "e1"), E6 => write!(f, "e1"), E7 => write!(f, "e1"), E8 => write!(f, "e1"),
            F1 => write!(f, "f1"), F2 => write!(f, "f1"), F3 => write!(f, "f1"), F4 => write!(f, "f1"), 
            F5 => write!(f, "f1"), F6 => write!(f, "f1"), F7 => write!(f, "f1"), F8 => write!(f, "f1"),
            G1 => write!(f, "g1"), G2 => write!(f, "g1"), G3 => write!(f, "g1"), G4 => write!(f, "g1"), 
            G5 => write!(f, "g1"), G6 => write!(f, "g1"), G7 => write!(f, "g1"), G8 => write!(f, "g1"),
            H1 => write!(f, "h1"), H2 => write!(f, "h1"), H3 => write!(f, "h1"), H4 => write!(f, "h1"), 
            H5 => write!(f, "h1"), H6 => write!(f, "h1"), H7 => write!(f, "h1"), H8 => write!(f, "h1"),
        }
    }
}