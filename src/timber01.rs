#![allow(non_snake_case)] //keeping Cpp names for the first few commits
//Cpp #include <SFML/Graphics.hpp>
extern crate sfml;

//Cpp using namespace sf;
use sfml::graphics::{Color, RenderWindow, RenderTarget, Texture, Transformable, Sprite};
use sfml::window::{Event, Key, style, VideoMode};
use sfml::system::{Vector2f};

//Cpp int main() {
pub fn main () {
	// Create a video mode object
	//Cpp VideoMode vm(1920, 1080);

	// Create and open a window for the game
	//Cpp RenderWindow window(vm, "Timber!!!", Style::Fullscreen);
    
    // Create a window with the same pixel depth as the desktop
    let desktop = VideoMode::desktop_mode();    
    let mut window = RenderWindow::new(VideoMode::new(1366, 768, desktop.bits_per_pixel),
                                "Timber!!!",
                                style::FULLSCREEN,
                                &Default::default())
                                .unwrap();
    //window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);
    
    
	// Create a texture to hold a graphic on the GPU
	//Cpp Texture textureBackground;

	// Load a graphic into the texture
	//Cpp textureBackground.loadFromFile("../resources/timber_res/graphics/background.png");
    let textureBackground = Texture::from_file("resources/timber_res/graphics/background.png").unwrap();
	
	// Create a sprite
	//Cpp Sprite spriteBackground;
    let mut spriteBackground = Sprite::new();
    
	// Attach the texture to the sprite
	//Cpp spriteBackground.setTexture(textureBackground);
    spriteBackground.set_texture(&textureBackground, true);
    
	// Set the spriteBackground to cover the screen
	//Cpp spriteBackground.setPosition(0, 0);
	spriteBackground.set_position(&Vector2f::new(0.0, 0.0)); // in Transformable

    // Make a tree sprite
    //Cpp Texture textureTree;
    let textureTree = Texture::from_file("resources/timber_res/graphics/tree.png").unwrap();
    //Cpp
    // Sprite spriteTree;
    // spriteTree.setTexture(textureTree);
    // spriteTree.setPosition(810, 0);
    let mut spriteTree = Sprite::new();
    spriteTree.set_texture(&textureTree, true);
    spriteTree.set_position(&Vector2f::new(810.0, 0.0));

	//Cpp while (window.isOpen()){
    loop {
		/*
		****************************************
		Handle the players input
		****************************************
		*/

		//Cpp if (Keyboard::isKeyPressed(Keyboard::Escape))
		//Cpp {
		//Cpp 	window.close();
		//Cpp }
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


		/*
		****************************************
		Draw the scene
		****************************************
		*/

		// Clear everything from the last frame
		//Cpp window.clear();
        window.clear(&Color::black());
        
		// Draw our game scene here
		//Cpp window.draw(spriteBackground);
		window.draw(&spriteBackground); //in RenderTarget
		window.draw(&spriteTree);

		// Show everything we just drew
		//Cpp window.display(); 
		window.display();


	}

	//Cpp return 0;
}


