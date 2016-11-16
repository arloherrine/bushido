use cards;
use house;

pub struct Player {
    pub hand: Vec<cards::Card>,
    pub daimyo: house::House,
    pub samurai: house::House,
    pub ally: Option<usize>,
    pub totalHonor: u8,
    pub isShogun: bool,
    pub name: String,
}

impl Player {
    pub fn new(name: &str, daimyo_card: cards::Card, hand: Vec<cards::Card>) -> Player {
        Player {
            hand: hand,
            daimyo: house::House::daimyo(daimyo_card),
            samurai: house::House::samurai(),
            ally: None,
            totalHonor: 0,
            isShogun: false,
            name: name.to_string()
        }
    }

    pub fn honorPerTurn(&self) -> i8 {
        if self.daimyo.contents.is_empty() {
            // TODO (ally daimyo honor + self samurai honor) / 2
            0
        } else {
            self.daimyo.honor() + self.samurai.honor()
        }
    }

    pub fn ki(&self) -> i8 {
        self.daimyo.ki() + self.samurai.ki()
    }

    pub fn strength(&self) -> i8 {
        // TODO add ally strength
        self.daimyo.strength() + self.samurai.strength()
    }

}