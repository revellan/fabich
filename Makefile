# PRODUCTION MAKEFILE

fabich:
	@cargo build --release

install: fabich
	@if [ "$(shell whoami)" != "root" ]; then \
		echo "You must be root to install this package!"; \
		exit 1; \
	fi
	@install -Dm755 target/release/fabich /usr/local/bin/fabich
	@echo "Installed to /usr/local/bin/fabich"

clean:
	@cargo clean

