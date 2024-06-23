
# Essaim de robots spatiaux pour l'exploration et l'astrobiologie

Ce projet développe un essaim de robots autonomes spécialisés pour l'exploration spatiale et la recherche astrobiologique. Les robots collaboreront pour mener des missions d'exploration et de recherche sur des corps célestes (planètes, lunes, astéroïdes) afin de recueillir des données sur la géologie, la chimie et les potentiels signes de vie.


## Features

- Carte 2D avec des obstacles et des ressources (énergie, minerais, points d'intérêt scientifique).
- Robots modulaires avec des spécialisations pour différentes tâches.
- Reconnaissance du terrain et collecte d'échantillons.
- Station centrale pour collecter les données et construire de nouveaux robots.
- Utilisation d'un Tileset pour les Textures
- Gestion des textures de la carte à l'aide d'un tileset unique, permettant une meilleure efficacité de la mémoire et des performances.
- Gestion de la Mémoire et des Performances avec les Threads
- Utilisation de threads pour optimiser la gestion de la mémoire et les performances du système.
- Initialisation des Types de Tuiles
- Initialisation des types de tuiles en fonction de leur position dans le tileset, assurant une attribution précise et ordonnée des textures.
- Génération de la Carte avec une Fonction de Noise
- Initialisation des tiles et de la carte à l'aide d'une fonction de bruit (noise) pour créer des variations et du contraste.
- Algorithme de Dijkstra pour le Pathfinding
- Implémentation de l'algorithme de Dijkstra pour trouver le chemin le plus court entre deux entités sur la carte.


## Tech Stack

- Rust


## Installation

```bash
bash
git clone <repo_url>
cargo install
cargo run