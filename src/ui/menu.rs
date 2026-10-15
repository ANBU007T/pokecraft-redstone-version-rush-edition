use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StartMenuOption {
    #[default]
    Pokedex,
    Pokemon,
    Bag,
    TrainerCard,
    Save,
    Option,
    Exit,
}

impl StartMenuOption {
    pub const ALL: [StartMenuOption; 7] = [
        StartMenuOption::Pokedex,
        StartMenuOption::Pokemon,
        StartMenuOption::Bag,
        StartMenuOption::TrainerCard,
        StartMenuOption::Save,
        StartMenuOption::Option,
        StartMenuOption::Exit,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            StartMenuOption::Pokedex => "POKéDEX",
            StartMenuOption::Pokemon => "POKéMON",
            StartMenuOption::Bag => "BAG",
            StartMenuOption::TrainerCard => "TRAINER",
            StartMenuOption::Save => "SAVE",
            StartMenuOption::Option => "OPTION",
            StartMenuOption::Exit => "EXIT",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActiveMenuScreen {
    #[default]
    None,
    StartMenu,
    PartyScreen,
    PcScreen,
    MartScreen,
}

#[derive(Resource, Default)]
pub struct MenuManager {
    pub screen: ActiveMenuScreen,
    pub start_cursor: usize,
    pub party_cursor: usize,
    pub pc_mode: usize, // 0: Withdraw, 1: Deposit, 2: Exit
    pub mart_mode: usize, // 0: Buy, 1: Sell, 2: Exit
}

impl MenuManager {
    pub fn is_open(&self) -> bool {
        self.screen != ActiveMenuScreen::None
    }

    pub fn open_start_menu(&mut self) {
        self.screen = ActiveMenuScreen::StartMenu;
        self.start_cursor = 0;
    }

    pub fn open_party(&mut self) {
        self.screen = ActiveMenuScreen::PartyScreen;
        self.party_cursor = 0;
    }

    pub fn open_pc(&mut self) {
        self.screen = ActiveMenuScreen::PcScreen;
        self.pc_mode = 0;
    }

    pub fn open_mart(&mut self) {
        self.screen = ActiveMenuScreen::MartScreen;
        self.mart_mode = 0;
    }

    pub fn close(&mut self) {
        self.screen = ActiveMenuScreen::None;
    }

    pub fn move_cursor(&mut self, delta: i32, max: usize) {
        let cur = self.start_cursor as i32 + delta;
        self.start_cursor = if cur < 0 {
            max - 1
        } else if cur >= max as i32 {
            0
        } else {
            cur as usize
        };
    }
}

pub fn menu_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut menu: ResMut<MenuManager>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        if menu.screen == ActiveMenuScreen::None {
            menu.open_start_menu();
        } else if menu.screen == ActiveMenuScreen::StartMenu {
            menu.close();
        }
    } else if keyboard_input.just_pressed(KeyCode::Escape) {
        if menu.is_open() {
            menu.close();
        }
    }

    if menu.screen == ActiveMenuScreen::StartMenu {
        if keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW) {
            menu.move_cursor(-1, StartMenuOption::ALL.len());
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) || keyboard_input.just_pressed(KeyCode::KeyS) {
            menu.move_cursor(1, StartMenuOption::ALL.len());
        }
        if keyboard_input.just_pressed(KeyCode::KeyZ) || keyboard_input.just_pressed(KeyCode::Space) {
            let selected = StartMenuOption::ALL[menu.start_cursor];
            match selected {
                StartMenuOption::Pokemon => menu.open_party(),
                StartMenuOption::Exit => menu.close(),
                _ => {}
            }
        }
    }
}
