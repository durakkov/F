# Woxel

Woxel — файловый менеджер для Android 7.0+ (minSdk 24) с **Flutter UI** и **C++ NDK core**.

## Структура монорепозитория

- `app_flutter/` — Flutter UI, state management, FFI-оркестрация.
- `android/` — Android host (Kotlin glue без UI, service/permissions/SAF/media APIs), Gradle/NDK/CMake.
- `core_cpp/` — C++ core (fs операции, поиск, индексация, категория/превью-кэш, архивы, интерфейсы протоколов).
- `bindings/` — C API headers, Dart FFI bindings, JNI bridge headers.
- `docs/` — архитектура, безопасность, permissions, FAQ.

## Что реализовано (MVP)

- Просмотр файлов (локальные пути + фильтры по категориям).
- Операции: copy/move/rename/delete/create folder через C++ core.
- Поиск по имени (текущая папка/глобально через индекс sqlite).
- Категоризация: MIME/extension/magic bytes fallback.
- Очередь задач операций с прогрессом и отменой.
- Android foreground service + notification для фоновых задач.
- Превью через Android Kotlin glue (MediaStore/ThumbnailUtils) + индекс кэша ключей в C++ sqlite.
- SAF hooks (через Android glue API) для проблемных областей записи.

## Сборка и запуск

### Требования
- Flutter stable
- Android SDK + NDK (через Android Studio)
- CMake 3.22+
- Ninja

### Команды

```bash
# C++ core unit tests
cmake -S core_cpp -B build/core -G Ninja
cmake --build build/core
ctest --test-dir build/core --output-on-failure

# Flutter dependencies + format + tests
cd app_flutter
flutter pub get
flutter test
flutter build apk --debug

# Android host build (если нужно отдельно)
cd ../android
./gradlew :app:assembleDebug
```

## Ограничения “системного” режима

Woxel может быть выбран как обработчик интентов (view/open/browse), но полноценный режим системного файлового менеджера требует:

- предустановку в `/system/priv-app`,
- подпись OEM/ROM ключами,
- platform privileged permissions на уровне прошивки.

На обычном устройстве: **Settings → Apps → Default apps** и выбор Woxel как приложения по умолчанию для поддерживаемых MIME/интентов.

## SAF и Android 7 ограничения

- Запись в root внешней SD может требовать SAF URI grant.
- Для SD/OTG используйте выбор дерева документов и персистентные grants.
- Подробности: `docs/permissions.md`.

## Roadmap (advanced)

- Полный TAR/GZ модуль.
- Доп. сетевые протоколы SMB/WebDAV и production-ready SFTP.
- Root mode (опционально, disabled by default).
- Расширенный storage analyzer и dedupe.
