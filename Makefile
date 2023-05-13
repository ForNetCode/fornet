# This is for .github/workflows.
.PHONY: release-mac-x86_64, release-mac-aarch64, release-linux, release-backend

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
	tar -C client/target/x86_64-apple-darwin/release/ -czvf release/fornet-mac-x86_64.tar.gz ./fornet ./fornet-cli	

release-mac-aarch64:
	mkdir protoc && cd protoc && wget https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-osx-aarch_64.zip && unzip protoc-21.9-osx-aarch_64.zip && sudo cp bin/protoc /usr/local/bin
	cp -r protoc/include/* protobuf/
	mkdir -p release 	
	cd client && cargo build --release --target=aarch64-apple-darwin
	strip client/target/aarch64-apple-darwin/release/fornet
	otool -L client/target/aarch64-apple-darwin/release/fornet
	strip client/target/aarch64-apple-darwin/release/fornet-cli
	otool -L client/target/aarch64-apple-darwin/release/fornet-cli	
	tar -C client/target/aarch64-apple-darwin/release/ -czvf release/fornet-mac-aarch64.tar.gz ./fornet ./fornet-cli

release-linux:	
	mkdir protoc && cd protoc && wget https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-linux-x86_64.zip && unzip protoc-21.9-linux-x86_64.zip && sudo cp bin/protoc /usr/bin
	cp -r protoc/include/* protobuf/
	sudo apt-get install -y build-essential libssl-dev cmake	
	mkdir release	
	cd client && cargo build --release --target=x86_64-unknown-linux-gnu
	strip client/target/x86_64-unknown-linux-gnu/release/fornet
	strip client/target/x86_64-unknown-linux-gnu/release/fornet-cli
	tar -C client/target/x86_64-unknown-linux-gnu/release/ -czvf release/fornet-linux-x86_64.tar.gz ./fornet ./fornet-cli

release-backend:
	cd admin-web && npm ci && npm run build:prod && cd ../
	cp -r admin-web/build/ command/docker/backend/web
	cd backend && sbt universal:packageBin && cd ../
	cp backend/target/universal/app-*.zip command/docker/backend/app.zip && cd command/docker/backend && unzip app.zip && rm app.zip && mv app-* app