use cards;

pub struct House {
    pub contents: Vec<cards::Card>,
}

pub enum HouseType {
    Daimyo,
    Samurai,
}

impl House {
    pub fn daimyo(daimyo_card: cards::Card) -> House {
        House {contents: vec![daimyo_card]}
    }

    pub fn samurai() -> House {
        House {contents: vec![cards::Card::samurai()]}
    }

    pub fn honor(&self) -> i8 {
        self.stat(0)
    }

    pub fn ki(&self) -> i8 {
        self.stat(1)
    }

    pub fn strength(&self) -> i8 {
        self.stat(2)
    }

    fn stat(&self, stat: usize) -> i8 {
        self.contents.iter().fold(0, |acc, card| if let Some(stats) = card.stats() { acc + stats[stat] } else { acc })
    }
}