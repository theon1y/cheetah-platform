# Relay Realtime Game Server

## Комната
- Совокупность клиентов и игровых объектов для организации сессионого взаимодействия (битвы)
- Обладает ограниченным временем жизни

## Клиент
- связь с удаленным приложением
- поддержка реконнекта (?)

## Группа доступа
- Игровые объекты и клиенты находятся в группах доступа. 
- Каждая группа доступа имеет идентификатор из диапазон 0-63.
- Клиент может изменять только те объекты, в группах которых он находится.
- Также изменения объекта рассылается всем клиентам, у которых с объектом есть общие группы.

## Игровой объект
- Асбтракция для объедения данных, команд и групп доступа.
- У объекта есть владелец - либо клиент либо root
- При удалении владельца - объект удаляется из комнаты 



