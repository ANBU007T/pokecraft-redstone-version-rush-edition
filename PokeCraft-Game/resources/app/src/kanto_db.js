// Complete Kanto Region 51 Locations Database
// Overworld coordinates mathematically synchronized with FireRed outdoor tiles (408 x 400)
export const KANTO_LOCATIONS = [
  {
    "id": "pallettown",
    "name": "Pallet Town",
    "type": "city",
    "csvType": "City",
    "connects": "Route 1; Route 21 (Surf)",
    "highlights": "Starting town. Professor Oak's Lab, player house, rival house.",
    "gym": "No Gym (Starter Town)",
    "pokemon": [
      "Bulbasaur (Gift)",
      "Charmander (Gift)",
      "Squirtle (Gift)",
      "Poliwag (Fish)",
      "Tentacool (Surf)"
    ],
    "pois": [
      "Prof. Oak's Lab",
      "Steve's House",
      "Daisy Oak (Town Map)"
    ],
    "items": [
      "Town Map",
      "Poké Balls x5",
      "Super Potion"
    ],
    "wx": 66,
    "wy": 268,
    "sx": 162,
    "sy": 670
  },
  {
    "id": "route1",
    "name": "Route 1",
    "type": "route",
    "csvType": "Route",
    "connects": "Pallet Town; Viridian City",
    "highlights": "Early wild Pokemon and ledges. Short path to Viridian.",
    "gym": "",
    "pokemon": [
      "Pidgey (Lv.2-5)",
      "Rattata (Lv.2-4)"
    ],
    "pois": [
      "Mart Delivery Clerk",
      "Tall Grass Fields"
    ],
    "items": [
      "Potion (Free sample)"
    ],
    "wx": 72,
    "wy": 240,
    "sx": 176,
    "sy": 600
  },
  {
    "id": "viridiancity",
    "name": "Viridian City",
    "type": "city",
    "csvType": "City",
    "connects": "Route 1; Route 2; Route 22",
    "highlights": "Poke Mart (Oak's Parcel), Trainer School, Pokemon Center. Gym (Giovanni, Earth Badge) opens after you have 7 badges.",
    "gym": "Gym 8: Giovanni (Earth Badge · Ground)",
    "pokemon": [
      "Poliwag (Pond)",
      "Tentacool (Surf)",
      "Goldeen (Fish)"
    ],
    "pois": [
      "Viridian Gym",
      "Pokémon Center",
      "Poké Mart",
      "Trainer Academy",
      "Old Man (Teachy TV)"
    ],
    "items": [
      "Oak's Parcel",
      "Teachy TV",
      "TM27 Return",
      "Earth Badge"
    ],
    "wx": 74,
    "wy": 207,
    "sx": 181,
    "sy": 518
  },
  {
    "id": "route22",
    "name": "Route 22",
    "type": "route",
    "csvType": "Route",
    "connects": "Viridian City; Route 23",
    "highlights": "Rival battles. West of Viridian toward the Pokemon League.",
    "gym": "Rival Battle #2 Checkpoint",
    "pokemon": [
      "Mankey (Lv.3-5)",
      "Spearow (Lv.3-5)",
      "Nidoran♀",
      "Nidoran♂",
      "Rattata"
    ],
    "pois": [
      "Pokémon League Front Gate",
      "Rival Battle Area"
    ],
    "items": [
      "Potion"
    ],
    "wx": 24,
    "wy": 202,
    "sx": 59,
    "sy": 505
  },
  {
    "id": "route23",
    "name": "Route 23",
    "type": "route",
    "csvType": "Route",
    "connects": "Route 22; Victory Road",
    "highlights": "Badge-check gates; you need all 8 badges to pass.",
    "gym": "8 Badge Checkpoint Gates",
    "pokemon": [
      "Arbok",
      "Sandslash",
      "Primeape",
      "Fearow",
      "Ditto",
      "Poliwhirl"
    ],
    "pois": [
      "8 Official Badge Checkpoints",
      "Water Maze"
    ],
    "items": [
      "Max Ether",
      "Ultra Ball",
      "Full Restore"
    ],
    "wx": 12,
    "wy": 110,
    "sx": 29,
    "sy": 275
  },
  {
    "id": "victoryroad",
    "name": "Victory Road",
    "type": "dungeon",
    "csvType": "Cave",
    "connects": "Route 23; Indigo Plateau",
    "highlights": "Final dungeon before the Pokemon League. Moltres is found here in FireRed/LeafGreen.",
    "gym": "Legendary Moltres Lair",
    "pokemon": [
      "Moltres (Lv.50)",
      "Onix",
      "Machoke",
      "Golbat",
      "Graveler",
      "Marowak"
    ],
    "pois": [
      "Strength Boulder Puzzles",
      "Moltres Perch",
      "Elite Gauntlet"
    ],
    "items": [
      "TM02 Dragon Claw",
      "TM37 Sandstorm",
      "TM07 Hail",
      "Rare Candy"
    ],
    "wx": 12,
    "wy": 50,
    "sx": 29,
    "sy": 125
  },
  {
    "id": "indigoplateau",
    "name": "Indigo Plateau",
    "type": "special",
    "csvType": "Special",
    "connects": "Victory Road",
    "highlights": "Pokemon League: Elite Four (Lorelei, Bruno, Agatha, Lance) then the Champion (your rival).",
    "gym": "Pokémon League: Elite Four & Champion",
    "pokemon": [
      "Elite Four: Lorelei, Bruno, Agatha, Lance, Champion Blue"
    ],
    "pois": [
      "League Main Hall",
      "Pokémon Center & Mart",
      "Hall of Fame"
    ],
    "items": [
      "Champion Title",
      "Hall of Fame Honor",
      "National Dex (Post-game)"
    ],
    "wx": 11,
    "wy": 17,
    "sx": 27,
    "sy": 43
  },
  {
    "id": "route2south",
    "name": "Route 2 (south)",
    "type": "route",
    "csvType": "Route",
    "connects": "Viridian City; Viridian Forest; Diglett's Cave",
    "highlights": "Cut tree and a gatehouse. Diglett's Cave entrance on this side.",
    "gym": "",
    "pokemon": [
      "Pidgey",
      "Rattata",
      "Caterpie",
      "Weedle"
    ],
    "pois": [
      "Cut Tree Path",
      "Diglett's Cave South Entrance",
      "Gatehouse"
    ],
    "items": [
      "Moon Stone",
      "Ether"
    ],
    "wx": 72,
    "wy": 160,
    "sx": 176,
    "sy": 400
  },
  {
    "id": "diglettscave",
    "name": "Diglett's Cave",
    "type": "dungeon",
    "csvType": "Cave",
    "connects": "Route 2; Route 11",
    "highlights": "Tunnel between Route 2 and Route 11. Diglett and Dugtrio.",
    "gym": "",
    "pokemon": [
      "Diglett (Lv.15-22)",
      "Dugtrio (Lv.29-31 Rare)"
    ],
    "pois": [
      "Route 2 to Route 11 Shortcut Tunnel",
      "Oak's Aide (Flash)"
    ],
    "items": [
      "HM05 Flash (Oak's Aide)",
      "Max Revive"
    ],
    "wx": 72,
    "wy": 145,
    "sx": 176,
    "sy": 363
  },
  {
    "id": "viridianforest",
    "name": "Viridian Forest",
    "type": "dungeon",
    "csvType": "Forest",
    "connects": "Route 2 (south); Route 2 (north)",
    "highlights": "Bug-type trainers and Pokemon. Pikachu can appear (FireRed/LeafGreen).",
    "gym": "",
    "pokemon": [
      "Pikachu (Rare 5%)",
      "Caterpie",
      "Metapod",
      "Weedle",
      "Kakuna",
      "Pidgey"
    ],
    "pois": [
      "Bug Catcher Gauntlet",
      "Hidden Clearings"
    ],
    "items": [
      "Poké Ball",
      "Antidote",
      "Potion"
    ],
    "wx": 72,
    "wy": 120,
    "sx": 176,
    "sy": 300
  },
  {
    "id": "route2north",
    "name": "Route 2 (north)",
    "type": "route",
    "csvType": "Route",
    "connects": "Viridian Forest; Pewter City",
    "highlights": "Leads to Pewter City.",
    "gym": "",
    "pokemon": [
      "Pidgey",
      "Nidoran♀",
      "Nidoran♂"
    ],
    "pois": [
      "North Gatehouse to Pewter City"
    ],
    "items": [
      "Potion"
    ],
    "wx": 72,
    "wy": 90,
    "sx": 176,
    "sy": 225
  },
  {
    "id": "pewtercity",
    "name": "Pewter City",
    "type": "city",
    "csvType": "City",
    "connects": "Route 2; Route 3",
    "highlights": "Gym (Brock, Boulder Badge). Museum of Science.",
    "gym": "Gym 1: Brock (Boulder Badge · Rock)",
    "pokemon": [
      "Geodude (Rock Smash)"
    ],
    "pois": [
      "Pewter Gym",
      "Pewter Museum of Science",
      "Pokémon Center",
      "Poké Mart"
    ],
    "items": [
      "Boulder Badge",
      "TM39 Rock Tomb",
      "Running Shoes",
      "Old Amber"
    ],
    "wx": 65,
    "wy": 86,
    "sx": 159,
    "sy": 215
  },
  {
    "id": "route3",
    "name": "Route 3",
    "type": "route",
    "csvType": "Route",
    "connects": "Pewter City; Mt. Moon",
    "highlights": "Trainers and ledges on the way to Mt. Moon.",
    "gym": "8 Trainers Gauntlet",
    "pokemon": [
      "Pidgey",
      "Spearow",
      "Jigglypuff (Rare 10%)",
      "Mankey"
    ],
    "pois": [
      "Mountain Foothills",
      "Route 3 Trainers Camp"
    ],
    "items": [
      "Oran Berry",
      "Persim Berry"
    ],
    "wx": 138,
    "wy": 80,
    "sx": 338,
    "sy": 200
  },
  {
    "id": "mtmoon",
    "name": "Mt. Moon",
    "type": "dungeon",
    "csvType": "Cave",
    "connects": "Route 3; Route 4",
    "highlights": "Fossils (Helix / Dome), Moon Stones, Clefairy, Zubat.",
    "gym": "Team Rocket Cavern Encounter",
    "pokemon": [
      "Clefairy (Rare 8%)",
      "Zubat",
      "Geodude",
      "Paras"
    ],
    "pois": [
      "Fossil Nerd Super Nerd",
      "Team Rocket Ambush",
      "Moon Stone Chamber"
    ],
    "items": [
      "Helix Fossil / Dome Fossil",
      "Moon Stone x2",
      "TM09 Bullet Seed",
      "TM46 Thief"
    ],
    "wx": 156,
    "wy": 60,
    "sx": 382,
    "sy": 150
  },
  {
    "id": "route4",
    "name": "Route 4",
    "type": "route",
    "csvType": "Route",
    "connects": "Mt. Moon; Cerulean City",
    "highlights": "Leads out of Mt. Moon to Cerulean.",
    "gym": "",
    "pokemon": [
      "Rattata",
      "Spearow",
      "Ekans",
      "Sandshrew"
    ],
    "pois": [
      "Route 4 Pokémon Center",
      "Magikarp Salesman (500¥)",
      "Move Tutors (Mega Punch/Kick)"
    ],
    "items": [
      "Magikarp (Lv.5)",
      "TM05 Roar",
      "Great Ball"
    ],
    "wx": 210,
    "wy": 60,
    "sx": 515,
    "sy": 150
  },
  {
    "id": "ceruleancity",
    "name": "Cerulean City",
    "type": "city",
    "csvType": "City",
    "connects": "Route 4; Route 5; Route 9; Route 24; Cerulean Cave",
    "highlights": "Gym (Misty, Cascade Badge). Bike Shop. Starting point for Nugget Bridge.",
    "gym": "Gym 2: Misty (Cascade Badge · Water)",
    "pokemon": [
      "Psyduck (Surf)",
      "Poliwag",
      "Goldeen",
      "Magikarp"
    ],
    "pois": [
      "Cerulean Gym",
      "Cerulean Bicycle Shop",
      "Burglarized House",
      "Berry Powder Man"
    ],
    "items": [
      "Cascade Badge",
      "TM03 Water Pulse",
      "Bicycle Voucher",
      "Powder Jar"
    ],
    "wx": 286,
    "wy": 60,
    "sx": 701,
    "sy": 150
  },
  {
    "id": "route24",
    "name": "Route 24",
    "type": "route",
    "csvType": "Route",
    "connects": "Cerulean City; Route 25",
    "highlights": "Nugget Bridge: a chain of trainers with a Nugget as the reward.",
    "gym": "5 Trainer Gauntlet + Rocket Recruiter",
    "pokemon": [
      "Abra (Teleport)",
      "Bellsprout",
      "Oddish",
      "Caterpie",
      "Metapod"
    ],
    "pois": [
      "Nugget Bridge",
      "Team Rocket Agent"
    ],
    "items": [
      "Nugget",
      "TM45 Attract"
    ],
    "wx": 288,
    "wy": 20,
    "sx": 706,
    "sy": 50
  },
  {
    "id": "route25",
    "name": "Route 25",
    "type": "route",
    "csvType": "Route",
    "connects": "Route 24; Bill's Sea Cottage",
    "highlights": "Leads to Bill's cottage.",
    "gym": "",
    "pokemon": [
      "Abra",
      "Bellsprout",
      "Oddish",
      "Pidgey",
      "Krabby"
    ],
    "pois": [
      "Scenic Coastal Trail",
      "Trainer Gauntlet"
    ],
    "items": [
      "TM43 Secret Power",
      "Potion"
    ],
    "wx": 336,
    "wy": 10,
    "sx": 824,
    "sy": 25
  },
  {
    "id": "billsseacottage",
    "name": "Bill's Sea Cottage",
    "type": "special",
    "csvType": "Special",
    "connects": "Route 25",
    "highlights": "Meet Bill; reward is the S.S. Ticket.",
    "gym": "Pokémaniac Bill Research Lab",
    "pokemon": [
      "Eevee (Data on PC)",
      "Clefairy (Transform Test)"
    ],
    "pois": [
      "Teleporter Experiment",
      "Bill's PC Database",
      "Sea View Deck"
    ],
    "items": [
      "S.S. Ticket (Board S.S. Anne)",
      "Eeveelution Pokédex Data"
    ],
    "wx": 360,
    "wy": 10,
    "sx": 882,
    "sy": 25
  },
  {
    "id": "ceruleancave",
    "name": "Cerulean Cave",
    "type": "dungeon",
    "csvType": "Cave (post-game)",
    "connects": "Cerulean City",
    "highlights": "Endgame cave with the strongest wild Pokemon, including Mewtwo. Needs Surf.",
    "gym": "Legendary Mewtwo Lair (Post-game)",
    "pokemon": [
      "Mewtwo (Lv.70)",
      "Ditto (Lv.50-60)",
      "Raichu",
      "Wobbuffet",
      "Parasect",
      "Magneton"
    ],
    "pois": [
      "Endgame Labyrinth",
      "Mewtwo Platform",
      "Subterranean Lake"
    ],
    "items": [
      "Mewtwo Catch",
      "Ultra Ball",
      "Max Elixir",
      "PP Up"
    ],
    "wx": 260,
    "wy": 35,
    "sx": 637,
    "sy": 88
  },
  {
    "id": "route5",
    "name": "Route 5",
    "type": "route",
    "csvType": "Route",
    "connects": "Cerulean City; Saffron City; Underground Path",
    "highlights": "Pokemon Day Care is on this route.",
    "gym": "",
    "pokemon": [
      "Meowth",
      "Pidgey",
      "Bellsprout",
      "Oddish"
    ],
    "pois": [
      "Pokémon Day Care",
      "Underground Path Entrance"
    ],
    "items": [
      "Day Care Leveling"
    ],
    "wx": 288,
    "wy": 100,
    "sx": 706,
    "sy": 250
  },
  {
    "id": "undergroundpath",
    "name": "Underground Path",
    "type": "special",
    "csvType": "Special",
    "connects": "Route 5 <-> Route 6; Route 7 <-> Route 8",
    "highlights": "Underground passages that link the north/south and east/west routes.",
    "gym": "",
    "pokemon": [
      "No wild Pokémon (Pedestrian tunnel)"
    ],
    "pois": [
      "Route 5 <-> 6 North-South Tunnel",
      "Route 7 <-> 8 East-West Tunnel"
    ],
    "items": [
      "Hidden Antidote",
      "Hidden Parlyz Heal"
    ],
    "wx": 288,
    "wy": 140,
    "sx": 706,
    "sy": 350
  },
  {
    "id": "saffroncity",
    "name": "Saffron City",
    "type": "city",
    "csvType": "City",
    "connects": "Routes 5, 6, 7, 8",
    "highlights": "Gym (Sabrina, Marsh Badge). Silph Co. headquarters (Team Rocket takeover). Fighting Dojo.",
    "gym": "Gym 6: Sabrina (Marsh Badge · Psychic)",
    "pokemon": [
      "Hitmonlee / Hitmonchan (Fighting Dojo Gift)",
      "Lapras (Silph Co. Gift)"
    ],
    "pois": [
      "Saffron Gym (Warp tiles)",
      "Fighting Dojo",
      "Silph Co. 11-Floor Tower",
      "Copycat's House",
      "Mr. Psychic"
    ],
    "items": [
      "Marsh Badge",
      "TM04 Calm Mind",
      "Master Ball",
      "Hitmonlee/chan",
      "Lapras",
      "TM29 Psychic"
    ],
    "wx": 288,
    "wy": 159,
    "sx": 706,
    "sy": 398
  },
  {
    "id": "route6",
    "name": "Route 6",
    "type": "route",
    "csvType": "Route",
    "connects": "Saffron City; Vermilion City",
    "highlights": "Short route south to Vermilion.",
    "gym": "",
    "pokemon": [
      "Meowth",
      "Pidgey",
      "Bellsprout",
      "Oddish",
      "Psyduck"
    ],
    "pois": [
      "Water Pond",
      "Underground Path South Entrance"
    ],
    "items": [
      "Sitrus Berry"
    ],
    "wx": 288,
    "wy": 195,
    "sx": 706,
    "sy": 488
  },
  {
    "id": "vermilioncity",
    "name": "Vermilion City",
    "type": "city",
    "csvType": "City",
    "connects": "Route 6; Route 11",
    "highlights": "Gym (Lt. Surge, Thunder Badge). S.S. Anne dock (HM Cut). Pokemon Fan Club.",
    "gym": "Gym 3: Lt. Surge (Thunder Badge · Electric)",
    "pokemon": [
      "Farfetch'd (In-game Trade)",
      "Poliwag",
      "Shellder",
      "Krabby"
    ],
    "pois": [
      "Vermilion Gym (Trash Cans)",
      "S.S. Anne Luxury Cruise Dock",
      "Pokémon Fan Club (Bike Voucher)",
      "Old Rod Fisherman"
    ],
    "items": [
      "Thunder Badge",
      "TM34 Shock Wave",
      "Bike Voucher",
      "Old Rod",
      "HM01 Cut (from S.S. Anne Captain)"
    ],
    "wx": 279,
    "wy": 222,
    "sx": 684,
    "sy": 555
  },
  {
    "id": "route11",
    "name": "Route 11",
    "type": "route",
    "csvType": "Route",
    "connects": "Vermilion City; Route 12; Diglett's Cave",
    "highlights": "Gatehouse where Oak's aide rewards Pokedex progress (Itemfinder).",
    "gym": "",
    "pokemon": [
      "Drowzee",
      "Ekans",
      "Sandshrew",
      "Spearow"
    ],
    "pois": [
      "East Gatehouse",
      "Oak's Aide (Itemfinder - 30 Mons)"
    ],
    "items": [
      "Itemfinder (Oak's Aide)",
      "Awakening",
      "Great Ball"
    ],
    "wx": 348,
    "wy": 220,
    "sx": 853,
    "sy": 550
  },
  {
    "id": "route7",
    "name": "Route 7",
    "type": "route",
    "csvType": "Route",
    "connects": "Saffron City; Celadon City",
    "highlights": "Short link between Saffron and Celadon.",
    "gym": "",
    "pokemon": [
      "Growlithe",
      "Vulpix",
      "Meowth",
      "Bellsprout",
      "Oddish"
    ],
    "pois": [
      "Underground Path West Entrance",
      "Celadon East Gate"
    ],
    "items": [
      "Weepinbell"
    ],
    "wx": 252,
    "wy": 140,
    "sx": 618,
    "sy": 350
  },
  {
    "id": "celadoncity",
    "name": "Celadon City",
    "type": "city",
    "csvType": "City",
    "connects": "Route 7; Route 16; Rocket Hideout",
    "highlights": "Gym (Erika, Rainbow Badge). Game Corner, Department Store, Celadon Mansion.",
    "gym": "Gym 4: Erika (Rainbow Badge · Grass)",
    "pokemon": [
      "Eevee (Mansion Roof Gift)",
      "Porygon (Game Corner Prize)",
      "Dratini (Prize)",
      "Koffing (Pond)"
    ],
    "pois": [
      "Celadon Gym (Cut tree)",
      "Rocket Game Corner",
      "Celadon Department Store",
      "Celadon Mansion (Game Freak)",
      "Diner (Coin Case)"
    ],
    "items": [
      "Rainbow Badge",
      "TM19 Giga Drain",
      "Eevee",
      "Coin Case",
      "Tea (Gatehouse Guard)"
    ],
    "wx": 228,
    "wy": 132,
    "sx": 559,
    "sy": 330
  },
  {
    "id": "rockethideout",
    "name": "Rocket Hideout",
    "type": "special",
    "csvType": "Special",
    "connects": "Celadon Game Corner",
    "highlights": "Team Rocket base under the Game Corner. Silph Scope is found here.",
    "gym": "Team Rocket Underground HQ (Boss Giovanni)",
    "pokemon": [
      "Koffing",
      "Raticate",
      "Zubat",
      "Drowzee",
      "Machop"
    ],
    "pois": [
      "Spinning Arrow Floor Mazes",
      "Elevator Key Puzzle",
      "Giovanni's Office"
    ],
    "items": [
      "Silph Scope (Reveals Ghosts)",
      "Lift Key",
      "TM49 Snatch",
      "Rare Candy",
      "HP Up"
    ],
    "wx": 210,
    "wy": 140,
    "sx": 515,
    "sy": 350
  },
  {
    "id": "route8",
    "name": "Route 8",
    "type": "route",
    "csvType": "Route",
    "connects": "Saffron City; Lavender Town",
    "highlights": "Link between Saffron and Lavender (Underground Path entrance).",
    "gym": "",
    "pokemon": [
      "Growlithe",
      "Vulpix",
      "Kadabra",
      "Meowth",
      "Pidgeotto"
    ],
    "pois": [
      "Underground Path East Entrance",
      "Saffron East Gate"
    ],
    "items": [
      "Rawst Berry",
      "Lum Berry"
    ],
    "wx": 366,
    "wy": 140,
    "sx": 897,
    "sy": 350
  },
  {
    "id": "route9",
    "name": "Route 9",
    "type": "route",
    "csvType": "Route",
    "connects": "Cerulean City; Route 10",
    "highlights": "Cut tree. Leads east to Rock Tunnel.",
    "gym": "",
    "pokemon": [
      "Spearow",
      "Fearow",
      "Rattata",
      "Raticate",
      "Ekans"
    ],
    "pois": [
      "Rocky Ledges",
      "Stream to Power Plant"
    ],
    "items": [
      "TM40 Aerial Ace",
      "Burn Heal"
    ],
    "wx": 348,
    "wy": 60,
    "sx": 853,
    "sy": 150
  },
  {
    "id": "route10",
    "name": "Route 10",
    "type": "route",
    "csvType": "Route",
    "connects": "Route 9; Rock Tunnel; Lavender Town; Power Plant",
    "highlights": "Rock Tunnel entrance in the north; Power Plant reached across water.",
    "gym": "",
    "pokemon": [
      "Voltorb",
      "Magnemite",
      "Spearow",
      "Ekans"
    ],
    "pois": [
      "Rock Tunnel North & South Entrances",
      "Route 10 Pokémon Center",
      "Power Plant Waterway"
    ],
    "items": [
      "Super Potion",
      "Cheri Berry"
    ],
    "wx": 396,
    "wy": 90,
    "sx": 971,
    "sy": 225
  },
  {
    "id": "rocktunnel",
    "name": "Rock Tunnel",
    "type": "dungeon",
    "csvType": "Cave",
    "connects": "Route 10 (north); Route 10 (south)",
    "highlights": "Dark cave; Flash makes it much easier.",
    "gym": "Pitch-Black Mountain Cavern",
    "pokemon": [
      "Onix",
      "Geodude",
      "Graveler",
      "Zubat",
      "Machop",
      "Kangaskhan"
    ],
    "pois": [
      "Multi-floor dark maze (Use Flash)",
      "Hiker & Picnicker Gauntlet"
    ],
    "items": [
      "Revive",
      "Escape Rope",
      "Max Ether",
      "Pearl"
    ],
    "wx": 396,
    "wy": 70,
    "sx": 971,
    "sy": 175
  },
  {
    "id": "powerplant",
    "name": "Power Plant",
    "type": "dungeon",
    "csvType": "Cave",
    "connects": "Route 10",
    "highlights": "Abandoned power plant. Zapdos is found here.",
    "gym": "Legendary Zapdos Lair",
    "pokemon": [
      "Zapdos (Lv.50)",
      "Electabuzz",
      "Voltorb / Electrode (Item Traps)",
      "Magnemite",
      "Magneton",
      "Pikachu"
    ],
    "pois": [
      "Abandoned Generator Complex",
      "Electrode Fake Item Balls",
      "Zapdos Generator Room"
    ],
    "items": [
      "Zapdos Catch",
      "TM25 Thunder",
      "Thunder Stone",
      "Max Elixir"
    ],
    "wx": 384,
    "wy": 100,
    "sx": 941,
    "sy": 250
  },
  {
    "id": "lavendertown",
    "name": "Lavender Town",
    "type": "city",
    "csvType": "City",
    "connects": "Route 8; Route 10; Route 12",
    "highlights": "Pokemon Tower (ghost-type area), Mr. Fuji's house, Name Rater.",
    "gym": "The Noble Purple Town of Spirits",
    "pokemon": [
      "Gastly",
      "Haunter",
      "Cubone"
    ],
    "pois": [
      "Pokémon Tower (7 Floors)",
      "Mr. Fuji's Volunteer House",
      "Name Rater"
    ],
    "items": [
      "Poké Flute (from Mr. Fuji)",
      "Cleanse Tag",
      "Soothe Bell"
    ],
    "wx": 390,
    "wy": 136,
    "sx": 956,
    "sy": 340
  },
  {
    "id": "route12",
    "name": "Route 12",
    "type": "route",
    "csvType": "Route",
    "connects": "Lavender Town; Route 11; Route 13",
    "highlights": "A sleeping Snorlax blocks the path until you have the Poke Flute. Fishing Guru gives the Super Rod.",
    "gym": "Sleeping Snorlax Checkpoint #1",
    "pokemon": [
      "Snorlax (Lv.30)",
      "Oddish",
      "Gloom",
      "Bellsprout",
      "Venonat"
    ],
    "pois": [
      "Sleeping Snorlax (Wake with Poké Flute)",
      "Fishing Guru's House (Super Rod)"
    ],
    "items": [
      "Super Rod (from Fishing Guru)",
      "Leftovers (under Snorlax)",
      "TM48 Skill Swap",
      "Iron"
    ],
    "wx": 396,
    "wy": 210,
    "sx": 971,
    "sy": 525
  },
  {
    "id": "route13",
    "name": "Route 13",
    "type": "route",
    "csvType": "Route",
    "connects": "Route 12; Route 14",
    "highlights": "Coastal route heading west toward Fuchsia.",
    "gym": "Maze of Fences & Trainers",
    "pokemon": [
      "Ditto",
      "Farfetch'd",
      "Pidgeotto",
      "Venonat",
      "Tentacool"
    ],
    "pois": [
      "Maze Hedge Trails",
      "Ocean Boardwalk"
    ],
    "items": [
      "PP Up",
      "Hidden Calcium"
    ],
    "wx": 372,
    "wy": 280,
    "sx": 912,
    "sy": 700
  },
  {
    "id": "route14",
    "name": "Route 14",
    "type": "route",
    "csvType": "Route",
    "connects": "Route 13; Route 15",
    "highlights": "Continues west.",
    "gym": "",
    "pokemon": [
      "Ditto",
      "Venonat",
      "Venomoth",
      "Pidgeotto",
      "Oddish"
    ],
    "pois": [
      "Coastal Biker Gauntlet"
    ],
    "items": [
      "Pinap Berry"
    ],
    "wx": 324,
    "wy": 300,
    "sx": 794,
    "sy": 750
  },
  {
    "id": "route15",
    "name": "Route 15",
    "type": "route",
    "csvType": "Route",
    "connects": "Route 14; Fuchsia City",
    "highlights": "Gatehouse where Oak's aide rewards more Pokedex progress (Exp. Share).",
    "gym": "",
    "pokemon": [
      "Pidgeotto",
      "Venonat",
      "Bellsprout",
      "Weepinbell",
      "Ditto"
    ],
    "pois": [
      "West Gatehouse to Fuchsia",
      "Oak's Aide (Exp. Share - 50 Mons)"
    ],
    "items": [
      "Exp. Share (Oak's Aide)",
      "TM18 Rain Dance"
    ],
    "wx": 276,
    "wy": 320,
    "sx": 676,
    "sy": 800
  },
  {
    "id": "route16",
    "name": "Route 16",
    "type": "route",
    "csvType": "Route",
    "connects": "Celadon City; Route 17",
    "highlights": "Another sleeping Snorlax blocks the way; house with HM Fly nearby.",
    "gym": "Sleeping Snorlax Checkpoint #2",
    "pokemon": [
      "Snorlax (Lv.30)",
      "Spearow",
      "Fearow",
      "Raticate",
      "Doduo"
    ],
    "pois": [
      "Sleeping Snorlax Gate",
      "Secret House through Cut Tree (HM02 Fly)"
    ],
    "items": [
      "HM02 Fly (from Secret Girl)",
      "Leftovers (under Snorlax)"
    ],
    "wx": 156,
    "wy": 140,
    "sx": 382,
    "sy": 350
  },
  {
    "id": "route17",
    "name": "Route 17",
    "type": "route",
    "csvType": "Route",
    "connects": "Route 16; Route 18",
    "highlights": "Cycling Road: Bicycle required, fast downhill route with many trainers.",
    "gym": "Cycling Road Speedway Gauntlet",
    "pokemon": [
      "Doduo",
      "Dodrio",
      "Fearow",
      "Raticate",
      "Ponyta"
    ],
    "pois": [
      "Steep Downhill Bicycle Slope",
      "Biker & Cue Ball Gangs",
      "Scenic Ocean Bridges"
    ],
    "items": [
      "Hidden Rare Candy",
      "Hidden Max Elixir",
      "Hidden PP Up"
    ],
    "wx": 144,
    "wy": 230,
    "sx": 353,
    "sy": 575
  },
  {
    "id": "route18",
    "name": "Route 18",
    "type": "route",
    "csvType": "Route",
    "connects": "Route 17; Fuchsia City",
    "highlights": "Southern end of Cycling Road.",
    "gym": "Cycling Road South Gate",
    "pokemon": [
      "Doduo",
      "Fearow",
      "Raticate"
    ],
    "pois": [
      "Cycling Road Inspection Gate",
      "Lignite Lookouts"
    ],
    "items": [
      "In-game Lickitung Trade"
    ],
    "wx": 162,
    "wy": 320,
    "sx": 397,
    "sy": 800
  },
  {
    "id": "fuchsiacity",
    "name": "Fuchsia City",
    "type": "city",
    "csvType": "City",
    "connects": "Route 15; Route 18; Safari Zone; Route 19",
    "highlights": "Gym (Koga, Soul Badge). Safari Zone, Warden's house.",
    "gym": "Gym 5: Koga (Soul Badge · Poison)",
    "pokemon": [
      "Magikarp",
      "Gyarados",
      "Poliwag",
      "Poliwhirl",
      "Seaking"
    ],
    "pois": [
      "Fuchsia Gym (Invisible Walls)",
      "Safari Zone Entrance",
      "Safari Warden's House (HM Strength)",
      "Good Rod Fisherman"
    ],
    "items": [
      "Soul Badge",
      "TM06 Toxic",
      "HM04 Strength (Gold Teeth reward)",
      "Good Rod"
    ],
    "wx": 217,
    "wy": 332,
    "sx": 532,
    "sy": 830
  },
  {
    "id": "safarizone",
    "name": "Safari Zone",
    "type": "special",
    "csvType": "Special",
    "connects": "Fuchsia City",
    "highlights": "Catch rare Pokemon with Safari Balls. Hidden items include HM Surf and HM Strength.",
    "gym": "Exclusive Wildlife Sanctuary",
    "pokemon": [
      "Chansey (Rare 1%)",
      "Kangaskhan (Rare 4%)",
      "Scyther",
      "Pinsir",
      "Tauros",
      "Dratini",
      "Dragonair"
    ],
    "pois": [
      "4 Expansive Wild Sectors",
      "Secret House (HM03 Surf)",
      "Lost Gold Teeth Clearing"
    ],
    "items": [
      "HM03 Surf (Secret House)",
      "Gold Teeth (Warden's Teeth)",
      "TM32 Double Team",
      "Quick Claw"
    ],
    "wx": 216,
    "wy": 290,
    "sx": 529,
    "sy": 725
  },
  {
    "id": "route19",
    "name": "Route 19",
    "type": "route",
    "csvType": "Sea route",
    "connects": "Fuchsia City; Route 20",
    "highlights": "Surf south; many swimmer trainers.",
    "gym": "",
    "pokemon": [
      "Tentacool",
      "Tentacruel",
      "Magikarp",
      "Shellder",
      "Horsea"
    ],
    "pois": [
      "South Ocean Surf Passage",
      "Swimmer Gauntlet"
    ],
    "items": [
      "Antidote"
    ],
    "wx": 216,
    "wy": 370,
    "sx": 529,
    "sy": 925
  },
  {
    "id": "route20",
    "name": "Route 20",
    "type": "route",
    "csvType": "Sea route",
    "connects": "Route 19; Cinnabar Island; Seafoam Islands",
    "highlights": "Surf west; leads past the Seafoam Islands.",
    "gym": "Seafoam Islands Ocean Channel",
    "pokemon": [
      "Tentacool",
      "Tentacruel",
      "Staryu",
      "Gyradados"
    ],
    "pois": [
      "Seafoam Islands Twin Entrances",
      "Swimmer Gauntlet"
    ],
    "items": [
      "Water Stone (Hidden)"
    ],
    "wx": 144,
    "wy": 390,
    "sx": 353,
    "sy": 975
  },
  {
    "id": "seafoamislands",
    "name": "Seafoam Islands",
    "type": "dungeon",
    "csvType": "Cave",
    "connects": "Route 20",
    "highlights": "Icy cave. Articuno is found here.",
    "gym": "Legendary Articuno Glacier Cave",
    "pokemon": [
      "Articuno (Lv.50)",
      "Seel",
      "Dewgong",
      "Slowpoke",
      "Slowbro",
      "Golbat",
      "Psyduck",
      "Golduck"
    ],
    "pois": [
      "Raging Water Current Puzzles (Strength Boulders)",
      "Freezing Glacial Deep Floor",
      "Articuno Perch"
    ],
    "items": [
      "Articuno Catch",
      "Ice Beam TM13",
      "Water Stone",
      "Big Pearl"
    ],
    "wx": 150,
    "wy": 390,
    "sx": 368,
    "sy": 975
  },
  {
    "id": "cinnabarisland",
    "name": "Cinnabar Island",
    "type": "city",
    "csvType": "City",
    "connects": "Route 20; Route 21; Pokemon Mansion",
    "highlights": "Gym (Blaine, Volcano Badge; needs the Secret Key). Pokemon Lab (fossil revival).",
    "gym": "Gym 7: Blaine (Volcano Badge · Fire)",
    "pokemon": [
      "Aerodactyl / Omanyte / Kabuto (Lab Fossil Revival)",
      "Grimer",
      "Muk",
      "Koffing"
    ],
    "pois": [
      "Cinnabar Gym (Quiz Machines · Locked)",
      "Pokémon Lab (Fossil Revival)",
      "Pokémon Mansion"
    ],
    "items": [
      "Volcano Badge",
      "TM38 Fire Blast",
      "Revived Fossil Pokémon",
      "TM35 Flamethrower"
    ],
    "wx": 74,
    "wy": 392,
    "sx": 181,
    "sy": 980
  },
  {
    "id": "pokemonmansion",
    "name": "Pokemon Mansion",
    "type": "special",
    "csvType": "Special",
    "connects": "Cinnabar Island",
    "highlights": "Burned-down mansion. The Secret Key for the Cinnabar Gym is found here.",
    "gym": "Burned Research Mansion (Mew & Mewtwo Birthplace)",
    "pokemon": [
      "Magmar",
      "Vulpix",
      "Growlithe",
      "Koffing",
      "Weezing",
      "Grimer",
      "Muk",
      "Ditto"
    ],
    "pois": [
      "Secret Switches in Statues",
      "Mewtwo Origin Research Journals",
      "Secret Key Vault"
    ],
    "items": [
      "Secret Key (Unlocks Cinnabar Gym)",
      "TM14 Blizzard",
      "TM22 SolarBeam",
      "Max Potion"
    ],
    "wx": 65,
    "wy": 385,
    "sx": 159,
    "sy": 963
  },
  {
    "id": "route21",
    "name": "Route 21",
    "type": "route",
    "csvType": "Sea route",
    "connects": "Cinnabar Island; Pallet Town",
    "highlights": "Surf north to close the loop back to Pallet Town.",
    "gym": "",
    "pokemon": [
      "Tangela (Grass Patch)",
      "Tentacool",
      "Tentacruel",
      "Poliwhirl"
    ],
    "pois": [
      "Northbound Surf Current back to Pallet Town",
      "Fisherman & Swimmer Gauntlet"
    ],
    "items": [
      "Stardust",
      "Pearl"
    ],
    "wx": 72,
    "wy": 330,
    "sx": 176,
    "sy": 825
  },
  {
    "id": "pokemontower",
    "name": "Pokemon Tower",
    "type": "dungeon",
    "csvType": "Special",
    "connects": "Lavender Town",
    "highlights": "Restless Spirit Marowak & Ghost Labyrinth. 7 floors of spirits.",
    "gym": "Restless Spirit Marowak & Ghost Labyrinth",
    "pokemon": [
      "Ghost Marowak (Boss)",
      "Gastly",
      "Haunter",
      "Cubone"
    ],
    "pois": [
      "Ghost Mist (Requires Silph Scope)",
      "Purified Healing Zone",
      "Mr. Fuji Rescue"
    ],
    "items": [
      "Silph Scope Revelation",
      "Elixir",
      "Awakening",
      "Rare Candy"
    ],
    "wx": 398,
    "wy": 135,
    "sx": 975,
    "sy": 338
  }
];
