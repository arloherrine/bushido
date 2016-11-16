use house;
use player;

pub struct Card {
    stats: Option<[i8; 3]>,
    id: usize,
    ninjaProof: bool,
    action: CardAction,
}

pub enum CardAction {
    None,
    Daimyo,
    Okugata,
    Army,
    Item,
    Castle,
    NinjaSpy,
    EliteNinjaSpy,
    NinjaAssassin,
    Dishonor,
}

pub enum ActionResult {
    MoveFromHouse(ActionTarget, ActionTarget),
    MoveFromHand(ActionTarget),
    Discard(ActionTarget),
    DiscardFromHand,
    Roll(u8),
    HonorChange(usize, i8),
    Dishonor(usize),
    War(usize, usize),
    Shogun(usize),
    Draw(usize),
}

pub enum ActionTarget {
    Target(CardDest),
    Steal(CardSource, CardDest),
    DoubleSteal(CardSource, CardDest, CardSource, CardDest),
}

pub struct CardSource {
    pub player_id: usize,
    pub house: house::HouseType,
    pub index: usize,
}

pub enum CardDest {
    Player(usize),
    House(usize, house::HouseType),
    Discard,
}

impl ActionTarget {
    fn daimyo(player_id: usize) -> ActionTarget {
        ActionTarget::Target(CardDest::House(player_id, house::HouseType::Daimyo))
    }

    fn samurai(player_id: usize) -> ActionTarget {
        ActionTarget::Target(CardDest::House(player_id, house::HouseType::Samurai))
    }
}

impl Card {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn stats(&self) -> Option<[i8; 3]> {
        self.stats
    }

    pub fn isNinjaProof(&self) -> bool {
        self.ninjaProof
    }

    pub fn samurai() -> Card {
        Card {stats: Some([0, 6, 0]), id: 0, ninjaProof: true, action: CardAction::None}
    }

    pub fn daimyo(id: usize, stats: [i8; 3]) -> Card {
        Card {id: id, stats: Some(stats), ninjaProof: true, action: CardAction::Daimyo}
    }

    pub fn okugata1() -> Card {
        Card {id: 7, stats: Some([10, 3, 0]), ninjaProof: true, action: CardAction::Okugata}
    }

    pub fn okugata2() -> Card {
        Card {id: 8, stats: Some([5, 4, 0]), ninjaProof: true, action: CardAction::Okugata}
    }

    pub fn ninjaSpy() -> Card {
        Card {id: 9, stats: None, ninjaProof: false, action: CardAction::NinjaSpy}
    }

    pub fn eliteNinjaSpy() -> Card {
        Card {id: 10, stats: None, ninjaProof: false, action: CardAction::EliteNinjaSpy}
    }

    pub fn ninjaAssassin() -> Card {
        Card {id: 11, stats: None, ninjaProof: false, action: CardAction::NinjaAssassin}
    }

    pub fn odwara() -> Card {
        Card {id: 12, stats: Some([5, 0, 3]), ninjaProof: true, action: CardAction::Castle}
    }

    pub fn osaka() -> Card {
        Card {id: 13, stats: Some([10, 1, 4]), ninjaProof: true, action: CardAction::Castle}
    }

    pub fn heron() -> Card {
        Card {id: 14, stats: Some([15, 2, 5]), ninjaProof: true, action: CardAction::Castle}
    }

    pub fn dishonor() -> Card {
        Card {id: 15, stats: None, ninjaProof: false, action: CardAction::Dishonor}
    }

    pub fn saveFace() -> Card {
        Card {id: 16, stats: None, ninjaProof: false, action: CardAction::None}
    }

    pub fn army1() -> Card {
        Card {id: 17, stats: Some([0, 0, 1]), ninjaProof: false, action: CardAction::Army}
    }

    pub fn army2() -> Card {
        Card {id: 18, stats: Some([0, 0, 2]), ninjaProof: false, action: CardAction::Army}
    }

    pub fn army3() -> Card {
        Card {id: 19, stats: Some([0, 0, 3]), ninjaProof: false, action: CardAction::Army}
    }

    pub fn monkArmy() -> Card {
        Card {id: 20, stats: Some([0, 1, 1]), ninjaProof: false, action: CardAction::Army}
    }

    pub fn noDachi() -> Card {
        Card {id: 21, stats: Some([5, 1, 3]), ninjaProof: false, action: CardAction::Item}
    }

    pub fn daisho() -> Card {
        Card {id: 22, stats: Some([10, 1, 1]), ninjaProof: false, action: CardAction::Item}
    }

    pub fn swordsmith() -> Card {
        Card {id: 23, stats: Some([10, 1, 4]), ninjaProof: false, action: CardAction::Item}
    }

    pub fn gun() -> Card {
        Card {id: 24, stats: Some([-20, -2, 6]), ninjaProof: false, action: CardAction::Item}
    }

    pub fn noh() -> Card {
        Card {id: 25, stats: Some([20, 3, 0]), ninjaProof: false, action: CardAction::Item}
    }

    pub fn houseGuard() -> Card {
        Card {id: 26, stats: None, ninjaProof: true, action: CardAction::Item}
    }

    pub fn cardActions(&self) -> Option<u8> {
        match self.action {
            CardAction::Dishonor => Some(2),
            CardAction::None => None,
            _ => Some(1),
        }
    }

    pub fn getActionTargets(&self, player_id: usize, players: &[player::Player]) -> Vec<ActionTarget> {
        match self.action {
            CardAction::Daimyo => {
                // TODO
                Vec::new()
            },
            CardAction::Okugata => {
                let ref player = players[player_id];
                let mut targets = Vec::new();
                if !player.daimyo.contents.iter().any(|card: &Card| card.id == 7 || card.id == 8) {
                    targets.push(ActionTarget::daimyo(player_id));
                }
                if !player.samurai.contents.iter().any(|card: &Card| card.id == 7 || card.id == 8) {
                    targets.push(ActionTarget::samurai(player_id));
                }
                targets
            },
            CardAction::NinjaSpy => {
                // TODO
                Vec::new()
            },
            CardAction::EliteNinjaSpy => {
                // TODO
                Vec::new()
            },
            CardAction::NinjaAssassin => {
                // TODO
                Vec::new()
            },
            CardAction::Castle => {
                if !players[player_id].daimyo.contents.iter().any(|card: &Card| card.id == 12 || card.id == 13 || card.id == 14) {
                    vec![ActionTarget::daimyo(player_id)]
                } else {
                    Vec::new()
                }
            },
            CardAction::Army => {
                let ref player = players[player_id];
                let mut targets = Vec::new();
                if player.daimyo.contents.iter().filter(|card: &&Card| card.id >= 17 && card.id <= 20).count() < 5 {
                    targets.push(ActionTarget::daimyo(player_id));
                }
                if player.samurai.contents.iter().filter(|card: &&Card| card.id >= 17 && card.id <= 20).count() < 5 {
                    targets.push(ActionTarget::samurai(player_id));
                }
                targets
            },
            CardAction::Item => {
                vec![ActionTarget::daimyo(player_id), ActionTarget::samurai(player_id)]
            },
            CardAction::Dishonor => {
                (0..players.len()).filter(|&x| x != player_id).map(|x| ActionTarget::Target(CardDest::Player(x))).collect()
            },
            CardAction::None => panic!("Requested action targets on non-action card")
        }
    }

    pub fn performAction(&self, player_id: usize, players: &[player::Player], target: ActionTarget) -> Vec<ActionResult> {
        match self.action {
            CardAction::Okugata | CardAction::Army | CardAction::Item | CardAction::Castle => {
                vec![ActionResult::MoveFromHand(target)]
            },
            CardAction::Daimyo => {
                // TODO
                Vec::new()
            },
            CardAction::NinjaSpy => {
                // TODO
                Vec::new()
            },
            CardAction::EliteNinjaSpy => {
                // TODO
                Vec::new()
            },
            CardAction::NinjaAssassin => {
                // TODO
                Vec::new()
            },
            CardAction::Dishonor => {
                if let ActionTarget::Target(CardDest::Player(player_id)) = target {
                    vec![ActionResult::Dishonor(player_id)]
                } else {
                    panic!("Invalid action target for dishonor")
                }
            },
            CardAction::None => panic!("Requested perform action on non-action card")
        }
    }
}
