use serde::{Deserialize, Serialize};

use bevy::prelude::{KeyCode, Reflect};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Reflect)]
#[derive(Deserialize, Serialize)]

#[rustfmt::skip]
pub enum Letter {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
}
use Letter::*;

impl std::fmt::Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Letter {
    pub fn from_keycode(code: KeyCode) -> Option<Self> {
        match code {
            KeyCode::KeyA => Some(A),
            KeyCode::KeyB => Some(B),
            KeyCode::KeyC => Some(C),
            KeyCode::KeyD => Some(D),
            KeyCode::KeyE => Some(E),
            KeyCode::KeyF => Some(F),
            KeyCode::KeyG => Some(G),
            KeyCode::KeyH => Some(H),
            KeyCode::KeyI => Some(I),
            KeyCode::KeyJ => Some(J),
            KeyCode::KeyK => Some(K),
            KeyCode::KeyL => Some(L),
            KeyCode::KeyM => Some(M),
            KeyCode::KeyN => Some(N),
            KeyCode::KeyO => Some(O),
            KeyCode::KeyP => Some(P),
            KeyCode::KeyQ => Some(Q),
            KeyCode::KeyR => Some(R),
            KeyCode::KeyS => Some(S),
            KeyCode::KeyT => Some(T),
            KeyCode::KeyU => Some(U),
            KeyCode::KeyV => Some(V),
            KeyCode::KeyW => Some(W),
            KeyCode::KeyX => Some(X),
            KeyCode::KeyY => Some(Y),
            KeyCode::KeyZ => Some(Z),
            _ => None,
        }
    }

    pub fn from_char(character: char) -> Option<Self> {
        match character {
            'A' | 'a' => Some(A),
            'B' | 'b' => Some(B),
            'C' | 'c' => Some(C),
            'D' | 'd' => Some(D),
            'E' | 'e' => Some(E),
            'F' | 'f' => Some(F),
            'G' | 'g' => Some(G),
            'H' | 'h' => Some(H),
            'I' | 'i' => Some(I),
            'J' | 'j' => Some(J),
            'K' | 'k' => Some(K),
            'L' | 'l' => Some(L),
            'M' | 'm' => Some(M),
            'N' | 'n' => Some(N),
            'O' | 'o' => Some(O),
            'P' | 'p' => Some(P),
            'Q' | 'q' => Some(Q),
            'R' | 'r' => Some(R),
            'S' | 's' => Some(S),
            'T' | 't' => Some(T),
            'U' | 'u' => Some(U),
            'V' | 'v' => Some(V),
            'W' | 'w' => Some(W),
            'X' | 'x' => Some(X),
            'Y' | 'y' => Some(Y),
            'Z' | 'z' => Some(Z),
            _ => None,
        }
    }

    pub fn from_string(letter: &str) -> Option<Self> {
        match letter {
            "A" | "a" => Some(A),
            "B" | "b" => Some(B),
            "C" | "c" => Some(C),
            "D" | "d" => Some(D),
            "E" | "e" => Some(E),
            "F" | "f" => Some(F),
            "G" | "g" => Some(G),
            "H" | "h" => Some(H),
            "I" | "i" => Some(I),
            "J" | "j" => Some(J),
            "K" | "k" => Some(K),
            "L" | "l" => Some(L),
            "M" | "m" => Some(M),
            "N" | "n" => Some(N),
            "O" | "o" => Some(O),
            "P" | "p" => Some(P),
            "Q" | "q" => Some(Q),
            "R" | "r" => Some(R),
            "S" | "s" => Some(S),
            "T" | "t" => Some(T),
            "U" | "u" => Some(U),
            "V" | "v" => Some(V),
            "W" | "w" => Some(W),
            "X" | "x" => Some(X),
            "Y" | "y" => Some(Y),
            "Z" | "z" => Some(Z),
            _ => None,
        }
    }
}
