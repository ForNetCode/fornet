# This is for .github/workflows.
.PHONY: release-mac-x86_64, release-mac-aarch_64, release-linux, release-backend

#base_dir := $(shell pwd)
	
release-mac-x86_64: 	
	mkdir protoc && cd protoc && wget https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-osx.x86_64.zip && unzip protoc-21.9-osx.x86_64.zip && cd ../
	export PATH=$(PWD)/protoc/bin:$$PATH
	brew install cmake	
	mkdir -p release
	cd client
	cargo build --release
	strip target/release/fornet
	otool -L target/release/fornet
	strip target/release/fornet-cli
	otool -L target/release/fornet-cli	
	tar -C ./target/release/ -czvf ../release/fornet-mac-x86_64.tar.gz ./fornet ./fornet-cli
	ls -lisah ../release/fornet-mac-x86_64.tar.gz

release-mac-aarch_64:
	mkdir protoc && cd protoc && wget https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-osx.aarch_64.zip && unzip protoc-21.9-osx.aarch_64.zip && cd ../
	export PATH=$(PWD)/protoc/bin:$$PATH
	brew install cmake
	mkdir -p release
	cd client
	cargo build --release
	strip target/release/fornet
	otool -L target/release/fornet
	strip target/release/fornet-cli
	otool -L target/release/fornet-cli	
	tar -C ./target/release/ -czvf ../release/fornet-mac-aarch_64.tar.gz ./fornet ./fornet-cli
	ls -lisah ../release/fornet-mac-aarch_64.tar.gz

release-linux:	
	mkdir protoc && cd protoc && wget https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-linux-x86_64.zip && unzip protoc-21.9-linux-x86_64.zip && cd ../
	export PATH=$(PWD)/protoc/bin:$$PATH
	apt-get install -y build-essential libssl-dev cmake	
	mkdir release
	pwd
	cd client
	cargo build --release
	strip target/release/fornet
	strip target/release/fornet-cli
	tar -C ./target/release/ -czvf ../release/fornet-linux-x86_64.tar.gz ./fornet ./fornet-cli

release-backend:
	cd admin-web && npm ci && npm run build:prod && cd ../
	cp -r admin-web/build/ command/docker/backend/web
	cd backend && sbt universal:packageBin && cd ../
	cp backend/target/universal/app-*.zip command/docker/backend/app.zip && cd command/docker/backend && unzip app.zip && rm app.zip && mv app-* app