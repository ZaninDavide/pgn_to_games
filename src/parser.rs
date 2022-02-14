use std::collections::HashMap;

use crate::moves::Move;

use std::rc::Rc;

#[derive(Clone, Debug)]
enum ParserState {
    Idle,
    IgnoreSpaces,
    Expect(char), // waits for the char given. Gives an error if finds anything different
    TagName(String), // parsing tag name
    TagValue(String, String), // parsing the value of the tag given (name, value)
    TagEnd, // the value tag section is finished and you are waiting for ]
    Comment(String), // parser is inside a comment
    San(String), // reading a san
    MoveNumber, // reading move numbering
}

impl ParserState {
    fn add(&mut self, c: char) {
        match self {
            ParserState::TagName(name) => {name.push(c)},
            ParserState::TagValue(name, value) => {value.push(c)},
            ParserState::Comment(text) => {text.push(c)},
            ParserState::San(san) => {san.push(c)},
            _ => {}
        }
    }
}

/*
pub struct ParserSettings {
    pub validate_moves: bool,
}

impl ParserSettings {
    fn new() -> ParserSettings {
        ParserSettings { validate_moves: false }
    }
}
*/

enum ParserSection {
    Tags, Game
}

#[derive(Debug)]
pub enum ParserError {
    // ErrorReadingFile, 
    // InvalidMove, 
    ErrorInvalidStateReached(String),
    SyntaxError(String),
}

pub struct ParsedPgn {
    pub root: Rc<Move>,
    pub tags: HashMap<String, String>
} 

/* for debugging
fn format_cursors(cursors: &Vec<Rc<Move>>) -> String {
    let mut str = String::from("");
    for c in cursors {
        str = format!("{}{} ", str, c.san.clone().unwrap_or(String::from("NOSAN")));
    }
    str
} 
*/

pub fn parse(pgn: &str) -> Result<ParsedPgn, ParserError> {
    // create new tree to fill with data
    let tree = Rc::new(Move::new_root());
    let mut tags: HashMap<String, String> = HashMap::new();
    
    let mut states: Vec<ParserState> = vec![ParserState::Idle];
    let mut section: ParserSection = ParserSection::Tags;
    
    let mut cursors: Vec<Rc<Move>> = vec![Rc::clone(&tree)];
    
    // parse
    for letter in pgn.chars() {
        // println!("{:?} con {:?} -> {}", states, format_cursors(&cursors), letter);
        // state machine

        // IGNORE SPACES
        match states.last().unwrap_or(&ParserState::Idle) { 
            ParserState::IgnoreSpaces => {
                if letter == ' ' || letter == '\n' { 
                    continue; 
                } {
                    // exit the ignoreSpaces state and go on with this character
                    states.pop();
                } 
            },
            _ => {}
        }
        // CHECK FOR EXPECTATIONS
        match states.last().unwrap_or(&ParserState::Idle) {
            ParserState::Expect(exp) => {
                if letter == *exp {
                    // expect fullfilled
                    states.pop();
                    continue;
                }else{
                    // expect not fullfilled
                    return Err(ParserError::SyntaxError(format!("Expecting '{}' and found '{}'", exp, letter)));
                }
            },
            _ => {} 
        }

        // consider your current state and "letter" to update state and tree
        match section {
            ParserSection::Tags => {
                // TAGS and HEADERS SECTION
                let state = states.last().unwrap_or(&ParserState::Idle).clone();
                match state {
                    // you are not doing anything useful
                    ParserState::Idle => {
                        match letter {
                            '[' => {
                                states.push(ParserState::TagName(String::new()));
                                states.push(ParserState::IgnoreSpaces);
                            },
                            '0'..='9' => {
                                section = ParserSection::Game;
                                states.push(ParserState::MoveNumber)
                            },
                            ']' => { 
                                return Err(ParserError::SyntaxError(String::from("Found ']' but and opening square braket is missing"))); }
                            _ => {}
                        }
                    },
                    // you are inside of a tag name (headers on top of the file)
                    ParserState::TagName(name) => {
                        match letter {
                            ' ' | '\n' => {
                                states.pop();
                                states.push(ParserState::TagValue(name, String::new()));
                                states.push(ParserState::Expect('"'));
                                states.push(ParserState::IgnoreSpaces);
                            },
                            ']' => {
                                return Err(ParserError::SyntaxError(format!("Found ']' but no value was specified for tag: {}", "name")));
                            },
                            _ => {
                                states.last_mut().unwrap().add(letter);
                            },
                        }
                    },
                    // you are reading the value of a tag
                    ParserState::TagValue(name, value) => {
                        match letter {
                            '"' => {
                                // end of tag, we can store the value and wait for the ] character
                                tags.insert(name, value);
                                states.pop();
                                states.push(ParserState::Expect(']'));
                                states.push(ParserState::IgnoreSpaces);
                            },
                            _ => {
                                states.last_mut().unwrap().add(letter);
                            }
                        }
                    },
                    _ => { 
                        return Err(ParserError::ErrorInvalidStateReached(
                            format!("Reached {:?} state inside tag section", state))
                        ); 
                    }
                }
            },
            ParserSection::Game => {
                // GAME MOVES and COMMENTS SECTION
                let state = states.last().unwrap_or(&ParserState::Idle).clone();
                match state {
                    ParserState::Idle => {
                        match letter {
                            '0'..='9' => {
                                states.push(ParserState::MoveNumber);
                            },
                            'a'..='z' | 'A'..='Z' => {
                                states.push(ParserState::San(String::from(letter)));
                            }
                            '(' => {
                                // start new variation, we need to push the parent of this cursor
                                let cursor = Rc::clone(&cursors[cursors.len() - 1]);
                                cursors.push(Rc::clone(&Rc::clone(&cursor).parent.borrow().upgrade().unwrap()));
                            },
                            ')' => {
                                // go on adding to the old variation
                                cursors.pop();
                            },
                            '*' => {
                                // the game has ended
                            }
                            ' ' | '\n' => {},
                            _ => {
                                return Err(ParserError::SyntaxError(format!(
                                    "Found invalid charcter '{}' while in Idle state", letter
                                )));
                            }
                        }
                    },
                    ParserState::MoveNumber => {
                        match letter {
                            '0'..='9' | '.' => {
                                // just go on
                            },
                            ' ' | '\n' => {
                                // enter state san
                                states.pop();
                                states.push(ParserState::San(String::new()));
                                states.push(ParserState::IgnoreSpaces);
                            }
                            _ => {
                                return Err(ParserError::SyntaxError(format!(
                                    "Found invalid charcter '{}' while in state MoveNumber", letter
                                )));
                            }
                        }
                    },
                    //
                    ParserState::San(san) => {
                        match letter {
                            '0'..='9' | 'a'..='z' | 'A'..='Z' | '+' | '=' | '!' | '-' => {
                                states.last_mut().unwrap().add(letter);
                            },
                            ' ' | '\n' | ')' | ';' | ',' => {
                                // san finished
                                // add a new move with this san to the moves of cursor
                                let cursor = cursors.pop().expect("Cannot add san if there is no cursor");
                                // add new move
                                let new_move = Rc::new(Move::new(san, Rc::clone(&cursor)));
                                // make the cursor be the move you just created
                                cursor.moves.borrow_mut().push(Rc::clone(&new_move));
                                if letter != ')' { 
                                    // make this move the new cursor
                                    cursors.push(new_move); 
                                } // else fallback to the old cursor
                                states.pop();
                            },
                            _ => {
                                return Err(ParserError::SyntaxError(format!(
                                    "Found invalid charcter '{}' while in state San", letter
                                )));
                            }
                        }
                    },
                    //
                    ParserState::Comment(comment) => {
    
                    },
                    _ => { 
                        return Err(ParserError::ErrorInvalidStateReached(
                            format!("Reached {:?} state inside game section", state))
                        );  
                    }
                }
            }
        }
    }

    Ok(ParsedPgn {
        root: Rc::clone(&tree), tags: tags
    })
}