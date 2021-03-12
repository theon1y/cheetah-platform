Сервер для локальной разработки поставляется в Unity пакете **games.cheetah.unity.relay**.

=== "windows"
    ```shell
    games.cheetah.unity.relay/Tests/Server/relay.exe
    ```
=== "linux"
    ```shell
    games.cheetah.unity.relay/Tests/Server/linux_relay
    ```
=== "macos"
    ```shell
    games.cheetah.unity.relay/Tests/Server/macos_relay
    ```
Для запуска сервера необходимо задать конфигурацию [комнаты](/components/relay/configuration/room/).

### Выбор пользователя для соединения с сервером

Выбор осуществляется среди пользователей заданных в конфигурации комнаты.

Используется для замены работы с ММ сервером.

```bash
curl localhost:8080/select-user/1
```

Где 1 - это id комнаты.

#### Возможные ответы

- Пользователь выбран для входа, вход необходимо осуществить за 5 секунд, иначе пользователь будет снова свободен для
  выбора.

  ```yaml
  {
    "Ok":
      {
        "id": 5, // идентификатор пользователя
        "private_key": [ 1,5,6,... ] // приватный ключ пользователя
      }
  }
  ```

- Комната не найдена
    ```yaml
    { "Err": "RoomNotFound" }     
    ```

- Нет свободного пользователя для входа
    ```yaml
    { "Ok": null }
    ```


