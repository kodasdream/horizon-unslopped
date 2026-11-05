<h1 align="center">Horizon Unslopped</h1>

<p align="center">
  <b>An unslopped version of the <a href="https://github.com/gerdaroot/Horizon-Offline-Launcher">Horizon Offline Launcher</a>: the same offline launcher for <a href="https://github.com/horizon-rust/horizon">Horizon</a> (Minecraft: Java Edition 26.3 rewritten in Rust), with the violet night sky, the pixel sunset and the beveled blocks taken out.</b>
</p>

<p align="center">
  <a href="https://github.com/kodasdream/horizon-unslopped/releases/latest"><b>Releases</b></a> ·
  <a href="#what-changed">What changed</a> ·
  <a href="https://github.com/gerdaroot/Horizon-Offline-Launcher#readme">Upstream README</a> ·
  <a href="#install">Install</a>
</p>

<p align="center">
  <a href="https://github.com/kodasdream/horizon-unslopped/actions/workflows/launcher.yml"><img src="https://github.com/kodasdream/horizon-unslopped/actions/workflows/launcher.yml/badge.svg" alt="Build"></a>
  <img src="https://img.shields.io/badge/Minecraft-Java%2026.3-5D9C3F" alt="Minecraft Java 26.3">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-2A2470" alt="Windows, Linux, macOS">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-43444a" alt="MIT"></a>
</p>

## What changed

The launcher works exactly as upstream's does: offline play, your own jar, your own skin and cape,
updates from this repository. Only the look is different.

- **One flat surface instead of a night sky.** The violet gradient, the pixel sunset, the drifting
  clouds, the twinkling stars, the skyline and the letter-by-letter wordmark opening are gone. The
  window is one neutral grey, in dark and light.
- **A neutral palette with one quiet accent.** A desaturated slate blue marks the one main action
  (Play) and the focus ring, and nothing else on the screen competes with it. The purple is gone.
- **Lists, not cards.** Versions, settings, mods and patch notes are rows with a hairline between
  them, instead of a stack of outlined boxes.
- **Plain shapes.** Flat fills, 1 px rules and one corner radius; no bevels, lips or pixel-cut
  corners. Geist for everything, at four small sizes, instead of a 56 px pixel display face.
- **A sponsor line, not a poster.** Sculk's card and banner are one line with the promo code, so the
  sidebar belongs to the launcher again.

Everything upstream documents still applies; this README does not repeat it. See the
[upstream README](https://github.com/gerdaroot/Horizon-Offline-Launcher#readme) for install steps,
the FAQ, offline play and mod support.

## Updates

This fork updates itself from **its own releases only** (`kodasdream/horizon-unslopped`), signed
with its own key. An upstream release can never replace this build, so the look stays as it is.

It also follows upstream: every six hours a workflow looks for a new
[upstream launcher release](https://github.com/gerdaroot/Horizon-Offline-Launcher/releases), merges
it into this repository and tags a release this repository builds and signs. If the merge conflicts
because upstream touched a file the UI rewrite changed, nothing is published and an issue names the
files that need a person.

## What it is

[Horizon](https://github.com/horizon-rust/horizon) is Minecraft: Java Edition 26.3 rewritten in Rust: its own renderer, world, mobs and redstone, compatible with vanilla worlds. This repository is a fork of it where:

- **No account is required.** Play starts the game offline under your own name, and everything works without one: mods, resource packs, shaders, instances, servers, friends. These builds have no Microsoft sign-in at all.
- **Your own skin and cape.** Pick a skin PNG (classic or slim arms) and a cape PNG in Settings → Account: you wear them in single player and on servers that send no skin of their own.
- **You can pick your own jar.** "Choose jar…" takes a `26.3.jar` from anywhere: TLauncher, your Downloads folder, a USB stick. Without a choice, the launcher finds it by itself in the folders of the official launcher, TLauncher, Prism, MultiMC, Modrinth, ATLauncher and GDLauncher.
- **Updates come from this repository only.** The launcher and the game update themselves from its releases, signed with its own key, so an upstream update can't bring the account requirement back.
- **Horizon's latest fixes.** The game is built from Horizon's current Nightly with this fork's changes on top.
- **A real Windows installer**, or one command on any system, with desktop and Start menu shortcuts.

Everything else works as in Horizon: mods, resource packs, instances, servers, playing with friends by invite code.

> Unofficial. Not affiliated with Mojang, Microsoft or the Horizon authors. No Minecraft files are included: the game needs your own copy of Minecraft: Java Edition 26.3.

## Install

<p align="center">
  <a href="https://github.com/kodasdream/horizon-unslopped/releases/latest"><img src="https://img.shields.io/badge/Download%20for%20Windows-Setup.exe-6C4AE0?style=for-the-badge&logo=windows&logoColor=white" alt="Download for Windows"></a>
</p>

### Windows

1. **[Download `Horizon-Unslopped-Setup.exe`](https://github.com/kodasdream/horizon-unslopped/releases/latest)**.
2. **Run it.** If Windows says "Windows protected your PC", click **More info → Run anyway** (the file just isn't code-signed).
3. **Install → Finish.** The launcher opens by itself; its shortcuts are in the Start menu and, if you ticked it, on the desktop.

No administrator rights needed: it installs for your user only, in `%LOCALAPPDATA%\Programs\HorizonOfflineLauncher`. To remove it: Settings → Apps → Horizon Offline Launcher → Uninstall. Your worlds and settings stay.

<details>
<summary>Without the installer: one command in PowerShell</summary>

Win+X → Terminal, then:

```powershell
irm https://github.com/kodasdream/horizon-unslopped/releases/latest/download/install.ps1 | iex
```

It puts the launcher in the same place, with the same shortcuts.
</details>

### Linux and macOS

```sh
curl -fsSL https://github.com/kodasdream/horizon-unslopped/releases/latest/download/install.sh | sh
```

Then start it with `horizon-offline-launcher`. On Linux it adds itself to the applications menu.

## First run

1. **Have Minecraft 26.3.** If you don't yet, download and start version **26.3** once in any launcher (official, TLauncher, Prism…).
2. **Open Horizon Offline Launcher.** It downloads the latest Horizon by itself.
3. **Press "Prepare"** on the Resources row. The launcher finds your `26.3.jar` and builds the textures and sounds from it: once, in a few minutes. If it can't find the jar, press **"Choose jar…"** and pick `26.3.jar`; it is usually in `.minecraft\versions\26.3\`.
4. **Press "Play".**

Your offline name is changed with the player button at the top right; its skin in Settings → Account.

## FAQ

**Will a jar of another version (1.21, say) work?** No. Horizon reproduces 26.3 exactly: its data and network protocol are tied to that version. The launcher checks the file's checksum and says so when the jar isn't the right one.

**Preparing asks for Java.** Preparing needs Java 25. The launcher looks for it in other launchers and downloads it when it finds none. If that fails, install [Java 25](https://adoptium.net/temurin/releases/?version=25) and try again.

**How do I play with friends?** Pause → Friends gives an invite code; LAN works too. Any server that lets players without an account in (`online-mode=false`) works; online-mode servers (Hypixel and the like) check every player's Minecraft license with Mojang and don't.

**Do other players see my skin?** Skins between players go through Mojang's servers, which only take a licensed account's. On servers for players without an account, set yours with the server's skin command (SkinsRestorer: `/skin <name or link>`): everyone sees that one. The skin you pick in the launcher is the one you see on yourself where the server gives none. Capes aren't drawn yet.

**Do resource packs work?** Yes, as in Horizon: Options → Resource Packs in the game, or the launcher's resource packs page.

**Where are my worlds?** Where the official Horizon launcher keeps them: both use the same data folder, so worlds, instances and settings are shared.

**How do I update?** The launcher does it by itself: it updates itself and offers new game versions from this repository's releases. Launchers 1.0.x never update themselves: run the installer (or the install command) once more to get one that does. Worlds and settings stay.

## Building from source

Windows (needs [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with "Desktop development with C++"):

```powershell
git clone https://github.com/kodasdream/horizon-unslopped
cd Horizon-Offline-Launcher
powershell -ExecutionPolicy Bypass -File build.ps1
```

`build.ps1` installs Rust when it is missing, finds the Windows SDK's shader compiler `fxc.exe` on any drive, builds the launcher and installs it with shortcuts.

Linux and macOS:

```sh
cd launcher
cargo build --release
./target/release/horizon-launcher
```

Linux needs `libxkbcommon-x11-dev libwayland-dev libx11-xcb-dev libfontconfig-dev libvulkan-dev libasound2-dev` first.

GitHub Actions builds Windows, Linux and macOS (Apple Silicon and Intel):

- A `launcher@1.3.0` tag: the launcher, with the Windows installer and the install scripts. It becomes the repository's latest release.
- The game, like Horizon, ships as Nightlies: running the `game` workflow puts this fork's client into Horizon's current Nightly files and publishes them as the `nightly` pre-release, "Nightly <date> · <commit>". The previous one stays a week as `nightly-<date>`.

Both are signed with the repository's key (the `RELEASE_SIGNING_KEY` secret); launchers refuse anything else.

## How it differs from Horizon

| | Horizon | Horizon Offline Launcher |
|---|---|---|
| Microsoft account | required | not needed |
| Your own skin offline | no | yes, Settings → Account |
| Windows installer | no, a zip | yes, Setup.exe |
| Your own jar file | no, launcher folders only | yes, "Choose jar…" |
| Launcher updates | automatic, from Horizon | automatic, from this repository |
| The game | Horizon releases | Horizon releases with this fork's client |

The launcher's changes: `launcher/src/account.rs` (signing in is no longer required), `launcher/src/sources.rs` (`from_jar`), `launcher/src/main.rs` (the UI), `launcher/src/backend.rs` (releases from this repository). In the game: `crates/horizon-protocol/src/frame.rs` (compressed packets as vanilla reads them), `crates/horizon-client/src/skins.rs` (the offline skin). The installer: `packaging/windows/setup.iss`.

## Credits and license

The game and nearly all of the code are [Horizon](https://github.com/horizon-rust/horizon), by Ludovic Marie and the Horizon contributors. MIT license, see [LICENSE](LICENSE). The game's full documentation: [playhorizon.dev/docs](https://playhorizon.dev/docs/).

---

## Русский

**Horizon Offline Launcher** — лаунчер для [Horizon](https://github.com/horizon-rust/horizon) (Minecraft: Java Edition 26.3, переписанный на Rust) без обязательного аккаунта Microsoft и с выбором своего jar.

<p>
  <a href="https://github.com/kodasdream/horizon-unslopped/releases/latest"><b>Скачать для Windows</b></a> ·
  <a href="#установка">Установка</a> ·
  <a href="#первый-запуск">Первый запуск</a> ·
  <a href="#вопросы">Вопросы</a> ·
  <a href="#what-it-is">English</a>
</p>

### Что это

[Horizon](https://github.com/horizon-rust/horizon) — это Minecraft: Java Edition 26.3, заново написанный на Rust: свои рендер, мир, мобы и редстоун, совместимые с ванильными мирами. Этот репозиторий — его форк, в котором:

- **Аккаунт не нужен.** «Играть» запускает игру офлайн под вашим ником, и без аккаунта работает всё: моды, ресурс-паки, шейдеры, сборки, серверы, друзья. Входа через Microsoft в этих сборках нет вообще.
- **Свой скин и плащ.** PNG скина (обычные или тонкие руки) и PNG плаща выбираются в «Настройки → Аккаунт»: они видны на вас в одиночной игре и на серверах, которые не дают свой скин.
- **Можно выбрать свой jar.** Кнопка «Выбрать jar…» берёт файл `26.3.jar` откуда угодно: из TLauncher, папки «Загрузки» или с флешки. Без выбора лаунчер сам ищет его в папках официального лаунчера, TLauncher, Prism, MultiMC, Modrinth, ATLauncher и GDLauncher.
- **Обновления только из этого репозитория.** Лаунчер и игра обновляются сами из его релизов, подписанных своим ключом, поэтому официальное обновление не вернёт обязательный вход.
- **Свежие исправления Horizon.** Игра собирается из текущего Nightly Horizon с изменениями этого форка поверх.
- **Настоящий установщик для Windows** или одна команда на любой системе. Ярлыки на рабочем столе и в меню «Пуск» создаются сами.

Всё остальное — как в Horizon: моды, ресурс-паки, сборки, серверы, игра с друзьями по коду.

> Неофициальный проект, не связан с Mojang, Microsoft и авторами Horizon. Файлов Minecraft здесь нет: игре нужен ваш собственный Minecraft: Java Edition 26.3.

### Установка

<p align="center">
  <a href="https://github.com/kodasdream/horizon-unslopped/releases/latest"><img src="https://img.shields.io/badge/%D0%A1%D0%BA%D0%B0%D1%87%D0%B0%D1%82%D1%8C%20%D0%B4%D0%BB%D1%8F%20Windows-Setup.exe-6C4AE0?style=for-the-badge&logo=windows&logoColor=white" alt="Скачать для Windows"></a>
</p>

**Windows**

1. **[Скачайте `Horizon-Unslopped-Setup.exe`](https://github.com/kodasdream/horizon-unslopped/releases/latest)**.
2. **Запустите его.** Если Windows покажет «Windows защитила ваш компьютер», нажмите **«Подробнее» → «Выполнить в любом случае»** (файл просто не подписан сертификатом).
3. **«Установить» → «Завершить».** Лаунчер откроется сам; ярлыки — в меню «Пуск» и, если отметили галочку, на рабочем столе.

Права администратора не нужны: установка только для вашего пользователя, в `%LOCALAPPDATA%\Programs\HorizonOfflineLauncher`. Удалить: «Параметры» → «Приложения» → Horizon Offline Launcher → «Удалить». Миры и настройки останутся.

<details>
<summary>Без установщика: одна команда в PowerShell</summary>

Win+X → «Терминал», затем:

```powershell
irm https://github.com/kodasdream/horizon-unslopped/releases/latest/download/install.ps1 | iex
```

Лаунчер встанет в то же место, с теми же ярлыками.
</details>

**Linux и macOS:**

```sh
curl -fsSL https://github.com/kodasdream/horizon-unslopped/releases/latest/download/install.sh | sh
```

После установки лаунчер запускается командой `horizon-offline-launcher`. На Linux он сам добавит себя в меню приложений.

### Первый запуск

1. **Нужен Minecraft 26.3.** Если его ещё нет, скачайте и один раз запустите версию **26.3** в любом лаунчере (официальном, TLauncher, Prism и т. д.).
2. **Откройте Horizon Offline Launcher.** Он сам скачает последнюю версию Horizon.
3. **Нажмите «Подготовить»** в строке «Ресурсы». Лаунчер найдёт ваш `26.3.jar` и соберёт из него текстуры и звуки. Это делается один раз и занимает несколько минут. Если jar не нашёлся, нажмите **«Выбрать jar…»** и укажите файл `26.3.jar`; обычно он лежит в `.minecraft\versions\26.3\`.
4. **Нажмите «Играть».**

Ник для офлайн-игры меняется вверху справа, кнопкой с именем игрока, а его скин — в «Настройки → Аккаунт».

### Вопросы

**Подойдёт jar другой версии, например 1.21?** Нет. Horizon воспроизводит именно 26.3, его данные и сетевой протокол привязаны к этой версии. Лаунчер проверяет контрольную сумму файла и, если jar не тот, прямо говорит об этом.

**«Подготовка» просит Java.** Для подготовки нужна Java 25. Лаунчер ищет её у других лаунчеров и, если не находит, скачивает сам. Если не вышло, установите [Java 25](https://adoptium.net/temurin/releases/?version=25) и повторите.

**Как играть с друзьями?** Пауза → «Друзья» даёт код приглашения, LAN тоже работает. Подходит любой сервер, который пускает игроков без аккаунта (`online-mode=false`). Серверы с online-mode (Hypixel и подобные) проверяют лицензию каждого игрока у Mojang и не пустят.

**Видят ли другие мой скин?** Скины между игроками раздают серверы Mojang, а туда загружается только скин лицензии. На серверах для игроков без аккаунта ставьте скин командой сервера (SkinsRestorer: `/skin <ник или ссылка>`) — его увидят все. Скин из лаунчера — тот, что вы видите на себе там, где сервер не дал свой. Плащи пока не рисуются.

**Ресурс-паки работают?** Да, как в Horizon: «Настройки → Пакеты ресурсов» в игре или страница ресурс-паков в лаунчере.

**Где мои миры?** Там же, где у официального лаунчера Horizon: обе программы используют одну папку данных, так что миры, сборки и настройки общие.

**Как обновиться?** Лаунчер делает это сам: обновляется и предлагает новые версии игры из релизов этого репозитория. Лаунчеры 1.0.x сами не обновляются: запустите установщик (или команду установки) ещё раз, чтобы получить тот, что умеет. Миры и настройки останутся на месте.

### Сборка из исходников

Windows (нужны [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) с «Разработкой классических приложений на C++»):

```powershell
git clone https://github.com/kodasdream/horizon-unslopped
cd Horizon-Offline-Launcher
powershell -ExecutionPolicy Bypass -File build.ps1
```

`build.ps1` сам поставит Rust, если его нет, найдёт компилятор шейдеров `fxc.exe` из Windows SDK на любом диске, соберёт лаунчер и установит его с ярлыками.

Linux и macOS:

```sh
cd launcher
cargo build --release
./target/release/horizon-launcher
```

На Linux перед сборкой нужны пакеты `libxkbcommon-x11-dev libwayland-dev libx11-xcb-dev libfontconfig-dev libvulkan-dev libasound2-dev`.

GitHub Actions собирает Windows, Linux и macOS (Apple Silicon и Intel):

- Тег `launcher@1.3.0` — лаунчер, с установщиком для Windows и скриптами установки. Становится последним релизом репозитория.
- Игра, как и у Horizon, выходит Nightly-сборками: запуск workflow `game` кладёт клиент этого форка в файлы текущего Nightly Horizon и публикует их пре-релизом `nightly`, «Nightly <дата> · <коммит>». Предыдущий хранится неделю как `nightly-<дата>`.

Оба подписываются ключом репозитория (секрет `RELEASE_SIGNING_KEY`), всё остальное лаунчеры отвергают.

### Чем отличается от Horizon

| | Horizon | Horizon Offline Launcher |
|---|---|---|
| Аккаунт Microsoft | обязателен | не нужен |
| Свой скин без аккаунта | нет | да, «Настройки → Аккаунт» |
| Установщик для Windows | нет, zip | да, Setup.exe |
| Свой jar-файл | нет, только папки лаунчеров | да, кнопка «Выбрать jar…» |
| Обновление лаунчера | само, из Horizon | само, из этого репозитория |
| Игра | релизы Horizon | релизы Horizon с клиентом этого форка |

Изменения лаунчера: `launcher/src/account.rs` (вход больше не обязателен), `launcher/src/sources.rs` (`from_jar`), `launcher/src/main.rs` (интерфейс), `launcher/src/backend.rs` (релизы из этого репозитория). В игре: `crates/horizon-protocol/src/frame.rs` (сжатые пакеты читаются как в ванилле), `crates/horizon-client/src/skins.rs` (офлайн-скин). Установщик: `packaging/windows/setup.iss`.

### Благодарности и лицензия

Игра и почти весь код — [Horizon](https://github.com/horizon-rust/horizon) от Ludovic Marie и участников проекта. Лицензия MIT, см. [LICENSE](LICENSE). Подробная документация по игре: [playhorizon.dev/docs](https://playhorizon.dev/docs/).
