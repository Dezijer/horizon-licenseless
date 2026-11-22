# Horizon Licenseless

Форк [Horizon](https://github.com/horizon-rust/horizon) от [Dezijer](https://github.com/Dezijer). Лаунчер в этом форке загружает публичные файлы Minecraft Java Edition с серверов Mojang и запускает клиент Horizon без входа в аккаунт Microsoft.

| Режим | Поддержка |
| --- | --- |
| Одиночная игра в Horizon | Без аккаунта Microsoft |
| Сервер с `online-mode=false` | Подключение без сессии Microsoft |
| Сервер с `online-mode=true` | Требует сессию Microsoft, которую лаунчер форка не передаёт |
| Обычные инстансы Minecraft Java Edition | Используют прежний вход через Microsoft |

Horizon проверяет контрольные суммы скачанных файлов. Игровые файлы Mojang загружаются при подготовке и не входят в этот репозиторий.

## Как запустить

Готового установщика форка пока нет. Изменения для офлайн-запуска находятся в ветке [`download-without-sign-in`](https://github.com/Dezijer/horizon-licenseless/tree/download-without-sign-in). Установи Rust 1.95 или новее и собери лаунчер из этой ветки.

```bash
git clone --branch download-without-sign-in https://github.com/Dezijer/horizon-licenseless.git
cd horizon-licenseless
cargo build --release --locked --manifest-path launcher/Cargo.toml
```

Запусти `launcher/target/release/horizon-launcher` (на Windows добавь `.exe`). Выбери сборку **Horizon** в разделе **Versions**, затем нажми **Download and prepare** или **Play**. Каталог версий по-прежнему использует релизы оригинального Horizon. Изменения офлайн-запуска находятся в собранном из этого форка лаунчере.

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
