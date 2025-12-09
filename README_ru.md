[![English](https://img.shields.io/badge/English-%F0%9F%87%AC%F0%9F%87%A7-blue?style=for-the-badge)](README.md)

# Модуль языка Rust для Plugify

Модуль языка Rust для Plugify позволяет разработчикам писать плагины на Rust для фреймворка Plugify. Этот модуль обеспечивает бесшовную интеграцию Rust-плагинов, позволяя ядру Plugify динамически их загружать и управлять ими.

## Возможности

- **Поддержка плагинов на Rust**: Пишите плагины на Rust и легко интегрируйте их с фреймворком Plugify.
- **Автоматическая экспортируемость**: Легко экспортируйте и импортируйте методы между плагинами и языковым модулем.
- **Инициализация и завершение**: Обрабатывайте запуск, инициализацию и завершение плагина с помощью событий модуля.
- **Взаимодействие между языками**: Общение с плагинами на других языках через автоматически сгенерированные интерфейсы.

## Начало работы

### Требования

- Компилятор Rust  
- Установленный фреймворк Plugify

### Установка

#### Вариант 1: Установка через менеджер плагинов Plugify

Вы можете установить модуль языка Rust с помощью менеджера пакетов Mamba, выполнив следующую команду:

```bash
mamba install -n your_env_name -c https://untrustedmodders.github.io/plugify-module-rust/ plugify-module-rust
```

#### Вариант 2: Ручная установка

1. Установите зависимости:

   a. Windows  
   > Настройка [CMake-инструментов через Visual Studio Installer](https://learn.microsoft.com/en-us/cpp/build/cmake-projects-in-visual-studio#installation)

   b. Linux:  
   ```sh
   sudo apt-get install -y build-essential cmake ninja-build
   ```

   c. Mac:  
   ```sh
   brew install cmake ninja
   ```

2. Клонируйте репозиторий:

   ```bash
   git clone https://github.com/untrustedmodders/plugify-module-rust.git --recursive
   cd plugify-module-rust
   ```

3. Соберите модуль языка Rust:

   ```bash
   mkdir build && cd build
   cmake ..
   cmake --build .
   ```

### Использование

1. **Интеграция с Plugify**

   Убедитесь, что модуль языка Rust находится в той же директории, что и ваша установка Plugify.

2. **Создание плагинов на Rust**

   Разрабатывайте свои плагины на Rust с использованием API Plugify. Подробности смотрите в [руководстве по созданию Rust-плагинов](https://untrustedmodders.github.io/languages/rust/first-plugin).

3. **Сборка и установка плагинов**

   Скомпилируйте ваши Rust-плагины и разместите скомпилированные библиотеки в директории, доступной для ядра Plugify.

4. **Запуск Plugify**

   Запустите фреймворк Plugify — он автоматически загрузит ваши Rust-плагины.

## Пример

### Инициализация модуля

```sh
cargo new my-rust-plugin
```

### Укажите тип модуля

```toml
[lib]
crate-type = ["cdylib"]
```

### Установка модуля rust-plugify

Обратите внимание, что версия должна начинаться с `v`.

```toml
[dependencies]
plugify = { git = "https://github.com/untrustedmodders/rust-plugify" }
```

```rust
use plugify::{register_plugin};

fn on_plugin_start() {
    println!("Rust: on_plugin_start")
}

fn on_plugin_update(_dt: f32) {
    println!("Rust: on_plugin_update")
}

fn on_plugin_end() {
    println!("Rust: on_plugin_end")
}

register_plugin!(
    start: on_plugin_start,
    update: on_plugin_update,
    end: on_plugin_end
);
```

## Документация

Полную документацию по созданию плагинов на Rust для Plugify можно найти в [официальной документации Plugify](https://untrustedmodders.github.io).

## Участие

Вы можете внести вклад, открыв issue или отправив pull request. Мы будем рады вашим идеям и предложениям!

## Лицензия

Этот модуль языка Rust для Plugify распространяется по лицензии [MIT](LICENSE).
