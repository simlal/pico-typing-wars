use defmt::Format;

#[derive(PartialEq, Eq, Format, Clone, Copy)]
pub enum ButtonRole {
    Player1,
    Player2,
}
pub struct Button {
    role: ButtonRole,
}
