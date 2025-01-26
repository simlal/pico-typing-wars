# Project: Pico Typing Wars

Projet de session pour le cours _IFT-769 - Systèmes Temps Réel_.

Avec la plateforme Raspberry Pi Pico, nous allons implémenter un jeu de rapidité de réponse (et possiblement de frappe)
avec des composants électroniques (bouton, LED, écran LCD, clavier USB).

Le projet est divisé en 2 parties:

1. **Partie 1**: Implémentation primaire d'un jeu de rapidité de temps de réaction avec bouton et LED
2. **Partie 2**: Implémentation d'un jeu de rapidité de frappe sur un clavier USB avec affichage sur écran LCD

## Table des matières

<!--toc:start-->

- [Project: Pico Typing Wars](#project-pico-typing-wars)
  - [Table des matières](#table-des-matières)
  - [Introduction :book:](#introduction-book)
    - [Vue d'ensemble](#vue-densemble)
      - [Partie 1](#partie-1)
      - [Partie 2](#partie-2)
    - [Le choix de Rust](#le-choix-de-rust)
  - [Écheancier et livraisons :calendar:](#écheancier-et-livraisons-calendar)
    - [Objectifs personnels](#objectifs-personnels)
  - [Environnement de développement :hammer_and_wrench:](#environnement-de-développement-hammerandwrench)
    - [Chaîne d'outils Rust standards](#chaîne-doutils-rust-standards)
    - [Ressources, librairies et outils pour le développement Rust sur systèmes embarqués](#ressources-librairies-et-outils-pour-le-développement-rust-sur-systèmes-embarqués)
    - [Débuggage avec `probe-rs` et système de build avec `cargo`](#débuggage-avec-probe-rs-et-système-de-build-avec-cargo)
      - [Debugging avec un 2e Raspberry Pi Pico](#debugging-avec-un-2e-raspberry-pi-pico)
      - [Système de buildj](#système-de-buildj)
      - [Projet `blinky`](#projet-blinky)
  - [Mise en place du matériel :rocket:](#mise-en-place-du-matériel-rocket)
    - [Matériel requis](#matériel-requis)
    - [Schéma de connexion et montage en mode démo](#schéma-de-connexion-et-montage-en-mode-démo)
  - [Pico Typing Wars :video_game:](#pico-typing-wars-videogame)
    - [Partie 1: Rapidité de réaction](#partie-1-rapidité-de-réaction)
    - [Partie 2: Rapidité de frappe et affichage](#partie-2-rapidité-de-frappe-et-affichage)
  - [Analyses et résultats :chart_with_upwards_trend:](#analyses-et-résultats-chartwithupwardstrend)
  - [Conclusion :checkered_flag:](#conclusion-checkeredflag)
  - [References :books:](#references-books)
  <!--toc:end-->

## Introduction :book:

### Vue d'ensemble

Le projet consiste à créer un jeu de rapidité de réponse et de frappe en utilisant la plateforme Raspberry Pi Pico. Le jeu est divisé en deux parties:

1. **Partie 1**: Implémentation primaire d'un jeu de rapidité de temps de réaction avec bouton et LED
2. **Partie 2**: Implémentation d'un jeu de rapidité de frappe sur un clavier USB avec affichage sur écran LCD

<!--underline that-->

#### Partie 1

Le jeu est conçu pour tester la rapidité de réaction et de frappe des joueurs. La partie 1 consiste globalement
à appuyer sur un bouton dès que la LED s'allume après un délai aléatoire suite au déclenchement du jeu. Il y aura
2 boutons et 2 LEDs pour permettre à 2 joueurs de jouer simultanément. Ainsi, la boucle de jeu consiste à:

1. Attendre un délai aléatoire
2. Allumer les 2 LEDs
3. Attendre que les joueurs appuient sur leur bouton respectif
4. Mesurer le temps de réaction et déclarer le gagnant
5. Faire clignoter la LED du gagnant
6. Répéter le jeu.

**À noter**: Implémenter les mécanismes de **reset**, **debounce** etc.

#### Partie 2

La partie 2 est une extension de la partie 1, où le jeu teste la rapidité de frappe des joueurs. Le jeu consiste à prendre
le gagnant de la partie 1 et le faire jouer à un jeu de rapidité de frappe. Le joueur doit répéter une séquence de caractères
affichée sur un écran LCD. Ces séquences seront des lignes de code aléatoires (Possiblement le code source ?).
Le joueur doit taper la séquence le plus rapidement possible dans un temps limité (en fonction du nombre de caractères).

### Le choix de Rust

<span style="color:orange">Rust</span> [0] est un langage de programmation moderne qui met l'accent sur la <span style="color:orange">sécurité mémoire, la rapidité et la concurrence</span>. Il prend en charge plusieurs paradigmes de programmation et peut être utilisé à diverses fins (par exemple, programmation système, développement backend/serveur, outils CLI, etc.).

Rust est également connu pour ses systèmes de propriété et de durées de vie, ainsi que son vérificateur de types strict, qui garantit la sécurité mémoire sans nécessiter de ramasse-miettes, grâce au mécanisme de vérification des emprunts du compilateur.
De plus, c'est un langage de plus en plus populaire autant en industrie qu'en académie [2].

Puisque le cours est axé sur les systèmes temps réel, Rust est un choix judicieux pour ce projet en raison de sa performance, de sa sécurité mémoire
et des contraintres de temps reél et des ressources limitées de la plateforme Raspberry Pi Pico.

## Écheancier et livraisons :calendar:

- [x] _L00_: Introduction et planification :calendar:
- [ ] _L01_: Progrès avec démonstration d'une partie du projet (partie 1 souhaitée!) :video_game:
- [ ] _L02_: Présentation avec démonstration :rocket: du projet final et rapport

| Livrable | Date limite | Description                                                                                                                                                                   |
| -------- | ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| L00      | 2025-01-27  | Introduction et planification. GitHub repo, documentation, matériels requis etc.                                                                                              |
| L01      | 2025-03-10  | Progrès avec démonstration d'une partie du projet. Exécution de la partie 1 souhaitée, sinon en voie de finalisation. Code source, documentation, vidéo de démonstration etc. |
| L02      | 2025-04-07  | Présentation finale du projet avec démonstration. Code source, documentation, vidéo de démonstration terminée avec Rapport final.                                             |

### Objectifs personnels

- Initiations à la programmation de systèmes embarqués, à la plateforme Raspberry Pi Pico et à la programmation en Rust.
- Initiations à l'utilisation de périphériques électroniques, protocoles de communications (I2C, SPI, USB).
- Développement de compétences en programmation temps réel, gestion de ressources et contraintes de temps.

## Environnement de développement :hammer_and_wrench:

Après avoir essayé d'utiliser à-priori le SDK C/C++ du Raspberry Pi Pico, j'ai décidé de me tourner vers Rust pour ce projet.
Quoique C soit le langage de prédilection pour les systèmes embarqués, mon impression initiale de la chaine d'outils
avec _CMake_ via le SDK Pico, le débuggage semi-fonctionnelle via un 2e pico avec _OpenOCD_ et _GDB_ ainsi qu'aucune gestion
native par le SDK pour le _multi-threading_ m'ont poussé à explorer une alternative.

J'ai également eu un problème avec l'initialisation des périphériques via le
SDK Pico, où l'exemple `blinky` ne fonctionnait pas. En utilisant le 2e pico débuggeur,
j'ai pu voir que le code s'exécutait, mais blockait dans le code d'initialisation dans la boucle suivante:

```c
while (!time_reached(t_before)) {
    uint32_t save = spin_lock_blocking(sleep_notifier.spin_lock);
    lock_internal_spin_unlock_with_wait(&sleep_notifier, save);
}
```

Une trace de cette démarche reste sur la branche `c-version-sdk`.
J'ai donc décidé de me tourner vers Rust pour ce projet.

Après avoir brièvement exploré l'introduction à Rust via **The Rust Programming Language** [1], j'ai décidé d'opter qui semblait
être un choix judicieux pour ce projet (ainsi que pour l'apprentissage personnel). La chaîne d'outils de Rust est bien intégrée
avec le développement de systèmes embarqués.

### Chaîne d'outils Rust standards

La chaine d'outils de Rust contient qui est géré via `rustup` contient:

- `rustc`: Compilateur Rust
- `cargo`: Gestion de package et build system
- `rustup`: Gestionnaire de version et chaines d'outils
- `rls`: Rust Language Server pour l'intégration avec les éditeurs de texte
- `rustfmt`: Formatter pour Rust
- `clippy`: Linter pour Rust
- `rust-analyzer`: Analyseur de code pour Rust (Interface avec les éditeurs)

### Ressources, librairies et outils pour le développement Rust sur systèmes embarqués

Premièrement, une première ressource clée est le livre **The Embedded Rust Book** [2] qui est une ressource complète pour le développement Rust sur systèmes embarqués.
De plus, il existe un [Rust Embedded Working Group](https://github.com/rust-embedded) qui fournit des outils, des
librairies et des ressources pour le développement Rust sur systèmes embarqués.

Ensuite, il existe plusieurs librairies et outils pour le développement Rust sur systèmes embarqués:

- `svd2rust`: Générateur de code Rust à partir de fichiers SVD (System View Description) pour les périphériques ARM. [Exécutable svd2rust](https://docs.rs/svd2rust/latest/svd2rust/)
- `probe-rs`: Outil de programmation et de débuggage pour les microcontrôleurs ARM Cortex-M. [Site Web Officiel](https://probe.rs/)
- `cortex-m`: Librairie pour le développement ARM Cortex-M en Rust. Inclus routines d'interruptions, gestion erreurs etc. [GitHub](https://github.com/rust-embedded/cortex-m)
- `embedded-hal`: Abstraction des périphériques pour les systèmes embarqués. [GitHub](https://github.com/rust-embedded/embedded-hal)
- `rp2040-pac`: Périphériques ARM Cortex-M0+ pour le Raspberry Pi Pico. [GitHub](https://github.com/rp-rs/rp2040-pac)
- `rp-rs/rp-hal`: HAL pour le Raspberry Pi Pico. [GitHub](https://github.com/rp-rs/rp-hal)
- `embassy-rs`: Framework asynchrone pour les systèmes embarqués. [Site Web Officiel](https://embassy.dev/)

### Débuggage avec `probe-rs` et système de build avec `cargo`

#### Debugging avec un 2e Raspberry Pi Pico

Tel que retrouvé dans la documentation du [Raspberry Pi Pico](https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf),
il est possible d'utiliser un 2e pico comme débuggeur pour le pico principale.
On peut voir le schéma de connexion ici où le pico débuggeur est connecté à l'ordinateur
et le pico principale est connecté au pico débuggeur via les pins `SWD` ici:

<img src="./media/pico-debug.png" alt="pico-debugging" width="400">

Ainsi, ça nous permet à la fois de simplifier le chargement des exécutables sur le pico principale et également de
débugger le code en utilisant `probe-rs` et `gdb`.

#### Système de buildj

Dans le contexte des systèmes embarqués, le projet `blinky` est un projet de démonstration qui consiste à faire clignoter une LED.
C'est l'équivalent du `hello world!` pour les systèmes embarqués.
Ainsi, je me suis inspiré de l'exemple du projet `blinky` avec `embassy-rs` pour mettre en place l'environnement de développement.

Ce framework nous permettra de gérer les périphériques et les interruptions de manière asynchrone sans avoir recours
à un RTOS (Real-Time Operating System). En minimisant les dépendances, un système de build avec `cargo` et `probe-rs`, nous avons un bon point de départ avec [blinky qui se trouve dans se répertoire](./blinky/).

Voici les éléments nécessaires pour établir un système de build avec `cargo` et un Pi Pico:

**`build.rs`**:

- Facilite l'intégration de la carte des addresses mémoires pour le pico
  avec le fichier `memory.x`. Est utilisé par les crates en lien avec les accès aux périphériques (_PAC_) et
  les abstraction du matériels (_HAL_).
- Passer des flags de compilation pour le linker et le compilateur. i.e. `--nmagic` permet de désactiver l'alignement des pages car nous n'utilisons pas un tel système de pagination de la mémoire dans un système embarqué comme le Pico.

**`memory.x`**:

- Fichier de configuration de la mémoire pour le linker. Définit les sections de mémoire pour le bootloader,
  la mémoire flash et la RAM du pico.

**`Cargo.toml`**:

- Fichier de configuration de `cargo` pour le projet. Contient les dépendances, les configurations de build pour la compilation.
- Contient également les informations de notre projet (nom, version, auteur etc.)

Les profiles `release` et `dev` sont configurés ici, c'est à dire que lorsque nous compilons notre projet avec
`cargo build --release`, les options de compilation pour la version de production sont utilisées. Dans notre cas,
nous avons les options suivantes:

```toml
# Configuration de build pour la version de production
[profile.release]
debug = 2  # Niveau de debuggage complet
lto = true  # Link Time Optimization actif, donc optimisation du code à la compilation
opt-level = 'z'  # Niveau d'optimisation pour la taille du binaire à minimiser
```

**`rust-toolchain.toml`**:

- Fichier de configuration pour `rustup` qui permet de spécifier la chaine d'outils (version et composantes) de Rust.

**`.cargo/config.toml`**:

- Fichier de configuration pour `cargo` qui permet de spécifier les options de build pour le projet.
- Dans notre cas, on spécifie `probe-rs` comme le `runner` pour le débuggage et la cible `thumbv6m-none-eabi` pour la compilation.

`thumbv6m-none-eabi` est la cible pour les microcontrôleurs ARM Cortex-M0 et M0+ (le processeur du Pico).

**`main.rs`**:

- Fichier source principal du projet.
- L'attribut `#![no_std]` indique que nous n'utilisons pas la librairie standard de Rust.
- L'attribut `#![no_main]` indique que nous n'utilisons pas la fonction `main` de Rust,
  mais plutot la fonction `embassy_executor::main` qui est fournie par le framework `embassy-rs`.

Il y aura plus de détails sur le fonctionnement de `embassy-rs` avec `async/await` plus loin.

#### Projet `blinky`

Le projet `blinky` est un projet de démonstration qui consiste à faire clignoter une LED sur le Raspberry Pi Pico.
En utilisant le framework `embassy-rs`, ceci nous permet de facilement faire clignoter une LED en prenant avantager
des fonctionnalités asynchrones (surtout pour le timer).

On voit la LED sur le pico (correspondant à la pin 25) clignoter à une fréquence de 1Hz:
<img src="./media/pico-blinky-live.gif" alt="blinky-live" width="300">

Sur la console:

<img src="./media/pico-blinky-console.gif" alt="blinky-console" width="800">

## Mise en place du matériel :rocket:

### Matériel requis

LIST TODO

## Pico Typing Wars :video_game:

### Partie 1: Rapidité de réaction

TODO

### Partie 2: Rapidité de frappe et affichage

TODO

## Analyses et résultats :chart_with_upwards_trend:

TODO

## Conclusion :checkered_flag:

TODO

## References :books:

<!-- As numbered footnotes-->

<a id="0">[0]</a> **Rust Programming Language**. Rust Foundation. 2024. https://www.rust-lang.org/
<a id="1">[1]</a> **The Rust Programming Language**. Klabnik, Steve, and Carol Nichols. 2nd ed., No Starch Press.
<a id="2">[2]</a> **Industry and academia support**. Rust For Linux. 2024. https://rust-for-linux.com/industry-and-academia-support
<a id="3">[3]</a> **The Embedded Rust Book**. Rust Embedded Working Group. 2024. https://docs.rust-embedded.org/book/
