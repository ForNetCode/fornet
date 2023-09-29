#set-ExecutionPolicy RemoteSigned
mkdir protoc
cd protoc
wget https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-win64.zip -outfile "protoc-21.9-win64.zip"
Expand-Archive protoc-21.9-win64.zip -DestinationPath ./
$env:Path += (";" + $PWD.path + "\bin")

#cp protoc/include/* protobuf/
cd ../client
cargo build --release
cd ../
mkdir release

wget https://github.com/ForNetCode/simple-windows-tun/releases/download/v0.1.0/fortun-win11-x86_64.zip -outfile "driver.zip"
Expand-Archive driver.zip -DestinationPath ./driver_tmp
mkdir driver
mv .\driver_tmp\fortun\* .\driver\

Compress-Archive -P ./client/target/release/fornet.exe -P ./driver -P ./client/target/release/fornet-cli.exe -DestinationPath ./release/fornet-window64.zip
