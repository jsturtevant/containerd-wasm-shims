$env:OPENSSL_NO_VENDOR="true"
$env:OPENSSL_DIR = "C:\Program Files\OpenSSL-Win64\"
cargo build
cp -force .\target\debug\containerd-shim-slight-v1.exe ../../containerd/bin/
ls ../../containerd/bin/containerd-shim-slight-v1.exe