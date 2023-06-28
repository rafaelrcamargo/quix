echo ""
echo "Starting release script!"
echo ""
echo "> Building..."
echo ""
# Windows: cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-pc-windows-msvc --release
# Mac M1: cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target aarch64-apple-darwin --release
echo ""
echo "> Moving..."
mv ./target/x86_64-pc-windows-msvc/release/quix.exe ./release/quix.exe
echo "> Compresing..."
echo ""
upx --best --lzma ./release/quix.exe
echo ""
echo "Release complete!"
echo ""
