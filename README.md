
# Projet émulateur de GameBoy 

Pour ce projet de programmation rust, nous avons choisi de créer un émulateur de GameBoy, el famoso console de Nintendo, et tout cela en Rust. Pour cela nous allons procédé comme ceci : Tout d'abord, nous avons programmé la partie CPU de la console, ie les instructions dans le registre de données, les instructions d'éxécutions puis la partie pour lire et écrire dans la mémoire. Ensuite, nous créerons la partie graphique permettant à l'utilisateur d'observer les inputs qu'il rentre. 
## Dépendence

- Cargo 1.72.1
- Rust 1.72.1
- minifb 0.14
- paste 1.0.14


## Execution

Pour lancer le projet, il suffit d'éxécuter à la racine la commande

```bash
  cargo run
```

## Avancement du projet

- L'émulateur compile sans erreur,
- Le choix de la cartouche fonctionne,
- clippy ne détecte pas d'amélioration,
- Le code est commenté (cargo doc --document-private-items),
- La communication avec l'utilisateur (écran et touches) fonctionne,
- Le système de sauvegarde fonctionne,
- La classe CPU (classe principale du programme) passe tous ses Unit Tests,

- L'exécution donne une boucle infinie pour 4 des cartouches testées,
- L'exécution donne une boucle de _rst 0x38_ pour la cartouche Pokemon Red.

**Considérant le manque de résultat en terme de fonctionnement de l'émulateur, l'affichage des instructions et de la ligne dans lesquelles ils ont été trouvées ont été laissés dans la branche principale.**

## Key mapping

Dans src/components/screen.rs:
- Gameboy UP = Host Z
- Gameboy DOWN = Host S
- Gameboy LEFT = Host Q
- Gameboy RIGHT = Host D
- Gameboy START = Host I
- Gameboy SELECT = Host O
- Gameboy A = Host K
- Gameboy B = Host L
- Fermer la gameboy (la fenêtre doit être sélectionnée) = Host ESCAPE.

## Architecture de fichiers
- Cargo.toml: dépendances,
- README.md: description/inscription,
- cartridges/: Contient les cartouches de jeu au format gb,
- save/: Contient les sauvegarde por chaque cartouches de jeu,
- src/components/: Contients l'émulation des parties physiques de la gameboy,
- src/state/: Contient les classes utilisées par certains composants pour décrire une partie de leur état,
- src/main.rs: Point d'entrée du programme (choisit la cartouche et délègue l'initialisation à src/components/cpu.rs).


## Auteurs
- [@Arthur Klein](https://github.com/arthur2klein)
- [@Emmanuella Ngougue Djeufa ](https://github.com/EmmaDjeufa)
- [@Antoine Vacossin](https://github.com/pasteqk)
- [@Alexandre Biscorray](https://github.com/hardintTech)

