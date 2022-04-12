mod clubs;
pub use clubs::http_get_club;
pub use clubs::http_get_clubs;
mod leagues;
pub use leagues::http_get_league;
pub use leagues::http_get_leagues;
mod nations;
pub use nations::http_get_nation;
pub use nations::http_get_nations;
mod players;
pub use players::http_get_player;
pub use players::http_get_players;