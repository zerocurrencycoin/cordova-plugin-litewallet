#!/bin/bash

#docker build --tag zecwalletmobile/android:latest docker

#docker run --rm -v $(pwd)/..:/opt/zecwalletmobile -v $(pwd)/target/registry:/root/.cargo/registry zecwalletmobile/android:latest bash -c "
#    cd /opt/zecwalletmobile/android && \
#    CC=i686-linux-android24-clang OPENSSL_DIR=/opt/openssl-1.1.1e/x86 cargo build --target i686-linux-android --release && i686-linux-android-strip ../target/i686-linux-android/release/librust.so && \
#    CC=armv7a-linux-androideabi24-clang OPENSSL_DIR=/opt/openssl-1.1.1e/armv7 cargo build --target armv7-linux-androideabi --release && arm-linux-androideabi-strip ../target/armv7-linux-androideabi/release/librust.so && \
#    CC=aarch64-linux-android24-clang OPENSSL_DIR=/opt/openssl-1.1.1e/aarch64 cargo build --target aarch64-linux-android --release && aarch64-linux-android-strip ../target/aarch64-linux-android/release/librust.so"


#build openssl
$PWD/buildopenssl.sh

export AR=/home/cryptoforge/Dev/new-mobile-wallet/NDK/arm64/bin/aarch64-linux-android-ar
export CC=/home/cryptoforge/Dev/new-mobile-wallet/NDK/arm64/bin/aarch64-linux-android-clang
export OPENSSL_DIR=../openssl-1.1.1e/aarch64
cargo build --target aarch64-linux-android --release && aarch64-linux-android-strip ../target/aarch64-linux-android/release/librust.so

export AR=/home/cryptoforge/Dev/new-mobile-wallet/NDK/arm/bin/arm-linux-androideabi-ar
export CC=/home/cryptoforge/Dev/new-mobile-wallet/NDK/arm/bin/arm-linux-androideabi-clang
export OPENSSL_DIR=../openssl-1.1.1e/armv7
cargo build --target armv7-linux-androideabi --release && arm-linux-androideabi-strip ../target/armv7-linux-androideabi/release/librust.so

export AR=/home/cryptoforge/Dev/new-mobile-wallet/NDK/x86/bin/i686-linux-android-ar
export CC=/home/cryptoforge/Dev/new-mobile-wallet/NDK/x86/bin/i686-linux-android-clang
export OPENSSL_DIR=../openssl-1.1.1e/x86
cargo build --target i686-linux-android --release && i686-linux-android-strip ../target/i686-linux-android/release/librust.so

#mkdir -p ../../android/app/src/main/jniLibs/arm64-v8a
#mkdir -p ../../android/app/src/main/jniLibs/armeabi-v7a
#mkdir -p ../../android/app/src/main/jniLibs/x86

#cp ../target/i686-linux-android/release/librust.so   ../../android/app/src/main/jniLibs/x86/
#cp ../target/armv7-linux-androideabi/release/librust.so  ../../android/app/src/main/jniLibs/armeabi-v7a/
#cp ../target/aarch64-linux-android/release/librust.so ../../android/app/src/main/jniLibs/arm64-v8a/

cp ../target/aarch64-linux-android/release/librust.so ../../android/litewallet-jni/libs/arm64-v8a/liblitewallet-jni.so
cp ../target/armv7-linux-androideabi/release/librust.so ../../android/litewallet-jni/libs/armeabi-v7a/liblitewallet-jni.so
cp ../target/i686-linux-android/release/librust.so ../../android/litewallet-jni/libs/x86/liblitewallet-jni.so
