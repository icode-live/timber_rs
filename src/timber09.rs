/*
    Copyright 2017 lerina 'J-Y' RAZAFY
    See licensing in LICENSE file, or at:
        https://opensource.org/licenses/MPL-2.0
    File: timber.rs
    Author: lerina 'J-Y' RAZAFY
    Description:
      Discovering Rust though game programming.
*/
#![allow(non_snake_case)] //keeping Cpp names for the first few commits
// The crates are in main.rs
//extern crate rand;
//extern crate sfml;
use rand::distributions::{IndependentSample, Range};
use rand;
use conv::prelude::*;

use sfml::graphics::{Color, RenderWindow, RenderTarget, Texture, Transformable, Sprite, FloatRect,
                     View, RectangleShape, Shape, Text, Font};
use sfml::window::{Event, Key, style, VideoMode};
use sfml::system::{Clock, Vector2f};
use sfml::audio::{Sound, SoundBuffer};


const NUM_BRANCHES :usize = 6;
const LEFT :u8 = 0;
const RIGHT :u8 = 1;
const NONE :u8 = 2;


fn updateBranches(seed :i32, branchPositions :&mut[u8; NUM_BRANCHES]) {
	// Move all the branches down one place
    for j in (1..NUM_BRANCHES-1).rev() {
		branchPositions[j] = branchPositions[j - 1];
    }

	// Spawn a new branch at position 0
	// LEFT, RIGHT or NONE
    let between = Range::new(0, seed);
    let mut rng = rand::thread_rng();
    let r = between.ind_sample(&mut rng);

	match r {
        0 => branchPositions[0] = LEFT,
        1 => branchPositions[0] = RIGHT,
        _ => branchPositions[0] = NONE,
	}
}




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
	let textRect :FloatRect = messageText.local_bounds();
	messageText.set_origin(&Vector2f::new(textRect.left +
                                          textRect.width / 2.0,
                                          textRect.top +
                                          textRect.height / 2.0));

	messageText.set_position(&Vector2f::new(1920.0 / 2., 1080.0 / 2.));

	scoreText.set_position(&Vector2f::new(20.0, 20.0));

	// Prepare 5 branches
	let textureBranch = Texture::from_file("resources/timber_res/graphics/branch.png").unwrap();

    // arrays of branch sprites.
    let mut branches :[Sprite; NUM_BRANCHES] = [Sprite::default(), Sprite::default(), 
                                                Sprite::default(), Sprite::default(), 
                                                Sprite::default(), Sprite::default() 
                                               ];
    
    let mut branchPositions :[u8; NUM_BRANCHES] = [LEFT; NUM_BRANCHES];

	// Set the texture for each branch sprite
	for i in 0..NUM_BRANCHES {
		branches[i].set_texture(&textureBranch, true);
		branches[i].set_position(&Vector2f::new(-2000., -2000.));

		// Set the sprite's origin to dead centre
		// We can then spin it round without changing its position
		branches[i].set_origin(&Vector2f::new(220., 20.));
	}


    let textureTree = Texture::from_file("resources/timber_res/graphics/tree.png").unwrap();

    let mut spriteTree = Sprite::new();
    spriteTree.set_texture(&textureTree, true);
    spriteTree.set_position(&Vector2f::new(810.0, 0.0));


	// Prepare the player
	let texturePlayer = Texture::from_file("resources/timber_res/graphics/player.png").unwrap();
	let mut spritePlayer = Sprite::new();
	spritePlayer.set_texture(&texturePlayer, true);
	spritePlayer.set_position(&Vector2f::new(580., 720.));

	// The player starts on the left
	let mut playerSide = LEFT;

	// Prepare the gravestone
	let textureRIP = Texture::from_file("resources/timber_res/graphics/rip.png").unwrap();
	let mut spriteRIP = Sprite::new();
	spriteRIP.set_texture(&textureRIP, true);
	spriteRIP.set_position(&Vector2f::new(600., 860.));

	// Prepare the axe
	let textureAxe = Texture::from_file("resources/timber_res/graphics/axe.png").unwrap();
	let mut spriteAxe = Sprite::new();
	spriteAxe.set_texture(&textureAxe, true);
	spriteAxe.set_position(&Vector2f::new(700., 830.));

	// Line the axe up with the tree
	const AXE_POSITION_LEFT :f32 = 700.;
	const AXE_POSITION_RIGHT :f32 = 1075.;

	// Prepare the flying log
	let textureLog = Texture::from_file("resources/timber_res/graphics/log.png").unwrap();
	let mut spriteLog = Sprite::new();
	spriteLog.set_texture(&textureLog, true);
	spriteLog.set_position(&Vector2f::new(810., 720.));

	// Some other useful log related variables
	let mut logActive :bool = false;
	let mut logSpeedX :f32 = 1000.;
	let mut logSpeedY :f32 = -1500.;

	// Control the player input
	let mut acceptInput :bool = false;

/*
    updateBranches(1, &mut branchPositions);
updateBranches(2, &mut branchPositions);
updateBranches(3, &mut branchPositions);
updateBranches(4, &mut branchPositions);
updateBranches(5, &mut branchPositions);
*/

	// Prepare the sound
	let chopBuffer = SoundBuffer::from_file("resources/timber_res/sound/chop.wav").unwrap();
	let mut chop = Sound::with_buffer(&chopBuffer);
    //chop.set_buffer(&chopBuffer);

	let deathBuffer = SoundBuffer::from_file("resources/timber_res/sound/death.wav").unwrap();
	let mut death = Sound::with_buffer(&deathBuffer);

	// Out of time
	let ootBuffer = SoundBuffer::from_file("resources/timber_res/sound/out_of_time.wav").unwrap();
	let mut outOfTime = Sound::with_buffer(&ootBuffer);

	
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
        while let Some(event) = window.poll_event() {
            //for event in window.poll_event() {
                match event {
                    Event::KeyReleased { code: Key::Left, .. } |
                    Event::KeyReleased { code: Key::Right, .. } if !paused => {
                    // Listen for key presses again
                    acceptInput = true;

                    // hide the axe
                    let y = spriteAxe.position().y;
                    spriteAxe.set_position(&Vector2f::new(2000., y as f32));
                    },
                    _ =>{}
                }
            //}//end while

        //for event in window.events() {
            match event {
                // Exit the game
                Event::Closed |
                Event::KeyPressed { code: Key::Escape, .. } => return,
                // Start the game
                Event::KeyPressed { code: Key::Return, .. } => {
                    paused = false;
                    score = 0;
                    timeRemaining = 5.;

                    // Make all the branches disappear
                    for i in 1..NUM_BRANCHES { 
                        branchPositions[i] = NONE;
                    }
                    
                    // Make sure the gravestone is hidden
                    spriteRIP.set_position(&Vector2f::new(675., 2000.));
                    
                    // Move the player into position
                    spritePlayer.set_position(&Vector2f::new(580., 720.));
                    acceptInput = true;

                }, //end Start the game
                _ => {}
            }

            // Wrap the player controls to
            // Make sure we are accepting input
            if acceptInput {
                match event {
                    // First handle pressing the right cursor key
                    Event::KeyPressed { code: Key::Right, .. } => {
                        // Make sure the player is on the right
                        playerSide = RIGHT;

                        score +=1;

                        // Add to the amount of time remaining
                        timeRemaining += (2. / f32::value_from(score).unwrap()) + 0.15;
                        let y = spriteAxe.position().y;
                        spriteAxe.set_position(&Vector2f::new(AXE_POSITION_RIGHT,
                                                     y  as f32));

                        spritePlayer.set_position(&Vector2f::new(1200., 720.));

                        // update the branches
                        updateBranches(score, &mut branchPositions);

                        // set the log flying to the left
                        spriteLog.set_position(&Vector2f::new(810., 720.));
                        logSpeedX = -5000.;
                        logActive = true;

                        acceptInput = false;

                        // Play a chop sound
                        chop.play();
                    },

                    // Handle the left cursor key
                    Event::KeyPressed { code: Key::Left, .. } => {
                        // Make sure the player is on the left
                        playerSide = LEFT;

                        score +=1;

                        // Add to the amount of time remaining
                        timeRemaining += (2. / f32::value_from(score).unwrap()) + 0.15;

                        let y = spriteAxe.position().y;
                        spriteAxe.set_position(&Vector2f::new(AXE_POSITION_LEFT,
                                                      y as f32));

                        spritePlayer.set_position(&Vector2f::new(580., 720.));

                        // update the branches
                        updateBranches(score, &mut branchPositions);

                        // set the log flying
                        spriteLog.set_position(&Vector2f::new(810., 720.));
                        logSpeedX = 5000.;
                        logActive = true;

                        acceptInput = false;

                        // Play a chop sound
                        chop.play();
                    },
                    _ => {}
                }//end match 
            }//end if acceptInput
        }//end while some event

        /*
		****************************************
		Update the scene
		****************************************
		*/
        
        if !paused {
            let dt = clock.restart().as_seconds();

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
                outOfTime.play();
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
                let x = spriteBee.position().x - (beeSpeed * dt);
                let y = spriteBee.position().y;
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
                let x = spriteCloud1.position().x + (cloud1Speed * dt);
                let y = spriteCloud1.position().y;
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
                let x = spriteCloud2.position().x + (cloud2Speed * dt);
                let y = spriteCloud2.position().y;
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
                let x = spriteCloud3.position().x + (cloud3Speed * dt);
                let y = spriteCloud3.position().y;
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
           

            // update the branch sprites
            for i in 0..NUM_BRANCHES {
                let height :f32 = f32::value_from(i * 150).unwrap() ;
                if branchPositions[i] == LEFT {
                    // Move the sprite to the left side
                    branches[i].set_position(&Vector2f::new(610., height));
                    // Flip the sprite round the other way
                    branches[i].set_rotation(180.);
                }
                else if branchPositions[i] == RIGHT {
                    // Move the sprite to the right side
                    branches[i].set_position(&Vector2f::new(1330., height));
                    // Set the sprite rotation to normal
                    branches[i].set_rotation(0.);
                }
                else {
                    // Hide the branch
                    branches[i].set_position(&Vector2f::new(3000., height));
                }
            }

            // Handle a flying log
            if logActive {
                let x = spriteLog.position().x as f32; 
                let y = spriteLog.position().y as f32; 
                spriteLog.set_position(&Vector2f::new( x + logSpeedX * dt,
                                                       y + logSpeedY * dt));
                // Has the log reached the right hand edge?
                if spriteLog.position().x < -100. || 
                   spriteLog.position().x > 2000.  {
                    // Set it up ready to be a whole new log next frame
                    logActive = false;
                    spriteLog.set_position(&Vector2f::new(810., 720.));
                }
            }


            // has the player been squished by a branch?
			if branchPositions[5] == playerSide {
				// death
				paused = true;
				acceptInput = false;

				// Draw the gravestone
				spriteRIP.set_position(&Vector2f::new(525., 760.));

				// hide the player
				spritePlayer.set_position(&Vector2f::new(2000., 660.));

				// Change the text of the message
				messageText.set_string("SQUISHED!!");

				// Center it on the screen
				let textRect = messageText.local_bounds();
				messageText.set_origin(&Vector2f::new(textRect.left +
					textRect.width / 2.0,
					textRect.top + textRect.height / 2.0));

				messageText.set_position(&Vector2f::new(1920. / 2.0, 1080. / 2.0));

				// Play the death sound
				death.play();
			}


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

        for i in 0..NUM_BRANCHES {
            window.draw(&branches[i]);
        }
        window.draw(&spriteTree);

        // Draw the player
        window.draw(&spritePlayer);
        // Draw the axe
        window.draw(&spriteAxe);
        // Draraw the flying log
        window.draw(&spriteLog);
        // Draw the gravestone
        window.draw(&spriteRIP);

        window.draw(&spriteBee);
        
        window.draw(&scoreText);
        window.draw(&timeBar);

        if paused {
            // Draw the pause message
            window.draw(&messageText);
        }

        // Show everything we just drew
        window.display();

    }//loop

}//main
