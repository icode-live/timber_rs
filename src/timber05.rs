#![allow(non_snake_case)] //keeping Cpp names for the first few commits
extern crate rand;
extern crate sfml;

use rand::distributions::{IndependentSample, Range};

use sfml::graphics::{Color, RenderWindow, RenderTarget, Texture, Transformable, Sprite, FloatRect,
                     View};
use sfml::window::{Event, Key, style, VideoMode};
use sfml::system::{Clock, Time, Vector2f};

pub fn main() {
    let mut clock = Clock::start();

    // Create a window with the same pixel depth as the desktop
    let desktop = VideoMode::desktop_mode();
    let mut window = RenderWindow::new(VideoMode::new(1366, 768, desktop.bits_per_pixel),
                                       "Timber!!!",
                                       style::FULLSCREEN,
                                       &Default::default())
            .unwrap();

    //Low res code :-(
    let view = View::from_rect(&FloatRect::new(0.0, 0.0, 1920.0, 1080.0));
    window.set_view(&view);

    //window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);

    // Create a texture to hold a graphic on the GPU
    // Load a graphic into the texture
    let textureBackground = Texture::from_file("resources/timber_res/graphics/background.png")
        .unwrap();

    // Create a sprite
    let mut spriteBackground = Sprite::new();

    // Attach the texture to the sprite
    spriteBackground.set_texture(&textureBackground, true);

    // Set the spriteBackground to cover the screen
    spriteBackground.set_position(&Vector2f::new(0.0, 0.0)); // in Transformable

    // Make a tree sprite
    let textureTree = Texture::from_file("resources/timber_res/graphics/tree.png").unwrap();

    let mut spriteTree = Sprite::new();
    spriteTree.set_texture(&textureTree, true);
    spriteTree.set_position(&Vector2f::new(810.0, 0.0));

    // Make a Bee sprite
    let textureBee = Texture::from_file("resources/timber_res/graphics/bee.png").unwrap();

    let mut spriteBee = Sprite::new();
    spriteBee.set_texture(&textureBee, true);
    spriteBee.set_position(&Vector2f::new(0.0, 800.0));

    let mut beeActive: bool = false;
    let mut beeSpeed: f32 = 0.0;

    // make the clouds
    let textureCloud = Texture::from_file("resources/timber_res/graphics/cloud.png").unwrap();

    let mut spriteCloud1 = Sprite::new();
    let mut spriteCloud2 = Sprite::new();
    let mut spriteCloud3 = Sprite::new();

    spriteCloud1.set_texture(&textureCloud, true);
    spriteCloud2.set_texture(&textureCloud, true);
    spriteCloud3.set_texture(&textureCloud, true);

    spriteCloud1.set_position(&Vector2f::new(0.0, 0.0));
    spriteCloud2.set_position(&Vector2f::new(0.0, 150.0));
    spriteCloud3.set_position(&Vector2f::new(0.0, 200.0));

    let mut cloud1Active: bool = false;
    let mut cloud2Active: bool = false;
    let mut cloud3Active: bool = false;

    let mut cloud1Speed: f32 = 0.0;
    let mut cloud2Speed: f32 = 0.0;
    let mut cloud3Speed: f32 = 0.0;

    /*
     * GAME LOOP:
     *      H. handle
     *      U. Update
     *      D. Draw
     **/
    loop {
        /*
		****************************************
		Handle the players input
		****************************************
		*/

        for event in window.events() {
            match event {
                Event::Closed |
                Event::KeyPressed { code: Key::Escape, .. } => return,
                _ => {}
            }
        }

        /*
		****************************************
		Update the scene
		****************************************
		*/

        let dt = clock.restart().as_seconds();

        if !beeActive {
            // How fast is the bee
            let between = Range::new(200., 400.);
            let mut rng = rand::thread_rng();
            beeSpeed = between.ind_sample(&mut rng);

            // How high is the bee
            let between = Range::new(500., 1000.);
            let height = between.ind_sample(&mut rng);
            spriteBee.set_position(&Vector2f::new(2000.0, height)); //starts off screen
            beeActive = true
        } else {
            //Move the bee
            let mut x = spriteBee.position().x - (beeSpeed * dt);
            let mut y = spriteBee.position().y;
            spriteBee.set_position(&Vector2f::new(x as f32, y as f32));

            // Has the bee reached the right edge of the screen?
            if spriteBee.position().x < -100.0 {
                // Set it up ready to be a whole new bee next frame
                beeActive = false;
            }
        }


        /*
		****************************************
		Draw the scene
		****************************************
		*/

        // Clear everything from the last frame
        window.clear(&Color::black());

        // Draw our game scene here
        window.draw(&spriteBackground); //in RenderTarget

        window.draw(&spriteCloud1);
        window.draw(&spriteCloud2);
        window.draw(&spriteCloud3);

        window.draw(&spriteTree);

        window.draw(&spriteBee);

        // Show everything we just drew
        window.display();


    }

}
