use color::Color;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Rank {
    General, // 帥,將
    Advisor, // 仕,士
    Elephant, // 相,象
    Chariot, // 俥,車
    Horse, // 傌,馬
    Soldier, // 兵,卒
    Cannon // 炮,砲
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Piece {
    pub rank: Rank,
    pub color: Color,
}

