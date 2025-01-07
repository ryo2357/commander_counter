# cargo apk build --target aarch64-linux-android --lib
cargo apk build  --lib
adb install -r target\debug\apk\commander_counter.apk 