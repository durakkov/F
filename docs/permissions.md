# Permissions и безопасность

## Android permissions
- `READ_EXTERNAL_STORAGE`
- `WRITE_EXTERNAL_STORAGE` (legacy use-cases на API < 29)
- `FOREGROUND_SERVICE`
- `POST_NOTIFICATIONS` (API 33+)

## SAF
- Для внешней SD/OTG используйте `ACTION_OPEN_DOCUMENT_TREE`.
- Сохраняйте `takePersistableUriPermission`.
- URI операции прокидываются в C++ через document handle abstraction.

## Безопасность
- Release builds не логируют приватные пути.
- Path traversal защита в C++ (`..`, null bytes, canonical checks).
- Ошибки типизированы: code/category/message.
