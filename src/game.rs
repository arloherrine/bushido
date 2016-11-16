use cards::{Card};
use player::{Player};
use house::{House, HouseType};

use rand::{thread_rng, Rng};
use std::fmt::Write;

pub struct Game {
    turn: u8,
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
            turn: 0,
            shogun_available: true,
            players: players,
            deck: deck,
            discard: Vec::new(),
        }
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
        write!(&mut result, "}}\n").unwrap();
        result
    }
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
