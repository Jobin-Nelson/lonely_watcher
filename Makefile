.PHONY: all
all:
	docker run --rm \
		-v $(CURDIR):/code \
		-w /code \
		rust:1.77-slim \
		cargo build --release
		

