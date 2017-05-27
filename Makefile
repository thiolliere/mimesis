check:
	cargo check --target asmjs-unknown-emscripten --release

build:
	rm -rf target/publication/html/*
	cargo build --target asmjs-unknown-emscripten --release
	cp target/asmjs-unknown-emscripten/release/mimesis.js target/publication/html/
	cp release.html target/publication/html/index.html

run: build
	firefox target/publication/html/index.html

doc:
	cargo doc --open &
	rustup doc

# android_log:
# 	~/android-sdk-linux/platform-tools/adb logcat | grep -ie AndroidGLue

# android_build:
# 	sudo docker run --rm -v `pwd`:/root/src -w /root/src tomaka/android-rs-glue cargo apk

# android_install:
# 	~/android-sdk-linux/platform-tools/adb install -r target/android-artifacts/build/bin/mimesis-debug.apk

# TODO https://developer.android.com/studio/publish/app-signing.html
# TODO zipalign -v -p 4 my-app-unsigned.apk my-app-unsigned-aligned.apk
# TODO apksigner sign --ks my-release-key.jks --out my-app-release.apk my-app-unsigned-aligned.apk
