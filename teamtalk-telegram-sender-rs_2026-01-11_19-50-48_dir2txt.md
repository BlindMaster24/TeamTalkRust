# Project Analysis for: `teamtalk-telegram-sender-rs`

**Root Directory:** `C:\Users\kirill\teamtalk-telegram-sender-rs`

## Project Structure

```text
teamtalk-telegram-sender-rs
├── Cargo.lock
├── Cargo.toml
├── GEMINI.md
├── README.md
├── config.toml.example
├── locales
│   ├── en
│   │   └── messages.ftl
│   └── ru
│       └── messages.ftl
├── migrations
│   └── 20260109092909_initial_schema.sql
└── src
    ├── bridge.rs
    ├── config.rs
    ├── db
    │   ├── admins.rs
    │   ├── bans.rs
    │   ├── deeplinks.rs
    │   ├── mod.rs
    │   ├── mutes.rs
    │   ├── subscriptions.rs
    │   ├── types.rs
    │   └── user_settings.rs
    ├── locales.rs
    ├── main.rs
    ├── tg_bot
    │   ├── admin_logic
    │   │   ├── bans.rs
    │   │   ├── mod.rs
    │   │   ├── subscriber_settings.rs
    │   │   ├── subscribers.rs
    │   │   └── utils.rs
    │   ├── callback_handlers
    │   │   ├── admin.rs
    │   │   ├── menu.rs
    │   │   ├── mod.rs
    │   │   ├── mute.rs
    │   │   ├── settings.rs
    │   │   ├── subscriber.rs
    │   │   └── unsub.rs
    │   ├── callbacks.rs
    │   ├── callbacks_types.rs
    │   ├── commands.rs
    │   ├── keyboards.rs
    │   ├── mod.rs
    │   ├── settings_logic.rs
    │   ├── state.rs
    │   └── utils.rs
    ├── tt_worker
    │   ├── commands.rs
    │   ├── events.rs
    │   ├── mod.rs
    │   └── reports.rs
    └── types.rs
```

## File Contents

---

### `Cargo.lock`

_[BINARY FILE SKIPPED]_

---

### `Cargo.toml`

```toml
[package]
name = "teamtalk-telegram-sender-rs"
version = "0.2.0"
edition = "2024"

[profile.release]
lto = true              # "Fat" LTO
codegen-units = 1
strip = true
panic = "abort"
opt-level = 3

[dependencies]
tokio = { version = "1.36", features = ["rt-multi-thread", "macros", "net", "time", "sync", "signal"] }
teloxide = { version = "0.17", default-features = false, features = ["macros", "native-tls", "ctrlc_handler"] }
sqlx = { version = "0.8", default-features = false, features = ["runtime-tokio-native-tls", "sqlite", "chrono", "macros", "migrate"] }
serde = { version = "1.0", features = ["derive"] }
toml = "0.9"
anyhow = "1.0"
chrono = { version = "0.4", default-features = false, features = ["clock", "serde", "std"] }
dashmap = "6.1"
uuid = { version = "1.7", features = ["fast-rng", "v7"] }
teamtalk = "1.0"
fluent-templates = { version = "0.13", features = ["macros"] }
unic-langid = "0.9"
self_update = { version = "0.42", features = ["archive-zip", "archive-tar", "compression-zip-deflate", "compression-flate2"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }
tracing-futures = "0.2"
```

---

### `config.toml.example`

```
[teamtalk]
host_name = "example.com"
port = 10333
user_name = "bot_username"
nick_name = "Telegram Bot"
password = "YOUR_TT_PASSWORD_HERE"
channel = "/"
channel_password = ""
encrypted = false
client_name = "telegrambot"
gender = "male"
server_name = ""
status_text = "Bot status text"
global_ignore_usernames = ["admin_bot"]

[telegram]
message_token = "YOUR_TELEGRAM_BOT_TOKEN_HERE"
event_token = "YOUR_EVENT_BOT_TOKEN_HERE"
admin_chat_id = 123456789

[general]
admin_username = "AdminTG"
default_lang = "ru"

[database]
db_file = "bot_data.db"
```

---

### `GEMINI.md`

```markdown
# GEMINI.md - teamtalk-telegram-sender-rs

## Project Overview

This project, `teamtalk-telegram-sender-rs`, is a Rust-based application that acts as a bridge between a TeamTalk server and the Telegram messaging platform. Its primary function is to monitor a TeamTalk server for user join/leave events and broadcast these events as notifications to subscribed Telegram users. It also facilitates administrative actions on the TeamTalk server (like kicking or banning users) directly from a Telegram chat.

The application is structured into three main components:

1.  **TeamTalk Worker (`tt_worker`):** A dedicated thread that connects to the TeamTalk server using the `teamtalk` crate. It listens for server events, processes them, and sends relevant information to the bridge. It also executes commands received from the bridge, such as kicking or banning users.
2.  **Telegram Bot (`tg_bot`):** An asynchronous component built with the `teloxide` framework. It handles all interactions with users on Telegram, including processing commands, managing user settings, and sending notifications.
3.  **Bridge (`bridge`):** The central communication hub that connects the TeamTalk worker and the Telegram bot. It receives events from the `tt_worker` and dispatches them as formatted messages to the `tg_bot` to be sent to subscribed users. It also relays administrative commands from the bot to the worker.

The application uses an SQLite database (`sqlx`) to persist user data, including subscriptions, notification preferences, and administrative roles. Configuration is managed through a `config.toml` file, which specifies connection details for both the TeamTalk server and the Telegram bots (one for general events and another for admin-specific messages).

## Building and Running

### Prerequisites

*   Rust toolchain
*   A running TeamTalk server
*   Telegram Bot tokens

### Configuration

1.  Copy the `config.toml.example` to `config.toml` (if `config.toml` doesn't exist).
2.  Edit `config.toml` to provide the necessary credentials:
    *   `[teamtalk]`: Host, port, and user credentials for the TeamTalk server.
    *   `[telegram]`: API tokens for the event and message bots, and the admin's chat ID.
    *   `[database]`: Path to the SQLite database file.

### Building

To build the project in release mode, run the following command:

```sh
cargo build --release
```

The optimized executable will be located at `target/release/teamtalk-telegram-sender-rs`.

### Running

To run the application, execute the compiled binary:

```sh
./target/release/teamtalk-telegram-sender-rs --config config.toml
```

## Development Conventions

*   **Asynchronous Runtime:** The project heavily relies on `tokio` for its asynchronous operations, particularly for the Telegram bot component.
*   **Concurrency:** The TeamTalk client runs in a separate OS thread to avoid blocking the async runtime, communicating with the rest of the application via channels. The bridge and Telegram bot run as async tasks on the Tokio runtime.
*   **State Management:** Application state (like the list of online users) is shared safely across threads and tasks using `Arc<DashMap>`.
*   **Database:** `sqlx` is used for all database interactions. The database access layer is organized into several modules within the `db` package. Instead of using traits for database operations, the implementation is done using inherent `impl` blocks on the `Database` struct. Each module (e.g., `admins`, `bans`) contributes methods to the `Database` struct, promoting code organization while maintaining a single, coherent database interface.
*   **Localization:** The application uses `fluent-templates` for localization, with message files (`.ftl`) stored in the `locales` directory for different languages.
*   **Modularity:** The code is organized into distinct modules (`tt_worker`, `tg_bot`, `bridge`, `db`, `config`) with clear responsibilities, promoting separation of concerns.
```

---

### `locales/en/messages.ftl`

```
# General
hello-start = Hello! Use /help to see available commands.
help-text =
    <b>Available Commands:</b>
    /who - Show online users.
    /settings - Access the interactive settings menu (language, notifications, mute lists, NOON feature).
    /unsub - Unsubscribe from notifications.
    /help - Show this help message.
    (Note: <code>/start</code> is used to initiate the bot and process deeplinks.)

    <b>Admin Commands:</b>
    /kick - Kick a user from the server (via buttons).
    /ban - Ban a user from the server (via buttons).
    /unban - Unban a user from the server (shows a list of banned users).
    /subscribers - View and manage subscribed users.
    /exit - Shut down the bot.

cmd-invalid-deeplink = Invalid or expired deeplink.
cmd-success-sub = You have successfully subscribed to notifications.
cmd-success-sub-guest = You have subscribed as a GUEST. Note: "NOON" mode is unavailable.
cmd-success-unsub = You have successfully unsubscribed from notifications.
cmd-relink = TeamTalk account linked successfully!
cmd-fail-account = Your TeamTalk account must have a username to subscribe.
cmd-fail-noon-guest = Feature unavailable. You must have a linked TeamTalk account to use NOON mode. Please subscribe from a registered account.
cmd-error = An error occurred. Please try again later.
cmd-no-users = No users found online.
cmd-unauth = You are not authorized to perform this action.
cmd-not-subscribed = You are not subscribed. Please request a link from the bot in TeamTalk via <code>/sub</code> command.
cmd-user-banned = Your Telegram account is banned from using this service.
cmd-tt-banned = The TeamTalk username '{ $name }' is banned.
cmd-shutting-down = Shutting down...

# Unsubscribe
cmd-desc-unsub = Unsubscribe from notifications
unsub-confirm-text = Are you sure you want to unsubscribe? This will delete your settings and stop all notifications.
unsub-cancelled = Operation cancelled. You remain subscribed.
btn-yes = Yes
btn-no = No

# Notifications
event-join = { $nickname } joined server { $server }
event-leave = { $nickname } left server { $server }

# Settings Menu
settings-title = <b>Settings</b>
msg-choose-lang = Please choose your language:
btn-lang = Language
btn-sub-settings = Subscription Settings
btn-notif-settings = Notification Settings

# Notification Settings
notif-settings-title = <b>Notification Settings</b>
btn-noon = NOON (Not on Online): { $status }
btn-mute-manage = Manage Mute List
resp-noon-updated = NOON status updated: { $status }

# Mute Management
mute-title = <b>Manage Mute List</b>

    { $mode_desc }

    ⚠️ <b>Note on Guests:</b> This server allows shared guest accounts. You cannot mute a specific guest individually. Adding a guest account to the blacklist will mute <b>ALL</b> users logged in as guests.

mute-mode-blacklist = Current mode is Blacklist. You receive notifications from everyone except those on the list.
mute-mode-whitelist = Current mode is Whitelist. You only receive notifications from users on the list.

display-guest-account = 👤 Guest Account
alert-mute-guest = ⚠️ WARNING: You are muting the shared Guest account. This will mute/unmute ALL users currently logged in as guests!

btn-mode-blacklist = { $marker } Blacklist Mode
btn-mode-whitelist = { $marker } Whitelist Mode
btn-manage-list = Manage { $mode }
btn-mute-server-list = Mute/Unmute from Server List

mode-blacklist = Blacklist
mode-whitelist = Whitelist

# User List Item Status
item-status-muted = { $name } (Status: Muted)
item-status-unmuted = { $name } (Status: Not Muted)

# Pagination / Lists
list-kick-title = Select a user to kick from { $server }:
list-ban-title = Select a user to ban from { $server }:
list-unban-title = Banned Users
list-subs-title = Here is the list of subscribers.
list-mute-title = Mute list for: { $name }
list-all-accs-title = All Server Accounts
list-link-title = Select a TeamTalk account to link to subscriber { $id }:
list-empty = The list is empty.
list-subs-empty = No subscribers found.
list-ban-empty = The ban list is empty.
list-mute-empty = The mute list is currently empty.
list-page = Page { $current }/{ $total }

btn-prev = ⬅️ Prev
btn-next = Next ➡️
btn-back = Back to { $dest }
btn-back-settings = Back to Settings
btn-back-notif = Back to Notifications
btn-back-mute = Back to Mute Management
btn-back-menu = Back to Main Menu
btn-back-subs = Back to Subscribers List
btn-back-user-actions = Back to User Actions
btn-back-manage-acc = Back to Manage Account

# Toast messages
toast-mute-mode-set = Mute list mode set to { $mode }.
toast-user-muted = { $user } has been { $action }.
toast-lang-updated = Language has been changed.
toast-command-sent = Command sent.
toast-user-banned = User was banned and their profile was deleted.
toast-user-unbanned = User has been successfully unbanned.
toast-subscriber-deleted = Subscriber deleted successfully.
toast-account-unlinked = Account { $user } has been unlinked.
toast-account-linked = Successfully linked TeamTalk account: { $user }.
toast-noon-toggled = NOON for subscriber { $id } set to: { $status }.
toast-lang-set = Language for subscriber { $id } changed to { $lang }.
toast-notif-set = Notification preference for subscriber { $id } set to: { $val }.
toast-mute-mode-sub-set = Mute list mode for subscriber { $id } set to: { $val }.

act-added-blacklist = added to blacklist
act-removed-blacklist = removed from blacklist
act-added-whitelist = added to whitelist
act-removed-whitelist = removed from whitelist

status-enabled = Enabled
status-disabled = Disabled

# Admin
admin-alert =
    Message from server <b>{ $server }</b>
    From <b>{ $nick }</b>:

    { $msg }
tt-msg-sent = Message sent to Telegram.
tt-msg-failed = Failed to send message to Telegram.

# TT Commands & Responses
tt-admin-added = Successfully added { $count } admins.
tt-admin-add-fail = Failed to add { $count } admins (already admins or invalid IDs).
tt-admin-removed = Successfully removed { $count } admins.
tt-admin-remove-fail = Failed to remove { $count } admins (not admins or invalid IDs).
tt-admin-no-ids = No valid admin IDs provided for adding or removing.
tt-admin-help-header =

    Admin commands (MAIN_ADMIN from config only):
tt-admin-help-cmds =
    /add_admin <Telegram ID> [<Telegram ID>...] - Add bot admin.
    /remove_admin <Telegram ID> [<Telegram ID>...] - Remove bot admin.

tt-report-header = There are { $count } users on the server { $server }:
tt-report-unauth = (not in a channel)
tt-sub-fail-nouser = Your TeamTalk account must have a username to subscribe.
tt-sub-link = Click this link to subscribe: { $link }
tt-unsub-link = Click this link to unsubscribe: { $link }
tt-error-generic = Error. Try again.

# Icons & Symbols
icon-muted = 🔇
icon-unmuted = 🔊
icon-checked = ✅
icon-unchecked = ⚪️
icon-check-simple = ✅

# TeamTalk Report
tt-report-root = the root channel
tt-report-row = <b>{ $users }</b> in { $channel }

# Subscription Settings
btn-sub-all = { $marker } All (Join & Leave)
btn-sub-join = { $marker } Join Only
btn-sub-leave = { $marker } Leave Only
btn-sub-none = { $marker } None
resp-sub-updated = Subscription setting updated to: { $text }.

# Menu
menu-title = <b>Main Menu:</b>
btn-menu-who = ℹ️ Who is online?
btn-menu-settings = ⚙️ Settings
btn-menu-help = ❓ Help
btn-menu-kick = 👢 Kick User
btn-menu-ban = 🚫 Ban User
btn-menu-unban = ✅ Unban User
btn-menu-subs = 👥 Subscribers
btn-menu-unsub = 🚪 Unsubscribe

# Subscriber Details
sub-details-title = <b>Subscriber: { $name }</b>
    Linked TT Account: { $tt_user }
    Language: { $lang }
    NOON (Not on Online): { $noon }
    Notifications: { $notif }
    Mute Mode: { $mode }

sub-manage-tt-title = Manage TeamTalk account link for subscriber { $id }:
sub-lang-title = Select new language for subscriber { $id }:
sub-notif-title = Select notification preference for subscriber { $id }:
sub-mode-title = Select mute list mode for subscriber { $id }:

btn-sub-delete = 🗑️ Delete Subscriber
btn-sub-ban = 🚫 Ban User (TG & TT)
btn-sub-manage-tt = 🔗 Manage TeamTalk Account
btn-sub-lang = 🗣️ Change Language
btn-sub-noon = 🌞 Toggle NOON
btn-sub-notif = 🔔 Set Notification Prefs
btn-sub-mute-mode = 🔇 Set Mute Mode
btn-sub-view-mute = 📜 View Mute List
btn-unban = ✅ Unban
btn-unlink = ➖ Unlink { $user }
btn-link-new = ➕ Link/Change TeamTalk Account

# Bot Command Descriptions
cmd-desc-menu = Show main menu with all commands
cmd-desc-who = Show online users in TeamTalk
cmd-desc-help = Show this help message
cmd-desc-settings = Access interactive settings menu
cmd-desc-kick = Kick TT user (admin, via buttons)
cmd-desc-ban = Ban TT user (admin, via buttons)
cmd-desc-unban = Unban user (shows a list of banned users)
cmd-desc-subscribers = View and manage subscribed users
cmd-desc-exit = Shut down the bot
val-none = None
```

---

### `locales/ru/messages.ftl`

```
# General
hello-start = Привет! Используйте /help для просмотра доступных команд.
help-text =
    <b>Доступные команды:</b>
    /who - Показать онлайн пользователей.
    /settings - Доступ к интерактивному меню настроек (язык, уведомления, списки мьютов, функция NOON).
    /unsub - Отписаться от уведомлений.
    /help - Показать это сообщение.
    (Примечание: <code>/start</code> используется для запуска бота и обработки deeplink-ссылок.)

    <b>Команды для администраторов:</b>
    /kick - Кикнуть пользователя с сервера (через кнопки).
    /ban - Забанить пользователя на сервере (через кнопки).
    /unban - Разбанить пользователя (показывает список забаненных).
    /subscribers - Просмотр и управление подписчиками.
    /exit - Выключить бота.

cmd-invalid-deeplink = Недействительная или истекшая ссылка.
cmd-success-sub = Вы успешно подписались на уведомления.
cmd-success-sub-guest = Вы подписались как ГОСТЬ. Примечание: режим "NOON" недоступен.
cmd-success-unsub = Вы успешно отписались от уведомлений.
cmd-relink = TeamTalk аккаунт успешно привязан!
cmd-fail-account = У вашего аккаунта TeamTalk должен быть username для подписки.
cmd-fail-noon-guest = Функция недоступна. Для режима NOON нужен привязанный аккаунт TeamTalk.
cmd-error = Произошла ошибка. Попробуйте позже.
cmd-no-users = Пользователей онлайн не найдено.
cmd-unauth = У вас нет прав для этого действия.
cmd-not-subscribed = Вы не подписаны. Запросите ссылку у бота в TeamTalk командой <code>/sub</code>.
cmd-user-banned = Ваш Telegram аккаунт заблокирован и не может использовать этот сервис.
cmd-tt-banned = Имя пользователя TeamTalk '{ $name }' забанено.
cmd-shutting-down = Выключение...

# Unsubscribe
cmd-desc-unsub = Отписаться от уведомлений
unsub-confirm-text = Вы уверены, что хотите отписаться? Это удалит ваши настройки и остановит все уведомления.
unsub-cancelled = Операция отменена. Вы остаетесь подписаны.
btn-yes = Да
btn-no = Нет

# Notifications
event-join = { $nickname } присоединился к серверу { $server }
event-leave = { $nickname } покинул сервер { $server }

# Settings Menu
settings-title = <b>Настройки</b>
msg-choose-lang = Пожалуйста, выберите ваш язык:
btn-lang = Язык (Language)
btn-sub-settings = Настройки подписки
btn-notif-settings = Настройки уведомлений

# Notification Settings
notif-settings-title = <b>Настройки уведомлений</b>
btn-noon = NOON (Не в сети): { $status }
btn-mute-manage = Управление списком игнорирования
resp-noon-updated = Статус NOON обновлен: { $status }

# Mute Management
mute-title = <b>Управление списком игнорирования</b>

    { $mode_desc }

    ⚠️ <b>О гостевых аккаунтах:</b> На сервере разрешено использование общей гостевой учетной записи. Заглушить конкретного гостя невозможно — добавление гостя в черный список скроет уведомления от <b>ВСЕХ</b>, кто сидит с этого аккаунта.

mute-mode-blacklist = Текущий режим: Черный список. Вы получаете уведомления от всех, КРОМЕ тех, кто в списке.
mute-mode-whitelist = Текущий режим: Белый список. Вы получаете уведомления ТОЛЬКО от пользователей в списке.

display-guest-account = 👤 Гостевой аккаунт
alert-mute-guest = ⚠️ ВНИМАНИЕ: Вы глушите общую гостевую учетную запись. Это заглушит/разглушит ВСЕХ пользователей, которые сидят как гости!

btn-mode-blacklist = { $marker } Черный список
btn-mode-whitelist = { $marker } Белый список
btn-manage-list = Управлять { $mode }
btn-mute-server-list = Добавить/убрать из списка сервера

mode-blacklist = Черным списком
mode-whitelist = Белым списком

# User List Item Status
item-status-muted = { $name } (Статус: в игноре)
item-status-unmuted = { $name } (Статус: не в игноре)

# Pagination / Lists
list-kick-title = Выберите пользователя для кика с сервера { $server }:
list-ban-title = Выберите пользователя для бана на сервере { $server }:
list-unban-title = Забаненные пользователи
list-subs-title = Вот список подписчиков.
list-mute-title = Список игнорирования для: { $name }
list-all-accs-title = Все учетные записи сервера
list-link-title = Выберите учетную запись TeamTalk для привязки к подписчику { $id }:
list-empty = Список пуст.
list-subs-empty = Подписчики не найдены.
list-ban-empty = Список забаненных пуст.
list-mute-empty = Список игнорирования пуст.
list-page = Страница { $current }/{ $total }

btn-prev = ⬅️ Назад
btn-next = Вперед ➡️
btn-back = Назад к { $dest }
btn-back-settings = Назад в Настройки
btn-back-notif = Назад в Уведомления
btn-back-mute = Назад в Mute-меню
btn-back-menu = Назад в Главное меню
btn-back-subs = Назад к списку подписчиков
btn-back-user-actions = Назад к действиям пользователя
btn-back-manage-acc = Назад к управлению аккаунтом

# Toast messages
toast-mute-mode-set = Режим списка игнорирования изменен на { $mode }.
toast-user-muted = { $user } был { $action }.
toast-lang-updated = Язык был изменен.
toast-command-sent = Команда отправлена.
toast-user-banned = Пользователь был забанен, а его профиль удален.
toast-user-unbanned = Пользователь успешно разбанен.
toast-subscriber-deleted = Подписчик успешно удален.
toast-account-unlinked = Аккаунт { $user } был отвязан.
toast-account-linked = Аккаунт TeamTalk успешно привязан: { $user }.
toast-noon-toggled = Статус NOON для подписчика { $id } установлен на: { $status }.
toast-lang-set = Язык для подписчика { $id } изменен на { $lang }.
toast-notif-set = Настройка уведомлений для подписчика { $id } установлена на: { $val }.
toast-mute-mode-sub-set = Режим списка игнорирования для подписчика { $id } установлен на: { $val }.

act-added-blacklist = добавлен в чёрный список
act-removed-blacklist = удалён из чёрного списка
act-added-whitelist = добавлен в белый список
act-removed-whitelist = удалён из белого списка

status-enabled = Включено
status-disabled = Выключено

# Admin
admin-alert =
    Сообщение с сервера <b>{ $server }</b>
    От <b>{ $nick }</b>:

    { $msg }
tt-msg-sent = Сообщение успешно отправлено в Telegram.
tt-msg-failed = Не удалось доставить сообщение в Telegram.

# TeamTalk Admin Commands
tt-admin-added = Успешно добавлено { $count } администраторов.
tt-admin-add-fail = Не удалось добавить { $count } администраторов (уже администраторы или неверные ID).
tt-admin-removed = Успешно удалено { $count } администраторов.
tt-admin-remove-fail = Не удалось удалить { $count } администраторов (не администраторы или неверные ID).
tt-admin-no-ids = Не указаны действительные ID администраторов для добавления или удаления.
tt-admin-help-header =

    Команды администратора (только для ГЛАВНОГО АДМИНА из конфигурации):
tt-admin-help-cmds =
    /add_admin <Telegram ID> [<Telegram ID>...] - Добавить админа бота.
    /remove_admin <Telegram ID> [<Telegram ID>...] - Удалить админа бота.

tt-report-header = На сервере { $server } сейчас { $count } пользователей:
tt-report-unauth = (не в канале)
tt-sub-fail-nouser = У вашего аккаунта TeamTalk должен быть установлен username для подписки.
tt-sub-link = Нажмите на эту ссылку, чтобы подписаться на уведомления: { $link }
tt-unsub-link = Нажмите на эту ссылку, чтобы отписаться от уведомлений: { $link }
tt-error-generic = Ошибка. Попробуйте позже.

# Icons & Symbols
icon-muted = 🔇
icon-unmuted = 🔊
icon-checked = ✅
icon-unchecked = ⚪️
icon-check-simple = ✅

# TeamTalk Report
tt-report-root = корневом канале
tt-report-row = <b>{ $users }</b> в { $channel }

# Subscription Settings
btn-sub-all = { $marker } Все (Вход и выход)
btn-sub-join = { $marker } Только вход
btn-sub-leave = { $marker } Только выход
btn-sub-none = { $marker } Нет
resp-sub-updated = Настройка подписки обновлена до: { $text }.

# Menu
menu-title = <b>Главное меню:</b>
btn-menu-who = ℹ️ Кто в сети?
btn-menu-settings = ⚙️ Настройки
btn-menu-help = ❓ Помощь
btn-menu-kick = 👢 Кикнуть пользователя
btn-menu-ban = 🚫 Забанить пользователя
btn-menu-unban = ✅ Разбанить пользователя
btn-menu-subs = 👥 Подписчики
btn-menu-unsub = 🚪 Отписаться

# Subscriber Details
sub-details-title = <b>Подписчик: { $name }</b>
    Привязанный аккаунт TT: { $tt_user }
    Язык: { $lang }
    NOON (Не в сети): { $noon }
    Уведомления: { $notif }
    Режим игнорирования: { $mode }

sub-manage-tt-title = Управление привязкой TeamTalk для подписчика { $id }:
sub-lang-title = Выберите новый язык для подписчика { $id }:
sub-notif-title = Выберите настройку уведомлений для подписчика { $id }:
sub-mode-title = Выберите режим тишины для подписчика { $id }:

btn-sub-delete = 🗑️ Удалить подписчика
btn-sub-ban = 🚫 Забанить (TG и TT)
btn-sub-manage-tt = 🔗 Управлять аккаунтом TeamTalk
btn-sub-lang = 🗣️ Сменить язык
btn-sub-noon = 🌞 Переключить NOON
btn-sub-notif = 🔔 Установить настройки уведомлений
btn-sub-mute-mode = 🔇 Установить режим игнорирования
btn-sub-view-mute = 📜 Просмотреть список игнорирования
btn-unban = ✅ Разбанить
btn-unlink = ➖ Отвязать { $user }
btn-link-new = ➕ Привязать/Изменить аккаунт TeamTalk

# Bot Command Descriptions
cmd-desc-menu = Показать главное меню со всеми командами
cmd-desc-who = Показать онлайн пользователей в TeamTalk
cmd-desc-help = Показать это справочное сообщение
cmd-desc-settings = Доступ к интерактивному меню настроек
cmd-desc-kick = Кикнуть пользователя TT (админ, через кнопки)
cmd-desc-ban = Забанить пользователя TT (админ, через кнопки)
cmd-desc-unban = Разбанить пользователя (показывает список забаненных)
cmd-desc-subscribers = Просмотр и управление подписанными пользователями
cmd-desc-exit = Выключить бота
val-none = Нет
```

---

### `migrations/20260109092909_initial_schema.sql`

```sql
CREATE TABLE IF NOT EXISTS user_settings (
    telegram_id INTEGER PRIMARY KEY,
    language_code TEXT NOT NULL DEFAULT 'en',
    notification_settings TEXT NOT NULL DEFAULT 'all',
    mute_list_mode TEXT NOT NULL DEFAULT 'blacklist',
    teamtalk_username TEXT,
    not_on_online_enabled BOOLEAN NOT NULL DEFAULT 0,
    not_on_online_confirmed BOOLEAN NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS muted_users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    muted_teamtalk_username TEXT NOT NULL,
    user_settings_telegram_id INTEGER NOT NULL,
    FOREIGN KEY(user_settings_telegram_id) REFERENCES user_settings(telegram_id),
    UNIQUE(user_settings_telegram_id, muted_teamtalk_username)
);
CREATE TABLE IF NOT EXISTS subscribed_users (
    telegram_id INTEGER PRIMARY KEY
);
CREATE TABLE IF NOT EXISTS admins (
    telegram_id INTEGER PRIMARY KEY
);
CREATE TABLE IF NOT EXISTS deeplinks (
    token TEXT PRIMARY KEY,
    action TEXT NOT NULL,
    payload TEXT,
    expected_telegram_id INTEGER,
    expiry_time DATETIME NOT NULL
);
CREATE TABLE IF NOT EXISTS ban_list (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    telegram_id INTEGER,
    teamtalk_username TEXT,
    ban_reason TEXT,
    banned_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_user_settings_tt_username ON user_settings(teamtalk_username);
CREATE INDEX IF NOT EXISTS idx_user_settings_notif ON user_settings(notification_settings);
CREATE INDEX IF NOT EXISTS idx_user_settings_mute_mode ON user_settings(mute_list_mode);
CREATE INDEX IF NOT EXISTS idx_ban_list_tg_id ON ban_list(telegram_id);
CREATE INDEX IF NOT EXISTS idx_ban_list_tt_username ON ban_list(teamtalk_username COLLATE NOCASE);
CREATE INDEX IF NOT EXISTS idx_muted_users_telegram_id ON muted_users(user_settings_telegram_id);
```

---

### `README.md`

```markdown
# TeamTalk to Telegram Bridge (Rust)

A robust, asynchronous bridge between a **TeamTalk 5** server and **Telegram**, written in Rust.

This bot monitors your TeamTalk server for user activity (joins, leaves) and sends notifications to subscribed Telegram users. It also provides a full suite of moderation tools, allowing administrators to kick or ban users directly from Telegram.

## 🚀 Features

*   **Real-time Notifications:** Receive alerts when users join or leave the server.
*   **Two-Way Interaction:**
    *   Chat messages sent to the bot in TeamTalk are forwarded to the Telegram Admin.
    *   Admins can reply from Telegram back to the TeamTalk user.
*   **Admin Tools:** Kick and Ban users via an interactive Telegram interface (buttons).
*   **User Settings:**
    *   **Mute Lists:** Blacklist or Whitelist specific users/channels.
    *   **NOON (Not On Online):** Smart feature that mutes notifications if you are currently logged into TeamTalk yourself.
    *   **Localization:** Multi-language support (English and Russian included).
*   **Account Linking:** Securely link TeamTalk accounts to Telegram IDs via Deep Links.
*   **High Performance:** Built with `Tokio`, `Teloxide`, and `SQLx` for speed and safety.

## 🛠 Prerequisites

Before building, ensure you have the following installed:

*   **Rust** (latest stable toolchain): [Install Rust](https://www.rust-lang.org/tools/install)
*   **TeamTalk Server**: Version 5.x.
*   **Telegram Bot**: You will need a bot token from [@BotFather](https://t.me/BotFather).

## 📦 Installation & Building

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/kirill-jjj/teamtalk-telegram-sender-rs.git
    cd teamtalk-telegram-sender-rs
    ```

2.  **Prepare Configuration:**
    Copy the example configuration file:
    ```bash
    cp config.toml.example config.toml
    ```
    *See the [Configuration](#-configuration) section below for details.*

3.  **Build the project:**
    Since this project uses `sqlx`, it supports offline building using cached query data (`sqlx-data.json`).
    ```bash
    cargo build --release
    ```

    The compiled binary will be located at `target/release/teamtalk-telegram-sender-rs`.

## ⚙️ Configuration

Edit `config.toml` with your settings.

```toml
[teamtalk]
host_name = "your.server.com"
port = 10333
user_name = "bot_account"
password = "bot_password"
nick_name = "Telegram Bot"
# Channel path to join (e.g., "/" for root)
channel = "/"
# Optional: Text displayed in the bot's status field
status_text = "I bridge events to Telegram"

[telegram]
# Token for the main interaction bot
event_token = "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"
# Token for sending admin alerts (can be the same as event_token)
message_token = "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"
# The Telegram Chat ID of the main administrator
admin_chat_id = 123456789

[general]
admin_username = "MainAdminTTAccount"
default_lang = "en" # 'en' or 'ru'

[database]
db_file = "bot_data.db"
```

## 🏃‍♂️ Running

Run the executable. You can optionally specify the config file path:

```bash
./target/release/teamtalk-telegram-sender-rs --config config.toml
```

On the first run, the bot will automatically create the SQLite database file (`bot_data.db`) and apply all necessary migrations.

## 🤖 Bot Commands

### User Commands
*   `/start` - Initialize the bot or process deep links.
*   `/help` - Show the help message.
*   `/menu` - Open the main interactive menu.
*   `/who` - Show a list of online users in TeamTalk grouped by channel.
*   `/settings` - Open subscription and notification settings.
*   `/unsub` - Unsubscribe from notifications.

### Admin Commands (Restricted)
*   `/kick` - Open an interactive list to kick a user.
*   `/ban` - Open an interactive list to ban a user.
*   `/unban` - Manage the ban list.
*   `/subscribers` - View and manage subscribed Telegram users.
*   `/exit` - Gracefully shut down the bot.

### TeamTalk Chat Commands
If you message the bot inside the TeamTalk client:
*   `/sub` - Generates a Deep Link to subscribe to notifications.
*   `/unsub` - Generates a link to unsubscribe.
*   `/help` - Shows available TT commands.

## 💻 Development

### Database Migrations
This project uses **SQLx** for database management. If you modify the database schema, you need `sqlx-cli`.

1.  **Install CLI:**
    ```bash
    cargo install sqlx-cli
    ```
2.  **Create a `.env` file** (do not commit this):
    ```env
    DATABASE_URL=sqlite:bot_data.db
    ```
3.  **Run migrations:**
    ```bash
    sqlx migrate run
    ```
4.  **Update cached queries** (before committing changes):
    ```bash
    cargo sqlx prepare
    ```

## 🌍 Localization

Translations are handled via **Fluent** (`fluent-templates`).
*   Language files are located in `locales/`.
*   Supported languages: **English (en)**, **Russian (ru)**.
*   The bot automatically detects the user's language preference or falls back to the default defined in `config.toml`.
```

---

### `src/bridge.rs`

```rust
use crate::{
    args,
    config::Config,
    db::Database,
    locales,
    types::{self, BridgeEvent},
};
use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::Arc;
use teloxide::ApiError;
use teloxide::RequestError;
use teloxide::{prelude::*, utils::html};
use tokio::task::JoinSet;

#[allow(clippy::too_many_arguments)]
pub async fn run_bridge(
    mut rx_bridge: tokio::sync::mpsc::Receiver<BridgeEvent>,
    db_clone: Database,
    online_users_by_username: Arc<DashMap<String, i32>>,
    config: Arc<Config>,
    event_bot: Option<Bot>,
    msg_bot: Option<Bot>,
    tx_tt_cmd: std::sync::mpsc::Sender<types::TtCommand>,
) {
    let default_lang = &config.general.default_lang;
    let admin_id = teloxide::types::ChatId(config.telegram.admin_chat_id);

    tracing::info!("🌉 [BRIDGE] Bridge task started.");
    while let Some(event) = rx_bridge.recv().await {
        match event {
            types::BridgeEvent::Broadcast {
                event_type,
                nickname,
                server_name,
                related_tt_username,
            } => {
                let bot = if let Some(bot) = &event_bot {
                    bot
                } else {
                    continue;
                };

                let recipients = match db_clone
                    .get_recipients_for_event(&related_tt_username, event_type)
                    .await
                {
                    Ok(r) if !r.is_empty() => r,
                    _ => continue,
                };

                let escaped_nick = teloxide::utils::html::escape(&nickname);
                let escaped_server = teloxide::utils::html::escape(&server_name);

                let key = match event_type {
                    crate::types::NotificationType::Join => "event-join",
                    crate::types::NotificationType::Leave => "event-leave",
                };

                let mut rendered_text_cache: HashMap<String, String> = HashMap::new();
                let mut set = JoinSet::new();

                for sub in recipients {
                    let bot = bot.clone();
                    let online_users_by_username = online_users_by_username.clone();

                    let text = rendered_text_cache
                        .entry(sub.language_code.clone())
                        .or_insert_with(|| {
                            let args = args!(
                                nickname = escaped_nick.clone(),
                                server = escaped_server.clone()
                            );
                            locales::get_text(&sub.language_code, key, args.as_ref())
                        })
                        .clone();

                    let db_for_closure = db_clone.clone();

                    set.spawn(async move {
                        let mut send_silent = false;

                        if sub.not_on_online_enabled
                            && sub.not_on_online_confirmed
                            && let Some(linked_tt) = &sub.teamtalk_username
                            && online_users_by_username.contains_key(linked_tt)
                        {
                            send_silent = true;
                        }

                        let res = bot
                            .send_message(teloxide::types::ChatId(sub.telegram_id), text)
                            .parse_mode(teloxide::types::ParseMode::Html)
                            .disable_notification(send_silent)
                            .await;

                        if let Err(e) = res {
                            tracing::warn!("Failed to send notification to {}: {}", sub.telegram_id, e);

                            if let RequestError::Api(api_err) = e {
                                match api_err {
                                    ApiError::BotBlocked |
                                    ApiError::UserDeactivated |
                                    ApiError::ChatNotFound => {
                                        tracing::info!("🗑️ [BRIDGE] Cleaning up: User {} is no longer reachable ({:?}).", sub.telegram_id, api_err);

                                        if let Err(db_err) = db_for_closure.delete_user_profile(sub.telegram_id).await {
                                            tracing::error!("❌ [BRIDGE] DB error during auto-cleanup for {}: {}", sub.telegram_id, db_err);
                                        } else {
                                            tracing::info!("✅ [BRIDGE] Profile for {} removed successfully.", sub.telegram_id);
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        }
                    });
                }

                while let Some(res) = set.join_next().await {
                    if let Err(e) = res {
                        tracing::error!(
                            "[BRIDGE] A notification task failed after joining: {:?}",
                            e
                        );
                    }
                }
            }
            types::BridgeEvent::ToAdmin {
                user_id,
                nick,
                tt_username,
                msg_content,
                server_name,
            } => {
                if let Some(bot) = &msg_bot {
                    let admin_settings =
                        db_clone.get_or_create_user(admin_id.0, default_lang).await;
                    let admin_lang = match admin_settings {
                        Ok(u) => u.language_code,
                        Err(e) => {
                            tracing::error!(
                                "Failed to get admin settings: {}. Defaulting to 'en'.",
                                e
                            );
                            "en".to_string()
                        }
                    };

                    let args_admin = args!(
                        server = html::escape(&server_name),
                        nick = html::escape(&nick),
                        msg = html::escape(&msg_content)
                    );
                    let text_admin =
                        locales::get_text(&admin_lang, "admin-alert", args_admin.as_ref());

                    let res = bot
                        .send_message(admin_id, &text_admin)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await;

                    let reply_lang = if !tt_username.is_empty() {
                        db_clone
                            .get_user_lang_by_tt_user(&tt_username)
                            .await
                            .unwrap_or_else(|| default_lang.to_string())
                    } else {
                        default_lang.to_string()
                    };

                    let key_reply = if res.is_ok() {
                        "tt-msg-sent"
                    } else {
                        "tt-msg-failed"
                    };
                    let reply_text = locales::get_text(&reply_lang, key_reply, None);

                    tx_tt_cmd
                        .send(types::TtCommand::ReplyToUser {
                            user_id,
                            text: reply_text,
                        })
                        .ok();
                } else {
                    tracing::debug!("Skipping Admin Alert: 'message_token' is not configured.");
                }
            }
            types::BridgeEvent::WhoReport { chat_id, text } => {
                if let Some(bot) = &event_bot {
                    let _ = bot
                        .send_message(teloxide::types::ChatId(chat_id), &text)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await;
                }
            }
        }
    }
}
```

---

### `src/config.rs`

```rust
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub general: GeneralConfig,
    pub database: DatabaseConfig,
    pub telegram: TelegramConfig,
    pub teamtalk: TeamTalkConfig,

    #[serde(default)]
    pub operational_parameters: OperationalParameters,
}

#[derive(Deserialize, Clone)]
pub struct GeneralConfig {
    #[serde(default = "default_lang")]
    pub default_lang: String,

    #[allow(dead_code)]
    pub admin_username: Option<String>,

    #[serde(default = "default_gender")]
    #[allow(dead_code)]
    pub gender: String,
}

fn default_lang() -> String {
    "en".to_string()
}

fn default_gender() -> String {
    "None".to_string()
}

#[derive(Deserialize, Clone)]
pub struct DatabaseConfig {
    pub db_file: String,
}

#[derive(Deserialize, Clone)]
pub struct TelegramConfig {
    pub event_token: Option<String>,
    pub message_token: Option<String>,
    pub admin_chat_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct TeamTalkConfig {
    pub host_name: String,
    pub port: u32,
    pub encrypted: bool,
    pub user_name: String,
    pub password: String,
    pub channel: String,
    pub channel_password: Option<String>,
    pub nick_name: String,
    #[serde(default)]
    pub status_text: String,
    pub client_name: String,
    pub server_name: Option<String>,
    #[serde(default)]
    pub global_ignore_usernames: Vec<String>,
    pub guest_username: Option<String>,
}

impl TeamTalkConfig {
    pub fn display_name(&self) -> &str {
        self.server_name
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or(&self.host_name)
    }
}

#[derive(Deserialize, Clone)]
pub struct OperationalParameters {
    pub deeplink_ttl_seconds: i64,
    pub tt_reconnect_retry_seconds: u64,
    #[allow(dead_code)]
    pub tt_reconnect_check_interval_seconds: u64,
}

impl Default for OperationalParameters {
    fn default() -> Self {
        Self {
            deeplink_ttl_seconds: 300,
            tt_reconnect_retry_seconds: 10,
            tt_reconnect_check_interval_seconds: 30,
        }
    }
}
```

---

### `src/db/admins.rs`

```rust
use anyhow::Result;

use crate::db::Database;

impl Database {
    pub async fn add_admin(&self, telegram_id: i64) -> Result<bool> {
        let res = sqlx::query!(
            "INSERT OR IGNORE INTO admins (telegram_id) VALUES (?)",
            telegram_id
        )
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected() > 0)
    }

    pub async fn remove_admin(&self, telegram_id: i64) -> Result<bool> {
        let res = sqlx::query!("DELETE FROM admins WHERE telegram_id = ?", telegram_id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }

    pub async fn get_all_admins(&self) -> Result<Vec<i64>> {
        let rows = sqlx::query_scalar!("SELECT telegram_id FROM admins")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows)
    }
}
```

---

### `src/db/bans.rs`

```rust
use anyhow::Result;
use chrono::Utc;

use super::{Database, types::BanEntry};

impl Database {
    pub async fn add_ban(
        &self,
        telegram_id: Option<i64>,
        teamtalk_username: Option<String>,
        reason: Option<String>,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        sqlx::query!(
            "INSERT INTO ban_list (telegram_id, teamtalk_username, ban_reason, banned_at) VALUES (?, ?, ?, ?)",
            telegram_id,
            teamtalk_username,
            reason,
            now
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_banned_users(&self) -> Result<Vec<BanEntry>> {
        let rows = sqlx::query_as!(BanEntry, "SELECT * FROM ban_list ORDER BY banned_at DESC")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows)
    }

    pub async fn remove_ban_by_id(&self, id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM ban_list WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn is_telegram_id_banned(&self, telegram_id: i64) -> Result<bool> {
        let record = sqlx::query!(
            "SELECT count(*) as count FROM ban_list WHERE telegram_id = ?",
            telegram_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(record.count > 0)
    }

    pub async fn is_teamtalk_username_banned(&self, tt_username: &str) -> Result<bool> {
        let record = sqlx::query!(
            "SELECT count(*) as count FROM ban_list WHERE teamtalk_username = ? COLLATE NOCASE",
            tt_username
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(record.count > 0)
    }
}
```

---

### `src/db/deeplinks.rs`

```rust
use anyhow::Result;
use chrono::{Duration, Utc};

use super::{Database, types::Deeplink};

impl Database {
    pub async fn create_deeplink(
        &self,
        token: &str,
        action: &str,
        payload: Option<&str>,
        ttl_seconds: i64,
    ) -> Result<()> {
        let expiry = Utc::now() + Duration::seconds(ttl_seconds);
        sqlx::query!(
            "INSERT INTO deeplinks (token, action, payload, expiry_time) VALUES (?, ?, ?, ?)",
            token,
            action,
            payload,
            expiry
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn resolve_deeplink(&self, token: &str) -> Result<Option<Deeplink>> {
        let dl = sqlx::query_as!(
            Deeplink,
            r#"
            SELECT
                token as "token!",
                action as "action!",
                payload,
                expected_telegram_id,
                expiry_time as "expiry_time!"
            FROM deeplinks WHERE token = ?
            "#,
            token
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(d) = dl {
            if d.expiry_time < Utc::now().naive_utc() {
                sqlx::query!("DELETE FROM deeplinks WHERE token = ?", token)
                    .execute(&self.pool)
                    .await?;
                return Ok(None);
            }
            sqlx::query!("DELETE FROM deeplinks WHERE token = ?", token)
                .execute(&self.pool)
                .await?;
            return Ok(Some(d));
        }
        Ok(None)
    }
}
```

---

### `src/db/mod.rs`

```rust
pub mod admins;
pub mod bans;
pub mod deeplinks;
pub mod mutes;
pub mod subscriptions;
pub mod types;
pub mod user_settings;

use anyhow::Result;
use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(db_file: &str) -> Result<Self> {
        let connect_options = SqliteConnectOptions::new()
            .filename(db_file)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal);

        let pool = SqlitePoolOptions::new()
            .connect_with(connect_options)
            .await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }
}
```

---

### `src/db/mutes.rs`

```rust
use crate::types::MuteListMode;
use anyhow::Result;

use super::Database;

impl Database {
    pub async fn update_mute_mode(&self, telegram_id: i64, mode: MuteListMode) -> Result<()> {
        let mode_str = mode.to_string();
        sqlx::query!(
            "UPDATE user_settings SET mute_list_mode = ? WHERE telegram_id = ?",
            mode_str,
            telegram_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_muted_users_list(&self, telegram_id: i64) -> Result<Vec<String>> {
        let rows = sqlx::query_scalar!(
            "SELECT muted_teamtalk_username FROM muted_users WHERE user_settings_telegram_id = ?",
            telegram_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }
}
```

---

### `src/db/subscriptions.rs`

```rust
use crate::types::NotificationType;
use anyhow::Result;

use super::{
    Database,
    types::{SubscriberInfo, UserSettings},
};

impl Database {
    pub async fn add_subscriber(&self, telegram_id: i64) -> Result<()> {
        sqlx::query!(
            "INSERT OR IGNORE INTO subscribed_users (telegram_id) VALUES (?)",
            telegram_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn is_subscribed(&self, telegram_id: i64) -> Result<bool> {
        let record = sqlx::query!(
            "SELECT count(*) as count FROM subscribed_users WHERE telegram_id = ?",
            telegram_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(record.count > 0)
    }

    pub async fn get_subscribers(&self) -> Result<Vec<SubscriberInfo>> {
        let rows = sqlx::query_as!(
            SubscriberInfo,
            r#"
            SELECT
                su.telegram_id as "telegram_id!",
                us.teamtalk_username,
                us.language_code as "language_code!"
            FROM subscribed_users su
            LEFT JOIN user_settings us ON su.telegram_id = us.telegram_id
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn get_recipients_for_event(
        &self,
        tt_username: &str,
        event_type: NotificationType,
    ) -> Result<Vec<UserSettings>> {
        let event_tag = match event_type {
            NotificationType::Join => "join",
            NotificationType::Leave => "leave",
        };

        let recipients = sqlx::query_as!(
            UserSettings,
            r#"
            SELECT
                us.telegram_id as "telegram_id!",
                us.language_code as "language_code!",
                us.notification_settings as "notification_settings!",
                us.mute_list_mode as "mute_list_mode!",
                us.teamtalk_username,
                us.not_on_online_enabled as "not_on_online_enabled!",
                us.not_on_online_confirmed as "not_on_online_confirmed!"
            FROM user_settings us
            JOIN subscribed_users su ON us.telegram_id = su.telegram_id
            LEFT JOIN muted_users mu ON us.telegram_id = mu.user_settings_telegram_id AND mu.muted_teamtalk_username = ?
            WHERE us.notification_settings != 'none'
            AND (
                (? = 'join' AND us.notification_settings != 'join_off')
                OR
                (? = 'leave' AND us.notification_settings != 'leave_off')
            )
            AND (
                (us.mute_list_mode = 'blacklist' AND mu.id IS NULL)
                OR
                (us.mute_list_mode = 'whitelist' AND mu.id IS NOT NULL)
            )
            "#,
            tt_username,
            event_tag,
            event_tag
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(recipients)
    }
}
```

---

### `src/db/types.rs`

```rust
use chrono::NaiveDateTime;

#[derive(sqlx::FromRow, Debug)]
pub struct UserSettings {
    pub telegram_id: i64,
    pub language_code: String,
    #[allow(dead_code)]
    pub notification_settings: String,
    #[allow(dead_code)]
    pub mute_list_mode: String,
    pub teamtalk_username: Option<String>,
    pub not_on_online_enabled: bool,
    pub not_on_online_confirmed: bool,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Deeplink {
    #[allow(dead_code)]
    pub token: String,
    pub action: String,
    pub payload: Option<String>,
    #[allow(dead_code)]
    pub expected_telegram_id: Option<i64>,
    pub expiry_time: NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug)]
pub struct BanEntry {
    pub id: i64,
    pub telegram_id: Option<i64>,
    pub teamtalk_username: Option<String>,
    #[allow(dead_code)]
    pub ban_reason: Option<String>,
    #[allow(dead_code)]
    pub banned_at: NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug)]
pub struct SubscriberInfo {
    pub telegram_id: i64,
    pub teamtalk_username: Option<String>,
    #[allow(dead_code)]
    pub language_code: String,
}
```

---

### `src/db/user_settings.rs`

```rust
use crate::types::NotificationSetting;
use anyhow::Result;

use super::{Database, types::UserSettings};

impl Database {
    pub async fn get_or_create_user(
        &self,
        telegram_id: i64,
        default_lang: &str,
    ) -> Result<UserSettings> {
        let user = sqlx::query_as!(
            UserSettings,
            r#"
            SELECT
                telegram_id as "telegram_id!",
                language_code as "language_code!",
                notification_settings as "notification_settings!",
                mute_list_mode as "mute_list_mode!",
                teamtalk_username,
                not_on_online_enabled as "not_on_online_enabled!",
                not_on_online_confirmed as "not_on_online_confirmed!"
            FROM user_settings
            WHERE telegram_id = ?
            "#,
            telegram_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(u) = user {
            Ok(u)
        } else {
            sqlx::query!(
                "INSERT INTO user_settings (telegram_id, language_code) VALUES (?, ?)",
                telegram_id,
                default_lang
            )
            .execute(&self.pool)
            .await?;

            Ok(UserSettings {
                telegram_id,
                language_code: default_lang.to_string(),
                notification_settings: "all".to_string(),
                mute_list_mode: "blacklist".to_string(),
                teamtalk_username: None,
                not_on_online_enabled: false,
                not_on_online_confirmed: false,
            })
        }
    }

    pub async fn get_user_lang_by_tt_user(&self, tt_username: &str) -> Option<String> {
        let res: Option<String> = match sqlx::query_scalar!(
            "SELECT language_code FROM user_settings WHERE teamtalk_username = ?",
            tt_username
        )
        .fetch_optional(&self.pool)
        .await
        {
            Ok(res) => res,
            Err(e) => {
                tracing::error!(
                    "Failed to get user lang for tt_user '{}': {}",
                    tt_username,
                    e
                );
                None
            }
        };

        res
    }

    pub async fn update_notification_setting(
        &self,
        telegram_id: i64,
        setting: NotificationSetting,
    ) -> Result<()> {
        let setting_str = setting.to_string();
        sqlx::query!(
            "UPDATE user_settings SET notification_settings = ? WHERE telegram_id = ?",
            setting_str,
            telegram_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_language(&self, telegram_id: i64, lang: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE user_settings SET language_code = ? WHERE telegram_id = ?",
            lang,
            telegram_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn toggle_noon(&self, telegram_id: i64) -> Result<bool> {
        let mut tx = self.pool.begin().await?;

        let current_val: i64 = sqlx::query_scalar!(
            "SELECT CAST(not_on_online_enabled AS INTEGER) FROM user_settings WHERE telegram_id = ?",
            telegram_id
        )
        .fetch_optional(&mut *tx)
        .await?
        .unwrap_or(0);

        let new_bool = current_val == 0;
        let new_int = if new_bool { 1 } else { 0 };

        tracing::debug!(
            "[DB] Toggling NOON for {}: current={}, new_bool={}",
            telegram_id,
            current_val,
            new_bool
        );

        sqlx::query!(
            "UPDATE user_settings SET not_on_online_enabled = ? WHERE telegram_id = ?",
            new_int,
            telegram_id
        )
        .execute(&mut *tx)
        .await?;

        if new_bool {
            sqlx::query!(
                "UPDATE user_settings SET not_on_online_confirmed = 1 WHERE telegram_id = ?",
                telegram_id
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(new_bool)
    }

    pub async fn link_tt_account(&self, telegram_id: i64, tt_username: &str) -> Result<()> {
        sqlx::query!("UPDATE user_settings SET teamtalk_username = ?, not_on_online_confirmed = 1 WHERE telegram_id = ?", tt_username, telegram_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn unlink_tt_account(&self, telegram_id: i64) -> Result<()> {
        sqlx::query!(
            "UPDATE user_settings SET teamtalk_username = NULL WHERE telegram_id = ?",
            telegram_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_user_profile(&self, telegram_id: i64) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
            "DELETE FROM subscribed_users WHERE telegram_id = ?",
            telegram_id
        )
        .execute(&mut *tx)
        .await?;
        sqlx::query!("DELETE FROM admins WHERE telegram_id = ?", telegram_id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!(
            "DELETE FROM muted_users WHERE user_settings_telegram_id = ?",
            telegram_id
        )
        .execute(&mut *tx)
        .await?;
        sqlx::query!(
            "DELETE FROM user_settings WHERE telegram_id = ?",
            telegram_id
        )
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }
}
```

---

### `src/locales.rs`

```rust
use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::{Loader, static_loader};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::LazyLock;
use unic_langid::LanguageIdentifier;

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

static LANG_RU: LazyLock<LanguageIdentifier> =
    LazyLock::new(|| "ru".parse().expect("Valid RU langid"));
static LANG_EN: LazyLock<LanguageIdentifier> =
    LazyLock::new(|| "en".parse().expect("Valid EN langid"));

fn get_lang_id(lang_code: &str) -> &LanguageIdentifier {
    match lang_code {
        "ru" => &LANG_RU,
        _ => &LANG_EN,
    }
}

pub fn get_text(
    lang_code: &str,
    key: &str,
    args: Option<&HashMap<Cow<'static, str>, FluentValue>>,
) -> String {
    let lang_id = get_lang_id(lang_code);

    if let Some(args_map) = args {
        LOCALES.lookup_with_args(lang_id, key, args_map)
    } else {
        LOCALES.lookup(lang_id, key)
    }
}

#[macro_export]
macro_rules! args {
    ( $($k:ident = $v:expr),* ) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert(
                std::borrow::Cow::Borrowed(stringify!($k)),
                fluent_templates::fluent_bundle::FluentValue::from($v)
            );
        )*
        Some(map)
    }};
}
```

---

### `src/main.rs`

```rust
use self_update::cargo_crate_version;

mod bridge;
mod config;
mod db;
mod locales;
mod tg_bot;
mod tt_worker;
mod types;

use anyhow::{Result, anyhow};
use dashmap::DashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::mpsc as std_mpsc;
use teamtalk::types::UserAccount;
use teloxide::{Bot, prelude::Requester};
use tokio::sync::mpsc as tokio_mpsc;
use tracing_subscriber::EnvFilter;

fn update_bot() -> Result<(), Box<dyn std::error::Error>> {
    let target = if cfg!(windows) { "windows" } else { "linux" };

    let status = self_update::backends::github::Update::configure()
        .repo_owner("kirill-jjj")
        .repo_name("teamtalk-telegram-sender-rs")
        .bin_name("teamtalk-telegram-sender-rs")
        .target(target)
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    println!("Update status: `{}`!", status.version());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--update".to_string()) {
        println!("Checking for updates...");
        if let Err(e) = update_bot() {
            eprintln!("Update failed: {}", e);
            std::process::exit(1);
        }
        println!("Update completed successfully! Please restart the bot.");
        std::process::exit(0);
    }

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    tracing::info!("🚀 Starting Application...");

    let args: Vec<String> = std::env::args().collect();
    let config_path = if let Some(idx) = args.iter().position(|a| a == "--config") {
        args.get(idx + 1)
            .cloned()
            .unwrap_or_else(|| "config.toml".to_string())
    } else {
        "config.toml".to_string()
    };

    tracing::info!("📂 Loading config from: {}", config_path);

    let config_content = std::fs::read_to_string(&config_path)?;
    let mut config: config::Config = toml::from_str(&config_content)?;

    let config_path_obj = Path::new(&config_path);
    let config_dir = config_path_obj.parent().unwrap_or_else(|| Path::new("."));

    let db_path_buf = if Path::new(&config.database.db_file).is_absolute() {
        Path::new(&config.database.db_file).to_path_buf()
    } else {
        config_dir.join(&config.database.db_file)
    };

    let db_path_str = db_path_buf
        .to_str()
        .ok_or_else(|| anyhow!("Invalid DB path"))?
        .to_string();
    tracing::info!("💾 Database path: {}", db_path_str);

    config.database.db_file = db_path_str.clone();

    let shared_config = Arc::new(config);

    let db = db::Database::new(&db_path_str).await?;

    let online_users: Arc<DashMap<i32, types::LiteUser>> = Arc::new(DashMap::new());
    let online_users_by_username: Arc<DashMap<String, i32>> = Arc::new(DashMap::new());
    let all_user_accounts: Arc<DashMap<String, UserAccount>> = Arc::new(DashMap::new());

    let (tx_bridge, rx_bridge) = tokio_mpsc::channel::<crate::types::BridgeEvent>(100);
    let (tx_tt_cmd, rx_tt_cmd) = std_mpsc::channel::<crate::types::TtCommand>();

    let event_bot = if let Some(token) = &shared_config.telegram.event_token {
        Some(Bot::new(token))
    } else {
        tracing::warn!(
            "⚠️ 'event_token' missing. Telegram interactions and notifications disabled."
        );
        None
    };

    let message_bot = if let Some(token) = &shared_config.telegram.message_token {
        Some(Bot::new(token))
    } else {
        tracing::warn!("⚠️ 'message_token' missing. Admin alerts disabled.");
        None
    };

    let bot_username = if let Some(bot) = &event_bot {
        let me = bot.get_me().await?;
        let username = me
            .username
            .clone()
            .ok_or_else(|| anyhow!("Bot must have a username!"))?;
        tracing::info!("✅ Interaction Bot username: @{}", username);
        Some(username)
    } else {
        None
    };

    let tt_users = online_users.clone();
    let tt_users_by_username = online_users_by_username.clone();
    let tt_accounts = all_user_accounts.clone();
    let tx_bridge_clone = tx_bridge.clone();
    let db_for_tt = db.clone();
    let rt_handle = tokio::runtime::Handle::current();
    let bot_username_for_tt = bot_username.clone();

    let config_for_worker = shared_config.clone();
    let tx_tt_cmd_for_worker = tx_tt_cmd.clone();

    let (tx_init, rx_init) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        tt_worker::run_teamtalk_thread(
            config_for_worker,
            tt_users,
            tt_users_by_username,
            tt_accounts,
            tx_bridge_clone,
            rx_tt_cmd,
            tx_tt_cmd_for_worker,
            db_for_tt,
            rt_handle,
            bot_username_for_tt,
            tx_init,
        );
    });

    match rx_init.recv() {
        Ok(Ok(_)) => tracing::info!("✅ TeamTalk Worker started successfully"),
        Ok(Err(e)) => return Err(anyhow!("❌ TeamTalk Worker failed to start: {}", e)),
        Err(_) => return Err(anyhow!("❌ TeamTalk Worker disconnected during startup")),
    }

    let event_bot_clone = event_bot.clone();
    let msg_bot_clone = message_bot.clone();
    let db_clone = db.clone();
    let users_by_username_clone = online_users_by_username.clone();

    let bridge_handle = tokio::spawn(bridge::run_bridge(
        rx_bridge,
        db_clone,
        users_by_username_clone,
        shared_config.clone(),
        event_bot_clone,
        msg_bot_clone,
        tx_tt_cmd.clone(),
    ));

    if let Some(bot) = event_bot {
        tg_bot::run_tg_bot(
            bot,
            db,
            online_users,
            all_user_accounts,
            tx_tt_cmd,
            shared_config,
        )
        .await;
    } else {
        let _ = bridge_handle.await;
    }

    Ok(())
}
```

---

### `src/tg_bot/admin_logic/bans.rs`

```rust
use crate::db::Database;
use crate::locales;
use crate::tg_bot::callbacks_types::{AdminAction, CallbackAction, MenuAction};
use crate::tg_bot::keyboards::create_user_list_keyboard;
use teloxide::prelude::*;

pub async fn send_unban_list(
    bot: &Bot,
    chat_id: teloxide::types::ChatId,
    db: &Database,
    lang: &str,
    page: usize,
) -> ResponseResult<()> {
    let entries = db.get_banned_users().await.unwrap_or_default();

    if entries.is_empty() {
        bot.send_message(chat_id, locales::get_text(lang, "list-ban-empty", None))
            .await?;
        return Ok(());
    }

    let keyboard = create_user_list_keyboard(
        &entries,
        page,
        |e| {
            let name = if let Some(tg) = e.telegram_id {
                format!("{}", tg)
            } else if let Some(tt) = &e.teamtalk_username {
                tt.clone()
            } else {
                "Unknown".to_string()
            };
            (
                name,
                CallbackAction::Admin(AdminAction::UnbanPerform {
                    ban_db_id: e.id,
                    page,
                }),
            )
        },
        |p| CallbackAction::Admin(AdminAction::UnbanList { page: p }),
        Some((
            locales::get_text(lang, "btn-back-menu", None),
            CallbackAction::Menu(MenuAction::Who),
        )),
        lang,
    );

    bot.send_message(chat_id, locales::get_text(lang, "list-unban-title", None))
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn edit_unban_list(
    bot: &Bot,
    msg: &Message,
    db: &Database,
    lang: &str,
    page: usize,
) -> ResponseResult<()> {
    let entries = db.get_banned_users().await.unwrap_or_default();

    if entries.is_empty() {
        bot.edit_message_text(
            msg.chat.id,
            msg.id,
            locales::get_text(lang, "list-ban-empty", None),
        )
        .await?;
        return Ok(());
    }

    let keyboard = create_user_list_keyboard(
        &entries,
        page,
        |e| {
            let name = if let Some(tg) = e.telegram_id {
                format!("{}", tg)
            } else if let Some(tt) = &e.teamtalk_username {
                tt.clone()
            } else {
                "Unknown".to_string()
            };
            (
                name,
                CallbackAction::Admin(AdminAction::UnbanPerform {
                    ban_db_id: e.id,
                    page,
                }),
            )
        },
        |p| CallbackAction::Admin(AdminAction::UnbanList { page: p }),
        Some((
            locales::get_text(lang, "btn-back-menu", None),
            CallbackAction::Menu(MenuAction::Who),
        )),
        lang,
    );

    bot.edit_message_text(
        msg.chat.id,
        msg.id,
        locales::get_text(lang, "list-unban-title", None),
    )
    .reply_markup(keyboard)
    .await?;
    Ok(())
}
```

---

### `src/tg_bot/admin_logic/mod.rs`

```rust
pub mod bans;
pub mod subscriber_settings;
pub mod subscribers;
pub mod utils;
```

---

### `src/tg_bot/admin_logic/subscriber_settings.rs`

```rust
use crate::args;
use crate::db::Database;
use crate::locales;
use crate::tg_bot::callbacks_types::{CallbackAction, SubAction};
use crate::tg_bot::keyboards::create_user_list_keyboard;
use teamtalk::types::UserAccount;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn send_sub_manage_tt_menu(
    bot: &Bot,
    msg: &Message,
    db: &Database,
    lang: &str,
    sub_id: i64,
    return_page: usize,
) -> ResponseResult<()> {
    let settings = match db.get_or_create_user(sub_id, "en").await {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Failed to get or create user {}: {}", sub_id, e);
            bot.edit_message_text(
                msg.chat.id,
                msg.id,
                locales::get_text(lang, "cmd-error", None),
            )
            .await?;
            return Ok(());
        }
    };
    let tt_user = settings.teamtalk_username;

    let args = args!(id = sub_id.to_string());
    let text = locales::get_text(lang, "sub-manage-tt-title", args.as_ref());

    let mut buttons = vec![];
    if let Some(user) = tt_user {
        let args_btn = args!(user = user);
        buttons.push(vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-unlink", args_btn.as_ref()),
            CallbackAction::Subscriber(SubAction::Unlink {
                sub_id,
                page: return_page,
            })
            .to_string(),
        )]);
    }
    buttons.push(vec![InlineKeyboardButton::callback(
        locales::get_text(lang, "btn-link-new", None),
        CallbackAction::Subscriber(SubAction::LinkList {
            sub_id,
            page: return_page,
            list_page: 0,
        })
        .to_string(),
    )]);
    buttons.push(vec![InlineKeyboardButton::callback(
        locales::get_text(lang, "btn-back-user-actions", None),
        CallbackAction::Subscriber(SubAction::Details {
            sub_id,
            page: return_page,
        })
        .to_string(),
    )]);

    bot.edit_message_text(msg.chat.id, msg.id, text)
        .reply_markup(InlineKeyboardMarkup::new(buttons))
        .await?;
    Ok(())
}

pub async fn send_sub_link_account_list(
    bot: &Bot,
    msg: &Message,
    user_accounts: &std::sync::Arc<dashmap::DashMap<String, UserAccount>>,
    lang: &str,
    target_id: i64,
    sub_page: usize,
    page: usize,
) -> ResponseResult<()> {
    let mut accounts: Vec<UserAccount> =
        user_accounts.iter().map(|kv| kv.value().clone()).collect();
    accounts.sort_by(|a, b| a.username.to_lowercase().cmp(&b.username.to_lowercase()));

    let keyboard = create_user_list_keyboard(
        &accounts,
        page,
        |acc| {
            (
                acc.username.clone(),
                CallbackAction::Subscriber(SubAction::LinkPerform {
                    sub_id: target_id,
                    page: sub_page,
                    username: acc.username.clone(),
                }),
            )
        },
        |p| {
            CallbackAction::Subscriber(SubAction::LinkList {
                sub_id: target_id,
                page: sub_page,
                list_page: p,
            })
        },
        Some((
            locales::get_text(lang, "btn-back-manage-acc", None),
            CallbackAction::Subscriber(SubAction::ManageTt {
                sub_id: target_id,
                page: sub_page,
            }),
        )),
        lang,
    );

    let args = args!(id = target_id.to_string());
    let text = locales::get_text(lang, "list-link-title", args.as_ref());

    bot.edit_message_text(msg.chat.id, msg.id, text)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn send_sub_lang_menu(
    bot: &Bot,
    msg: &Message,
    lang: &str,
    target_id: i64,
    return_page: usize,
) -> ResponseResult<()> {
    let args = args!(id = target_id.to_string());
    let text = locales::get_text(lang, "sub-lang-title", args.as_ref());

    let mk_btn = |lbl: &str, l_code: &str| {
        InlineKeyboardButton::callback(
            lbl,
            CallbackAction::Subscriber(SubAction::LangSet {
                sub_id: target_id,
                page: return_page,
                lang: l_code.to_string(),
            })
            .to_string(),
        )
    };

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![mk_btn("🇷🇺 Русский", "ru")],
        vec![mk_btn("🇬🇧 English", "en")],
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-back-user-actions", None),
            CallbackAction::Subscriber(SubAction::Details {
                sub_id: target_id,
                page: return_page,
            })
            .to_string(),
        )],
    ]);

    bot.edit_message_text(msg.chat.id, msg.id, text)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn send_sub_notif_menu(
    bot: &Bot,
    msg: &Message,
    lang: &str,
    target_id: i64,
    return_page: usize,
) -> ResponseResult<()> {
    let args = args!(id = target_id.to_string());
    let text = locales::get_text(lang, "sub-notif-title", args.as_ref());

    let marker_args = args!(marker = "");

    let btn_all = locales::get_text(lang, "btn-sub-all", marker_args.as_ref());
    let btn_join = locales::get_text(lang, "btn-sub-join", marker_args.as_ref());
    let btn_leave = locales::get_text(lang, "btn-sub-leave", marker_args.as_ref());
    let btn_none = locales::get_text(lang, "btn-sub-none", marker_args.as_ref());

    let mk_act = |val: &str| {
        CallbackAction::Subscriber(SubAction::NotifSet {
            sub_id: target_id,
            page: return_page,
            val: val.to_string(),
        })
        .to_string()
    };

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(btn_all, mk_act("all"))],
        vec![InlineKeyboardButton::callback(btn_join, mk_act("join_off"))],
        vec![InlineKeyboardButton::callback(
            btn_leave,
            mk_act("leave_off"),
        )],
        vec![InlineKeyboardButton::callback(btn_none, mk_act("none"))],
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-back-user-actions", None),
            CallbackAction::Subscriber(SubAction::Details {
                sub_id: target_id,
                page: return_page,
            })
            .to_string(),
        )],
    ]);

    bot.edit_message_text(msg.chat.id, msg.id, text)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn send_sub_mute_mode_menu(
    bot: &Bot,
    msg: &Message,
    lang: &str,
    target_id: i64,
    return_page: usize,
) -> ResponseResult<()> {
    let args = args!(id = target_id.to_string());
    let text = locales::get_text(lang, "sub-mode-title", args.as_ref());

    let bl_text = locales::get_text(lang, "mode-blacklist", None);
    let wl_text = locales::get_text(lang, "mode-whitelist", None);

    let mk_act = |mode: &str| {
        CallbackAction::Subscriber(SubAction::ModeSet {
            sub_id: target_id,
            page: return_page,
            mode: mode.to_string(),
        })
        .to_string()
    };

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(bl_text, mk_act("blacklist"))],
        vec![InlineKeyboardButton::callback(wl_text, mk_act("whitelist"))],
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-back-user-actions", None),
            CallbackAction::Subscriber(SubAction::Details {
                sub_id: target_id,
                page: return_page,
            })
            .to_string(),
        )],
    ]);

    bot.edit_message_text(msg.chat.id, msg.id, text)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn send_sub_mute_list(
    bot: &Bot,
    msg: &Message,
    db: &Database,
    lang: &str,
    target_id: i64,
    sub_page: usize,
    page: usize,
) -> ResponseResult<()> {
    let muted: Vec<String> = db.get_muted_users_list(target_id).await.unwrap_or_default();

    let user_name = format!("{}", target_id);
    let args = args!(name = user_name);
    let title = locales::get_text(lang, "list-mute-title", args.as_ref());

    let keyboard = create_user_list_keyboard(
        &muted,
        page,
        |username| (username.clone(), CallbackAction::NoOp), // Список просмотра, действия не нужны
        |p| {
            CallbackAction::Subscriber(SubAction::MuteView {
                sub_id: target_id,
                page: sub_page,
                view_page: p,
            })
        },
        Some((
            locales::get_text(lang, "btn-back-user-actions", None),
            CallbackAction::Subscriber(SubAction::Details {
                sub_id: target_id,
                page: sub_page,
            }),
        )),
        lang,
    );

    bot.edit_message_text(msg.chat.id, msg.id, title)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}
```

---

### `src/tg_bot/admin_logic/subscribers.rs`

```rust
use crate::args;
use crate::db::Database;
use crate::locales;
use crate::tg_bot::admin_logic::utils::format_tg_user;
use crate::tg_bot::callbacks_types::{AdminAction, CallbackAction, MenuAction, SubAction};
use crate::tg_bot::keyboards::create_user_list_keyboard;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode};

struct SubDisplayInfo {
    telegram_id: i64,
    display_name: String,
    tt_username: Option<String>,
}

pub async fn send_subscribers_list(
    bot: &Bot,
    chat_id: teloxide::types::ChatId,
    db: &Database,
    lang: &str,
    page: usize,
) -> ResponseResult<()> {
    let subs = db.get_subscribers().await.unwrap_or_default();

    if subs.is_empty() {
        bot.send_message(chat_id, locales::get_text(lang, "list-subs-empty", None))
            .await?;
        return Ok(());
    }

    let display_list = prepare_display_list(bot, subs).await;

    let keyboard = create_user_list_keyboard(
        &display_list,
        page,
        |s| {
            let mut parts = vec![s.display_name.clone()];
            if let Some(tt) = &s.tt_username {
                parts.push(format!("TT: {}", tt));
            }
            let name = parts.join(", ");
            (
                name,
                CallbackAction::Subscriber(SubAction::Details {
                    sub_id: s.telegram_id,
                    page,
                }),
            )
        },
        |p| CallbackAction::Admin(AdminAction::SubsList { page: p }),
        Some((
            locales::get_text(lang, "btn-back-menu", None),
            CallbackAction::Menu(MenuAction::Who),
        )),
        lang,
    );

    bot.send_message(chat_id, locales::get_text(lang, "list-subs-title", None))
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn edit_subscribers_list(
    bot: &Bot,
    msg: &Message,
    db: &Database,
    lang: &str,
    page: usize,
) -> ResponseResult<()> {
    let subs = db.get_subscribers().await.unwrap_or_default();

    if subs.is_empty() {
        bot.edit_message_text(
            msg.chat.id,
            msg.id,
            locales::get_text(lang, "list-subs-empty", None),
        )
        .await?;
        return Ok(());
    }

    let display_list = prepare_display_list(bot, subs).await;

    let keyboard = create_user_list_keyboard(
        &display_list,
        page,
        |s| {
            let mut parts = vec![s.display_name.clone()];
            if let Some(tt) = &s.tt_username {
                parts.push(format!("TT: {}", tt));
            }
            let name = parts.join(", ");
            (
                name,
                CallbackAction::Subscriber(SubAction::Details {
                    sub_id: s.telegram_id,
                    page,
                }),
            )
        },
        |p| CallbackAction::Admin(AdminAction::SubsList { page: p }),
        Some((
            locales::get_text(lang, "btn-back-menu", None),
            CallbackAction::Menu(MenuAction::Who),
        )),
        lang,
    );

    bot.edit_message_text(
        msg.chat.id,
        msg.id,
        locales::get_text(lang, "list-subs-title", None),
    )
    .reply_markup(keyboard)
    .await?;
    Ok(())
}

async fn prepare_display_list(
    bot: &Bot,
    subs: Vec<crate::db::types::SubscriberInfo>,
) -> Vec<SubDisplayInfo> {
    let mut display_list = Vec::new();
    for sub in subs {
        let display_name = match bot.get_chat(teloxide::types::ChatId(sub.telegram_id)).await {
            Ok(chat) => format_tg_user(&chat),
            Err(_) => sub.telegram_id.to_string(),
        };
        display_list.push(SubDisplayInfo {
            telegram_id: sub.telegram_id,
            display_name,
            tt_username: sub.teamtalk_username,
        });
    }
    display_list.sort_by(|a, b| {
        a.display_name
            .to_lowercase()
            .cmp(&b.display_name.to_lowercase())
    });
    display_list
}

pub async fn send_subscriber_details(
    bot: &Bot,
    msg: &Message,
    db: &Database,
    lang: &str,
    sub_id: i64,
    return_page: usize,
) -> ResponseResult<()> {
    let settings = db
        .get_or_create_user(sub_id, "en")
        .await
        .unwrap_or_else(|_| crate::db::types::UserSettings {
            telegram_id: sub_id,
            language_code: "en".to_string(),
            notification_settings: "all".to_string(),
            mute_list_mode: "blacklist".to_string(),
            teamtalk_username: None,
            not_on_online_enabled: false,
            not_on_online_confirmed: false,
        });

    let display_name = match bot.get_chat(teloxide::types::ChatId(sub_id)).await {
        Ok(chat) => format_tg_user(&chat),
        Err(_) => sub_id.to_string(),
    };

    let notif_map = |s: &str| match s {
        "all" => locales::get_text(lang, "btn-sub-all", args!(marker = "").as_ref()),
        "join_off" => locales::get_text(lang, "btn-sub-leave", args!(marker = "").as_ref()),
        "leave_off" => locales::get_text(lang, "btn-sub-join", args!(marker = "").as_ref()),
        "none" => locales::get_text(lang, "btn-sub-none", args!(marker = "").as_ref()),
        _ => s.to_string(),
    };

    let mode_map = |s: &str| match s {
        "blacklist" => locales::get_text(lang, "mode-blacklist", None),
        "whitelist" => locales::get_text(lang, "mode-whitelist", None),
        _ => s.to_string(),
    };

    let args = args!(
        name = display_name,
        tt_user = settings
            .teamtalk_username
            .unwrap_or_else(|| locales::get_text(lang, "val-none", None)),
        lang = settings.language_code,
        noon = if settings.not_on_online_enabled {
            locales::get_text(lang, "status-enabled", None)
        } else {
            locales::get_text(lang, "status-disabled", None)
        },
        notif = notif_map(&settings.notification_settings),
        mode = mode_map(&settings.mute_list_mode)
    );

    let text = locales::get_text(lang, "sub-details-title", args.as_ref());

    let btn = |text_key: &str, action: SubAction| {
        InlineKeyboardButton::callback(
            locales::get_text(lang, text_key, None),
            CallbackAction::Subscriber(action).to_string(),
        )
    };

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![btn(
            "btn-sub-delete",
            SubAction::Delete {
                sub_id,
                page: return_page,
            },
        )],
        vec![btn(
            "btn-sub-ban",
            SubAction::Ban {
                sub_id,
                page: return_page,
            },
        )],
        vec![btn(
            "btn-sub-manage-tt",
            SubAction::ManageTt {
                sub_id,
                page: return_page,
            },
        )],
        vec![btn(
            "btn-sub-lang",
            SubAction::LangMenu {
                sub_id,
                page: return_page,
            },
        )],
        vec![btn(
            "btn-sub-noon",
            SubAction::NoonToggle {
                sub_id,
                page: return_page,
            },
        )],
        vec![btn(
            "btn-sub-notif",
            SubAction::NotifMenu {
                sub_id,
                page: return_page,
            },
        )],
        vec![btn(
            "btn-sub-mute-mode",
            SubAction::ModeMenu {
                sub_id,
                page: return_page,
            },
        )],
        vec![btn(
            "btn-sub-view-mute",
            SubAction::MuteView {
                sub_id,
                page: return_page,
                view_page: 0,
            },
        )],
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-back-subs", None),
            CallbackAction::Admin(AdminAction::SubsList { page: return_page }).to_string(),
        )],
    ]);

    bot.edit_message_text(msg.chat.id, msg.id, text)
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}
```

---

### `src/tg_bot/admin_logic/utils.rs`

```rust
pub fn format_tg_user(chat: &teloxide::types::ChatFullInfo) -> String {
    let full_name = match (chat.first_name(), chat.last_name()) {
        (Some(f), Some(l)) => format!("{} {}", f, l),
        (Some(f), None) => f.to_string(),
        (None, Some(l)) => l.to_string(),
        (None, None) => String::new(),
    };

    let username = chat
        .username()
        .map(|u| format!("@{}", u))
        .unwrap_or_default();

    if !full_name.is_empty() {
        if !username.is_empty() {
            format!("{} ({})", full_name, username)
        } else {
            full_name
        }
    } else if !username.is_empty() {
        username
    } else {
        chat.id.0.to_string()
    }
}
```

---

### `src/tg_bot/callback_handlers/admin.rs`

```rust
use crate::args;
use crate::locales;
use crate::tg_bot::admin_logic::bans::{edit_unban_list, send_unban_list};
use crate::tg_bot::admin_logic::subscribers::{edit_subscribers_list, send_subscribers_list};
use crate::tg_bot::callbacks_types::{AdminAction, CallbackAction};
use crate::tg_bot::keyboards::create_user_list_keyboard;
use crate::tg_bot::state::AppState;
use crate::types::{LiteUser, TtCommand};
use teloxide::prelude::*;

pub async fn handle_admin(
    bot: Bot,
    q: CallbackQuery,
    state: AppState,
    action: AdminAction,
    lang: &str,
) -> ResponseResult<()> {
    let msg = match q.message {
        Some(teloxide::types::MaybeInaccessibleMessage::Regular(m)) => m,
        _ => return Ok(()),
    };
    let chat_id = msg.chat.id;
    let db = &state.db;
    let online_users = &state.online_users;
    let config = &state.config;

    match action {
        AdminAction::KickList { page } => {
            let mut users: Vec<LiteUser> = online_users.iter().map(|u| u.value().clone()).collect();
            users.sort_by(|a, b| a.nickname.to_lowercase().cmp(&b.nickname.to_lowercase()));

            let args = args!(server = config.teamtalk.display_name().to_string());
            let title = locales::get_text(lang, "list-kick-title", args.as_ref());

            let keyboard = create_user_list_keyboard(
                &users,
                page,
                |u| {
                    (
                        u.nickname.clone(),
                        CallbackAction::Admin(AdminAction::KickPerform { user_id: u.id }),
                    )
                },
                |p| CallbackAction::Admin(AdminAction::KickList { page: p }),
                None,
                lang,
            );

            if page == 0 && !msg.text().unwrap_or("").contains("Page") {
                bot.send_message(chat_id, title)
                    .reply_markup(keyboard)
                    .await?;
            } else {
                bot.edit_message_text(chat_id, msg.id, title)
                    .reply_markup(keyboard)
                    .await?;
            }
            bot.answer_callback_query(q.id).await?;
        }
        AdminAction::BanList { page } => {
            let mut users: Vec<LiteUser> = online_users.iter().map(|u| u.value().clone()).collect();
            users.sort_by(|a, b| a.nickname.to_lowercase().cmp(&b.nickname.to_lowercase()));

            let args = args!(server = config.teamtalk.display_name().to_string());
            let title = locales::get_text(lang, "list-ban-title", args.as_ref());

            let keyboard = create_user_list_keyboard(
                &users,
                page,
                |u| {
                    (
                        u.nickname.clone(),
                        CallbackAction::Admin(AdminAction::BanPerform { user_id: u.id }),
                    )
                },
                |p| CallbackAction::Admin(AdminAction::BanList { page: p }),
                None,
                lang,
            );

            if page == 0 && !msg.text().unwrap_or("").contains("Page") {
                bot.send_message(chat_id, title)
                    .reply_markup(keyboard)
                    .await?;
            } else {
                bot.edit_message_text(chat_id, msg.id, title)
                    .reply_markup(keyboard)
                    .await?;
            }
            bot.answer_callback_query(q.id).await?;
        }
        AdminAction::KickPerform { user_id } => {
            state.tx_tt.send(TtCommand::KickUser { user_id }).ok();
            bot.answer_callback_query(q.id)
                .text(locales::get_text(lang, "toast-command-sent", None))
                .await?;
        }
        AdminAction::BanPerform { user_id } => {
            if let Some(u) = online_users.get(&user_id) {
                if let Err(e) = db
                    .add_ban(
                        None,
                        Some(u.username.clone()),
                        Some("Banned via Telegram".to_string()),
                    )
                    .await
                {
                    tracing::error!("Failed to add ban: {}", e);
                }
                if let Ok(Some(tg_id)) = sqlx::query_scalar::<_, i64>(
                    "SELECT telegram_id FROM user_settings WHERE teamtalk_username = ?",
                )
                .bind(&u.username)
                .fetch_optional(&db.pool)
                .await
                {
                    db.delete_user_profile(tg_id).await.ok();
                    db.add_ban(
                        Some(tg_id),
                        Some(u.username.clone()),
                        Some("TG+TT Ban".to_string()),
                    )
                    .await
                    .ok();
                }
                state.tx_tt.send(TtCommand::BanUser { user_id }).ok();
                bot.answer_callback_query(q.id)
                    .text(locales::get_text(lang, "toast-command-sent", None))
                    .await?;
            } else {
                bot.answer_callback_query(q.id)
                    .text(locales::get_text(lang, "cmd-no-users", None))
                    .show_alert(true)
                    .await?;
            }
        }
        AdminAction::UnbanList { page } => {
            if page == 0 && !msg.text().unwrap_or("").contains("Page") {
                send_unban_list(&bot, chat_id, db, lang, 0).await?;
            } else {
                edit_unban_list(&bot, &msg, db, lang, page).await?;
            }
            bot.answer_callback_query(q.id).await?;
        }
        AdminAction::UnbanPerform { ban_db_id, page } => {
            db.remove_ban_by_id(ban_db_id).await.ok();
            bot.answer_callback_query(q.id)
                .text(locales::get_text(lang, "toast-user-unbanned", None))
                .await?;
            edit_unban_list(&bot, &msg, db, lang, page).await?;
        }
        AdminAction::SubsList { page } => {
            if page == 0 && !msg.text().unwrap_or("").contains("Page") {
                send_subscribers_list(&bot, chat_id, db, lang, 0).await?;
            } else {
                edit_subscribers_list(&bot, &msg, db, lang, page).await?;
            }
            bot.answer_callback_query(q.id).await?;
        }
    }
    Ok(())
}
```

---

### `src/tg_bot/callback_handlers/menu.rs`

```rust
use crate::locales;
use crate::tg_bot::callbacks_types::{CallbackAction, MenuAction, UnsubAction};
use crate::tg_bot::state::AppState;
use crate::types::TtCommand;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode};

pub async fn handle_menu(
    bot: Bot,
    q: CallbackQuery,
    state: AppState,
    action: MenuAction,
    lang: &str,
) -> ResponseResult<()> {
    let msg = match q.message {
        Some(teloxide::types::MaybeInaccessibleMessage::Regular(m)) => m,
        _ => return Ok(()),
    };
    let chat_id = msg.chat.id;

    match action {
        MenuAction::Who => {
            state
                .tx_tt
                .send(TtCommand::Who {
                    chat_id: chat_id.0,
                    lang: lang.to_string(),
                })
                .ok();
            bot.answer_callback_query(q.id).await?;
        }
        MenuAction::Help => {
            bot.send_message(chat_id, locales::get_text(lang, "help-text", None))
                .parse_mode(ParseMode::Html)
                .await?;
            bot.answer_callback_query(q.id).await?;
        }
        MenuAction::Unsub => {
            let text = locales::get_text(lang, "unsub-confirm-text", None);
            let keyboard = InlineKeyboardMarkup::new(vec![vec![
                InlineKeyboardButton::callback(
                    locales::get_text(lang, "btn-yes", None),
                    CallbackAction::Unsub(UnsubAction::Confirm).to_string(),
                ),
                InlineKeyboardButton::callback(
                    locales::get_text(lang, "btn-no", None),
                    CallbackAction::Unsub(UnsubAction::Cancel).to_string(),
                ),
            ]]);

            bot.send_message(chat_id, text)
                .reply_markup(keyboard)
                .await?;

            bot.answer_callback_query(q.id).await?;
        }
        MenuAction::Settings => {}
    }
    Ok(())
}
```

---

### `src/tg_bot/callback_handlers/mod.rs`

```rust
pub mod admin;
pub mod menu;
pub mod mute;
pub mod settings;
pub mod subscriber;
pub mod unsub;
```

---

### `src/tg_bot/callback_handlers/mute.rs`

```rust
use crate::args;
use crate::locales;
use crate::tg_bot::callbacks_types::MuteAction;
use crate::tg_bot::settings_logic::{render_mute_list, render_mute_list_strings, send_mute_menu};
use crate::tg_bot::state::AppState;
use crate::types::{MuteListMode, TtCommand};
use teamtalk::types::UserAccount;
use teloxide::prelude::*;

pub async fn handle_mute(
    bot: Bot,
    q: CallbackQuery,
    state: AppState,
    action: MuteAction,
    lang: &str,
) -> ResponseResult<()> {
    let msg = match &q.message {
        Some(teloxide::types::MaybeInaccessibleMessage::Regular(m)) => m,
        _ => return Ok(()),
    };
    let telegram_id = q.from.id.0 as i64;
    let db = &state.db;

    match action {
        MuteAction::ModeSet { mode } => {
            let new_mode = MuteListMode::from(mode.as_str());
            db.update_mute_mode(telegram_id, new_mode.clone())
                .await
                .ok();
            bot.answer_callback_query(q.id)
                .text(locales::get_text(
                    lang,
                    "toast-mute-mode-set",
                    args!(mode = new_mode.to_string()).as_ref(),
                ))
                .await?;
            send_mute_menu(&bot, msg, lang, &new_mode.to_string()).await?;
        }
        MuteAction::Menu { mode } => {
            send_mute_menu(&bot, msg, lang, &mode).await?;
        }
        MuteAction::List { page } => {
            let muted = db
                .get_muted_users_list(telegram_id)
                .await
                .unwrap_or_default();
            let guest_username = state.config.teamtalk.guest_username.as_deref();
            render_mute_list_strings(
                &bot,
                msg,
                telegram_id,
                lang,
                &muted,
                page,
                false,
                "list-mute-title",
                guest_username,
            )
            .await?;
        }
        MuteAction::Toggle { username, page } => {
            toggle_mute(db, telegram_id, &username).await;

            let args = args!(user = username.clone(), action = "toggled");
            bot.answer_callback_query(q.id)
                .text(locales::get_text(lang, "toast-user-muted", args.as_ref()))
                .await?;

            let muted = db
                .get_muted_users_list(telegram_id)
                .await
                .unwrap_or_default();
            let guest_username = state.config.teamtalk.guest_username.as_deref();
            render_mute_list_strings(
                &bot,
                msg,
                telegram_id,
                lang,
                &muted,
                page,
                false,
                "list-mute-title",
                guest_username,
            )
            .await?;
        }
        MuteAction::ServerList { page } => {
            state.tx_tt.send(TtCommand::LoadAccounts).ok();
            let user_accounts = &state.user_accounts;
            let mut accounts: Vec<UserAccount> =
                user_accounts.iter().map(|kv| kv.value().clone()).collect();
            accounts.sort_by(|a, b| a.username.to_lowercase().cmp(&b.username.to_lowercase()));

            let guest_username = state.config.teamtalk.guest_username.as_deref();
            render_mute_list(
                &bot,
                msg,
                db,
                telegram_id,
                lang,
                &accounts,
                page,
                "list-all-accs-title",
                guest_username,
            )
            .await?;
        }
        MuteAction::ServerToggle { username, page } => {
            toggle_mute(db, telegram_id, &username).await;

            let args = args!(user = username.clone(), action = "toggled");
            bot.answer_callback_query(q.id)
                .text(locales::get_text(lang, "toast-user-muted", args.as_ref()))
                .await?;

            let user_accounts = &state.user_accounts;
            let mut accounts: Vec<UserAccount> =
                user_accounts.iter().map(|kv| kv.value().clone()).collect();
            accounts.sort_by(|a, b| a.username.to_lowercase().cmp(&b.username.to_lowercase()));
            let guest_username = state.config.teamtalk.guest_username.as_deref();

            render_mute_list(
                &bot,
                msg,
                db,
                telegram_id,
                lang,
                &accounts,
                page,
                "list-all-accs-title",
                guest_username,
            )
            .await?;
        }
    }

    Ok(())
}

async fn toggle_mute(db: &crate::db::Database, telegram_id: i64, username: &str) {
    let is_muted = sqlx::query_scalar::<_, i32>("SELECT count(*) FROM muted_users WHERE user_settings_telegram_id = ? AND muted_teamtalk_username = ?")
        .bind(telegram_id).bind(username).fetch_one(&db.pool).await.unwrap_or(0) > 0;

    let query = if is_muted {
        "DELETE FROM muted_users WHERE user_settings_telegram_id = ? AND muted_teamtalk_username = ?"
    } else {
        "INSERT INTO muted_users (user_settings_telegram_id, muted_teamtalk_username) VALUES (?, ?)"
    };
    let _ = sqlx::query(query)
        .bind(telegram_id)
        .bind(username)
        .execute(&db.pool)
        .await;
}
```

---

### `src/tg_bot/callback_handlers/settings.rs`

```rust
use crate::args;
use crate::locales;
use crate::tg_bot::callbacks_types::{CallbackAction, SettingsAction};
use crate::tg_bot::settings_logic::{
    send_main_settings_edit, send_mute_menu, send_notif_settings, send_sub_settings,
};
use crate::tg_bot::state::AppState;
use crate::types::NotificationSetting;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn handle_settings(
    bot: Bot,
    q: CallbackQuery,
    state: AppState,
    action: SettingsAction,
    lang: &str,
) -> ResponseResult<()> {
    let msg = match q.message {
        Some(teloxide::types::MaybeInaccessibleMessage::Regular(m)) => m,
        _ => return Ok(()),
    };
    let chat_id = msg.chat.id;
    let telegram_id = q.from.id.0 as i64;
    let db = &state.db;

    match action {
        SettingsAction::Main => {
            send_main_settings_edit(&bot, &msg, lang).await?;
        }
        SettingsAction::LangSelect => {
            let keyboard = InlineKeyboardMarkup::new(vec![
                vec![InlineKeyboardButton::callback(
                    "🇷🇺 Русский",
                    CallbackAction::Settings(SettingsAction::LangSet {
                        lang: "ru".to_string(),
                    })
                    .to_string(),
                )],
                vec![InlineKeyboardButton::callback(
                    "🇬🇧 English",
                    CallbackAction::Settings(SettingsAction::LangSet {
                        lang: "en".to_string(),
                    })
                    .to_string(),
                )],
                vec![InlineKeyboardButton::callback(
                    locales::get_text(lang, "btn-back-settings", None),
                    CallbackAction::Settings(SettingsAction::Main).to_string(),
                )],
            ]);
            bot.edit_message_text(
                chat_id,
                msg.id,
                locales::get_text(lang, "msg-choose-lang", None),
            )
            .reply_markup(keyboard)
            .await?;
        }
        SettingsAction::LangSet { lang: new_lang } => {
            db.update_language(telegram_id, &new_lang).await.ok();
            bot.answer_callback_query(q.id)
                .text(locales::get_text(&new_lang, "toast-lang-updated", None))
                .await?;
            send_main_settings_edit(&bot, &msg, &new_lang).await?;
        }
        SettingsAction::SubSelect => {
            send_sub_settings(&bot, &msg, db, telegram_id, lang).await?;
        }
        SettingsAction::SubSet { setting } => {
            let new_setting = NotificationSetting::from(setting.as_str());
            db.update_notification_setting(telegram_id, new_setting.clone())
                .await
                .ok();

            let text_key = match new_setting {
                NotificationSetting::All => "btn-sub-all",
                NotificationSetting::JoinOff => "btn-sub-join",
                NotificationSetting::LeaveOff => "btn-sub-leave",
                NotificationSetting::None => "btn-sub-none",
            };
            let setting_text = locales::get_text(lang, text_key, args!(marker = "").as_ref());
            bot.answer_callback_query(q.id)
                .text(locales::get_text(
                    lang,
                    "resp-sub-updated",
                    args!(text = setting_text).as_ref(),
                ))
                .await?;
            send_sub_settings(&bot, &msg, db, telegram_id, lang).await?;
        }
        SettingsAction::NotifSelect => {
            send_notif_settings(&bot, &msg, db, telegram_id, lang).await?;
        }
        SettingsAction::NoonToggle => {
            let user_settings = db.get_or_create_user(telegram_id, "en").await.ok();
            if let Some(u) = user_settings {
                if u.teamtalk_username.is_none() {
                    bot.answer_callback_query(q.id)
                        .text(locales::get_text(lang, "cmd-fail-noon-guest", None))
                        .show_alert(true)
                        .await?;
                    return Ok(());
                }
                match db.toggle_noon(telegram_id).await {
                    Ok(new_val) => {
                        let status = if new_val {
                            locales::get_text(lang, "status-enabled", None)
                        } else {
                            locales::get_text(lang, "status-disabled", None)
                        };

                        let _ = bot
                            .answer_callback_query(q.id)
                            .text(locales::get_text(
                                lang,
                                "resp-noon-updated",
                                args!(status = status).as_ref(),
                            ))
                            .await;

                        if let Err(e) = send_notif_settings(&bot, &msg, db, telegram_id, lang).await
                            && !e.to_string().contains("message is not modified")
                        {
                            return Err(e);
                        }
                    }
                    Err(e) => {
                        tracing::error!("DB error in toggle_noon: {}", e);
                        bot.answer_callback_query(q.id)
                            .text(locales::get_text(lang, "cmd-error", None))
                            .show_alert(true)
                            .await?;
                    }
                }
            }
        }
        SettingsAction::MuteManage => {
            if let Ok(u) = db.get_or_create_user(telegram_id, "en").await {
                send_mute_menu(&bot, &msg, lang, &u.mute_list_mode).await?;
            }
        }
    }
    Ok(())
}
```

---

### `src/tg_bot/callback_handlers/subscriber.rs`

```rust
use crate::tg_bot::admin_logic::subscriber_settings::{
    send_sub_lang_menu, send_sub_link_account_list, send_sub_manage_tt_menu, send_sub_mute_list,
    send_sub_mute_mode_menu, send_sub_notif_menu,
};
use crate::tg_bot::admin_logic::subscribers::{edit_subscribers_list, send_subscriber_details};
use crate::tg_bot::callbacks_types::SubAction;
use crate::tg_bot::state::AppState;
use crate::types::{MuteListMode, NotificationSetting, TtCommand};
use crate::{args, locales};
use teloxide::prelude::*;

pub async fn handle_subscriber_actions(
    bot: Bot,
    q: CallbackQuery,
    state: AppState,
    action: SubAction,
    lang: &str,
) -> ResponseResult<()> {
    let msg = match q.message {
        Some(teloxide::types::MaybeInaccessibleMessage::Regular(m)) => m,
        _ => return Ok(()),
    };
    let db = &state.db;
    let user_accounts = &state.user_accounts;
    let tx_tt = &state.tx_tt;

    match action {
        SubAction::Details { sub_id, page } => {
            send_subscriber_details(&bot, &msg, db, lang, sub_id, page).await?;
            bot.answer_callback_query(q.id).await?;
        }
        SubAction::Delete { sub_id, page } => {
            db.delete_user_profile(sub_id).await.ok();
            bot.answer_callback_query(q.id)
                .text(locales::get_text(lang, "toast-subscriber-deleted", None))
                .show_alert(true)
                .await?;
            edit_subscribers_list(&bot, &msg, db, lang, page).await?;
        }
        SubAction::Ban { sub_id, page } => {
            let tt_user = sqlx::query_scalar::<_, String>(
                "SELECT teamtalk_username FROM user_settings WHERE telegram_id = ?",
            )
            .bind(sub_id)
            .fetch_optional(&db.pool)
            .await
            .unwrap_or(None);

            db.add_ban(Some(sub_id), tt_user, Some("Admin Ban".to_string()))
                .await
                .ok();
            db.delete_user_profile(sub_id).await.ok();

            bot.answer_callback_query(q.id)
                .text(locales::get_text(lang, "toast-user-banned", None))
                .show_alert(true)
                .await?;
            edit_subscribers_list(&bot, &msg, db, lang, page).await?;
        }
        SubAction::ManageTt { sub_id, page } => {
            send_sub_manage_tt_menu(&bot, &msg, db, lang, sub_id, page).await?;
        }
        SubAction::Unlink { sub_id, page } => {
            db.unlink_tt_account(sub_id).await.ok();
            bot.answer_callback_query(q.id)
                .text(locales::get_text(
                    lang,
                    "toast-account-unlinked",
                    args!(user = sub_id.to_string()).as_ref(),
                ))
                .show_alert(true)
                .await?;
            send_sub_manage_tt_menu(&bot, &msg, db, lang, sub_id, page).await?;
        }
        SubAction::LinkList {
            sub_id,
            page,
            list_page,
        } => {
            tx_tt.send(TtCommand::LoadAccounts).ok();
            send_sub_link_account_list(&bot, &msg, user_accounts, lang, sub_id, page, list_page)
                .await?;
        }
        SubAction::LinkPerform {
            sub_id,
            page,
            username,
        } => {
            db.link_tt_account(sub_id, &username).await.ok();
            bot.answer_callback_query(q.id)
                .text(locales::get_text(
                    lang,
                    "toast-account-linked",
                    args!(user = username).as_ref(),
                ))
                .show_alert(true)
                .await?;
            send_sub_manage_tt_menu(&bot, &msg, db, lang, sub_id, page).await?;
        }
        SubAction::LangMenu { sub_id, page } => {
            send_sub_lang_menu(&bot, &msg, lang, sub_id, page).await?;
        }
        SubAction::LangSet {
            sub_id,
            page,
            lang: new_lang,
        } => {
            db.update_language(sub_id, &new_lang).await.ok();
            bot.answer_callback_query(q.id)
                .text(locales::get_text(
                    lang,
                    "toast-lang-set",
                    args!(id = sub_id.to_string(), lang = new_lang).as_ref(),
                ))
                .await?;
            send_subscriber_details(&bot, &msg, db, lang, sub_id, page).await?;
        }
        SubAction::NotifMenu { sub_id, page } => {
            send_sub_notif_menu(&bot, &msg, lang, sub_id, page).await?;
        }
        SubAction::NotifSet { sub_id, page, val } => {
            db.update_notification_setting(sub_id, NotificationSetting::from(val.as_str()))
                .await
                .ok();
            bot.answer_callback_query(q.id)
                .text(locales::get_text(
                    lang,
                    "toast-notif-set",
                    args!(id = sub_id.to_string(), val = val).as_ref(),
                ))
                .await?;
            send_subscriber_details(&bot, &msg, db, lang, sub_id, page).await?;
        }
        SubAction::NoonToggle { sub_id, page } => {
            db.toggle_noon(sub_id).await.ok();
            let status = "toggled";
            bot.answer_callback_query(q.id)
                .text(locales::get_text(
                    lang,
                    "toast-noon-toggled",
                    args!(id = sub_id.to_string(), status = status).as_ref(),
                ))
                .await?;
            send_subscriber_details(&bot, &msg, db, lang, sub_id, page).await?;
        }
        SubAction::ModeMenu { sub_id, page } => {
            send_sub_mute_mode_menu(&bot, &msg, lang, sub_id, page).await?;
        }
        SubAction::ModeSet { sub_id, page, mode } => {
            db.update_mute_mode(sub_id, MuteListMode::from(mode.as_str()))
                .await
                .ok();
            bot.answer_callback_query(q.id)
                .text(locales::get_text(
                    lang,
                    "toast-mute-mode-sub-set",
                    args!(id = sub_id.to_string(), val = mode).as_ref(),
                ))
                .await?;
            send_subscriber_details(&bot, &msg, db, lang, sub_id, page).await?;
        }
        SubAction::MuteView {
            sub_id,
            page,
            view_page,
        } => {
            send_sub_mute_list(&bot, &msg, db, lang, sub_id, page, view_page).await?;
        }
    }
    Ok(())
}
```

---

### `src/tg_bot/callback_handlers/unsub.rs`

```rust
use crate::locales;
use crate::tg_bot::callbacks_types::UnsubAction;
use crate::tg_bot::state::AppState;
use teloxide::prelude::*;

pub async fn handle_unsub_action(
    bot: Bot,
    q: CallbackQuery,
    state: AppState,
    action: UnsubAction,
    lang: &str,
) -> ResponseResult<()> {
    let msg = match q.message {
        Some(teloxide::types::MaybeInaccessibleMessage::Regular(m)) => m,
        _ => return Ok(()),
    };
    let telegram_id = q.from.id.0 as i64;
    let db = &state.db;

    match action {
        UnsubAction::Confirm => {
            if let Err(e) = db.delete_user_profile(telegram_id).await {
                tracing::error!("Failed to unsubscribe user {}: {}", telegram_id, e);
                bot.answer_callback_query(q.id)
                    .text("Database error")
                    .await?;
                return Ok(());
            }
            bot.edit_message_text(
                msg.chat.id,
                msg.id,
                locales::get_text(lang, "cmd-success-unsub", None),
            )
            .await?;
            bot.answer_callback_query(q.id)
                .text(locales::get_text(lang, "cmd-success-unsub", None))
                .await?;
        }
        UnsubAction::Cancel => {
            bot.edit_message_text(
                msg.chat.id,
                msg.id,
                locales::get_text(lang, "unsub-cancelled", None),
            )
            .await?;
            bot.answer_callback_query(q.id).await?;
        }
    }
    Ok(())
}
```

---

### `src/tg_bot/callbacks.rs`

```rust
use crate::locales;
use crate::tg_bot::callback_handlers::{admin, menu, mute, settings, subscriber, unsub};
use crate::tg_bot::callbacks_types::CallbackAction;
use crate::tg_bot::state::AppState;
use std::str::FromStr;
use teloxide::prelude::*;
use teloxide::types::MaybeInaccessibleMessage;

pub async fn answer_callback(bot: Bot, q: CallbackQuery, state: AppState) -> ResponseResult<()> {
    let query_id = q.id.clone();
    let telegram_id = q.from.id.0 as i64;
    let callback_data_str = q.data.clone().unwrap_or_default();

    let db = &state.db;
    let config = &state.config;

    let _msg = match &q.message {
        Some(MaybeInaccessibleMessage::Regular(m)) => m,
        _ => return Ok(()),
    };

    let user_settings = match db
        .get_or_create_user(telegram_id, &config.general.default_lang)
        .await
    {
        Ok(settings) => settings,
        Err(e) => {
            tracing::error!(
                "Failed to get/create user {} in callback: {}",
                telegram_id,
                e
            );
            bot.answer_callback_query(q.id)
                .text("Database error.")
                .show_alert(true)
                .await?;
            return Ok(());
        }
    };
    let lang = &user_settings.language_code;

    if let Ok(false) = db.is_subscribed(telegram_id).await {
        bot.answer_callback_query(query_id)
            .text(locales::get_text(lang, "cmd-not-subscribed", None))
            .show_alert(true)
            .await?;
        return Ok(());
    }

    let action = match CallbackAction::from_str(&callback_data_str) {
        Ok(a) => a,
        Err(e) => {
            tracing::warn!(
                "Unknown or legacy callback data '{}': {}",
                callback_data_str,
                e
            );
            return Ok(());
        }
    };

    match action {
        CallbackAction::Menu(menu_act) => {
            menu::handle_menu(bot, q, state, menu_act, lang).await?;
        }
        CallbackAction::Admin(admin_act) => {
            admin::handle_admin(bot, q, state, admin_act, lang).await?;
        }
        CallbackAction::Settings(sett_act) => {
            settings::handle_settings(bot, q, state, sett_act, lang).await?;
        }
        CallbackAction::Subscriber(sub_act) => {
            subscriber::handle_subscriber_actions(bot, q, state, sub_act, lang).await?;
        }
        CallbackAction::Mute(mute_act) => {
            mute::handle_mute(bot, q, state, mute_act, lang).await?;
        }
        CallbackAction::Unsub(unsub_act) => {
            unsub::handle_unsub_action(bot, q, state, unsub_act, lang).await?;
        }
        CallbackAction::NoOp => {
            bot.answer_callback_query(q.id).await?;
        }
    }

    Ok(())
}
```

---

### `src/tg_bot/callbacks_types.rs`

```rust
use anyhow::{Result, anyhow};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum CallbackAction {
    Menu(MenuAction),
    Admin(AdminAction),
    Settings(SettingsAction),
    Subscriber(SubAction),
    Mute(MuteAction),
    Unsub(UnsubAction),
    NoOp,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MenuAction {
    Who,
    Settings,
    Help,
    Unsub,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AdminAction {
    KickList { page: usize },
    KickPerform { user_id: i32 },
    BanList { page: usize },
    BanPerform { user_id: i32 },
    UnbanList { page: usize },
    UnbanPerform { ban_db_id: i64, page: usize },
    SubsList { page: usize },
}

#[derive(Debug, PartialEq, Clone)]
pub enum SettingsAction {
    Main,
    LangSelect,
    LangSet { lang: String },
    SubSelect,
    SubSet { setting: String },
    NotifSelect,
    NoonToggle,
    MuteManage,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SubAction {
    Details {
        sub_id: i64,
        page: usize,
    },
    Delete {
        sub_id: i64,
        page: usize,
    },
    Ban {
        sub_id: i64,
        page: usize,
    },
    ManageTt {
        sub_id: i64,
        page: usize,
    },
    Unlink {
        sub_id: i64,
        page: usize,
    },
    LinkList {
        sub_id: i64,
        page: usize,
        list_page: usize,
    },
    LinkPerform {
        sub_id: i64,
        page: usize,
        username: String,
    },
    LangMenu {
        sub_id: i64,
        page: usize,
    },
    LangSet {
        sub_id: i64,
        page: usize,
        lang: String,
    },
    NotifMenu {
        sub_id: i64,
        page: usize,
    },
    NotifSet {
        sub_id: i64,
        page: usize,
        val: String,
    },
    NoonToggle {
        sub_id: i64,
        page: usize,
    },
    ModeMenu {
        sub_id: i64,
        page: usize,
    },
    ModeSet {
        sub_id: i64,
        page: usize,
        mode: String,
    },
    MuteView {
        sub_id: i64,
        page: usize,
        view_page: usize,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum MuteAction {
    ModeSet { mode: String },
    Menu { mode: String },
    List { page: usize },
    Toggle { username: String, page: usize },
    ServerList { page: usize },
    ServerToggle { username: String, page: usize },
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnsubAction {
    Confirm,
    Cancel,
}

impl fmt::Display for CallbackAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallbackAction::Menu(a) => write!(f, "m:{}", a),
            CallbackAction::Admin(a) => write!(f, "a:{}", a),
            CallbackAction::Settings(a) => write!(f, "s:{}", a),
            CallbackAction::Subscriber(a) => write!(f, "u:{}", a),
            CallbackAction::Mute(a) => write!(f, "mt:{}", a),
            CallbackAction::Unsub(a) => write!(f, "x:{}", a),
            CallbackAction::NoOp => write!(f, "noop"),
        }
    }
}

impl fmt::Display for MenuAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenuAction::Who => write!(f, "who"),
            MenuAction::Settings => write!(f, "set"),
            MenuAction::Help => write!(f, "hlp"),
            MenuAction::Unsub => write!(f, "uns"),
        }
    }
}

impl fmt::Display for AdminAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdminAction::KickList { page } => write!(f, "kl:{}", page),
            AdminAction::KickPerform { user_id } => write!(f, "kp:{}", user_id),
            AdminAction::BanList { page } => write!(f, "bl:{}", page),
            AdminAction::BanPerform { user_id } => write!(f, "bp:{}", user_id),
            AdminAction::UnbanList { page } => write!(f, "ul:{}", page),
            AdminAction::UnbanPerform { ban_db_id, page } => write!(f, "up:{}:{}", ban_db_id, page),
            AdminAction::SubsList { page } => write!(f, "sl:{}", page),
        }
    }
}

impl fmt::Display for SettingsAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingsAction::Main => write!(f, "main"),
            SettingsAction::LangSelect => write!(f, "lsel"),
            SettingsAction::LangSet { lang } => write!(f, "lset:{}", lang),
            SettingsAction::SubSelect => write!(f, "ssel"),
            SettingsAction::SubSet { setting } => write!(f, "sset:{}", setting),
            SettingsAction::NotifSelect => write!(f, "nsel"),
            SettingsAction::NoonToggle => write!(f, "noon"),
            SettingsAction::MuteManage => write!(f, "mm"),
        }
    }
}

impl fmt::Display for SubAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubAction::Details { sub_id, page } => write!(f, "det:{}:{}", sub_id, page),
            SubAction::Delete { sub_id, page } => write!(f, "del:{}:{}", sub_id, page),
            SubAction::Ban { sub_id, page } => write!(f, "ban:{}:{}", sub_id, page),
            SubAction::ManageTt { sub_id, page } => write!(f, "mtt:{}:{}", sub_id, page),
            SubAction::Unlink { sub_id, page } => write!(f, "unl:{}:{}", sub_id, page),
            SubAction::LinkList {
                sub_id,
                page,
                list_page,
            } => write!(f, "llst:{}:{}:{}", sub_id, page, list_page),
            SubAction::LinkPerform {
                sub_id,
                page,
                username,
            } => write!(f, "lprf:{}:{}:{}", sub_id, page, username),
            SubAction::LangMenu { sub_id, page } => write!(f, "lmn:{}:{}", sub_id, page),
            SubAction::LangSet { sub_id, page, lang } => {
                write!(f, "lset:{}:{}:{}", sub_id, page, lang)
            }
            SubAction::NotifMenu { sub_id, page } => write!(f, "nmn:{}:{}", sub_id, page),
            SubAction::NotifSet { sub_id, page, val } => {
                write!(f, "nset:{}:{}:{}", sub_id, page, val)
            }
            SubAction::NoonToggle { sub_id, page } => write!(f, "noon:{}:{}", sub_id, page),
            SubAction::ModeMenu { sub_id, page } => write!(f, "mmn:{}:{}", sub_id, page),
            SubAction::ModeSet { sub_id, page, mode } => {
                write!(f, "mset:{}:{}:{}", sub_id, page, mode)
            }
            SubAction::MuteView {
                sub_id,
                page,
                view_page,
            } => write!(f, "mvw:{}:{}:{}", sub_id, page, view_page),
        }
    }
}

impl fmt::Display for MuteAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MuteAction::ModeSet { mode } => write!(f, "mset:{}", mode),
            MuteAction::Menu { mode } => write!(f, "menu:{}", mode),
            MuteAction::List { page } => write!(f, "lst:{}", page),
            MuteAction::Toggle { username, page } => write!(f, "tgl:{}:{}", page, username),
            MuteAction::ServerList { page } => write!(f, "slst:{}", page),
            MuteAction::ServerToggle { username, page } => write!(f, "stgl:{}:{}", page, username),
        }
    }
}

impl fmt::Display for UnsubAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnsubAction::Confirm => write!(f, "yes"),
            UnsubAction::Cancel => write!(f, "no"),
        }
    }
}

impl FromStr for CallbackAction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(CallbackAction::NoOp);
        }

        let mut parts = s.splitn(2, ':');
        let prefix = parts.next().ok_or_else(|| anyhow!("Empty callback"))?;
        let rest = parts.next().unwrap_or("");

        match prefix {
            "m" => Ok(CallbackAction::Menu(MenuAction::from_str(rest)?)),
            "a" => Ok(CallbackAction::Admin(AdminAction::from_str(rest)?)),
            "s" => Ok(CallbackAction::Settings(SettingsAction::from_str(rest)?)),
            "u" => Ok(CallbackAction::Subscriber(SubAction::from_str(rest)?)),
            "mt" => Ok(CallbackAction::Mute(MuteAction::from_str(rest)?)),
            "x" => Ok(CallbackAction::Unsub(UnsubAction::from_str(rest)?)),
            _ => Err(anyhow!("Unknown category: {}", prefix)),
        }
    }
}

impl FromStr for MenuAction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "who" => Ok(MenuAction::Who),
            "set" => Ok(MenuAction::Settings),
            "hlp" => Ok(MenuAction::Help),
            "uns" => Ok(MenuAction::Unsub),
            _ => Err(anyhow!("Unknown menu action")),
        }
    }
}

impl FromStr for AdminAction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let cmd = parts.next().ok_or_else(|| anyhow!("No admin cmd"))?;

        match cmd {
            "kl" => Ok(AdminAction::KickList {
                page: parts.next().unwrap_or("0").parse()?,
            }),
            "kp" => Ok(AdminAction::KickPerform {
                user_id: parts.next().ok_or(anyhow!("No ID"))?.parse()?,
            }),
            "bl" => Ok(AdminAction::BanList {
                page: parts.next().unwrap_or("0").parse()?,
            }),
            "bp" => Ok(AdminAction::BanPerform {
                user_id: parts.next().ok_or(anyhow!("No ID"))?.parse()?,
            }),
            "ul" => Ok(AdminAction::UnbanList {
                page: parts.next().unwrap_or("0").parse()?,
            }),
            "up" => {
                let id = parts.next().ok_or(anyhow!("No ID"))?.parse()?;
                let page = parts.next().unwrap_or("0").parse()?;
                Ok(AdminAction::UnbanPerform {
                    ban_db_id: id,
                    page,
                })
            }
            "sl" => Ok(AdminAction::SubsList {
                page: parts.next().unwrap_or("0").parse()?,
            }),
            _ => Err(anyhow!("Unknown admin cmd")),
        }
    }
}

impl FromStr for SettingsAction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let cmd = parts.next().ok_or_else(|| anyhow!("No setting cmd"))?;

        match cmd {
            "main" => Ok(SettingsAction::Main),
            "lsel" => Ok(SettingsAction::LangSelect),
            "lset" => Ok(SettingsAction::LangSet {
                lang: parts.next().unwrap_or("en").to_string(),
            }),
            "ssel" => Ok(SettingsAction::SubSelect),
            "sset" => Ok(SettingsAction::SubSet {
                setting: parts.next().unwrap_or("all").to_string(),
            }),
            "nsel" => Ok(SettingsAction::NotifSelect),
            "noon" => Ok(SettingsAction::NoonToggle),
            "mm" => Ok(SettingsAction::MuteManage),
            _ => Err(anyhow!("Unknown setting cmd")),
        }
    }
}

impl FromStr for SubAction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let cmd = parts.next().ok_or_else(|| anyhow!("No sub cmd"))?;
        let sub_id: i64 = parts.next().ok_or(anyhow!("No sub_id"))?.parse()?;
        let page: usize = parts.next().unwrap_or("0").parse()?;

        let get_rest =
            |iter: std::str::Split<'_, char>| -> String { iter.collect::<Vec<&str>>().join(":") };

        match cmd {
            "det" => Ok(SubAction::Details { sub_id, page }),
            "del" => Ok(SubAction::Delete { sub_id, page }),
            "ban" => Ok(SubAction::Ban { sub_id, page }),
            "mtt" => Ok(SubAction::ManageTt { sub_id, page }),
            "unl" => Ok(SubAction::Unlink { sub_id, page }),
            "llst" => {
                let list_page = parts.next().unwrap_or("0").parse()?;
                Ok(SubAction::LinkList {
                    sub_id,
                    page,
                    list_page,
                })
            }
            "lprf" => {
                let username = get_rest(parts);
                Ok(SubAction::LinkPerform {
                    sub_id,
                    page,
                    username,
                })
            }
            "lmn" => Ok(SubAction::LangMenu { sub_id, page }),
            "lset" => Ok(SubAction::LangSet {
                sub_id,
                page,
                lang: parts.next().unwrap_or("en").to_string(),
            }),
            "nmn" => Ok(SubAction::NotifMenu { sub_id, page }),
            "nset" => Ok(SubAction::NotifSet {
                sub_id,
                page,
                val: parts.next().unwrap_or("all").to_string(),
            }),
            "noon" => Ok(SubAction::NoonToggle { sub_id, page }),
            "mmn" => Ok(SubAction::ModeMenu { sub_id, page }),
            "mset" => Ok(SubAction::ModeSet {
                sub_id,
                page,
                mode: parts.next().unwrap_or("blacklist").to_string(),
            }),
            "mvw" => {
                let view_page = parts.next().unwrap_or("0").parse()?;
                Ok(SubAction::MuteView {
                    sub_id,
                    page,
                    view_page,
                })
            }
            _ => Err(anyhow!("Unknown sub cmd")),
        }
    }
}

impl FromStr for MuteAction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let cmd = parts.next().ok_or_else(|| anyhow!("No mute cmd"))?;

        let get_rest =
            |iter: std::str::Split<'_, char>| -> String { iter.collect::<Vec<&str>>().join(":") };

        match cmd {
            "mset" => Ok(MuteAction::ModeSet {
                mode: parts.next().unwrap_or("blacklist").to_string(),
            }),
            "menu" => Ok(MuteAction::Menu {
                mode: parts.next().unwrap_or("blacklist").to_string(),
            }),
            "lst" => Ok(MuteAction::List {
                page: parts.next().unwrap_or("0").parse()?,
            }),
            "tgl" => {
                let page = parts.next().unwrap_or("0").parse()?;
                let username = get_rest(parts);
                Ok(MuteAction::Toggle { username, page })
            }
            "slst" => Ok(MuteAction::ServerList {
                page: parts.next().unwrap_or("0").parse()?,
            }),
            "stgl" => {
                let page = parts.next().unwrap_or("0").parse()?;
                let username = get_rest(parts);
                Ok(MuteAction::ServerToggle { username, page })
            }
            _ => Err(anyhow!("Unknown mute cmd")),
        }
    }
}

impl FromStr for UnsubAction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yes" => Ok(UnsubAction::Confirm),
            "no" => Ok(UnsubAction::Cancel),
            _ => Err(anyhow!("Unknown unsub cmd")),
        }
    }
}
```

---

### `src/tg_bot/commands.rs`

```rust
use crate::args;
use crate::locales;
use crate::tg_bot::admin_logic::bans::send_unban_list;
use crate::tg_bot::admin_logic::subscribers::send_subscribers_list;
use crate::tg_bot::callbacks_types::{AdminAction, CallbackAction, UnsubAction};
use crate::tg_bot::keyboards::{create_main_menu_keyboard, create_user_list_keyboard};
use crate::tg_bot::settings_logic::send_main_settings;
use crate::tg_bot::state::AppState;
use crate::tg_bot::utils::ensure_subscribed;
use crate::types::{LiteUser, TtCommand};
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode};
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "Available Commands:")]
pub enum Command {
    #[command(description = "Start")]
    Start(String),
    #[command(description = "Main Menu")]
    Menu,
    #[command(description = "Help")]
    Help,
    #[command(description = "Who is online")]
    Who,
    #[command(description = "Settings")]
    Settings,
    #[command(description = "Unsubscribe")]
    Unsub,
    #[command(description = "Kick (Admin)")]
    Kick,
    #[command(description = "Ban (Admin)")]
    Ban,
    #[command(description = "Unban (Admin)")]
    Unban,
    #[command(description = "Subscribers (Admin)")]
    Subscribers,
    #[command(description = "Exit (Admin)")]
    Exit,
}

pub async fn answer_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
    state: AppState,
) -> ResponseResult<()> {
    let user = if let Some(user) = &msg.from {
        user
    } else {
        return Ok(());
    };
    let telegram_id = user.id.0 as i64;

    let db = &state.db;
    let config = &state.config;
    let online_users = &state.online_users;
    let tx_tt = &state.tx_tt;

    let settings = match db
        .get_or_create_user(telegram_id, &config.general.default_lang)
        .await
    {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Failed to get or create user {}: {}", telegram_id, e);
            bot.send_message(msg.chat.id, "Database error. Please try again later.")
                .await?;
            return Ok(());
        }
    };
    let lang = &settings.language_code;
    let is_admin = db
        .get_all_admins()
        .await
        .unwrap_or_default()
        .contains(&telegram_id);

    match cmd {
        Command::Start(token) => {
            if !token.is_empty() {
                if let Ok(Some(deeplink)) = db.resolve_deeplink(&token).await {
                    match deeplink.action.as_str() {
                        "subscribe" => {
                            if db.is_telegram_id_banned(telegram_id).await.unwrap_or(false) {
                                bot.send_message(
                                    msg.chat.id,
                                    locales::get_text(lang, "cmd-user-banned", None),
                                )
                                .await?;
                                return Ok(());
                            }

                            if let Some(tt_nick) = &deeplink.payload
                                && db
                                    .is_teamtalk_username_banned(tt_nick)
                                    .await
                                    .unwrap_or(false)
                            {
                                let args = args!(name = tt_nick.clone());
                                bot.send_message(
                                    msg.chat.id,
                                    locales::get_text(lang, "cmd-tt-banned", args.as_ref()),
                                )
                                .await?;
                                return Ok(());
                            }

                            db.add_subscriber(telegram_id).await.ok();

                            if let Some(tt_nick) = deeplink.payload {
                                db.link_tt_account(telegram_id, &tt_nick).await.ok();
                                let msg_key = "cmd-success-sub";
                                bot.send_message(
                                    msg.chat.id,
                                    locales::get_text(lang, msg_key, None),
                                )
                                .await?;
                            } else {
                                let msg_key = "cmd-success-sub-guest";
                                bot.send_message(
                                    msg.chat.id,
                                    locales::get_text(lang, msg_key, None),
                                )
                                .parse_mode(ParseMode::Html)
                                .await?;
                            }
                        }
                        "unsubscribe" => {
                            db.delete_user_profile(telegram_id).await.ok();
                            bot.send_message(
                                msg.chat.id,
                                locales::get_text(lang, "cmd-success-unsub", None),
                            )
                            .await?;
                        }
                        _ => {
                            bot.send_message(
                                msg.chat.id,
                                locales::get_text(lang, "cmd-invalid-deeplink", None),
                            )
                            .await?;
                        }
                    }
                } else {
                    bot.send_message(
                        msg.chat.id,
                        locales::get_text(lang, "cmd-invalid-deeplink", None),
                    )
                    .await?;
                }
            } else {
                bot.send_message(msg.chat.id, locales::get_text(lang, "hello-start", None))
                    .await?;
            }
        }
        Command::Menu => {
            if !ensure_subscribed(&bot, &msg, db, lang).await {
                return Ok(());
            }
            let keyboard = create_main_menu_keyboard(lang, is_admin);
            bot.send_message(msg.chat.id, locales::get_text(lang, "menu-title", None))
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
        }
        Command::Help => {
            if !ensure_subscribed(&bot, &msg, db, lang).await {
                return Ok(());
            }
            bot.send_message(msg.chat.id, locales::get_text(lang, "help-text", None))
                .parse_mode(ParseMode::Html)
                .await?;
        }
        Command::Who => {
            if !ensure_subscribed(&bot, &msg, db, lang).await {
                return Ok(());
            }
            let _ = tx_tt.send(TtCommand::Who {
                chat_id: msg.chat.id.0,
                lang: lang.clone(),
            });
        }
        Command::Settings => {
            if !ensure_subscribed(&bot, &msg, db, lang).await {
                return Ok(());
            }
            send_main_settings(&bot, msg.chat.id, lang).await?;
        }
        Command::Unsub => {
            if !ensure_subscribed(&bot, &msg, db, lang).await {
                return Ok(());
            }
            let text = locales::get_text(lang, "unsub-confirm-text", None);
            let keyboard = InlineKeyboardMarkup::new(vec![vec![
                InlineKeyboardButton::callback(
                    locales::get_text(lang, "btn-yes", None),
                    CallbackAction::Unsub(UnsubAction::Confirm).to_string(),
                ),
                InlineKeyboardButton::callback(
                    locales::get_text(lang, "btn-no", None),
                    CallbackAction::Unsub(UnsubAction::Cancel).to_string(),
                ),
            ]]);

            bot.send_message(msg.chat.id, text)
                .reply_markup(keyboard)
                .await?;
        }
        Command::Kick | Command::Ban => {
            if !is_admin {
                bot.send_message(msg.chat.id, locales::get_text(lang, "cmd-unauth", None))
                    .await?;
                return Ok(());
            }

            let mut users: Vec<LiteUser> = online_users.iter().map(|u| u.value().clone()).collect();
            users.sort_by(|a, b| a.nickname.to_lowercase().cmp(&b.nickname.to_lowercase()));

            let is_kick = matches!(cmd, Command::Kick);
            let title_key = if is_kick {
                "list-kick-title"
            } else {
                "list-ban-title"
            };

            let args = args!(server = config.teamtalk.display_name().to_string());
            let title = locales::get_text(lang, title_key, args.as_ref());

            let keyboard = create_user_list_keyboard(
                &users,
                0,
                move |u| {
                    let action = if is_kick {
                        AdminAction::KickPerform { user_id: u.id }
                    } else {
                        AdminAction::BanPerform { user_id: u.id }
                    };
                    (u.nickname.clone(), CallbackAction::Admin(action))
                },
                move |p| {
                    let action = if is_kick {
                        AdminAction::KickList { page: p }
                    } else {
                        AdminAction::BanList { page: p }
                    };
                    CallbackAction::Admin(action)
                },
                None,
                lang,
            );

            bot.send_message(msg.chat.id, title)
                .reply_markup(keyboard)
                .await?;
        }
        Command::Unban => {
            if !is_admin {
                bot.send_message(msg.chat.id, locales::get_text(lang, "cmd-unauth", None))
                    .await?;
                return Ok(());
            }
            send_unban_list(&bot, msg.chat.id, db, lang, 0).await?;
        }
        Command::Subscribers => {
            if !is_admin {
                bot.send_message(msg.chat.id, locales::get_text(lang, "cmd-unauth", None))
                    .await?;
                return Ok(());
            }
            send_subscribers_list(&bot, msg.chat.id, db, lang, 0).await?;
        }
        Command::Exit => {
            if !is_admin {
                bot.send_message(msg.chat.id, locales::get_text(lang, "cmd-unauth", None))
                    .await?;
                return Ok(());
            }
            bot.send_message(
                msg.chat.id,
                locales::get_text(lang, "cmd-shutting-down", None),
            )
            .await?;
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            std::process::exit(0);
        }
    }
    Ok(())
}
```

---

### `src/tg_bot/keyboards.rs`

```rust
use crate::locales;
use crate::tg_bot::callbacks_types::CallbackAction;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub const USERS_PER_PAGE: usize = 10;

pub fn create_pagination_keyboard<F>(
    current_page: usize,
    total_pages: usize,
    page_builder: F,
    back_btn: Option<(String, CallbackAction)>,
    lang: &str,
) -> InlineKeyboardMarkup
where
    F: Fn(usize) -> CallbackAction,
{
    let mut buttons = vec![];
    let mut nav_row = vec![];

    if current_page > 0 {
        let data = page_builder(current_page - 1).to_string();
        nav_row.push(InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-prev", None),
            data,
        ));
    }

    if total_pages > 0 && current_page < total_pages - 1 {
        let data = page_builder(current_page + 1).to_string();
        nav_row.push(InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-next", None),
            data,
        ));
    }

    if !nav_row.is_empty() {
        buttons.push(nav_row);
    }

    if let Some((text, action)) = back_btn {
        buttons.push(vec![InlineKeyboardButton::callback(
            text,
            action.to_string(),
        )]);
    }

    InlineKeyboardMarkup::new(buttons)
}

pub fn create_user_list_keyboard<T, FMap, FPage>(
    items: &[T],
    page: usize,
    item_mapper: FMap,
    page_builder: FPage,
    back_btn: Option<(String, CallbackAction)>,
    lang: &str,
) -> InlineKeyboardMarkup
where
    FMap: Fn(&T) -> (String, CallbackAction),
    FPage: Fn(usize) -> CallbackAction,
{
    let total_items = items.len();
    let total_pages = total_items.div_ceil(USERS_PER_PAGE);
    let page = if total_pages == 0 {
        0
    } else {
        page.min(total_pages - 1)
    };

    let start = page * USERS_PER_PAGE;
    let end = (start + USERS_PER_PAGE).min(total_items);
    let slice = if start < total_items {
        &items[start..end]
    } else {
        &[]
    };

    let mut buttons = vec![];
    for item in slice {
        let (name, action) = item_mapper(item);
        buttons.push(vec![InlineKeyboardButton::callback(
            name,
            action.to_string(),
        )]);
    }

    let nav_kb = create_pagination_keyboard(page, total_pages, page_builder, back_btn, lang);

    let mut final_buttons = buttons;
    for row in nav_kb.inline_keyboard {
        final_buttons.push(row);
    }
    InlineKeyboardMarkup::new(final_buttons)
}

pub fn create_main_menu_keyboard(lang: &str, is_admin: bool) -> InlineKeyboardMarkup {
    use crate::tg_bot::callbacks_types::{AdminAction, MenuAction};

    let mut buttons = vec![
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-menu-who", None),
            CallbackAction::Menu(MenuAction::Who).to_string(),
        )],
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-menu-settings", None),
            CallbackAction::Settings(crate::tg_bot::callbacks_types::SettingsAction::Main)
                .to_string(),
        )],
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-menu-unsub", None),
            CallbackAction::Menu(MenuAction::Unsub).to_string(),
        )],
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-menu-help", None),
            CallbackAction::Menu(MenuAction::Help).to_string(),
        )],
    ];

    if is_admin {
        buttons.push(vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-menu-kick", None),
            CallbackAction::Admin(AdminAction::KickList { page: 0 }).to_string(),
        )]);
        buttons.push(vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-menu-ban", None),
            CallbackAction::Admin(AdminAction::BanList { page: 0 }).to_string(),
        )]);
        buttons.push(vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-menu-unban", None),
            CallbackAction::Admin(AdminAction::UnbanList { page: 0 }).to_string(),
        )]);
        buttons.push(vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-menu-subs", None),
            CallbackAction::Admin(AdminAction::SubsList { page: 0 }).to_string(),
        )]);
    }

    InlineKeyboardMarkup::new(buttons)
}
```

---

### `src/tg_bot/mod.rs`

```rust
pub mod admin_logic;
pub mod callback_handlers;
pub mod callbacks;
pub mod callbacks_types;
pub mod commands;
pub mod keyboards;
pub mod settings_logic;
pub mod state;
pub mod utils;

use crate::config::Config;
use crate::db::Database;
use crate::locales;
use crate::types::{LiteUser, TtCommand};
use dashmap::DashMap;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use teamtalk::types::UserAccount;
use teloxide::{
    prelude::*,
    types::{BotCommand, BotCommandScope, Recipient},
};

use self::commands::Command;
use self::state::AppState;

pub async fn run_tg_bot(
    event_bot: Bot,
    db: Database,
    online_users: Arc<DashMap<i32, LiteUser>>,
    user_accounts: Arc<DashMap<String, UserAccount>>,
    tx_tt_cmd: Sender<TtCommand>,
    config: Arc<Config>,
) {
    let state = AppState {
        db: db.clone(),
        online_users,
        user_accounts,
        tx_tt: tx_tt_cmd,
        config: config.clone(),
    };

    if let Err(e) = set_bot_commands(&event_bot, &db, &config).await {
        tracing::error!("Failed to set bot commands: {}", e);
    }

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(commands::answer_command),
        )
        .branch(Update::filter_callback_query().endpoint(callbacks::answer_callback));

    Dispatcher::builder(event_bot, handler)
        .dependencies(dptree::deps![state])
        .enable_ctrlc_handler()
        .error_handler(std::sync::Arc::new(
            |err: teloxide::errors::RequestError| async move {
                let err_str = err.to_string();
                if !err_str.contains("TerminatedByOtherGetUpdates") {
                    tracing::error!("❌ [TELEGRAM] Update listener error: {}", err);
                }
            },
        ))
        .build()
        .dispatch()
        .await;
}

async fn set_bot_commands(
    bot: &Bot,
    db: &Database,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let languages = vec!["en", "ru"];

    let default_lang = &config.general.default_lang;
    let global_commands = get_user_commands(default_lang);
    bot.set_my_commands(global_commands)
        .scope(BotCommandScope::AllPrivateChats)
        .await?;

    for lang in &languages {
        if lang == default_lang {
            continue;
        }
        let cmds = get_user_commands(lang);
        bot.set_my_commands(cmds)
            .scope(BotCommandScope::AllPrivateChats)
            .language_code(*lang)
            .await?;
    }

    let admin_ids = db.get_all_admins().await.unwrap_or_default();
    for admin_id in admin_ids {
        let user_settings = db
            .get_or_create_user(admin_id, default_lang)
            .await
            .unwrap_or_else(|_| crate::db::types::UserSettings {
                telegram_id: admin_id,
                language_code: default_lang.clone(),
                notification_settings: "all".to_string(),
                mute_list_mode: "blacklist".to_string(),
                teamtalk_username: None,
                not_on_online_enabled: false,
                not_on_online_confirmed: false,
            });

        let admin_cmds = get_admin_commands(&user_settings.language_code);

        bot.set_my_commands(admin_cmds)
            .scope(BotCommandScope::Chat {
                chat_id: Recipient::Id(teloxide::types::ChatId(admin_id)),
            })
            .await
            .ok();
    }

    Ok(())
}

fn get_user_commands(lang: &str) -> Vec<BotCommand> {
    vec![
        BotCommand::new("menu", locales::get_text(lang, "cmd-desc-menu", None)),
        BotCommand::new("who", locales::get_text(lang, "cmd-desc-who", None)),
        BotCommand::new(
            "settings",
            locales::get_text(lang, "cmd-desc-settings", None),
        ),
        BotCommand::new("unsub", locales::get_text(lang, "cmd-desc-unsub", None)),
        BotCommand::new("help", locales::get_text(lang, "cmd-desc-help", None)),
    ]
}

fn get_admin_commands(lang: &str) -> Vec<BotCommand> {
    let mut cmds = get_user_commands(lang);
    cmds.extend(vec![
        BotCommand::new("kick", locales::get_text(lang, "cmd-desc-kick", None)),
        BotCommand::new("ban", locales::get_text(lang, "cmd-desc-ban", None)),
        BotCommand::new("unban", locales::get_text(lang, "cmd-desc-unban", None)),
        BotCommand::new(
            "subscribers",
            locales::get_text(lang, "cmd-desc-subscribers", None),
        ),
        BotCommand::new("exit", locales::get_text(lang, "cmd-desc-exit", None)),
    ]);
    cmds
}
```

---

### `src/tg_bot/settings_logic.rs`

```rust
use crate::args;
use crate::db::Database;
use crate::locales;
use crate::tg_bot::callbacks_types::{CallbackAction, MuteAction, SettingsAction};
use crate::tg_bot::keyboards::create_user_list_keyboard;
use crate::types::NotificationSetting;
use teamtalk::types::UserAccount;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode};

pub async fn send_main_settings(
    bot: &Bot,
    chat_id: teloxide::types::ChatId,
    lang: &str,
) -> ResponseResult<()> {
    let text = locales::get_text(lang, "settings-title", None);
    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-lang", None),
            CallbackAction::Settings(SettingsAction::LangSelect).to_string(),
        )],
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-sub-settings", None),
            CallbackAction::Settings(SettingsAction::SubSelect).to_string(),
        )],
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-notif-settings", None),
            CallbackAction::Settings(SettingsAction::NotifSelect).to_string(),
        )],
    ]);
    bot.send_message(chat_id, text)
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub async fn send_main_settings_edit(bot: &Bot, msg: &Message, lang: &str) -> ResponseResult<()> {
    let text = locales::get_text(lang, "settings-title", None);
    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-lang", None),
            CallbackAction::Settings(SettingsAction::LangSelect).to_string(),
        )],
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-sub-settings", None),
            CallbackAction::Settings(SettingsAction::SubSelect).to_string(),
        )],
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-notif-settings", None),
            CallbackAction::Settings(SettingsAction::NotifSelect).to_string(),
        )],
    ]);
    bot.edit_message_text(msg.chat.id, msg.id, text)
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub async fn send_sub_settings(
    bot: &Bot,
    msg: &Message,
    db: &Database,
    telegram_id: i64,
    lang: &str,
) -> ResponseResult<()> {
    let settings = match db.get_or_create_user(telegram_id, "en").await {
        Ok(s) => {
            tracing::debug!(
                "[UI] Fetched settings for {}: enabled={}",
                telegram_id,
                s.not_on_online_enabled
            );
            s
        }
        Err(e) => {
            tracing::error!("Failed to get or create user {}: {}", telegram_id, e);
            bot.edit_message_text(
                msg.chat.id,
                msg.id,
                locales::get_text(lang, "cmd-error", None),
            )
            .await?;
            return Ok(());
        }
    };
    let current_notif = NotificationSetting::from(settings.notification_settings.as_str());

    let check_icon = locales::get_text(lang, "icon-check-simple", None);
    let mk = |ns: NotificationSetting| {
        if ns == current_notif {
            check_icon.clone()
        } else {
            "".to_string()
        }
    };

    let btn_all = locales::get_text(
        lang,
        "btn-sub-all",
        args!(marker = mk(NotificationSetting::All)).as_ref(),
    );
    let btn_join = locales::get_text(
        lang,
        "btn-sub-join",
        args!(marker = mk(NotificationSetting::LeaveOff)).as_ref(),
    );
    let btn_leave = locales::get_text(
        lang,
        "btn-sub-leave",
        args!(marker = mk(NotificationSetting::JoinOff)).as_ref(),
    );
    let btn_none = locales::get_text(
        lang,
        "btn-sub-none",
        args!(marker = mk(NotificationSetting::None)).as_ref(),
    );

    let mk_act = |val: &str| {
        CallbackAction::Settings(SettingsAction::SubSet {
            setting: val.to_string(),
        })
        .to_string()
    };

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(btn_all, mk_act("all"))],
        vec![InlineKeyboardButton::callback(
            btn_join,
            mk_act("leave_off"),
        )],
        vec![InlineKeyboardButton::callback(
            btn_leave,
            mk_act("join_off"),
        )],
        vec![InlineKeyboardButton::callback(btn_none, mk_act("none"))],
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-back-settings", None),
            CallbackAction::Settings(SettingsAction::Main).to_string(),
        )],
    ]);

    bot.edit_message_text(
        msg.chat.id,
        msg.id,
        locales::get_text(lang, "btn-sub-settings", None),
    )
    .reply_markup(keyboard)
    .parse_mode(ParseMode::Html)
    .await?;
    Ok(())
}

pub async fn send_notif_settings(
    bot: &Bot,
    msg: &Message,
    db: &Database,
    telegram_id: i64,
    lang: &str,
) -> ResponseResult<()> {
    let settings = match db.get_or_create_user(telegram_id, "en").await {
        Ok(s) => {
            tracing::debug!(
                "[UI] Fetched settings for {}: enabled={}",
                telegram_id,
                s.not_on_online_enabled
            );
            s
        }
        Err(e) => {
            tracing::error!("Failed to get or create user {}: {}", telegram_id, e);
            bot.edit_message_text(
                msg.chat.id,
                msg.id,
                locales::get_text(lang, "cmd-error", None),
            )
            .await?;
            return Ok(());
        }
    };
    let status_text = if settings.not_on_online_enabled {
        locales::get_text(lang, "status-enabled", None)
    } else {
        locales::get_text(lang, "status-disabled", None)
    };
    let noon_text = locales::get_text(lang, "btn-noon", args!(status = status_text).as_ref());

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            noon_text,
            CallbackAction::Settings(SettingsAction::NoonToggle).to_string(),
        )],
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-mute-manage", None),
            CallbackAction::Settings(SettingsAction::MuteManage).to_string(),
        )],
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-back-settings", None),
            CallbackAction::Settings(SettingsAction::Main).to_string(),
        )],
    ]);

    bot.edit_message_text(
        msg.chat.id,
        msg.id,
        locales::get_text(lang, "notif-settings-title", None),
    )
    .reply_markup(keyboard)
    .parse_mode(ParseMode::Html)
    .await?;
    Ok(())
}

pub async fn send_mute_menu(
    bot: &Bot,
    msg: &Message,
    lang: &str,
    current_mode: &str,
) -> ResponseResult<()> {
    let mode_desc_key = if current_mode == "blacklist" {
        "mute-mode-blacklist"
    } else {
        "mute-mode-whitelist"
    };
    let mode_desc = locales::get_text(lang, mode_desc_key, None);
    let args = args!(mode_desc = mode_desc);
    let text = locales::get_text(lang, "mute-title", args.as_ref());

    let icon_checked = locales::get_text(lang, "icon-checked", None);
    let icon_unchecked = locales::get_text(lang, "icon-unchecked", None);

    let bl_marker = if current_mode == "blacklist" {
        &icon_checked
    } else {
        &icon_unchecked
    };
    let wl_marker = if current_mode == "whitelist" {
        &icon_checked
    } else {
        &icon_unchecked
    };

    let btn_bl_text = locales::get_text(
        lang,
        "btn-mode-blacklist",
        args!(marker = bl_marker).as_ref(),
    );
    let btn_wl_text = locales::get_text(
        lang,
        "btn-mode-whitelist",
        args!(marker = wl_marker).as_ref(),
    );

    let current_mode_display = if current_mode == "blacklist" {
        locales::get_text(lang, "mode-blacklist", None)
    } else {
        locales::get_text(lang, "mode-whitelist", None)
    };

    let btn_manage_text = locales::get_text(
        lang,
        "btn-manage-list",
        args!(mode = current_mode_display).as_ref(),
    );

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback(
                btn_bl_text,
                CallbackAction::Mute(MuteAction::ModeSet {
                    mode: "blacklist".to_string(),
                })
                .to_string(),
            ),
            InlineKeyboardButton::callback(
                btn_wl_text,
                CallbackAction::Mute(MuteAction::ModeSet {
                    mode: "whitelist".to_string(),
                })
                .to_string(),
            ),
        ],
        vec![InlineKeyboardButton::callback(
            btn_manage_text,
            CallbackAction::Mute(MuteAction::List { page: 0 }).to_string(),
        )],
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-mute-server-list", None),
            CallbackAction::Mute(MuteAction::ServerList { page: 0 }).to_string(),
        )],
        vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-back-notif", None),
            CallbackAction::Settings(SettingsAction::NotifSelect).to_string(),
        )],
    ]);

    bot.edit_message_text(msg.chat.id, msg.id, text)
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn render_mute_list(
    bot: &Bot,
    msg: &Message,
    db: &Database,
    telegram_id: i64,
    lang: &str,
    accounts: &[UserAccount],
    page: usize,
    title_key: &str,
    guest_username: Option<&str>,
) -> ResponseResult<()> {
    let muted_users: Vec<String> = db
        .get_muted_users_list(telegram_id)
        .await
        .unwrap_or_default();
    let muted_set: std::collections::HashSet<_> = muted_users.into_iter().collect();

    let keyboard = create_user_list_keyboard(
        accounts,
        page,
        |acc| {
            let is_muted = muted_set.contains(&acc.username);
            let icon_key = if is_muted {
                "item-status-muted"
            } else {
                "item-status-unmuted"
            };

            let display_name = if Some(acc.username.as_str()) == guest_username {
                locales::get_text(lang, "display-guest-account", None)
            } else {
                acc.username.clone()
            };

            let args = args!(name = display_name);
            let display_text = locales::get_text(lang, icon_key, args.as_ref());
            (
                display_text,
                CallbackAction::Mute(MuteAction::ServerToggle {
                    username: acc.username.clone(),
                    page,
                }),
            )
        },
        |p| CallbackAction::Mute(MuteAction::ServerList { page: p }),
        Some((
            locales::get_text(lang, "btn-back-mute", None),
            CallbackAction::Settings(SettingsAction::MuteManage),
        )),
        lang,
    );

    let text = locales::get_text(lang, title_key, None);
    bot.edit_message_text(msg.chat.id, msg.id, text)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn render_mute_list_strings(
    bot: &Bot,
    msg: &Message,
    _telegram_id: i64,
    lang: &str,
    items: &[String],
    page: usize,
    _is_server_list: bool,
    title_key: &str,
    guest_username: Option<&str>,
) -> ResponseResult<()> {
    if items.is_empty() {
        let text = locales::get_text(lang, "list-mute-empty", None);
        let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
            locales::get_text(lang, "btn-back-mute", None),
            CallbackAction::Settings(SettingsAction::MuteManage).to_string(),
        )]]);
        bot.edit_message_text(msg.chat.id, msg.id, text)
            .reply_markup(keyboard)
            .await?;
        return Ok(());
    }

    let mut sorted_items = items.to_vec();
    sorted_items.sort_by_key(|a| a.to_lowercase());

    let keyboard = create_user_list_keyboard(
        &sorted_items,
        page,
        |username| {
            let display_name = if Some(username.as_str()) == guest_username {
                locales::get_text(lang, "display-guest-account", None)
            } else {
                username.clone()
            };

            let args = args!(name = display_name);
            let display_text = locales::get_text(lang, "item-status-muted", args.as_ref());
            (
                display_text,
                CallbackAction::Mute(MuteAction::Toggle {
                    username: username.clone(),
                    page,
                }),
            )
        },
        |p| CallbackAction::Mute(MuteAction::List { page: p }),
        Some((
            locales::get_text(lang, "btn-back-mute", None),
            CallbackAction::Settings(SettingsAction::MuteManage),
        )),
        lang,
    );

    let user_name = format!("{}", _telegram_id);
    let args = args!(name = user_name);
    let text = locales::get_text(lang, title_key, args.as_ref());

    bot.edit_message_text(msg.chat.id, msg.id, text)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}
```

---

### `src/tg_bot/state.rs`

```rust
use crate::config::Config;
use crate::db::Database;
use crate::types::{LiteUser, TtCommand};
use dashmap::DashMap;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use teamtalk::types::UserAccount;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub online_users: Arc<DashMap<i32, LiteUser>>,
    pub user_accounts: Arc<DashMap<String, UserAccount>>,
    pub tx_tt: Sender<TtCommand>,
    pub config: Arc<Config>,
}
```

---

### `src/tg_bot/utils.rs`

```rust
use crate::db::Database;
use crate::locales;
use teloxide::prelude::*;
use teloxide::types::ParseMode;

pub async fn ensure_subscribed(bot: &Bot, msg: &Message, db: &Database, lang: &str) -> bool {
    if let Ok(true) = db.is_subscribed(msg.chat.id.0).await {
        true
    } else {
        bot.send_message(
            msg.chat.id,
            locales::get_text(lang, "cmd-not-subscribed", None),
        )
        .parse_mode(ParseMode::Html)
        .await
        .ok();
        false
    }
}
```

---

### `src/tt_worker/commands.rs`

```rust
use crate::args;
use crate::locales;
use crate::tt_worker::WorkerContext;
use crate::types::TtCommand;
use teamtalk::Client;
use teamtalk::types::TextMessage;
use uuid::Uuid;

pub(super) fn handle_text_message(client: &Client, ctx: &WorkerContext, msg: TextMessage) {
    if msg.from_id == client.my_id() {
        return;
    }

    let real_name_from_client = client.get_server_properties().map(|p| p.name);
    let tx_tt_cmd = ctx.tx_tt_cmd.clone();

    let db = ctx.db.clone();
    let online_users = ctx.online_users.clone();

    let default_lang = ctx.config.general.default_lang.clone();
    let admin_username = ctx.config.general.admin_username.clone();
    let tt_config = ctx.config.teamtalk.clone();
    let deeplink_ttl = ctx.config.operational_parameters.deeplink_ttl_seconds;

    let bot_username = ctx.bot_username.clone();
    let tx_bridge = ctx.tx_bridge.clone();

    ctx.rt.spawn(async move {
        if msg.msg_type == teamtalk::client::ffi::TextMsgType::MSGTYPE_USER {
            let content = msg.text.trim();
            let from_uid = msg.from_id.0;

            let (nick, username): (String, String) = if let Some(u) = online_users.get(&from_uid) {
                (u.nickname.clone(), u.username.clone())
            } else {
                ("Unknown".to_string(), "".to_string())
            };

            tracing::info!("💬 [TT_WORKER] Msg from {}: {}", nick, content);

            let reply_lang = if !username.is_empty() {
                db.get_user_lang_by_tt_user(&username)
                    .await
                    .unwrap_or_else(|| default_lang.clone())
            } else {
                default_lang.clone()
            };

            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.is_empty() {
                return;
            }
            let cmd = parts[0].to_lowercase();

            let send_reply = |text: String| {
                let _ = tx_tt_cmd.send(TtCommand::ReplyToUser {
                    user_id: from_uid,
                    text,
                });
            };

            if cmd == "/sub" {
                if let Some(bot_user) = &bot_username {
                    let is_guest = username.is_empty()
                        || tt_config
                            .guest_username
                            .as_ref()
                            .map(|g| g == &username)
                            .unwrap_or(false);

                    let payload = if is_guest {
                        None
                    } else {
                        Some(username.as_str())
                    };

                    let token = Uuid::now_v7().to_string().replace('-', "");
                    let res = db
                        .create_deeplink(&token, "subscribe", payload, deeplink_ttl)
                        .await;

                    match res {
                        Ok(_) => {
                            let link = format!("https://t.me/{}?start={}", bot_user, token);
                            let text = locales::get_text(
                                &reply_lang,
                                "tt-sub-link",
                                args!(link = link).as_ref(),
                            );
                            send_reply(text);
                        }
                        Err(_) => {
                            let text = locales::get_text(&reply_lang, "tt-error-generic", None);
                            send_reply(text);
                        }
                    }
                } else {
                    send_reply(
                        "Telegram integration is currently disabled (Event Token missing)."
                            .to_string(),
                    );
                }
            } else if cmd == "/unsub" {
                if let Some(bot_user) = &bot_username {
                    let token = Uuid::now_v7().to_string().replace('-', "");
                    let res = db
                        .create_deeplink(&token, "unsubscribe", None, deeplink_ttl)
                        .await;

                    match res {
                        Ok(_) => {
                            let link = format!("https://t.me/{}?start={}", bot_user, token);
                            let text = locales::get_text(
                                &reply_lang,
                                "tt-unsub-link",
                                args!(link = link).as_ref(),
                            );
                            send_reply(text);
                        }
                        Err(_) => {
                            let text = locales::get_text(&reply_lang, "tt-error-generic", None);
                            send_reply(text);
                        }
                    }
                } else {
                    send_reply(
                        "Telegram integration is currently disabled (Event Token missing)."
                            .to_string(),
                    );
                }
            } else if cmd == "/help" {
                let is_main_admin = admin_username
                    .as_ref()
                    .map(|u| u == &username)
                    .unwrap_or(false);
                let mut help_msg = locales::get_text(&reply_lang, "help-text", None);
                if is_main_admin {
                    let header = locales::get_text(&reply_lang, "tt-admin-help-header", None);
                    let cmds = locales::get_text(&reply_lang, "tt-admin-help-cmds", None);
                    help_msg.push_str(&header);
                    help_msg.push_str(&cmds);
                }
                send_reply(help_msg);
            } else if cmd == "/add_admin" {
                let is_main_admin = admin_username
                    .as_ref()
                    .map(|u| u == &username)
                    .unwrap_or(false);
                if !is_main_admin {
                    let text = locales::get_text(&reply_lang, "cmd-unauth", None);
                    send_reply(text);
                    return;
                }
                if parts.len() < 2 {
                    let text = locales::get_text(&reply_lang, "tt-admin-no-ids", None);
                    send_reply(text);
                    return;
                }
                let mut added_count = 0;
                let mut failed_count = 0;
                for id_str in &parts[1..] {
                    if let Ok(tg_id) = id_str.parse::<i64>() {
                        let success = db.add_admin(tg_id).await.unwrap_or(false);
                        if success {
                            added_count += 1;
                        }
                    } else {
                        failed_count += 1;
                    }
                }
                if added_count > 0 {
                    let args = args!(count = added_count);
                    let text = locales::get_text(&reply_lang, "tt-admin-added", args.as_ref());
                    send_reply(text);
                }
                if failed_count > 0 {
                    let args = args!(count = failed_count);
                    let text = locales::get_text(&reply_lang, "tt-admin-add-fail", args.as_ref());
                    send_reply(text);
                }
            } else if cmd == "/remove_admin" {
                let is_main_admin = admin_username
                    .as_ref()
                    .map(|u| u == &username)
                    .unwrap_or(false);
                if !is_main_admin {
                    let text = locales::get_text(&reply_lang, "cmd-unauth", None);
                    send_reply(text);
                    return;
                }
                if parts.len() < 2 {
                    let text = locales::get_text(&reply_lang, "tt-admin-no-ids", None);
                    send_reply(text);
                    return;
                }
                let mut removed_count = 0;
                let mut failed_count = 0;
                for id_str in &parts[1..] {
                    if let Ok(tg_id) = id_str.parse::<i64>() {
                        let success = db.remove_admin(tg_id).await.unwrap_or(false);
                        if success {
                            removed_count += 1;
                        } else {
                            failed_count += 1;
                        }
                    } else {
                        failed_count += 1;
                    }
                }
                if removed_count > 0 {
                    let args = args!(count = removed_count);
                    let text = locales::get_text(&reply_lang, "tt-admin-removed", args.as_ref());
                    send_reply(text);
                }
                if failed_count > 0 {
                    let args = args!(count = failed_count);
                    let text =
                        locales::get_text(&reply_lang, "tt-admin-remove-fail", args.as_ref());
                    send_reply(text);
                }
            } else {
                let server_name = tt_config
                    .server_name
                    .as_deref()
                    .filter(|s| !s.is_empty())
                    .or(real_name_from_client.as_deref().filter(|s| !s.is_empty()))
                    .unwrap_or(&tt_config.host_name)
                    .to_string();

                let _ = tx_bridge
                    .send(crate::types::BridgeEvent::ToAdmin {
                        user_id: from_uid,
                        nick,
                        tt_username: username,
                        msg_content: content.to_string(),
                        server_name,
                    })
                    .await;
            }
        }
    });
}
```

---

### `src/tt_worker/events.rs`

```rust
use crate::tt_worker::WorkerContext;
use crate::tt_worker::commands;
use crate::types::{BridgeEvent, LiteUser, NotificationType};
use std::time::{Duration, Instant};
use teamtalk::client::ReconnectHandler;
use teamtalk::{Client, Event, Message};

pub(super) fn handle_sdk_event(
    client: &Client,
    ctx: &WorkerContext,
    event: Event,
    msg: Message,
    is_connected: &mut bool,
    reconnect_handler: &mut ReconnectHandler,
    ready_time: &mut Option<Instant>,
) {
    tracing::trace!("📥 [TT_WORKER] Event received: {:?}", event);
    let tt_config = &ctx.config.teamtalk;

    match event {
        Event::ConnectSuccess => {
            *is_connected = true;
            reconnect_handler.mark_connected();
            client.login(
                &tt_config.nick_name,
                &tt_config.user_name,
                &tt_config.password,
                &tt_config.client_name,
            );
        }
        e if e.is_reconnect_needed_with(&[Event::MySelfKicked]) => {
            *is_connected = false;
            reconnect_handler.mark_disconnected();
            ctx.online_users.clear();
            *ready_time = None;
            tracing::warn!(
                "❌ [TT_WORKER] Disconnection event ({:?}). Reconnect pending...",
                e
            );
        }
        Event::MySelfLoggedIn => {
            client.set_status_message(&tt_config.status_text);
            let chan_id = client.get_channel_id_from_path(&tt_config.channel);
            if chan_id.0 > 0 {
                client.join_channel(chan_id, tt_config.channel_password.as_deref().unwrap_or(""));
            }
            *ready_time = Some(std::time::Instant::now());
            ctx.user_accounts.clear();
            client.list_user_accounts(0, 1000);
        }

        Event::UserAccount => {
            if let Some(account) = msg.account()
                && !account.username.is_empty()
            {
                ctx.user_accounts.insert(account.username.clone(), account);
            }
        }
        Event::UserAccountCreated | Event::UserAccountRemoved => {
            ctx.user_accounts.clear();
            client.list_user_accounts(0, 1000);
        }

        Event::UserUpdate => {
            if let Some(user) = msg.user()
                && let Some(mut existing_lite_user) = ctx.online_users.get_mut(&user.id.0)
            {
                if existing_lite_user.username != user.username {
                    if !existing_lite_user.username.is_empty() {
                        ctx.online_users_by_username
                            .remove(&existing_lite_user.username);
                    }
                    if !user.username.is_empty() {
                        ctx.online_users_by_username
                            .insert(user.username.clone(), user.id.0);
                    }
                    existing_lite_user.username = user.username.clone();
                }

                if existing_lite_user.nickname != user.nickname {
                    tracing::info!(
                        "🔄 [TT_WORKER] Nickname changed for {}: {} -> {}",
                        user.username,
                        existing_lite_user.nickname,
                        user.nickname
                    );
                    existing_lite_user.nickname = user.nickname.clone();
                }
            }
        }
        Event::UserLoggedIn => {
            if let Some(user) = msg.user()
                && user.id != client.my_id()
            {
                let nickname = user.nickname.clone();

                let channel_name = client
                    .get_channel(user.channel_id)
                    .map(|c| c.name)
                    .unwrap_or_else(|| "Unknown".to_string());

                let lite_user = LiteUser {
                    id: user.id.0,
                    nickname: nickname.clone(),
                    username: user.username.clone(),
                    channel_name,
                };
                if !lite_user.username.is_empty() {
                    ctx.online_users_by_username
                        .insert(lite_user.username.clone(), lite_user.id);
                }
                ctx.online_users.insert(user.id.0, lite_user.clone());

                let is_ready = ready_time
                    .map(|t| t.elapsed() >= Duration::from_secs(2))
                    .unwrap_or(false);

                if is_ready && !tt_config.global_ignore_usernames.contains(&user.username) {
                    let real_name = client.get_server_properties().map(|p| p.name);
                    let server_name = tt_config
                        .server_name
                        .as_deref()
                        .filter(|&s| !s.is_empty())
                        .or(real_name.as_deref().filter(|&s| !s.is_empty()))
                        .unwrap_or(&tt_config.host_name)
                        .to_string();

                    let _ = ctx.tx_bridge.blocking_send(BridgeEvent::Broadcast {
                        event_type: NotificationType::Join,
                        nickname,
                        server_name,
                        related_tt_username: user.username.clone(),
                    });
                }
            }
        }
        Event::UserJoined => {
            if let Some(user) = msg.user()
                && user.id != client.my_id()
            {
                let nickname = user.nickname.clone();
                let channel_name = client
                    .get_channel(user.channel_id)
                    .map(|c| c.name)
                    .unwrap_or_else(|| "Unknown".to_string());

                let lite_user = LiteUser {
                    id: user.id.0,
                    nickname,
                    username: user.username.clone(),
                    channel_name,
                };
                if !lite_user.username.is_empty() {
                    ctx.online_users_by_username
                        .insert(lite_user.username.clone(), lite_user.id);
                }
                ctx.online_users.insert(user.id.0, lite_user);
            }
        }

        Event::UserLoggedOut => {
            if let Some(user) = msg.user()
                && let Some((_, u)) = ctx.online_users.remove(&user.id.0)
            {
                if !u.username.is_empty() {
                    ctx.online_users_by_username.remove(&u.username);
                }
                if user.id != client.my_id() {
                    let is_ready = ready_time
                        .map(|t| t.elapsed() >= Duration::from_secs(2))
                        .unwrap_or(false);
                    if is_ready && !tt_config.global_ignore_usernames.contains(&u.username) {
                        let real_name = client.get_server_properties().map(|p| p.name);
                        let server_name = tt_config
                            .server_name
                            .as_deref()
                            .filter(|&s| !s.is_empty())
                            .or(real_name.as_deref().filter(|&s| !s.is_empty()))
                            .unwrap_or(&tt_config.host_name)
                            .to_string();

                        let _ = ctx.tx_bridge.blocking_send(BridgeEvent::Broadcast {
                            event_type: NotificationType::Leave,
                            nickname: u.nickname.clone(),
                            server_name,
                            related_tt_username: u.username.clone(),
                        });
                    }
                }
            }
        }
        Event::UserLeft => {
            if let Some(user) = msg.user() {
                let chan = client.get_channel(user.channel_id);
                let channel_name = chan
                    .as_ref()
                    .map(|c| c.name.clone())
                    .unwrap_or_else(|| "Unknown".to_string());

                if let Some(mut u) = ctx.online_users.get_mut(&user.id.0) {
                    u.channel_name = channel_name;
                }
            }
        }

        Event::TextMessage => {
            if let Some(txt_msg) = msg.text() {
                commands::handle_text_message(client, ctx, txt_msg);
            }
        }

        _ => {}
    }
}
```

---

### `src/tt_worker/mod.rs`

```rust
pub mod commands;
pub mod events;
pub mod reports;

use crate::config::Config;
use crate::db::Database;
use crate::types::{BridgeEvent, LiteUser, TtCommand};
use dashmap::DashMap;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use teamtalk::Client;
use teamtalk::client::{ConnectParams, ReconnectConfig, ReconnectHandler};
use teamtalk::types::{UserAccount, UserId};

pub struct WorkerContext {
    pub config: Arc<Config>,
    pub online_users: Arc<DashMap<i32, LiteUser>>,
    pub online_users_by_username: Arc<DashMap<String, i32>>,
    pub user_accounts: Arc<DashMap<String, UserAccount>>,
    pub tx_bridge: tokio::sync::mpsc::Sender<BridgeEvent>,
    pub tx_tt_cmd: Sender<TtCommand>,
    pub db: Database,
    pub rt: tokio::runtime::Handle,
    pub bot_username: Option<String>,
}

#[allow(clippy::too_many_arguments)]
pub fn run_teamtalk_thread(
    config: Arc<Config>,
    online_users: Arc<DashMap<i32, LiteUser>>,
    online_users_by_username: Arc<DashMap<String, i32>>,
    user_accounts: Arc<DashMap<String, UserAccount>>,
    tx_bridge: tokio::sync::mpsc::Sender<BridgeEvent>,
    rx_cmd: Receiver<TtCommand>,
    tx_cmd_clone: Sender<TtCommand>,
    db: Database,
    rt: tokio::runtime::Handle,
    bot_username: Option<String>,
    tx_init: std::sync::mpsc::Sender<Result<(), String>>,
) {
    let tt_config = &config.teamtalk;
    let _reconnect_interval = config.operational_parameters.tt_reconnect_retry_seconds;

    let ctx = WorkerContext {
        config: config.clone(),
        online_users: online_users.clone(),
        online_users_by_username,
        user_accounts,
        tx_bridge,
        tx_tt_cmd: tx_cmd_clone,
        db,
        rt,
        bot_username,
    };

    let client = match Client::new() {
        Ok(c) => {
            let _ = tx_init.send(Ok(()));
            c
        }
        Err(e) => {
            let err_msg = format!("Failed to initialize TeamTalk SDK: {}", e);
            tracing::error!("{}", err_msg);
            let _ = tx_init.send(Err(err_msg));
            return;
        }
    };
    let mut ready_time: Option<std::time::Instant> = None;
    let mut is_connected = false;

    let mut reconnect_handler = ReconnectHandler::new(ReconnectConfig {
        min_delay: Duration::from_millis(200),
        max_delay: Duration::from_secs(60),
        max_attempts: u32::MAX,
        stability_threshold: Duration::from_secs(10),
    });

    let connect_params = ConnectParams {
        host: &tt_config.host_name,
        tcp: tt_config.port as i32,
        udp: tt_config.port as i32,
        encrypted: tt_config.encrypted,
    };

    tracing::info!(
        "🔌 [TT_WORKER] Connecting to {}:{} (Encrypted: {})...",
        tt_config.host_name,
        tt_config.port,
        tt_config.encrypted
    );

    let _ = client.connect(
        connect_params.host,
        connect_params.tcp,
        connect_params.udp,
        connect_params.encrypted,
    );

    loop {
        if !is_connected {
            client.handle_reconnect(&connect_params, &mut reconnect_handler);
        }

        while let Ok(cmd) = rx_cmd.try_recv() {
            match cmd {
                TtCommand::ReplyToUser { user_id, text } => {
                    client.send_to_user(UserId(user_id), &text);
                }
                TtCommand::KickUser { user_id } => {
                    client.kick_user(UserId(user_id), teamtalk::types::ChannelId(0));
                }
                TtCommand::BanUser { user_id } => {
                    client.ban_user(UserId(user_id), client.my_channel_id());
                }
                TtCommand::Who { chat_id, lang } => {
                    reports::handle_who_command(&client, &ctx, chat_id, lang);
                }
                TtCommand::LoadAccounts => {
                    tracing::info!("📥 [TT_WORKER] Requesting full user accounts list...");
                    client.list_user_accounts(0, 1000);
                }
            }
        }

        while let Some((event, msg)) = client.poll(100) {
            events::handle_sdk_event(
                &client,
                &ctx,
                event,
                msg,
                &mut is_connected,
                &mut reconnect_handler,
                &mut ready_time,
            );
        }
    }
}
```

---

### `src/tt_worker/reports.rs`

```rust
use crate::args;
use crate::locales;
use crate::tt_worker::WorkerContext;
use crate::types::BridgeEvent;
use std::fmt::Write;
use teamtalk::Client;

pub(super) fn handle_who_command(client: &Client, ctx: &WorkerContext, chat_id: i64, lang: String) {
    let tt_config = &ctx.config.teamtalk;

    let real_name = client.get_server_properties().map(|p| p.name);
    let server_name = tt_config
        .server_name
        .as_deref()
        .filter(|&s| !s.is_empty())
        .or(real_name.as_deref().filter(|&s| !s.is_empty()))
        .unwrap_or(&tt_config.host_name)
        .to_string();

    let users = client.get_server_users();
    let mut channels_data: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    let mut unauth_users: Vec<String> = Vec::new();

    for user in &users {
        let nickname = user.nickname.clone();

        if user.channel_id.0 <= 0 {
            unauth_users.push(nickname);
            continue;
        }

        let chan = client.get_channel(user.channel_id);
        let chan_name = chan.as_ref().map(|c| c.name.clone()).unwrap_or_default();

        let chan_display = if chan_name.is_empty() && user.channel_id.0 == 1 {
            "ROOT_MARKER".to_string()
        } else {
            chan_name
        };
        channels_data
            .entry(chan_display)
            .or_default()
            .push(nickname);
    }

    let total = users.len();

    let header_args = args!(server = server_name, count = total);
    let header = locales::get_text(&lang, "tt-report-header", header_args.as_ref());

    let mut report = String::with_capacity(1024);
    writeln!(report, "{}\n", header).unwrap();

    for (chan_name, mut nicks) in channels_data {
        nicks.sort_by_key(|a| a.to_lowercase());

        let user_list = nicks.join(", ");

        let location = if chan_name == "ROOT_MARKER" {
            locales::get_text(&lang, "tt-report-root", None)
        } else {
            chan_name
        };

        let row_args = args!(users = user_list, channel = location);
        let row_text = locales::get_text(&lang, "tt-report-row", row_args.as_ref());

        writeln!(report, "{}", row_text).unwrap();
    }
    if !unauth_users.is_empty() {
        let unauth_label = locales::get_text(&lang, "tt-report-unauth", None);
        writeln!(report, "{} {}", unauth_users.join(", "), unauth_label).unwrap();
    }

    let _ = ctx.tx_bridge.blocking_send(BridgeEvent::WhoReport {
        chat_id,
        text: report.trim_end().to_string(),
    });
}
```

---

### `src/types.rs`

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationSetting {
    All,
    JoinOff,
    LeaveOff,
    None,
}

impl fmt::Display for NotificationSetting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotificationSetting::All => write!(f, "all"),
            NotificationSetting::JoinOff => write!(f, "join_off"),
            NotificationSetting::LeaveOff => write!(f, "leave_off"),
            NotificationSetting::None => write!(f, "none"),
        }
    }
}

impl From<&str> for NotificationSetting {
    fn from(s: &str) -> Self {
        match s {
            "join_off" => NotificationSetting::JoinOff,
            "leave_off" => NotificationSetting::LeaveOff,
            "none" => NotificationSetting::None,
            _ => NotificationSetting::All,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MuteListMode {
    Blacklist,
    Whitelist,
}

impl fmt::Display for MuteListMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MuteListMode::Blacklist => write!(f, "blacklist"),
            MuteListMode::Whitelist => write!(f, "whitelist"),
        }
    }
}

impl From<&str> for MuteListMode {
    fn from(s: &str) -> Self {
        match s {
            "whitelist" => MuteListMode::Whitelist,
            _ => MuteListMode::Blacklist,
        }
    }
}

#[derive(Debug)]
pub enum BridgeEvent {
    Broadcast {
        event_type: NotificationType,
        nickname: String,
        server_name: String,
        related_tt_username: String,
    },
    ToAdmin {
        user_id: i32,
        nick: String,
        tt_username: String,
        msg_content: String,
        server_name: String,
    },
    WhoReport {
        chat_id: i64,
        text: String,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NotificationType {
    Join,
    Leave,
}

#[derive(Debug)]
pub enum TtCommand {
    ReplyToUser { user_id: i32, text: String },
    KickUser { user_id: i32 },
    BanUser { user_id: i32 },
    Who { chat_id: i64, lang: String },
    LoadAccounts,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteUser {
    pub id: i32,
    pub nickname: String,
    pub username: String,
    pub channel_name: String,
}
```

---

_Analysis generated by dir2txt._
