#![allow(non_snake_case)] //keeping Cpp names for the first few commits
// The crates are in main.rs
//extern crate rand;
//extern crate sfml;

use rand::distributions::{IndependentSample, Range};
//use rand::{thread_rng};
use rand;

use sfml::graphics::{Color, RenderWindow, RenderTarget, Texture, Transformable, Sprite, FloatRect,
                     View, RectangleShape, Shape, Text, Font};
use sfml::window::{Event, Key, style, VideoMode};
use sfml::system::{Clock, Vector2f};


pub fn main() {
   
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

    //
    let mut clock = Clock::start();

	// Time bar
	let mut timeBar = RectangleShape::new();
	let timeBarStartWidth = 400.0f32;
	let timeBarHeight = 80.0f32;

	timeBar.set_size(&Vector2f::new(timeBarStartWidth, timeBarHeight));
	timeBar.set_fill_color(&Color::red());
	timeBar.set_position(&Vector2f::new((1920. / 2.) - timeBarStartWidth / 2., 980.0));

	//let mut gameTimeTotal = Time::new();
	let mut timeRemaining = 6.0f32;
	let timeBarWidthPerSecond :f32 = timeBarStartWidth / timeRemaining;
    
    // Track whether the game is running
    let mut paused :bool = true;

   	// Draw some text
	let mut score :i32 = 0;

	// We need to choose a font before we create messageText 
    let font = Font::from_file("resources/timber_res/fonts/KOMIKAP_.ttf").unwrap();

	let mut messageText = Text::default();
	let mut scoreText = Text::default();
	
    // Set the font to our message
	messageText.set_font(&font);
	scoreText.set_font(&font);

	// Assign the actual message
	messageText.set_string("Press Enter to start!");
	scoreText.set_string("Score = 0");

	// Make it really big
	messageText.set_character_size(75);
	scoreText.set_character_size(100);

	// Choose a color
	messageText.set_fill_color(&Color::white());
	scoreText.set_fill_color(&Color::white());

	// Position the text
	let mut textRect :FloatRect = messageText.local_bounds();
	messageText.set_origin(&Vector2f::new(textRect.left +
                                          textRect.width / 2.0,
                                          textRect.top +
                                          textRect.height / 2.0));

	messageText.set_position(&Vector2f::new(1920.0 / 2., 1080.0 / 2.));

	scoreText.set_position(&Vector2f::new(20.0, 20.0));

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
                // Exit the game
                Event::Closed |
                Event::KeyPressed { code: Key::Escape, .. } => return,
                // Start the game
                Event::KeyPressed { code: Key::Return, .. } => {
                    paused = false;
                    score = 0;
                    timeRemaining = 5.;
                },
                _ => {}
            }
        }

        /*
		****************************************
		Update the scene
		****************************************
		*/
        
        if !paused {
            let mut dt = clock.restart().as_seconds();

            // Subtract from the amount of time remaining
            timeRemaining -= dt; //.asSeconds();
            // size up the time bar
            timeBar.set_size(&Vector2f::new(timeBarWidthPerSecond *
            timeRemaining, timeBarHeight));

            if timeRemaining <= 0.0 {
                // Pause the game
                paused = true;
                // Change the message shown to the player
                messageText.set_string("Out of time!!");
                //Reposition the text based on its new size
                let textRect = messageText.local_bounds();
                messageText.set_origin(&Vector2f::new(textRect.left +
                textRect.width / 2.0,
                textRect.top +
                textRect.height / 2.0));
                messageText.set_position(&Vector2f::new(1920. / 2., 1080. / 2.));
            }


            // Update bee
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

                // Has the bee reached the left edge of the screen?
                if spriteBee.position().x < -100.0 {
                    // Set it up ready to be a whole new bee next frame
                    beeActive = false;
                }
            }

            // Update cloud1
            if !cloud1Active {
                // How fast is the cloud
                let between = Range::new(5., 50.);
                let mut rng = rand::thread_rng();
                cloud1Speed = between.ind_sample(&mut rng);

                // How high is the cloud
                let between = Range::new(0., 150.);
                let height = between.ind_sample(&mut rng);
                spriteCloud1.set_position(&Vector2f::new(-200.0, height)); //starts off screen
                cloud1Active = true
            } else {
                //Move the cloud
                let mut x = spriteCloud1.position().x + (cloud1Speed * dt);
                let mut y = spriteCloud1.position().y;
                spriteCloud1.set_position(&Vector2f::new(x as f32, y as f32));

                // Has the cloud reached the right edge of the screen?
                if spriteCloud1.position().x > 1920.0 {
                    // Set it up ready to be a whole new cloud next frame
                    cloud1Active = false;
                }
            }

            // Update cloud2
            if !cloud2Active {
                // How fast is the cloud
                let between = Range::new(10., 100.);
                let mut rng = rand::thread_rng();
                cloud2Speed = between.ind_sample(&mut rng);

                // How high is the cloud
                let between = Range::new(150., 300.);
                let height = between.ind_sample(&mut rng);
                spriteCloud2.set_position(&Vector2f::new(-200.0, height)); //starts off screen
                cloud2Active = true
            } else {
                //Move the cloud
                let mut x = spriteCloud2.position().x + (cloud2Speed * dt);
                let mut y = spriteCloud2.position().y;
                spriteCloud2.set_position(&Vector2f::new(x as f32, y as f32));

                // Has the cloud reached the right edge of the screen?
                if spriteCloud2.position().x > 1920.0 {
                    // Set it up ready to be a whole new cloud next frame
                    cloud2Active = false;
                }
            }

            // Update cloud3
            if !cloud3Active {
                // How fast is the cloud
                let between = Range::new(30., 200.);
                let mut rng = rand::thread_rng();
                cloud3Speed = between.ind_sample(&mut rng);

                // How high is the cloud
                let between = Range::new(200., 450.);
                let height = between.ind_sample(&mut rng);
                spriteCloud3.set_position(&Vector2f::new(-200.0, height)); //starts off screen
                cloud3Active = true
            } else {
                //Move the cloud
                let mut x = spriteCloud3.position().x + (cloud3Speed * dt);
                let mut y = spriteCloud3.position().y;
                spriteCloud3.set_position(&Vector2f::new(x as f32, y as f32));

                // Has the cloud reached the right edge of the screen?
                if spriteCloud3.position().x > 1920.0 {
                    // Set it up ready to be a whole new cloud next frame
                    cloud3Active = false;
                }
            }

            // update the score text
            score +=1;
            scoreText.set_string(&format!("Score = {}", score));
            
        }//end if paused else


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
        
        window.draw(&scoreText);
        window.draw(&timeBar);

        if paused {
            // Draw the pause message
            window.draw(&messageText);
        }

        // Show everything we just drew
        window.display();

    }

}
