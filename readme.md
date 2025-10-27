

Персеры пока в процессе написания. 
Пока определена структура для проекта.

Сборка библиотеки:
```sh
cargo build -p rr-parser-lib --verbose
```
Сборка cli-утилиты :
```sh
cargo build -p rr-file-processor --verbose
```
Запуск тестов для библиоетеки
```sh
cargo test -p rr-parser-lib --verbose
```
Запуск тестов для cli-утилиты
```sh
cargo test -p rr-file-processor --verbose
```
# Запуск утилиты (пока поломан) 
```sh
target/debug/rr-file-processor \
                --in-format csv --out-format xml \
                --input  - \
                --output output/formatted/stdin_csv_to_xml
```
# refs
https://github.com/r3bb1t/coverage_formats/blob/main/Readme.md