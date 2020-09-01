//Usando o cargo update para verificar as versões e download
//Usando o cargo build para instalar de fato as versões das dependências do .toml

//Fazer o uso das dependências
extern crate rand;
extern crate piston_window;

//Para abrir exportar o modulo fora do arquivo main
mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;
use crate::draw::to_coord_u32;

const BACK_COLOR: Color = [0.2, 0.2, 0.2, 1.0];

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new(
        "Snake by Agiota",
        [to_coord_u32(width), to_coord_u32(height)],

    ).exit_on_esc(true).build().unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next()
    {
        if let Some(Button::Keyboard(key)) = event.press_args()
        {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _|
        {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg|
        {
            game.update(arg.dt);
        });
    }
}
