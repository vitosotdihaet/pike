# Cargo plugin for Picodata plugins

Плагин к cargo с функциями для упрощения разработки плагинов к Пикодате.

## Установка

TODO: После публикации на crates.io принцип установки будет существенно упрощен.

```bash
git clone git@git.picodata.io:picodata/plugin/cargo.git
cd cargo
cargo install --path . --bin cargo-pike --locked --force
```

## Quickstart

Начнем работу с новым плагином:

```bash
cargo pike plugin new test_plugin

cd test_plugin
cargo build
```

Запустим кластер, конфигурацию которого можно задать в `./topology.toml`

```bash
cargo pike run
```

В вашем распоряжении окажется рабочий кластер с установленным плагином.
Остановим кластер комбинацией `Ctrl+C` или же командой `cargo pike stop` в отдельном окне.

Если вам нужно собрать архив для поставки на сервера, это можно сделать командой:

```bash
cargo pike plugin pack
```

В папке `target` появиться желанный архив.

## Команды

### `--help`

Для всех команд есть флаг `--help` выводящий справку по использованию.

```bash
cargo pike --help
```

### `run`

Запуск кластера пикодаты по файлу `topology.toml`. Автоматически запускает плагины указанные в топологии.

Пример топологии:

```toml
[tiers.default]
instances = 2
replication_factor = 2

[[tiers.default.services]]
name = "main"
plugin = "plugin_name"
```

```bash
cargo pike run --topology topology.toml --data-dir ./tmp
```

Для отключения автоматической установки и включения плагинов можно использовать опцию `--disable-install-plugins`.

### `stop`

Остановить кластер можно либо комбинацией клавиш Ctrl+C в терминале, где вызывалась команда `cargo pike run`, либо в другом окне командой:

```bash
cargo pike stop --data-dir ./tmp
```

При помощи `--data-dir` указывается путь до директории с файлами кластера _(значение по умолчанию: ./tmp)_

Вывод:

```bash
[*] stopping picodata cluster, data folder: ./tmp
[*] stopping picodata instance: i_1
[*] stopping picodata instance: i_2
[*] stopping picodata instance: i_3
[*] stopping picodata instance: i_4
```

### `plugin clean`

Очистка дата-каталогов пикодаты.

```bash
cargo pike clean
```

### `plugin new`

Создание нового проекта плагина из шаблона.

```bash
cargo pike plugin new name_of_new_plugin
```

Автоматически инициализирует в проект git. Для отключения этого поведения можно воспользоваться флагом `--without-git`.

### `plugin init`

Создание нового проекта плагина из шаблона в текущей папке.

```bash
cargo pike plugin init
```

Автоматически инициализирует в проект git. Для отключения этого поведения можно воспользоваться флагом `--without-git`.

### `plugin pack`

Сборка всех нужных для поставки плагина файлов в один архив (для деплоя или поставки).

```bash
cargo pike plugin pack
```

Команда `plugin pack` соберёт релизную версию плагина в новый архив в директории `target` проекта.

### `config apply`

Применение конфигурации сервисов плагина к запущенному командой `run` кластеру пикодаты.

Пример файла конфигурации сервисов:

```yaml
# plugin_config.yaml

main: # имя сервиса
  value: changed # пример параметра конфигурации
```

```bash
cargo pike config apply
```
