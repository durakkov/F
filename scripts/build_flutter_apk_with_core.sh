#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
FLUTTER_DIR="$ROOT_DIR/app_flutter"
HOST_ANDROID_DIR="$ROOT_DIR/android"

echo "[1/6] Removing previous Flutter APK artifacts"
rm -f "$FLUTTER_DIR/build/app/outputs/flutter-apk/app-debug.apk" \
      "$FLUTTER_DIR/build/app/outputs/flutter-apk/app-debug.apk.sha1"

if [[ ! -d "$FLUTTER_DIR/android" ]]; then
  echo "[2/6] Creating Flutter Android host project"
  (cd "$FLUTTER_DIR" && flutter create . --platforms=android --org com.woxel.app)
else
  echo "[2/6] Flutter Android host already exists"
fi

echo "[3/6] Ensuring local Dart bindings path is available"
mkdir -p "$FLUTTER_DIR/lib/bindings/dart"
cp "$ROOT_DIR/bindings/dart/woxel_bindings.dart" "$FLUTTER_DIR/lib/bindings/dart/woxel_bindings.dart"

TMP_EXTRACT_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_EXTRACT_DIR"' EXIT

WRAPPER_PROPS="$HOST_ANDROID_DIR/gradle/wrapper/gradle-wrapper.properties"
if [[ -f "$WRAPPER_PROPS" ]] && grep -q "gradle-4\." "$WRAPPER_PROPS"; then
  echo "[4/6] Detected legacy Gradle 4 wrapper in android/. Removing legacy wrapper"
  rm -f "$HOST_ANDROID_DIR/gradlew" "$HOST_ANDROID_DIR/gradlew.bat"
  rm -rf "$HOST_ANDROID_DIR/gradle/wrapper"
fi

if [[ ! -x "$HOST_ANDROID_DIR/gradlew" ]]; then
  if [[ -x "$FLUTTER_DIR/android/gradlew" ]]; then
    echo "[4/6] Gradle wrapper not found in android/. Copying wrapper from app_flutter/android"
    cp "$FLUTTER_DIR/android/gradlew" "$HOST_ANDROID_DIR/gradlew"
    [[ -f "$FLUTTER_DIR/android/gradlew.bat" ]] && cp "$FLUTTER_DIR/android/gradlew.bat" "$HOST_ANDROID_DIR/gradlew.bat"
    mkdir -p "$HOST_ANDROID_DIR/gradle/wrapper"
    cp "$FLUTTER_DIR/android/gradle/wrapper/gradle-wrapper.jar" "$HOST_ANDROID_DIR/gradle/wrapper/gradle-wrapper.jar"
    cp "$FLUTTER_DIR/android/gradle/wrapper/gradle-wrapper.properties" "$HOST_ANDROID_DIR/gradle/wrapper/gradle-wrapper.properties"
    chmod +x "$HOST_ANDROID_DIR/gradlew"
  else
    echo "[4/6] Gradle wrapper not found. Generating wrapper via system gradle (forcing modern Gradle)"
    (cd "$HOST_ANDROID_DIR" && gradle wrapper --gradle-version 8.14.3)
    chmod +x "$HOST_ANDROID_DIR/gradlew"
  fi
fi

echo "[4/6] Building Android host APK that contains libwoxel_core.so"
(cd "$HOST_ANDROID_DIR" && ./gradlew :app:assembleDebug --no-daemon)

HOST_APK="$HOST_ANDROID_DIR/app/build/outputs/apk/debug/app-debug.apk"
if [[ ! -f "$HOST_APK" ]]; then
  echo "Host APK not found: $HOST_APK" >&2
  exit 1
fi

echo "[5/6] Extracting libwoxel_core.so from host APK and copying into Flutter jniLibs"
unzip -qo "$HOST_APK" "lib/*/libwoxel_core.so" -d "$TMP_EXTRACT_DIR"
for ABI_DIR in "$TMP_EXTRACT_DIR"/lib/*; do
  ABI="$(basename "$ABI_DIR")"
  mkdir -p "$FLUTTER_DIR/android/app/src/main/jniLibs/$ABI"
  cp "$ABI_DIR/libwoxel_core.so" "$FLUTTER_DIR/android/app/src/main/jniLibs/$ABI/libwoxel_core.so"
done

echo "[6/6] Building Flutter debug APK"
(cd "$FLUTTER_DIR" && flutter clean && flutter pub get && flutter build apk --debug)

echo "Done. APK: $FLUTTER_DIR/build/app/outputs/flutter-apk/app-debug.apk"
