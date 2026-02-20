# Архитектура Woxel

## Принципы
- UI только во Flutter.
- Бизнес-логика и data-flow в C++ core.
- Kotlin слой — platform adapters only (SAF, MediaStore, permissions, notifications, service, PackageManager).

## Слои

### C++ (Hexagonal)
- `domain/`: entities, use-cases, error model.
- `data/`: filesystem/sqlite adapters.
- `platform/`: Android bridge abstractions.
- `thumbnail/`: queue + cache index + dedupe.
- `archive/`: ZIP реализация + TAR/GZ интерфейсы.
- `network/`: SFTP stub + protocol interfaces.

### Bindings
- C ABI (`extern "C"`) для Dart FFI.
- JNI helper для Android-specific hooks.

### Flutter
- Экраны + state (ChangeNotifier).
- Никакой тяжелой логики в UI.
