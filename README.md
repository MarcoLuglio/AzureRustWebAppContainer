# Exemplo mínimo de Rust + Web API + Contêiner Docker
Exemplo mínimo de uma imagem de um contêiner Docker rodando uma aplicação Rust de linha de comando respondendo a requisições http na porta 80.
Recomendado apenas para testar o deploy em um servidor de nuvem como Microsoft Azure, AWS ou Google cloud platform.
Contém um Dockerfile e uma aplicação simples que podem ser utilizados para estudo, prova de conceito ou como ponto de partida.

Uma aplicação em produção deveria provavelmente seguir uma das duas alternativas:
* Utilizar uma crate de um servidor http completo, como actix-web
* Rodar um servidor nginx que serve de proxy para a aplicação

Isso permitirá a aplicação lidar com timeouts, certificados, etc.