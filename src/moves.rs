// use crate::square::Square;
// use crate::piece::Piece;

use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;

#[derive(Debug)]
pub struct Move {
    pub san: Option<String>,
    // pub from: Option<Square>,
    // pub to: Option<Square>,
    // pub promotion: Option<Piece>,
    // pub piece: Option<Piece>,
    // pub fen: Option<String>,
    pub parent: RefCell<Weak<Move>>,
    pub moves: RefCell<Vec<Rc<Move>>>,
}

impl Move {
    pub fn new_root() -> Move {
        Move {
            san: None, // from: None, to: None, promotion: None, piece: None, fen: None, 
            parent: RefCell::new(Weak::new()), 
            moves: RefCell::new(vec![]),
        }
    }

    pub fn new(san: String, parent: Rc<Move>) -> Move {
        Move {
            san: Some(san), // from: None, to: None, promotion: None, piece: None, fen: None, 
            parent: RefCell::new(Rc::downgrade(&parent)), 
            moves: RefCell::new(vec![]),
        }
    }

    fn graph_rec(&self, count: usize) -> String {
        let moves = self.moves.borrow();
        let san = self.san.clone().unwrap_or(String::from(""));
        if moves.len() == 0 {
            return san;
        } else if moves.len() == 1 {
            if san.len() == 0 {
                return format!("{}", moves[0].graph_rec(count));
            }else{
                return format!("{} {}", san, moves[0].graph_rec(count));
            }
        }else{
            let mut str = san;
            for m in &*moves {
                str = format!("{}\n{} {}", str, " |".repeat(count + 1), m.graph_rec(count + 1));
            }
            return str;
        }
    }

    pub fn graph(&self) -> String {
        return self.graph_rec(0);
    }

    // pub fn is_root(&self) -> bool {
    //     match self.parent.borrow().upgrade() {
    //         Some(_) => { true },
    //         None => { false },
    //     }
    // } 
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"san\": \"{}\", \"moves\": [", self.san.clone().unwrap_or(String::from("NOSAN")));
        let mut first = true;
        for m in &*self.moves.borrow() {
            if(first) {
                write!(f, "{}", m);
                first = false;
            }else{
                write!(f, ", {}", m);
            }
        }
        write!(f, "]}}")
    }
}