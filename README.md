# Horizon Licenseless

**[Скачать исходники офлайн-версии (ZIP)](https://github.com/Dezijer/horizon-licenseless/archive/refs/heads/download-without-sign-in.zip)** · [Открыть ветку офлайн-версии](https://github.com/Dezijer/horizon-licenseless/tree/download-without-sign-in)

Форк [Horizon](https://github.com/horizon-rust/horizon) от [Dezijer](https://github.com/Dezijer) для одиночной игры и серверов с `online-mode=false`. Ветка `download-without-sign-in` запускает клиент Horizon без аккаунта Microsoft. Серверы, которые требуют авторизацию Microsoft, отклонят подключение.

## Сборка и первый запуск

Готового установщика форка пока нет. Скачай ZIP по ссылке выше и распакуй его или клонируй нужную ветку:

```bash
git clone --branch download-without-sign-in https://github.com/Dezijer/horizon-licenseless.git
cd horizon-licenseless
```

Установи Rust 1.95 или новее и выполни в папке проекта:

```bash
cargo build --release --locked --manifest-path launcher/Cargo.toml
```

1. Запусти `launcher/target/release/horizon-launcher` (на Windows — `horizon-launcher.exe`).
2. В разделе **Versions** установи сборку **Horizon**.
3. Нажми **Download and prepare**, затем **Play**. Лаунчер скачает необходимые файлы с серверов Mojang и сверит контрольные суммы.

Каталог **Versions** использует сборки движка оригинального Horizon. Для запуска без аккаунта нужен лаунчер, собранный из ветки форка по инструкции выше. Файлы Mojang не входят в ZIP с исходниками.

## Многопоточность

Horizon уже обрабатывает чанки в фоновых потоках. Для выделенного сервера можно включить параллельную обработку части тика в `server.toml`.

```toml
[engine]
parallel_tick = true
```

[Документация клиента](docs/native-client.md) · [Настройка сервера](docs/hosting.md#parallel-tick-parallel_tick)

## Авторы и лицензия

Оригинальный Horizon основал [Ludovic Marie](https://lu4ovic.com). Его развивают команда Horizon и [участники проекта](https://github.com/horizon-rust/horizon/graphs/contributors).

Код Horizon распространяется по [GNU LGPL 3.0 или более поздней версии](LICENSE). Код, выпущенный до перехода с MIT, также доступен по [MIT](LICENSES/MIT.txt). Сторонние компоненты сохраняют свои [лицензии](THIRD_PARTY_NOTICES.md).

Horizon Licenseless — неофициальный проект, не связанный с Mojang или Microsoft. Minecraft и его ресурсы принадлежат их правообладателям. [Minecraft EULA](https://www.minecraft.net/en-us/eula)
