// build with "wasm-pack build --target web"

use wasm_bindgen::prelude::*;
use std::ops::{Index, IndexMut};

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum PieceTeam {
    White,
    Black
}

impl PieceTeam {
    pub const fn can_damage(self, other: Self) -> bool{
        use PieceTeam::*;
        match self {
            White => {
                match other {
                    White => false,
                    Black => true,
                }
            },
            Black => {
                match other {
                    White => true,
                    Black => false,
                }
            }
        }
    }

    pub const fn can_heal(self, other: Self) -> bool {
        return !self.can_damage(other);
    }
}

impl PieceType {
    pub const fn max_health(&self) -> usize {
        use PieceType::*;
        match self {
            Pawn => 100,
            Knight => 500,
            Bishop => 200,
            Rook => 1000,
            Queen => 750,
            King => 1000
        }
    }
}

#[wasm_bindgen]
extern "C" {
    pub type JsCoord;

    #[wasm_bindgen(method, getter)]
    fn x(this: &JsCoord) -> usize;

    #[wasm_bindgen(method, getter)]
    fn y(this: &JsCoord) -> usize;

}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_piece(a: Option<Piece>);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_coord(a: Coordinate);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize
}

impl From<&JsCoord> for Coordinate {
    fn from(item: &JsCoord) -> Coordinate {
        Coordinate{x: item.x(), y: item.y()}
    }
}

#[wasm_bindgen]
impl Coordinate{
    #[wasm_bindgen(constructor)]
    pub fn new(x:usize, y:usize) -> Coordinate {
        Coordinate{x,y}
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub team: PieceTeam,
    pub piece_type: PieceType,
    pub health: usize
}

#[wasm_bindgen]
impl Piece {
    #[wasm_bindgen]
    pub fn relative_health(&self) -> f32 {
        self.health as f32 / self.piece_type.max_health() as f32
    }
}

impl Piece {
    pub fn new(team: PieceTeam, piece_type: PieceType) -> Piece {
        Piece{piece_type, team, health: piece_type.max_health()}
    }

    pub fn attack_strength(&self) -> usize {
        use PieceType::*;
        match self.piece_type {
            Pawn => 2,
            Knight => 6,
            Bishop => 5,
            Rook => 6,
            Queen => 5,
            King => 3
        }
    }

    pub fn healing_strength(&self) -> usize {
        use PieceType::*;
        match self.piece_type {
            Pawn => 1,
            Knight => 0,
            Bishop => 1,
            Rook => 1,
            Queen => 0,
            King => 2
        }
    }

}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct Board {
    pieces: [Option<Piece>; 64]
}

use std::fmt;
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rank_index = 0;
        loop {
            for _ in 0..8 {
                write!(f,"+------")?;
            }
            write!(f,"+\n")?;
            if rank_index == 8 {
                break;
            }
            if rank_index == 8{
                break;
            }
            for i in 0..8 {
                use PieceTeam::*;
                match self[Coordinate{x:rank_index, y:i}] {
                    Some(p) => match p.team {
                        White => write!(f, "|White ")?,
                        Black => write!(f, "|Black ")?,
                    },
                    None => write!(f, "|      ")?
                }
            }
            write!(f,"|\n")?;
            for i in 0..8 {
                use PieceType::*;
                match self[Coordinate{x:rank_index,y:i}] {
                    Some(p) => match p.piece_type {
                        Pawn => write!(f, "| Pawn ")?,
                        Knight => write!(f, "|Knight")?,
                        Bishop => write!(f, "|Bishop")?,
                        Rook => write!(f, "| Rook ")?,
                        Queen => write!(f, "|Queen ")?,
                        King => write!(f, "| King ")?
                    },
                    None => write!(f, "|      ")?
                }
            }
            write!(f,"|\n")?;
            for i in 0..8 {
                match self[Coordinate{x:rank_index,y:i}]{
                    Some(p) => write!(f,"|{:^6}", p.health)?,
                    None => write!(f, "|      ")?,
                }
            }
            write!(f,"|\n")?;

            rank_index += 1;
        }
        Ok(())
    }
}

impl Index<Coordinate> for Board {
    type Output = Option<Piece>;
    fn index(&self, c: Coordinate) -> &Self::Output {
        &self.pieces[(c.x<<3)+c.y]
    }
}

impl IndexMut<Coordinate> for Board {
    fn index_mut(&mut self, c: Coordinate) -> &mut Self::Output {
        &mut self.pieces[(c.x<<3)+c.y]
    }
}

impl Board {
    pub fn blank() -> Self {
        Board{pieces: [None; 64]}
    }

    pub fn move_piece(&mut self, from: Coordinate, to: Coordinate) -> bool
    {
        if self.can_move_to(from, to) {
            self.pieces.swap(
                from.x*8 + from.y,
                to.x*8 + to.y
            );
            true
        } else {
            false
        }
    }

    pub fn attacked_positions(&self, attacker_coords: Coordinate) -> Vec<Coordinate> {
        let mut attacked_coords = Vec::new();
        let in_bounds = |rel_file, rel_rank| {
            let new_file = (attacker_coords.x as isize) + rel_file;
            let new_rank = (attacker_coords.y as isize) + rel_rank;
            if 0 <= new_file && new_file < 8
            && 0 <= new_rank && new_rank < 8 {
                Some(Coordinate{x:new_file as usize, y: new_rank as usize})
            } else {
                None
            }
        };
        if let Some(attacker) = self[attacker_coords] {
            use PieceType::*;
            use PieceTeam::*;
            match attacker.piece_type {
                Pawn => {
                    let rel_rank = match attacker.team {
                        White => 1,
                        Black => -1
                    };
                    for rel_file in [-1,1] {
                        if let Some(c) = in_bounds(rel_file, rel_rank) {
                            attacked_coords.push(c);
                        }
                    }
                },
                Knight => {
                    for rel_rank in [-1,1] {
                        for rel_file in [-2,2] {
                            if let Some(c) = in_bounds(rel_file, rel_rank) {
                                attacked_coords.push(c);
                            }
                        }
                    }
                    for rel_rank in [-2,2] {
                        for rel_file in [-1,1] {
                            if let Some(c) = in_bounds(rel_file, rel_rank) {
                                attacked_coords.push(c);
                            }
                        }
                    }
                },
                Bishop => {
                    for direction in [(1,1),(1,-1),(-1,1),(-1,-1)] {
                        let mut distance = 1;
                        while let Some(c) = in_bounds(distance*direction.0, distance*direction.1) {
                            attacked_coords.push(c);
                            if let Some(_) = self[c] {
                                // We hit a piece, so we can't attack any further squares
                                break;
                            }
                            distance += 1;
                        }
                    }
                },
                Rook => {
                    for direction in [(0,1),(0,-1),(1,0),(-1,0)] {
                        let mut distance = 1;
                        while let Some(c) = in_bounds(distance*direction.0, distance*direction.1) {
                            attacked_coords.push(c);
                            if let Some(_) = self[c] {
                                // We hit a piece, so we can't attack any further squares
                                break;
                            }
                            distance += 1;
                        }
                    }
                },
                Queen => {
                    for direction in [(0,1),(0,-1),(1,0),(-1,0),(1,1),(1,-1),(-1,1),(-1,-1)] {
                        let mut distance = 1;
                        while let Some(c) = in_bounds(distance*direction.0, distance*direction.1) {
                            attacked_coords.push(c);
                            if let Some(_) = self[c] {
                                // We hit a piece, so we can't attack any further squares
                                break;
                            }
                            distance += 1;
                        }
                    }
                },
                King => {
                    for direction in [(0,1),(0,-1),(1,0),(-1,0),(1,1),(1,-1),(-1,1),(-1,-1)] {
                        if let Some(c) = in_bounds(direction.0, direction.1) {
                            attacked_coords.push(c);
                        }
                    }
                }
            }
        }
        attacked_coords
    }

    pub fn moveable_positions(&self, candidate_coords: Coordinate) -> Vec<Coordinate> {
        let mut free_coords = Vec::<Coordinate>::new();
        let in_bounds = |rel_file, rel_rank| {
            let new_file = (candidate_coords.x as isize) + rel_rank;
            let new_rank = (candidate_coords.y as isize) + rel_file;
            if 0 <= new_file && new_file < 8
            && 0 <= new_rank && new_rank < 8 {
                Some(Coordinate{x:new_file as usize, y: new_rank as usize})
            } else {
                None
            }
        };

        let mut add_if_no_piece = |c: Coordinate| {
            if let None = self[c] {
                free_coords.push(c);
                true
            } else {
                false
            }
        };

        if let Some(piece) = self[candidate_coords] {
            use PieceType::*;
            match piece.piece_type {
                Pawn => {
                    let (direction, required_rank) = match piece.team {
                        PieceTeam::White => ( 1, 1),
                        PieceTeam::Black => (-1, 6)
                    };
                    if let Some(c) = in_bounds(direction, 0) {
                        if add_if_no_piece(c) && candidate_coords.y == required_rank {
                            if let Some(c) = in_bounds(direction*2, 0) {
                                add_if_no_piece(c);
                            }
                        }
                        else {
                            println!("Could not move to that square. {:?}",c);
                        }
                    }
                },
                Knight => {
                    for rel_rank in [-1,1] {
                        for rel_file in [-2,2] {
                            if let Some(c) = in_bounds(rel_file, rel_rank) {
                                add_if_no_piece(c);
                            }
                        }
                    }
                    for rel_rank in [-2,2] {
                        for rel_file in [-1,1] {
                            if let Some(c) = in_bounds(rel_file, rel_rank) {
                                add_if_no_piece(c);
                            }
                        }
                    }
                },
                Bishop => {
                    for direction in [(1,1),(1,-1),(-1,1),(-1,-1)] {
                        let mut distance = 1;
                        while let Some(c) = in_bounds(distance*direction.0, distance*direction.1) {
                            if !add_if_no_piece(c){
                                // We hit a piece, so we can't move to any further squares
                                break;
                            }
                            distance += 1;
                        }
                    }
                },
                Rook => {
                    for direction in [(0,1),(0,-1),(1,0),(-1,0)] {
                        let mut distance = 1;
                        while let Some(c) = in_bounds(distance*direction.0, distance*direction.1) {
                            if !add_if_no_piece(c){
                                // We hit a piece, so we can't move to any further squares
                                break;
                            }
                            distance += 1;
                        }
                    }
                },
                Queen => {
                    for direction in [(0,1),(0,-1),(1,0),(-1,0),(1,1),(1,-1),(-1,1),(-1,-1)] {
                        let mut distance = 1;
                        while let Some(c) = in_bounds(distance*direction.0, distance*direction.1) {
                            if !add_if_no_piece(c){
                                // We hit a piece, so we can't move to any further squares
                                break;
                            }
                            distance += 1;
                        }
                    }
                },
                King => {
                    for direction in [(0,1),(0,-1),(1,0),(-1,0),(1,1),(1,-1),(-1,1),(-1,-1)] {
                        if let Some(c) = in_bounds(direction.0, direction.1) {
                            add_if_no_piece(c);
                        }
                    }
                }
            }
        }
        free_coords
    }

    pub fn can_move_to(&self, from_coords: Coordinate, to_coords: Coordinate) -> bool {
        for i in self.moveable_positions(from_coords).into_iter() {
            if to_coords == i {
                return true;
            }
        }
        false
    }

    // pub fn render(&self, canvas: &mut web_sys::CanvasRenderingContext2d){

    // }
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn default_setup() -> Self {
        use PieceType::*;
        use PieceTeam::*;
        let mut pieces = [None; 64];
        for (coord, team, piece) in [
            ((0,0), White, Rook),
            ((1,0), White, Knight),
            ((2,0), White, Bishop),
            ((3,0), White, Queen),
            ((4,0), White, King),
            ((5,0), White, Bishop),
            ((6,0), White, Knight),
            ((7,0), White, Rook),
            ((0,7), Black, Rook),
            ((1,7), Black, Knight),
            ((2,7), Black, Bishop),
            ((3,7), Black, Queen),
            ((4,7), Black, King),
            ((5,7), Black, Bishop),
            ((6,7), Black, Knight),
            ((7,7), Black, Rook)
        ].into_iter().chain(
            (0..8).map(|i| ((i,1), White, Pawn))
        ).chain(
            (0..8).map(|i| ((i,6), Black, Pawn))
        ) {
            pieces[(coord.0<<3) + coord.1] = Some(Piece::new(team,piece));
        }
        Board{pieces}
    }

    #[wasm_bindgen(js_name = movePiece)]
    pub fn js_move_piece(&mut self, from: &JsCoord, to: &JsCoord) -> bool
    {
        let (from, to) = (from.into(), to.into());
        log_piece(self[from]);
        log_piece(self[to]);
        self.move_piece(from, to)
    }

    #[wasm_bindgen]
    pub fn get_piece_at(&self, position: &JsCoord) -> Option<Piece> {
        self[position.into()]
    }

    #[wasm_bindgen]
    pub fn run_damage_calculation(&mut self) {
        let mut delta_health = [[0_isize; 8]; 8];
        for attacker_rank in 0..8 {
            for attacker_file in 0..8 {
                if let Some(attacker) = self[Coordinate{x:attacker_rank,y:attacker_file}] {
                    for c in self.attacked_positions(Coordinate{x:attacker_rank, y:attacker_file}) {
                        if let Some(attacked_piece) = self[c] {
                            if attacker.team.can_damage(attacked_piece.team) {
                                delta_health[c.x][c.y] -= attacker.attack_strength() as isize;
                            }
                            if attacker.team.can_heal(attacked_piece.team) {
                                delta_health[c.x][c.y] += attacker.healing_strength() as isize;
                            }
                        }
                    }
                }
            }
        }
        println!("{:?}",delta_health);
        for rank in 0..8 {
            for file in 0..8 {
                if let Some(ref mut piece) = self[Coordinate{x:rank,y:file}] {
                    piece.health = (((piece.health as isize) + delta_health[rank][file]).max(0) as usize).min(piece.piece_type.max_health());
                    if piece.health == 0 {
                        println!("{:?} at ({},{}) died.", piece, rank, file);
                        self[Coordinate{x:rank,y:file}] = None;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn main() {
        // let window = web_sys::window().expect("window exists");
        //     let canvas = window.document().expect("document exists")
        //     .get_element_by_id("chessboard").expect("chess board exists");
        // let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();
        // let width: u32 = canvas.width();
        // let mut context = canvas.get_context("2d").unwrap().unwrap()
        //     .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

        let mut board = Board::default_setup();
        println!("{}", board);
        println!("{:?}", board.moveable_positions(Coordinate{x:0,y:1}));
        for i in 0..8 {
            assert!(board.can_move_to(Coordinate{x:i,y:1}, Coordinate{x:i, y:3}));
            assert!(board.can_move_to(Coordinate{x:i,y:6}, Coordinate{x:i, y:4}));
        }

        for _ in 0..19 {
            board.run_damage_calculation();
            println!("{}", board);
            // board.render(&mut context);
        }
        board[Coordinate{x:3, y:5}] = Some(Piece::new(PieceTeam::White, PieceType::Knight));

        // println!("{}", Board::blank());
    }
}