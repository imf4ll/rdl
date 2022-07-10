install:
	cargo install --path .
	mkdir ${HOME}/.config/rdl
	cp ./assets/config.json ${HOME}/.config/rdl/
