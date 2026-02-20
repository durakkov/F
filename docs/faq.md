# FAQ

## Почему не всё через Kotlin?
Требование проекта: core-логика должна быть в C++ (NDK), Kotlin только glue.

## Можно ли сделать полностью системным?
Только с OEM/ROM интеграцией в privileged partition и platform signature.

## Почему превью генерирует Android слой?
Для совместимости и доступа к MediaStore/ThumbnailUtils/ContentResolver API.
