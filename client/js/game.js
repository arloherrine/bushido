
var game = new Phaser.Game(1100, 600, Phaser.AUTO, '', { preload: preload, create: create, update: update });

function preload() {
    game.load.image('army_1', 'assets/army_1.png');
    game.load.image('army_2', 'assets/army_2.png');
    game.load.image('army_3', 'assets/army_3.png');
    game.load.image('castle_1', 'assets/castle_1.png');
    game.load.image('castle_2', 'assets/castle_2.png');
    game.load.image('castle_3', 'assets/castle_3.png');
    game.load.image('daimyo_1', 'assets/daimyo_1.png');
    game.load.image('daimyo_2', 'assets/daimyo_2.png');
    game.load.image('daimyo_3', 'assets/daimyo_3.png');
    game.load.image('daimyo_4', 'assets/daimyo_4.png');
    game.load.image('daimyo_5', 'assets/daimyo_5.png');
    game.load.image('daisho', 'assets/daisho.png');
    game.load.image('dishonor', 'assets/dishonor.png');
    game.load.image('elite_ninja', 'assets/elite_ninja.png');
    game.load.image('gun', 'assets/gun.png');
    game.load.image('house_guard', 'assets/house_guard.png');
    game.load.image('monk_army', 'assets/monk_army.png');
    game.load.image('ninja', 'assets/ninja.png');
    game.load.image('ninja_assassin', 'assets/ninja_assassin.png');
    game.load.image('no_dachi', 'assets/no_dachi.png');
    game.load.image('noh_theater', 'assets/noh_theater.png');
    game.load.image('okugata_1', 'assets/okugata_1.png');
    game.load.image('okugata_2', 'assets/okugata_2.png');
    game.load.image('logo', 'assets/samurai_logo.png');
    game.load.image('samurai', 'assets/samurai.png');
    game.load.image('save_face', 'assets/save_face.png');
    game.load.image('swordsmith', 'assets/swordsmith.png');
}

var background_group;
var midground_group;
var foreground_group;
var deck = [];
var player_hand = [];
var houses = [];
var stats = [];
var hand = [];
var selected_card;
var end_turn_bmd;
var declare_war_bmd;
var declare_shogun_bmd;
var shogun_indicator;
var active_buttons = [];
/* TODO
shogun flag per player
shogun available thing
turn indicator
move history/action indicator
*/

function create_images() {
    end_turn_bmd = game.add.bitmapData(80, 20);
    end_turn_bmd.fill(0x00, 0x00, 0x00);
    end_turn_bmd.rect(2, 2, 76, 16, "#3333ff");
    end_turn_bmd.text("End Turn", 10, 14, "14px Times", "#dddddd", true);

    declare_war_bmd = game.add.bitmapData(90, 20);
    declare_war_bmd.fill(0x00, 0x00, 0x00);
    declare_war_bmd.rect(2, 2, 86, 16, "#ff3333");
    declare_war_bmd.text("Declare War", 10, 14, "14px Times", "#dddddd", true);

    declare_shogun_bmd = game.add.bitmapData(110, 20);
    declare_shogun_bmd.fill(0x00, 0x00, 0x00);
    declare_shogun_bmd.rect(2, 2, 106, 16, "#33dd33");
    declare_shogun_bmd.text("Declare Shogun", 10, 14, "14px Times", "#dddddd", true);
}

function create() {
    background_group = game.add.group()
    midground_group = game.add.group()
    foreground_group = game.add.group()

    create_images();

    var background = game.add.graphics(0, 0, background_group);
    background.beginFill(0xcccccc);
    background.drawRect(0, 0, 1100, 600);
    background.endFill();

    // TODO draw deck, etc.

    aClient = new HttpClient();
    aClient.get('/api/teststart', function(response) {
        game_state = response['state'];
        moves = response['moves'];
        start_game(game_state);
        enableMoves(moves);
    });
}

function enableMoves(moves) {
    for (var i in moves) {
        var move_tokens = moves[i].split(" ");
        switch (move_tokens[0]) {
            case "end_turn":
                var button = game.make.button(500, 345, end_turn_bmd, function() {
                    // TODO submit end turn action
                    // TODO disable all turn inputs
                });
                //button.loadTexture(end_turn_bmd);
                foreground_group.add(button);
                active_buttons.push(button);
                break;
            case "declare_shogun":
                var button = game.make.button(160, 345, null, function() {
                    // TODO submit declare shogun action
                    // TODO disable all turn inputs
                });
                button.loadTexture(declare_shogun_bmd);
                foreground_group.add(button);
                active_buttons.push(button);
                break;
            case "declare_war":
                // TODO move button based on target player
                var button = game.make.button(500, 400, null, function() {
                    // TODO submit declare war action
                    // TODO disable all turn inputs
                });
                button.loadTexture(declare_war_bmd);
                foreground_group.add(button);
                active_buttons.push(button);
                break;
            case "card_action":
                var hand_index = parseInt(move_tokens[1].split("_")[1]);
                const card_sprite = hand[hand_index];
                const card_border = game.add.graphics(card_sprite.x - 2, card_sprite.y - 2, midground_group);
                card_border.beginFill(0xff8800);
                card_border.drawRect(0, 0, 74, 74);
                card_border.endFill();
                card_sprite.events.onInputDown.addOnce(function() {
                    card_border.destroy();
                    // TODO add handler to this to replace this
                    // TODO add handlers to next targets
                    // TODO disable all turn inputs
                });
                break;
        }
    }
}

function start_game(game_state) {
    
    draw_this_player(game_state);

    for (var i = 1; i < 4; i++) { // TODO actual number of players
        var player_id = (i + game_state.you) % 4; // TODO actual number of players
        player_state = game_state.players[player_id];

        var base_x = 5 + (800 / 3) * (i - 1);
        var background = game.add.graphics(0, 0, background_group);
        background.beginFill(0x000000);
        background.drawRect(base_x - 5, 0, 801 / 3, 180);
        background.endFill();
        background.beginFill(0x999999);
        background.drawRect(base_x - 3, 2, 263, 176);
        background.endFill();

        var stats = [
            game.add.text(base_x, 5, player_state.total_honor, {font: "16px Arial", fill: "#00ff00"}),
            game.add.text(base_x + 20, 5, "(" + player_state.honor_per_turn + "/turn)", {font: "16px Arial", fill: "#00ff00"}),
            game.add.text(base_x + 90, 5, player_state.ki, {font: "16px Arial", fill: "#0000ff"}),
            game.add.text(base_x + 120, 5, player_state.strength, {font: "16px Arial", fill: "#ff0000"}),
        ];
        stats[player_id] = stats;

        const daimyo = [];
        for (const j in player_state.daimyo) {
            const card_data = player_state.daimyo[j];
            var card = create_card(card_data);
            const card_x = base_x + j * 30;
            var card_sprite = game.add.sprite(card_x, 25, card);
            card_sprite.inputEnabled = true;
            card_sprite.events.onInputDown.add(function() {
                if (selected_card) {
                    selected_card.destroy();
                }
                selected_card = game.add.sprite(800, 200, create_card(card_data, true))
            });
            card_sprite.events.onInputOver.add(function() {
                for (var k = parseInt(j) + 1; k < daimyo.length; k++) {
                    game.add.tween(daimyo[k]).to( { x: card_x + 40 + (k - j) * 30}, 200).start();
                }
            });
            card_sprite.events.onInputOut.add(function() {
                for (var k = parseInt(j) + 1; k < daimyo.length; k++) {
                    game.add.tween(daimyo[k]).to( { x: card_x + (k - j) * 30}, 200).start();
                }
            });
            daimyo.push(card_sprite);
        }

        const samurai = [];
        for (const j in player_state.samurai) {
            const card_data = player_state.samurai[j];
            var card = create_card(card_data);
            const card_x = base_x + j * 30;
            var card_sprite = game.add.sprite(card_x, 100, card);
            card_sprite.inputEnabled = true;
            card_sprite.events.onInputDown.add(function() {
                if (selected_card) {
                    selected_card.destroy();
                }
                selected_card = game.add.sprite(800, 200, create_card(card_data, true))
            });
            card_sprite.events.onInputOver.add(function() {
                for (var k = parseInt(j) + 1; k < samurai.length; k++) {
                    game.add.tween(samurai[k]).to( { x: card_x + 40 + (k - j) * 30}, 200).start();
                }
            });
            card_sprite.events.onInputOut.add(function() {
                for (var k = parseInt(j) + 1; k < samurai.length; k++) {
                    game.add.tween(samurai[k]).to( { x: card_x + (k - j) * 30}, 200).start();
                }
            });
            samurai.push(card_sprite);
        }
        houses[player_id] = {daimyo: daimyo, samurai: samurai};
    }
}

function draw_this_player(game_state) {
    var background = game.add.graphics(0, 0, background_group);
    background.beginFill(0x000000);
    background.drawRect(0, 340, 800, 260);
    background.endFill();
    background.beginFill(0x9999ff);
    background.drawRect(2, 342, 796, 256);
    background.endFill();

    var player_state = game_state.players[game_state.you];

    var stat_start = 320;
    var stat_y = 345;
    var stats = [
        game.add.text(stat_start, stat_y, player_state.total_honor, {font: "16px Arial", fill: "#00ff00"}),
        game.add.text(stat_start + 20, stat_y, "(" + player_state.honor_per_turn + "/turn)", {font: "16px Arial", fill: "#00ff00"}),
        game.add.text(stat_start + 90, stat_y, player_state.ki, {font: "16px Arial", fill: "#0000ff"}),
        game.add.text(stat_start + 120, stat_y, player_state.strength, {font: "16px Arial", fill: "#ff0000"}),
    ];
    stats[game_state.you] = stats;

    var house_len = player_state.samurai.length + 1;
    if (player_state.daimyo.length > house_len) {
        house_len = player_state.daimyo.length;
    }
    var padding = (800 - house_len * 75) / 2;
    if (padding < 5) {
        padding = 5;
    }

    var daimyo = [];
    for (var j in player_state.daimyo) {
        const card_data = player_state.daimyo[j];
        var card = create_card(card_data);
        var card_sprite = game.add.sprite(padding + j * 75, 365, card);
        card_sprite.inputEnabled = true;
        card_sprite.events.onInputDown.add(function() {
            if (selected_card) {
                selected_card.destroy();
            }
            selected_card = game.add.sprite(800, 200, create_card(card_data, true))
        });
        daimyo.push(card_sprite);
    }

    var samurai = [];
    for (var j in player_state.samurai) {
        const card_data = player_state.samurai[j];
        var card = create_card(card_data);
        var card_sprite = game.add.sprite(padding + j * 75, 440, card);
        card_sprite.inputEnabled = true;
        card_sprite.events.onInputDown.add(function() {
            if (selected_card) {
                selected_card.destroy();
            }
            selected_card = game.add.sprite(800, 200, create_card(card_data, true))
        });
        samurai.push(card_sprite);
    }
    houses[game_state.you] = {daimyo: daimyo, samurai: samurai};

    
    padding = (800 - game_state.hand.length * 75) / 2;
    for (var j in game_state.hand) {
        const card_data = game_state.hand[j];
        var card = create_card(card_data);
        var card_sprite = game.add.sprite(padding + j * 75, 525, card);
        card_sprite.inputEnabled = true;
        card_sprite.events.onInputDown.add(function() {
            if (selected_card) {
                selected_card.destroy();
            }
            selected_card = game.add.sprite(800, 200, create_card(card_data, true))
        });
        hand.push(card_sprite);
    }
}

function stat_text(player_index, text, line, color) {
    var position;
    switch (player_index) {
        case 0:
            position = [150 + line * 30, 350];
        case 1:
            position = [80, 140];
        case 2:
            position = [650, 100];
        case 3:
            position = [640, 390];
    }
    //return game.add.text(position[0], position[1], text, {font: "16px Arial", fill: color});
}

function card_house_position(player_index, samurai, house_index) {
    var rel_x = 50 + house_index * 75;
    var rel_y = 20;
    if (samurai) {
        rel_y += 75;
    }
    return absolute_position(player_index, rel_x, rel_y);
}

function absolute_position(player_index, x, y) {
    switch (player_index) {
        case 0:
            return [130 + x, 350 + y];
        case 1:
            return [100 - y, 130 + x];
        case 2:
            return [620 - x, 100 - y];
        case 3:
            return [630 + y, 340 - x];
    }
}

function create_card(card_index, large) {
    if (large) {
        var base = game.add.bitmapData(300, 400);
        base.fill(0x00, 0x00, 0x00);
        base.rect(2, 2, 296, 396, "#ffffff");
    } else {
        var base = game.add.bitmapData(70, 70);
        base.fill(0x00, 0x00, 0x00);
        base.rect(1, 1, 68, 68, "#ffffff");
    }
    switch (card_index) {
        case 0:
            write_card_title(large, base, "Samurai");
            write_card_stats(large, base, 0, 6, 0);
            draw_card_image(large, base, 'samurai');
            break;
        case 1:
            write_card_title(large, base, "Daimyo");
            write_card_stats(large, base, 30, 1, 3);
            draw_card_image(large, base, 'daimyo_1');
            break;
        case 2:
            write_card_title(large, base, "Daimyo");
            write_card_stats(large, base, 20, 1, 5);
            draw_card_image(large, base, 'daimyo_2');
            break;
        case 3:
            write_card_title(large, base, "Daimyo");
            write_card_stats(large, base, 15, 3, 2);
            draw_card_image(large, base, 'daimyo_3');
            break;
        case 4:
            write_card_title(large, base, "Daimyo");
            write_card_stats(large, base, 15, 1, 3);
            draw_card_image(large, base, 'daimyo_3');
            break;
        case 5:
            write_card_title(large, base, "Daimyo");
            write_card_stats(large, base, 10, 0, 2);
            draw_card_image(large, base, 'daimyo_4');
            break;
        case 6:
            write_card_title(large, base, "Daimyo");
            write_card_stats(large, base, 5, 0, 1);
            draw_card_image(large, base, 'daimyo_5');
            break;
        case 7:
            write_card_title(large, base, "Okugata");
            write_card_stats(large, base, 10, 3, 0);
            draw_card_image(large, base, 'okugata_1');
            break;
        case 8:
            write_card_title(large, base, "Okugata");
            write_card_stats(large, base, 5, 4, 0);
            draw_card_image(large, base, 'okugata_2');
            break;
        case 9:
            write_card_title(large, base, "Ninja Spy");
            draw_card_image(large, base, 'ninja');
            break;
        case 10:
            write_card_title(large, base, "Elite Ninja", "Spy");
            draw_card_image(large, base, 'elite_ninja');
            break;
        case 11:
            write_card_title(large, base, "Ninja Assassin");
            draw_card_image(large, base, 'ninja_assassin');
            break;
        case 12:
            write_card_title(large, base, "Odwara Castle");
            write_card_stats(large, base, 5, 0, 3);
            draw_card_image(large, base, 'castle_1');
            break;
        case 13:
            write_card_title(large, base, "Osaka Castle");
            write_card_stats(large, base, 10, 1, 4);
            draw_card_image(large, base, 'castle_2');
            break;
        case 14:
            write_card_title(large, base, "Castle of the", "White Heron");
            write_card_stats(large, base, 15, 2, 5);
            draw_card_image(large, base, 'castle_3');
            break;
        case 15:
            write_card_title(large, base, "Dishonor");
            draw_card_image(large, base, 'dishonor');
            break;
        case 16:
            write_card_title(large, base, "Save Face");
            draw_card_image(large, base, 'save_face');
            break;
        case 17:
            write_card_title(large, base, "Army");
            write_card_stats(large, base, 0, 0, 1);
            draw_card_image(large, base, 'army_1');
            break;
        case 18:
            write_card_title(large, base, "Army");
            write_card_stats(large, base, 0, 0, 2);
            draw_card_image(large, base, 'army_2');
            break;
        case 19:
            write_card_title(large, base, "Army");
            write_card_stats(large, base, 0, 0, 3);
            draw_card_image(large, base, 'army_3');
            break;
        case 20:
            write_card_title(large, base, "Warrior Monk", "Army");
            write_card_stats(large, base, 0, 1, 1);
            draw_card_image(large, base, 'monk_army');
            break;
        case 21:
            write_card_title(large, base, "Ancestor's", "No-Dachi");
            write_card_stats(large, base, 5, 1, 3);
            draw_card_image(large, base, 'no_dachi');
            break;
        case 22:
            write_card_title(large, base, "Ancestor's", "Daisho");
            write_card_stats(large, base, 10, 1, 1);
            draw_card_image(large, base, 'daisho');
            break;
        case 23:
            write_card_title(large, base, "Swordsmith", "Masamune");
            write_card_stats(large, base, 10, 1, 4);
            draw_card_image(large, base, 'swordsmith');
            break;
        case 24:
            write_card_title(large, base, "Gunpowder", "Weapons");
            write_card_stats(large, base, -20, -2, 6);
            draw_card_image(large, base, 'gun');
            break;
        case 25:
            write_card_title(large, base, "Noh Theater");
            write_card_stats(large, base, 20, 3, 0);
            draw_card_image(large, base, 'noh_theater');
            break;
        case 26:
            write_card_title(large, base, "House Guard");
            draw_card_image(large, base, 'house_guard');
            break;
    }
    return base;
}

function write_card_title(large, base, title, title_2) {
    if (large) {
        base.text(title, 10, 36, "36px Times", "#000000", false);
        if (title_2) {
            base.text(title_2, 30, 72, "36px Times", "#000000", false);
        }
    } else {
        base.text(title, 2, 12, "12px Times", "#000000", false);
        if (title_2) {
            base.text(title_2, 7, 24, "12px Times", "#000000", false);
        }
    }
}

function write_card_stats(large, base, honor, ki, strength) {
    if (large) {
        base.circle(55, 110, 20, "#000000");
        base.circle(155, 110, 20, "#000000");
        base.circle(255, 110, 20, "#000000");

        base.text(honor, 42, 115, "24px Times", "#00ff00", false);
        base.text(ki, 150, 115, "24px Times", "#9999ff", false);
        base.text(strength, 250, 115, "24px Times", "#ff7777", false);
    } else {
        base.circle(9, 30, 7, "#000000");
        base.circle(9, 45, 7, "#000000");
        base.circle(9, 60, 7, "#000000");

        base.text(honor, 5, 33, "10px Times", "#00ff00", false);
        base.text(ki, 6, 48, "10px Times", "#9999ff", false);
        base.text(strength, 6, 63, "10px Times", "#ff7777", false);
    }
}

function draw_card_image(large, base, key) {
    if (large) {
        base.draw(key, 51, 152, 200, 200);
    } else {
        base.draw(key, 24, 32, 35, 35);
    }
}

function update() {

}

var HttpClient = function() {
    this.get = function(aUrl, aCallback) {
        var anHttpRequest = new XMLHttpRequest();
        anHttpRequest.onreadystatechange = function() {
            if (anHttpRequest.readyState == 4 && anHttpRequest.status == 200)
                console.log(anHttpRequest.responseText);
                aCallback(JSON.parse(anHttpRequest.responseText));
        }

        anHttpRequest.open( "GET", aUrl, true );
        anHttpRequest.send( null );
    }
}
