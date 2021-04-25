# Кодировщик сообщений
Шифрование построено на алгоритме `ChaCha20`, обмен ключом кодирования - `curve25519`
## Сборка и запуск
- в папке репозитория `docker build -t vezdekod-enc:latest .` - собираем конт
- `docker run --rm -it vezdekod-enc:latest src/target/release/encoder` - запускаем кодировщик
- `docker run --rm -it vezdekod-enc:latest src/target/release/decoder` - запускаем декодировщик
- остановка приложения - Ctrl+D

## Кодировщик
- при запуске он захочет доступы до SMTP сервера почты. Если при вопросе SMTP сервера нажать Enter, то E-mail отправщик не будет сконфигурирован, но приложение продолжит работать
- На сообщение `Enter message to encrypt: ` он хочет сообщение для кодировки. Если ввести в него `\\`, то откроется режим редактирования сообщения в GNU Nano. Сохранение - Ctrl+O, выход - Ctrl+X
- Далее он спросит об отправителе сообщения. 
  - `print` - вывести сообщение в терминал
  - `email` - заслать по E-mail'у. Также спросит ящик получателя сообщения
- после этого он отправит сообщение и выведет ключ дешифрования

### Пример работы

![пример](/docs/encoder.png)

## Декодировщик
- на сообщение `Enter message to decrypt:` он хочет зашифрованное сообщение
- на сообщение `Enter decode key:` он хочет ключ дешифровки
- потом он выведет результат декодирования

### Пример работы
Сообщение - `OBgxgeiQM99iyWSLcjwinuybLD4gmtL+rx8tCf8WsTC79JOl7NxuFE8mZKOFq54WqtA0HtslonGiZ58rGuaa+Ps0mvvdPVt1Lu4=`
Ключ - `OXsw2usryyarp6ccy1bxy4Th7wNR2GAbOJWdH2k/YAk=`

![пример](/docs/decoder.png)