DESTDIR=
prefix=/usr/local
bindir=$(prefix)/bin

default: build

build:
	cargo build --release

install: build
	mkdir -p $(DESTDIR)$(bindir)
	cp -a target/release/julia-pkg $(DESTDIR)$(bindir)/

clean:
	rm -rf target
