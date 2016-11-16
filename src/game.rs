use cards::{Card, ActionResult, ActionTarget, CardDest, CardSource};
use player::{Player};
use house::{House, HouseType};

use rand::{thread_rng, Rng};
use std::fmt::Write;

pub struct Game {
    turn: usize,
    actions_used: u8,
    shogun_available: bool,
    players: Vec<Player>,
    deck: Vec<Card>,
    discard: Vec<Card>,
}

impl Game {
    pub fn new(player_names: &[&str]) -> Game {
        let mut deck = Vec::new();
        deck.push(Card::daimyo(1, [30, 1, 3]));
        deck.push(Card::daimyo(2, [20, 1, 5]));
        deck.push(Card::daimyo(3, [15, 3, 2]));
        deck.push(Card::daimyo(4, [15, 1, 3]));
        for _ in 0..3 { deck.push(Card::daimyo(5, [10, 0, 2])); }
        for _ in 0..3 { deck.push(Card::daimyo(6, [5, 0, 1])); }

        let mut rng = thread_rng();
        rng.shuffle(&mut deck);

        let mut player_daimyos: Vec<Card> = deck.drain(0..player_names.len()).collect();

        for _ in 0..5 { deck.push(Card::okugata1()); }
        for _ in 0..5 { deck.push(Card::okugata2()); }

        for _ in 0..8 { deck.push(Card::ninjaSpy()); }
        for _ in 0..3 { deck.push(Card::eliteNinjaSpy()); }
        for _ in 0..5 { deck.push(Card::ninjaAssassin()); }

        deck.push(Card::odwara());
        deck.push(Card::osaka());
        deck.push(Card::heron());

        for _ in 0..5 { deck.push(Card::dishonor()); }
        for _ in 0..17 { deck.push(Card::saveFace()); }

        for _ in 0..10 { deck.push(Card::army1()); }
        for _ in 0..6 { deck.push(Card::army2()); }
        for _ in 0..4 { deck.push(Card::army3()); }
        for _ in 0..6 { deck.push(Card::monkArmy()); }

        deck.push(Card::noDachi());
        deck.push(Card::daisho());
        deck.push(Card::swordsmith());
        deck.push(Card::gun());
        deck.push(Card::noh());
        for _ in 0..8 { deck.push(Card::houseGuard()); }

        rng.shuffle(&mut deck);

        let mut players = Vec::new();
        for name in player_names {
            players.push(Player::new(name, player_daimyos.pop().unwrap(), deck.drain(0..7).collect()));
        }
        //let players = player_names.iter().zip(player_daimyos.iter()).map(|(name, &daimyo_card)| Player::new(name, daimyo_card, deck.drain(0..7).collect())).collect();
        Game {
            turn: 0, // TODO randomize who goes first
            actions_used: 0,
            shogun_available: true,
            players: players,
            deck: deck,
            discard: Vec::new(),
        }
    }

    pub fn getMoves(&self) -> String {
        let player = &self.players[self.turn];
        let card_actions = (player.ki() / 3) as u8 - self.actions_used;
        let mut result = String::new();
        write!(&mut result, "[\n");
        for (i, card) in player.hand.iter().enumerate() {
            if let Some(actions) = card.cardActions() {
                if actions <= card_actions {
                    card.getActionTargets(self.turn, &self.players).iter().map(|actionTarget| {
                        write!(&mut result, "    \"card_action hand_{} ", i);
                        match *actionTarget {
                            ActionTarget::Target(ref dest) => writeCardDestToString(&mut result, &dest),
                            ActionTarget::Steal(ref source, ref dest) => {
                                write!(&mut result, "steal_{}_{}_{} ", source.player_id, source.house, source.index);
                                writeCardDestToString(&mut result, &dest);
                            },
                            ActionTarget::DoubleSteal(ref source1, ref dest1, ref source2, ref dest2) => {
                                write!(&mut result, "steal_{}_{}_{} ", source1.player_id, source1.house, source1.index);
                                writeCardDestToString(&mut result, &dest1);
                                write!(&mut result, " steal_{}_{}_{} ", source2.player_id, source2.house, source2.index);
                                writeCardDestToString(&mut result, &dest2);
                            }
                        }
                        write!(&mut result, "\",\n");
                    }).count();
                }
            }
        }

        self.players.iter().enumerate()
                .filter(|&(player_id, player)| player_id != self.turn && (player.isShogun || player.daimyo.contents.iter().any(|card| card.id() == 12 || card.id() == 13 || card.id() == 14)))
                .map(|(player_id, _)| write!(&mut result, "    \"declare_war {}\",\n", player_id));

        if self.shogun_available {
            write!(&mut result, "    \"declare_shogun\",\n");
        }

        write!(&mut result, "    \"end_turn\"\n]");
        result
    }

    pub fn executeMove(&mut self, input: &str) -> Vec<ActionResult> {
        // TODO parse input
        Vec::new()
    }

    pub fn serialize(&self, player_id: usize) -> String {
        let mut result = String::new();
        write!(&mut result, "{{\n").unwrap();
        write!(&mut result, "    \"you\": {},\n", player_id).unwrap();
        write!(&mut result, "    \"turn\": {},\n", self.turn).unwrap();
        write!(&mut result, "    \"shogun_available\": {},\n", self.shogun_available).unwrap();
        write!(&mut result, "    \"hand\": ").unwrap();
        writeCardsToString(&mut result, &self.players[player_id].hand);
        write!(&mut result, ",\n").unwrap();

        write!(&mut result, "    \"players\": [\n").unwrap();
        let mut player_iter = self.players.iter();
        writePlayerToString(&mut result, player_iter.next().unwrap());
        player_iter.map(|player| {
            write!(&mut result, ",\n").unwrap();
            writePlayerToString(&mut result, player);
        }).count();
        write!(&mut result, "\n    ]\n").unwrap();
        write!(&mut result, "}}").unwrap();
        result
    }
}

fn writeCardDestToString(mut result: &mut String, dest: &CardDest) {
    match *dest {
        CardDest::Player(player_id) => write!(&mut result, "player_{}", player_id),
        CardDest::House(player_id, ref house_type) => write!(&mut result, "house_{}_{}", player_id, house_type),
        CardDest::Discard => write!(&mut result, "discard"),
    };
}

fn writePlayerToString(mut result: &mut String, player: &Player) {
    write!(&mut result, "        {{\n").unwrap();
    write!(&mut result, "            \"name\": \"{}\",\n", player.name).unwrap();
    write!(&mut result, "            \"total_honor\": {},\n", player.totalHonor).unwrap();
    write!(&mut result, "            \"honor_per_turn\": {},\n", player.honorPerTurn()).unwrap();
    write!(&mut result, "            \"ki\": {},\n", player.ki()).unwrap();
    write!(&mut result, "            \"strength\": {},\n", player.strength()).unwrap();
    write!(&mut result, "            \"shogun\": {},\n", player.isShogun).unwrap();
    write!(&mut result, "            \"daimyo\": ").unwrap();
    writeCardsToString(&mut result, &player.daimyo.contents);
    write!(&mut result, ",\n").unwrap();
    write!(&mut result, "            \"samurai\": ").unwrap();
    writeCardsToString(&mut result, &player.samurai.contents);
    write!(&mut result, "\n").unwrap();
    //write!(&mut result, "            samurai: [{}],\n", player.samurai.contents.iter().map(|card| card.id().to_string()).collect().join(", ")).unwrap();
    write!(&mut result, "        }}").unwrap();
}

fn writeCardsToString(mut dest: &mut String, elements: &[Card]) {
    let mut elem_iter = elements.iter();
    write!(&mut dest, "[{}", elem_iter.next().unwrap().id());
    elem_iter.map(|card: &Card| write!(dest, ", {}", card.id()).unwrap()).count();
    write!(&mut dest, "]");
}
