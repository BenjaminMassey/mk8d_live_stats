#[derive(Debug, strum_macros::EnumString)]
pub enum Item {
    Banana,
    Blooper,
    BlueShell,
    BombOmb,
    Boomerang,
    BulletBill,
    Coin,
    CrazyEight,
    FireFlower,
    GoldenMushroom,
    GreenShell,
    Horn,
    Lightning,
    Mushroom,
    PiranhaPlant,
    RedShell,
    Star,
    TripleBanana,
    TripleMushroom,
    TripleRedShell,
}

pub struct Items {
    pub first: Option<Item>,
    pub second: Option<Item>,
}

pub struct Player {
    pub items: Items,
} // TODO: other elements such as placement or something
