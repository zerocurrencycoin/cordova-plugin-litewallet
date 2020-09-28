#!/bin/bash

export NDK=/home/cryptoforge/Dev/new-mobile-wallet/NDK

#build openssl
#$PWD/buildopenssl.sh

export AR=$NDK/arm64/bin/aarch64-linux-android-ar
export CC=$NDK/arm64/bin/aarch64-linux-android-clang
export OPENSSL_DIR=$(pwd)/openssl-1.1.1e/openssl-1.1.1e/aarch64
cargo build --target aarch64-linux-android --release

export AR=$NDK/arm/bin/arm-linux-androideabi-ar
export CC=$NDK/NDK/arm/bin/arm-linux-androideabi-clang
export OPENSSL_DIR=$(pwd)/openssl-1.1.1e/openssl-1.1.1e/armv7
cargo build --target armv7-linux-androideabi --release

export AR=$NDK/x86/bin/i686-linux-android-ar
export CC=$NDK/x86/bin/i686-linux-android-clang
export OPENSSL_DIR=$(pwd)/openssl-1.1.1e/openssl-1.1.1e/x86
cargo build --target i686-linux-android --release

cp ../target/aarch64-linux-android/release/librust.so ../../android/litewallet-jni/libs/arm64-v8a/liblitewallet-jni.so
cp ../target/armv7-linux-androideabi/release/librust.so ../../android/litewallet-jni/libs/armeabi-v7a/liblitewallet-jni.so
cp ../target/i686-linux-android/release/librust.so ../../android/litewallet-jni/libs/x86/liblitewallet-jni.so
