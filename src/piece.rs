use std::fmt;

pub enum Piece {
    Pawn, Knight, Bishop, Rook, Queen, King 
}

use Piece::*;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Pawn => "P", Knight => "N", Bishop => "B", Rook => "R", Queen => "Q", King => "K" 
        })
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Pawn => "P", Knight => "N", Bishop => "B", Rook => "R", Queen => "Q", King => "K" 
        })
    }
}