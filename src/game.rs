use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, spritebatch::SpriteBatch, DrawParam, FilterMode, Image, WrapMode};
use ggez::{Context, GameResult};

use crate::camera::Camera;
use crate::dialogbox::DialogBox;
use crate::entity::Operable;
use crate::world::World;

pub struct Game {
    world: World,
    spritebatch: SpriteBatch,
    dialogbox: DialogBox,
    camera: Camera,
}

impl Game {
    pub fn new(context: &mut Context) -> GameResult<Game> {
        let mut image = Image::new(context, "/tileset.png")?;
        image.set_filter(FilterMode::Nearest);
        image.set_wrap(WrapMode::Mirror, WrapMode::Mirror);
        let world = World::new(context);
        let dimensions = world.get_dimensions();

        Ok(Game {
            world,
            spritebatch: SpriteBatch::new(image),
            dialogbox: DialogBox::new(context),
            camera: Camera::new(dimensions),
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        self.world.update();
        self.camera.give_center(self.world.player.get_position());

        if !self.world.player_in_talking_range() {
            self.dialogbox.give_dialogtree(None);
        }

        self.dialogbox.update();
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::BLACK);

        self.world.draw(&mut self.spritebatch);

        graphics::draw(
            context,
            &self.spritebatch,
            DrawParam::default().dest(self.camera.draw),
        )?;

        self.dialogbox.draw(context)?;

        self.spritebatch.clear();

        graphics::present(context)?;

        Ok(())
    }

    fn key_up_event(&mut self, _: &mut Context, keycode: KeyCode, _: KeyMods) {
        self.world.give_key_up(keycode);
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: KeyCode,
        _: KeyMods,
        repeat: bool,
    ) {
        if !repeat {
            match keycode {
                KeyCode::Q => context.continuing = false,
                KeyCode::E => self.dialogbox.give_dialogtree(self.world.get_dialogtree()),
                _ => self.world.give_key_down(keycode),
            }
        }
    }
}
