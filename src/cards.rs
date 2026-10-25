use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Suit { Spades, Hearts, Diamonds, Clubs }
impl Suit { pub fn symbol(self) -> char { match self { Self::Spades => '♠', Self::Hearts => '♥', Self::Diamonds => '♦', Self::Clubs => '♣' } } #[allow(dead_code)] pub fn red(self) -> bool { matches!(self, Self::Hearts | Self::Diamonds) } }

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Rank { Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King }
impl Rank { pub fn value(self) -> u8 { match self { Self::Ace => 11, Self::Two => 2, Self::Three => 3, Self::Four => 4, Self::Five => 5, Self::Six => 6, Self::Seven => 7, Self::Eight => 8, Self::Nine => 9, Self::Ten | Self::Jack | Self::Queen | Self::King => 10 } } pub fn label(self) -> &'static str { match self { Self::Ace => "A", Self::Two => "2", Self::Three => "3", Self::Four => "4", Self::Five => "5", Self::Six => "6", Self::Seven => "7", Self::Eight => "8", Self::Nine => "9", Self::Ten => "10", Self::Jack => "J", Self::Queen => "Q", Self::King => "K" } } }

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Card { pub rank: Rank, pub suit: Suit }
impl Card { pub fn text(self) -> String { format!("{}{}", self.rank.label(), self.suit.symbol()) } }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shoe { pub cards: Vec<Card> }
impl Shoe { pub fn new() -> Self { let mut cards = Vec::with_capacity(312); for _ in 0..6 { for suit in [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] { for rank in [Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King] { cards.push(Card { rank, suit }); } } } cards.shuffle(&mut rand::rng()); Self { cards } } pub fn deal(&mut self) -> Card { if self.cards.len() <= 78 { *self = Self::new(); } self.cards.pop().expect("new shoe is non-empty") } }

#[cfg(test)] mod tests { use super::*; #[test] fn shoe_has_six_decks() { assert_eq!(Shoe::new().cards.len(), 312); } }
