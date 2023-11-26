# Folder PWD

[![CI](https://github.com/Ttanesque/folder_pwd/actions/workflows/rust.yml/badge.svg)](https://github.com/Ttanesque/folder_pwd/actions/workflows/rust.yml)

> Folder PWD va parcourir et afficher tout les répertoires présent dans un répertoires donnés (en paramètre ou courant)

## Pourquoi ?

Ripgrep n'a pas d'options pour afficher que les répertoires et était un bon projet rapide. Une alternative maintenue et complète est [fd]([url](https://github.com/sharkdp/fd))

## Dépendences

Pour la mise en place de la ligne de command [clap](https://lib.rs/crates/clap) est
utilisé pour parser les Arguments et fournir une commande d'aide facile.
Pour le parcours du systèmes de fichier [ignore](https://lib.rs/crates/ignore) de
[RipGrep](https://github.com/BurntSushi/ripgrep/) est utilisé, il permet d'avoir le 
support des fichiers caché et des .gitignore.
Pour logger en mode DEBUG 2 crate sont utilisé [log](https://lib.rs/crates/log) une
api de log et [env_logger](https://lib.rs/crates/env_logger) un logger simple et facilement configurable.
