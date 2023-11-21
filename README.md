Rust Atari 2600 Emulator
========================

Building/Running:

    Install Rust:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh	
    Install SDL:
	linux (debian based): 
		apt-get install libsdl2-dev
	rasbian (64-bit): 
		apt-get install libsdl2-dev
	rasberry pi (ubuntu mate 64-bit): 
		# Release 22.04 LTS (Jammy Jellyfish) 64-bit
		# Need to upgrade so 'sdl2' will install.
		apt-get update
		apt-get upgrade
		apt-get install git curl libsdl2-dev

		# 'pipewire' appears to be a good sound driver on the raspberry pi
		# SDL_AUDIODRIVER=pipewire 
	OSX: 
		brew install sdl2

Build and run:
    cargo run --release 

Rust dependencies:
        cargo add argh
        cargo add sdl2
        cargo add bitfield
