use pokecraft_bevy::pokemon::data::{DexRegistry, make_mon};
use pokecraft_bevy::pokemon::party::{PlayerParty, PARTY_MAX};
use pokecraft_bevy::pokemon::pc::{PcStorage, BOX_CAPACITY, TOTAL_BOXES};
use pokecraft_bevy::items::bag::{PlayerBag, PocketType, classify_item};
use pokecraft_bevy::ui::menu::{MenuManager, ActiveMenuScreen, StartMenuOption};

fn load_dex() -> DexRegistry {
    let paths = ["assets", "../assets"];
    for p in paths {
        if let Ok(dex) = DexRegistry::load_from_dir(p) {
            return dex;
        }
    }
    panic!("Could not load pokemon.json for Stage 6 tests");
}

#[test]
fn test_party_management_and_swapping() {
    let dex = load_dex();
    let mut party = PlayerParty::new();

    assert!(party.is_empty());
    assert_eq!(PARTY_MAX, 5, "Steve occupies 6th slot, so party max is 5");

    // Add 5 Pokemon
    let pika = make_mon("PIKACHU", 20, &dex).unwrap();
    let charm = make_mon("CHARMANDER", 18, &dex).unwrap();
    let squirt = make_mon("SQUIRTLE", 18, &dex).unwrap();
    let bulb = make_mon("BULBASAUR", 18, &dex).unwrap();
    let pidgey = make_mon("PIDGEY", 15, &dex).unwrap();

    assert!(party.add(pika));
    assert!(party.add(charm));
    assert!(party.add(squirt));
    assert!(party.add(bulb));
    assert!(party.add(pidgey));
    assert_eq!(party.len(), 5);
    assert!(party.is_full());

    // 6th Pokemon should be rejected (must go to PC)
    let extra = make_mon("CATERPIE", 5, &dex).unwrap();
    assert!(!party.add(extra), "Party cannot exceed PARTY_MAX (5)");

    // Test Swapping positions 0 (Pikachu) and 1 (Charmander)
    assert_eq!(party.members[0].species, "PIKACHU");
    assert_eq!(party.members[1].species, "CHARMANDER");

    assert!(party.swap(0, 1));
    assert_eq!(party.members[0].species, "CHARMANDER");
    assert_eq!(party.members[1].species, "PIKACHU");

    // Test Fainting and Healing
    party.members[0].hp = 0;
    assert_eq!(party.first_alive(), Some(1));
    assert!(!party.all_fainted());

    party.heal_all();
    assert_eq!(party.members[0].hp, party.members[0].max_hp);
    assert_eq!(party.first_alive(), Some(0));
}

#[test]
fn test_pc_storage_boxes_and_transfer() {
    let dex = load_dex();
    let mut pc = PcStorage::new();
    let mut party = PlayerParty::new();

    assert_eq!(pc.boxes.len(), TOTAL_BOXES);
    assert_eq!(TOTAL_BOXES, 14);
    assert_eq!(BOX_CAPACITY, 30);
    assert_eq!(pc.total_stored(), 0);

    // Give party 2 Pokemon
    let pika = make_mon("PIKACHU", 25, &dex).unwrap();
    let geo = make_mon("GEODUDE", 20, &dex).unwrap();
    party.add(pika);
    party.add(geo);

    // Deposit Pikachu from party into PC
    assert!(pc.deposit_from_party(&mut party, 0));
    assert_eq!(party.len(), 1);
    assert_eq!(party.members[0].species, "GEODUDE");
    assert_eq!(pc.total_stored(), 1);
    assert_eq!(pc.current().count(), 1);
    assert_eq!(pc.current().slots[0].as_ref().unwrap().species, "PIKACHU");

    // Withdraw Pikachu back into party
    assert!(pc.withdraw_to_party(&mut party, 0));
    assert_eq!(party.len(), 2);
    assert_eq!(party.members[1].species, "PIKACHU");
    assert_eq!(pc.total_stored(), 0);

    // Box switching
    assert_eq!(pc.current_box, 0);
    pc.next_box();
    assert_eq!(pc.current_box, 1);
    assert_eq!(pc.current().name, "BOX 2");
    pc.prev_box();
    assert_eq!(pc.current_box, 0);
    assert_eq!(pc.current().name, "BOX 1");
}

#[test]
fn test_item_bag_pockets_and_currency() {
    let mut bag = PlayerBag::new();

    // Starter funds: 3000 Yen
    assert_eq!(bag.money, 3000);
    assert!(bag.can_afford(1000));
    assert!(!bag.can_afford(5000));

    // Buy item for 1200 Yen
    assert!(bag.spend_money(1200));
    assert_eq!(bag.money, 1800);

    // Earn battle prize money: 800 Yen
    bag.add_money(800);
    assert_eq!(bag.money, 2600);

    // Test Pocket classification
    assert_eq!(classify_item("poke_ball"), PocketType::PokeBalls);
    assert_eq!(classify_item("ultra_ball"), PocketType::PokeBalls);
    assert_eq!(classify_item("oran_berry"), PocketType::Berries);
    assert_eq!(classify_item("bicycle"), PocketType::KeyItems);
    assert_eq!(classify_item("town_map"), PocketType::KeyItems);
    assert_eq!(classify_item("old_rod"), PocketType::KeyItems);
    assert_eq!(classify_item("iron_pickaxe"), PocketType::Items);
    assert_eq!(classify_item("potion"), PocketType::Items);
}

#[test]
fn test_menu_navigation_system() {
    let mut menu = MenuManager::default();

    assert_eq!(menu.screen, ActiveMenuScreen::None);
    assert!(!menu.is_open());

    // Open Start Menu
    menu.open_start_menu();
    assert_eq!(menu.screen, ActiveMenuScreen::StartMenu);
    assert_eq!(menu.start_cursor, 0);
    assert_eq!(StartMenuOption::ALL[menu.start_cursor], StartMenuOption::Pokedex);

    // Navigate Down
    menu.move_cursor(1, StartMenuOption::ALL.len());
    assert_eq!(menu.start_cursor, 1);
    assert_eq!(StartMenuOption::ALL[menu.start_cursor], StartMenuOption::Pokemon);

    // Open Party Screen
    menu.open_party();
    assert_eq!(menu.screen, ActiveMenuScreen::PartyScreen);

    // Close Menu
    menu.close();
    assert_eq!(menu.screen, ActiveMenuScreen::None);
}
