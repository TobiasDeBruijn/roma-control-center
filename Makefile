run: flutter_rust_bridge_codegen
	flutter run

RUST_SRC=$(shell find ./backend -type f -name "*.rs" -not -path "./target/*")
.PHONY: flutter_rust_bridge_codegen
flutter_rust_bridge_codegen: ${RUST_SRC} backend/Cargo.toml backend/Cargo.lock
	flutter_rust_bridge_codegen \
    -r backend/src/bridge.rs \
    -d lib/bridge_generated.dart \
	