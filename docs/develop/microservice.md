# Разработка микросервисов

## Обработка ошибок

### Ошибки при выполнении пользовательских запросов

- не раскрываем суть ошибок для клиентов, только статус, чтобы исключить возможность анализа уязвимых мест;
- детальную информацию об ошибки логируем в tracing::error

### Ошибки при выполнении внутренних запросов

- передаем детальную информации в grpc статусе
- детальную информацию об ошибки логируем в tracing::error (для настройки алертов и анализа работоспособности сервиса)