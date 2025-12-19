name := 'cosmic-sys-monitor'
appid := 'io.github.marcos.SysMonitor'
rootdir := ''
prefix := '/usr'
destdir := ''

base-dir := destdir / prefix
cargo-target-dir := env('CARGO_TARGET_DIR', 'target')
metainfo-dst := base-dir / 'share' / 'metainfo' / appid + '.metainfo.xml'
bin-dst := base-dir / 'bin' / name
desktop-dst := base-dir / 'share' / 'applications' / appid + '.desktop'
icon-dst := base-dir / 'share' / 'icons' / 'hicolor' / 'symbolic' / 'apps' / appid + '-symbolic.svg'

default: build-release

clean:
    cargo clean

build-debug *args:
    cargo build {{args}}

build-release *args: (build-debug '--release' args)

check *args:
    cargo clippy --all-features {{args}} -- -W clippy::pedantic

run *args:
    env RUST_BACKTRACE=full cargo run --release {{args}}

install: build-release
    install -Dm0755 {{ cargo-target-dir / 'release' / name }} {{bin-dst}}
    install -Dm0644 resources/app.desktop {{desktop-dst}}
    install -Dm0644 resources/app.metainfo.xml {{metainfo-dst}}
    install -Dm0644 resources/icon-symbolic.svg {{icon-dst}}

uninstall:
	rm -f {{bin-dst}} {{desktop-dst}} {{icon-dst}} {{metainfo-dst}}
