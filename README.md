Find extensions
================

Эта утилита предназначена для рекурсивного нахождения расширений файлов.

Установка
---------------------
```
$ git clone https://gitlab.com/saruman9/find_extension.git
$ cargo install
```

Моментальный запуск
-------------------
```
$ cargo run -- PATH [PATH1] [PATH2] ...
```

Пример вывода
-------------
```
$ find_extension ~/Downloads ~/rust.ico ~/rust.png
```
```
gz
  /home/neo/Downloads/dbip-city-2015-11.csv.gz
ico
  /home/neo/rust.ico
iso
  /home/neo/Downloads/Image/archlinux-2016.03.01-dual.iso
  /home/neo/Downloads/Image/ubuntu-12.04.5-server-amd64.iso
png
  /home/neo/rust.png
pdf
  /home/neo/Downloads/deep.pdf
  /home/neo/Downloads/elkhound_cc04.pdf
  /home/neo/Downloads/15095B.pdf
  /home/neo/Downloads/1996-A Faster Earley Parser-fastEarley-1.pdf
  /home/neo/Downloads/understand_api.pdf
  /home/neo/Downloads/schmitz-scp10.pdf
  /home/neo/Downloads/CC-2009.pdf
  /home/neo/Downloads/main.pdf
  /home/neo/Downloads/Earley-Parsing.pdf
```
