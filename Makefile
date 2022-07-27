DEPLOY_BRANCH=gh-pages
DEPLOY_DIR=./dist

DESKTOP_TARGET=x86_64-unknown-linux-gnu

.PHONY: all clean build-web dev-web build-desktop dev-desktop deploy

all: build-web

dev-web:
	dioxus serve

build-web:
	dioxus build --release

build-desktop:
	cargo build --release --no-default-features --features desktop --target $(DESKTOP_TARGET)

dev-desktop:
	cargo run --no-default-features --features desktop --target $(DESKTOP_TARGET)

clean:
	rm -rf ./target "$(DEPLOY_DIR)"

deploy:
	rm -rf "$(DEPLOY_DIR)"
	git worktree add "$(DEPLOY_DIR)" "$(DEPLOY_BRANCH)"
	make build-web
	(cd "$(DEPLOY_DIR)" && git add --all && git commit -m 'Deploy pages' && git push origin "$(DEPLOY_BRANCH)")
	git worktree remove --force "$(DEPLOY_DIR)"
