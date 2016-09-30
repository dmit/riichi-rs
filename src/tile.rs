extern crate rand;

use rand::Rng;
use std::cmp;
use std::slice;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rank::One => write!(f, "one"),
            Rank::Two => write!(f, "two"),
            Rank::Three => write!(f, "three"),
            Rank::Four => write!(f, "four"),
            Rank::Five => write!(f, "five"),
            Rank::Six => write!(f, "six"),
            Rank::Seven => write!(f, "seven"),
            Rank::Eight => write!(f, "eight"),
            Rank::Nine => write!(f, "nine"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Direction {
    East,
    South,
    West,
    North,
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::East => write!(f, "east"),
            Direction::South => write!(f, "south"),
            Direction::West => write!(f, "west"),
            Direction::North => write!(f, "north"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Color {
    White,
    Green,
    Red,
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Color::White => write!(f, "white"),
            Color::Green => write!(f, "green"),
            Color::Red => write!(f, "red"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Icon {
    Plain(char),
    Red(char),
}
impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Icon::Plain(ch) => write!(f, "{}", ch),
            Icon::Red(ch) => write!(f, "{}*", ch),
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Tile {
    Pin(Rank, Icon),
    Sou(Rank, Icon),
    Wan(Rank, Icon),
    Wind(Direction, Icon),
    Dragon(Color, Icon),
}
impl Tile {
    fn icon(&self) -> Icon {
        match *self {
            Tile::Pin(_, icon) |
            Tile::Sou(_, icon) |
            Tile::Wan(_, icon) |
            Tile::Wind(_, icon) |
            Tile::Dragon(_, icon) => icon,
        }
    }

    fn same_type(a: Tile, b: Tile) -> bool {
        match (a, b) {
            (Tile::Pin(..), Tile::Pin(..)) |
            (Tile::Sou(..), Tile::Sou(..)) |
            (Tile::Wan(..), Tile::Wan(..)) |
            (Tile::Wind(..), Tile::Wind(..)) |
            (Tile::Dragon(..), Tile::Dragon(..)) => true,
            _ => false,
        }
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.icon())
    }
}
impl Eq for Tile {}
impl PartialEq for Tile {
    fn eq(&self, other: &Tile) -> bool {
        match (*self, *other) {
            (Tile::Pin(a, _), Tile::Pin(b, _)) |
            (Tile::Sou(a, _), Tile::Sou(b, _)) |
            (Tile::Wan(a, _), Tile::Wan(b, _)) => a == b,
            (Tile::Wind(a, _), Tile::Wind(b, _)) => a == b,
            (Tile::Dragon(a, _), Tile::Dragon(b, _)) => a == b,
            _ => false,
        }
    }
}

const ONE_PIN: Tile = Tile::Pin(Rank::One, Icon::Plain('🀙'));
const TWO_PIN: Tile = Tile::Pin(Rank::Two, Icon::Plain('🀚'));
const THREE_PIN: Tile = Tile::Pin(Rank::Three, Icon::Plain('🀛'));
const FOUR_PIN: Tile = Tile::Pin(Rank::Four, Icon::Plain('🀜'));
const FIVE_PIN: Tile = Tile::Pin(Rank::Five, Icon::Plain('🀝'));
const SIX_PIN: Tile = Tile::Pin(Rank::Six, Icon::Plain('🀞'));
const SEVEN_PIN: Tile = Tile::Pin(Rank::Seven, Icon::Plain('🀟'));
const EIGHT_PIN: Tile = Tile::Pin(Rank::Eight, Icon::Plain('🀠'));
const NINE_PIN: Tile = Tile::Pin(Rank::Nine, Icon::Plain('🀡'));
const RED_FIVE_PIN: Tile = Tile::Pin(Rank::Five, Icon::Red('🀝'));

const ONE_SOU: Tile = Tile::Sou(Rank::One, Icon::Plain('🀐'));
const TWO_SOU: Tile = Tile::Sou(Rank::Two, Icon::Plain('🀑'));
const THREE_SOU: Tile = Tile::Sou(Rank::Three, Icon::Plain('🀒'));
const FOUR_SOU: Tile = Tile::Sou(Rank::Four, Icon::Plain('🀓'));
const FIVE_SOU: Tile = Tile::Sou(Rank::Five, Icon::Plain('🀔'));
const SIX_SOU: Tile = Tile::Sou(Rank::Six, Icon::Plain('🀕'));
const SEVEN_SOU: Tile = Tile::Sou(Rank::Seven, Icon::Plain('🀖'));
const EIGHT_SOU: Tile = Tile::Sou(Rank::Eight, Icon::Plain('🀗'));
const NINE_SOU: Tile = Tile::Sou(Rank::Nine, Icon::Plain('🀘'));
const RED_FIVE_SOU: Tile = Tile::Sou(Rank::Five, Icon::Red('🀔'));

const ONE_WAN: Tile = Tile::Wan(Rank::One, Icon::Plain('🀇'));
const TWO_WAN: Tile = Tile::Wan(Rank::Two, Icon::Plain('🀈'));
const THREE_WAN: Tile = Tile::Wan(Rank::Three, Icon::Plain('🀉'));
const FOUR_WAN: Tile = Tile::Wan(Rank::Four, Icon::Plain('🀊'));
const FIVE_WAN: Tile = Tile::Wan(Rank::Five, Icon::Plain('🀋'));
const SIX_WAN: Tile = Tile::Wan(Rank::Six, Icon::Plain('🀌'));
const SEVEN_WAN: Tile = Tile::Wan(Rank::Seven, Icon::Plain('🀍'));
const EIGHT_WAN: Tile = Tile::Wan(Rank::Eight, Icon::Plain('🀎'));
const NINE_WAN: Tile = Tile::Wan(Rank::Nine, Icon::Plain('🀏'));
const RED_FIVE_WAN: Tile = Tile::Wan(Rank::Five, Icon::Red('🀋'));

const EAST_WIND: Tile = Tile::Wind(Direction::East, Icon::Plain('🀀'));
const SOUTH_WIND: Tile = Tile::Wind(Direction::South, Icon::Plain('🀁'));
const WEST_WIND: Tile = Tile::Wind(Direction::West, Icon::Plain('🀂'));
const NORTH_WIND: Tile = Tile::Wind(Direction::North, Icon::Plain('🀃'));

const WHITE_DRAGON: Tile = Tile::Dragon(Color::White, Icon::Plain('🀆'));
const GREEN_DRAGON: Tile = Tile::Dragon(Color::Green, Icon::Plain('🀅'));
const RED_DRAGON: Tile = Tile::Dragon(Color::Red, Icon::Plain('🀄'));

const WITH_DORA: [Tile; 34] = [ONE_PIN,
                               TWO_PIN,
                               THREE_PIN,
                               FOUR_PIN,
                               RED_FIVE_PIN,
                               SIX_PIN,
                               SEVEN_PIN,
                               EIGHT_PIN,
                               NINE_PIN,

                               ONE_SOU,
                               TWO_SOU,
                               THREE_SOU,
                               FOUR_SOU,
                               RED_FIVE_SOU,
                               SIX_SOU,
                               SEVEN_SOU,
                               EIGHT_SOU,
                               NINE_SOU,

                               ONE_WAN,
                               TWO_WAN,
                               THREE_WAN,
                               FOUR_WAN,
                               RED_FIVE_WAN,
                               SIX_WAN,
                               SEVEN_WAN,
                               EIGHT_WAN,
                               NINE_WAN,

                               EAST_WIND,
                               SOUTH_WIND,
                               WEST_WIND,
                               NORTH_WIND,

                               WHITE_DRAGON,
                               GREEN_DRAGON,
                               RED_DRAGON];

const WITHOUT_DORA: [Tile; 34] = [ONE_PIN,
                                  TWO_PIN,
                                  THREE_PIN,
                                  FOUR_PIN,
                                  FIVE_PIN,
                                  SIX_PIN,
                                  SEVEN_PIN,
                                  EIGHT_PIN,
                                  NINE_PIN,

                                  ONE_SOU,
                                  TWO_SOU,
                                  THREE_SOU,
                                  FOUR_SOU,
                                  FIVE_SOU,
                                  SIX_SOU,
                                  SEVEN_SOU,
                                  EIGHT_SOU,
                                  NINE_SOU,

                                  ONE_WAN,
                                  TWO_WAN,
                                  THREE_WAN,
                                  FOUR_WAN,
                                  FIVE_WAN,
                                  SIX_WAN,
                                  SEVEN_WAN,
                                  EIGHT_WAN,
                                  NINE_WAN,

                                  EAST_WIND,
                                  SOUTH_WIND,
                                  WEST_WIND,
                                  NORTH_WIND,

                                  WHITE_DRAGON,
                                  GREEN_DRAGON,
                                  RED_DRAGON];

trait Tiles {
    fn iter(&self) -> slice::Iter<Tile>;

    fn fmt_impl(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = self.iter()
            .map(|t| format!("{}", t))
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{}", str)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Group {
    Chi,
    Pon,
    Kan,
}

pub struct Hand(Vec<Tile>);
impl Hand {
    pub fn new(tiles: &[Tile]) -> Hand {
        let mut hand = Vec::with_capacity(4 * 4 + 2); // four kans + head
        hand.extend_from_slice(tiles);
        hand.sort();

        Hand(hand)
    }

    pub fn take(&mut self, pos: u8) -> Option<Tile> {
        let pos = pos as usize;
        if pos < self.0.len() {
            Some(self.0.remove(pos))
        } else {
            None
        }
    }

    pub fn put(&mut self, tile: Tile) {
        self.0.push(tile);
        self.0.sort();
    }

    /// Returns the number of tiles needed to reach tenpai.
    pub fn shanten(&self) -> u8 {
        let pairs = self.pairs();
        let seven_pairs = 6 - cmp::min(pairs, 6);
        let thirteen_orphans = 13 - cmp::min(self.orphans(), 13);
        let groups = self.groups();
        let other = 8; //TODO

        cmp::min(seven_pairs, cmp::min(thirteen_orphans, other))
    }

    fn pairs(&self) -> u8 {
        let mut pairs = 0;
        let mut prev: Option<Tile> = None;

        for tile in self.iter() {
            match prev {
                Some(t) => {
                    if t == *tile {
                        pairs += 1;
                        prev = None;
                    } else {
                        prev = Some(*tile);
                    }
                }
                None => prev = Some(*tile),
            }
        }

        pairs
    }

    fn orphans(&self) -> u8 {
        let mut orphans: u16 = 0;
        let mut has_pair = false;

        for tile in self.iter() {
            let idx = match *tile {
                Tile::Pin(Rank::One, _) => Some(0),
                Tile::Pin(Rank::Nine, _) => Some(1),
                Tile::Sou(Rank::One, _) => Some(2),
                Tile::Sou(Rank::Nine, _) => Some(3),
                Tile::Wan(Rank::One, _) => Some(4),
                Tile::Wan(Rank::Nine, _) => Some(5),
                Tile::Wind(Direction::East, _) => Some(6),
                Tile::Wind(Direction::South, _) => Some(7),
                Tile::Wind(Direction::West, _) => Some(8),
                Tile::Wind(Direction::North, _) => Some(9),
                Tile::Dragon(Color::White, _) => Some(10),
                Tile::Dragon(Color::Green, _) => Some(11),
                Tile::Dragon(Color::Red, _) => Some(12),
                _ => None,
            };

            if let Some(i) = idx {
                let mask = 1 << i;
                if orphans & mask > 0 {
                    has_pair = true;
                } else {
                    orphans |= mask;
                }
            }
        }

        orphans.count_ones() as u8 + has_pair as u8
    }

    fn groups(&self) -> u8 {
        // TODO
        0
    }

    fn group_type(tiles: &[Tile]) -> Option<Group> {
        let same_type = tiles.iter()
            .skip(1)
            .all(|&t| Tile::same_type(t, tiles[0]));
        let same_value = tiles.iter()
            .skip(1)
            .all(|&t| t == tiles[0]);

        match (tiles.len(), same_type, same_value) {
            (4, true, true) => Some(Group::Kan),
            (3, true, true) => Some(Group::Pon),
            (3, true, false) => {
                let consecutive = match (tiles[0], tiles[1], tiles[2]) {
                    (Tile::Pin(a, _), Tile::Pin(b, _), Tile::Pin(c, _)) |
                    (Tile::Sou(a, _), Tile::Sou(b, _), Tile::Sou(c, _)) |
                    (Tile::Wan(a, _), Tile::Wan(b, _), Tile::Wan(c, _)) => {
                        b as i8 - a as i8 == 1 && c as i8 - b as i8 == 1
                    }
                    _ => false,
                };

                if consecutive { Some(Group::Chi) } else { None }
            }
            _ => None,
        }
    }
}
impl Tiles for Hand {
    fn iter(&self) -> slice::Iter<Tile> {
        self.0.iter()
    }
}
impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_impl(f)
    }
}

pub struct Set(Vec<Tile>);
impl Set {
    pub fn new() -> Set {
        let cnt = WITH_DORA.len();
        let mut tiles = Vec::with_capacity(cnt * 4);
        tiles.extend(WITH_DORA.iter().cloned());
        tiles.extend(WITHOUT_DORA.iter().cloned().cycle().take(cnt * 3));

        Set(tiles)
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut self.0);
    }

    pub fn take_hand(&mut self) -> Option<Hand> {
        let hand_size = 13;
        if self.0.len() >= hand_size {
            let mut taken = Vec::with_capacity(hand_size);
            taken.extend(self.iter().rev().take(hand_size));

            let remaining_size = self.size() as usize - hand_size;
            self.0.truncate(remaining_size);

            Some(Hand::new(&taken))
        } else {
            None
        }
    }

    pub fn take(&mut self) -> Option<Tile> {
        self.0.pop()
    }

    pub fn size(&self) -> u8 {
        self.0.len() as u8
    }
}
impl Tiles for Set {
    fn iter(&self) -> slice::Iter<Tile> {
        self.0.iter()
    }
}
impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_impl(f)
    }
}

#[cfg(test)]
mod tests {}
