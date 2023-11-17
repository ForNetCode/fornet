# This is for .github/workflows.
.PHONY: release-mac-x86_64, release-mac-aarch64, release-linux, release-backend, release-linux-aarch64, release-android

#base_dir := $(shell pwd)

#export PROTOC := $(shell pwd)/protoc/bin

release-mac-x86_64: 	
	mkdir protoc && cd protoc && wget https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-osx-x86_64.zip && unzip protoc-21.9-osx-x86_64.zip && sudo cp bin/protoc /usr/local/bin
	cp -r protoc/include/* protobuf/
	mkdir -p release
	cd client && cargo build --release  --target=x86_64-apple-darwin
	strip client/target/x86_64-apple-darwin/release/fornet
	otool -L client/target/x86_64-apple-darwin/release/fornet
	strip client/target/x86_64-apple-darwin/release/fornet-cli
	otool -L client/target/x86_64-apple-darwin/release/fornet-cli
	cp client/target/x86_64-apple-darwin/release/fornet ./release/
	cp client/target/x86_64-apple-darwin/release/fornet-cli ./release/	

# brew install wget
release-mac-aarch64:
	mkdir protoc && cd protoc && wget https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-osx-aarch_64.zip && unzip protoc-21.9-osx-aarch_64.zip && sudo cp bin/protoc /usr/local/bin
	cp -r protoc/include/* protobuf/
	mkdir -p release
	cd client && cargo build --release --target=aarch64-apple-darwin
	strip client/target/aarch64-apple-darwin/release/fornet
	otool -L client/target/aarch64-apple-darwin/release/fornet
	strip client/target/aarch64-apple-darwin/release/fornet-cli
	otool -L client/target/aarch64-apple-darwin/release/fornet-cli
	cp client/target/aarch64-apple-darwin/release/fornet ./release/
	cp client/target/aarch64-apple-darwin/release/fornet-cli ./release/

release-linux-aarch64:
    mkdir protoc && cd protoc && wget https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-linux-aarch64.zip && unzip protoc-21.9-linux-aarch64.zip && sudo cp bin/protoc /usr/bin
	cp -r protoc/include/* protobuf/
	sudo apt-get install -y build-essential	
	mkdir release	
	cd client && cargo build --release --target=aarch64-unknown-linux-gnu
	strip client/target/aarch64-unknown-linux-gnu/release/fornet
	strip client/target/aarch64-unknown-linux-gnu/release/fornet-cli
	cp client/target/aarch64-unknown-linux-gnu/release/fornet ./release/
	cp client/target/aarch64-unknown-linux-gnu/release/fornet-cli ./release/


release-linux:	
	mkdir protoc && cd protoc && wget https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-linux-x86_64.zip && unzip protoc-21.9-linux-x86_64.zip && sudo cp bin/protoc /usr/bin
	cp -r protoc/include/* protobuf/
	sudo apt-get install -y build-essential	
	mkdir release
	cd client && cargo build --release --target=x86_64-unknown-linux-gnu
	strip client/target/x86_64-unknown-linux-gnu/release/fornet
	strip client/target/x86_64-unknown-linux-gnu/release/fornet-cli
	cp client/target/x86_64-unknown-linux-gnu/release/fornet ./release
	cp client/target/x86_64-unknown-linux-gnu/release/fornet-cli ./release

#TODO
release-windows:	
	vcpkg install --triplet=x64-windows-static-md openssl


release-backend:
	cd admin-web && npm ci && npm run build:prod && cd ../
	cp -r admin-web/build/ command/docker/backend/web
	cd backend && sbt universal:packageBin && cd ../
	mkdir -p release
	cp backend/target/universal/app-*.zip release/app.zip && cd release && unzip app.zip && rm app.zip	
	cp backend/target/universal/app-*.zip command/docker/backend/app.zip && cd command/docker/backend && unzip app.zip && rm app.zip && mv app-* app

release-android:
	cargo install cargo-ndk
	cd app && flutter pub get && flutter build apk

