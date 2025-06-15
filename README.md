# rust-interopt

Cria um bliblioteca compartilhada (DLL ou SO) que pode ser usada por Rust ou por outras linguagens de programação.

A interface de acesso da biblioteca é definida em C e há um aplicativo C++ que usa essa biblioteca.

O projeto C usa CMake e compila a biblioteca compartilhada como dependencia antes de compilar o aplicativo de exemplo.