mkdir protoc
cd protoc
Invoke-WebRequest  https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-win64.zip -outfile "protoc-21.9-win64.zip"
Expand-Archive protoc-21.9-win64.zip -DestinationPath ./
$env:Path += (";" + $PWD.path + "\bin")

#cp protoc/include/* protobuf/
cd ../client
cargo build --release