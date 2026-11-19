# Les Archives du Silence

A GPE-powered archaeological puzzle game where knowledge,
deduction and a forgotten language unlock the world.

## Vision

Comprendre plutôt que deviner.
Découvrir plutôt qu'accumuler.
Apprendre pour agir sur le monde.

## Etat du projet

**Prototype 001 natif Rust : version experimentale non validee par des joueurs.**

Aucune autre fonctionnalite de gameplay n'est annoncee comme implementee.

## Joueur beta : conserver la surprise

Si tu veux decouvrir l'enigme sans spoilers, **ne lis pas les fichiers
`src/lib.rs`, `docs/prototypes/`, `docs/reviews/` et les tests** avant
d'avoir joue. Le code du jeu et la documentation de conception contiennent
inevitablement des informations sur les solutions.

### Lancer la version native

Pre-requis : Rust 1.97.1, environnement natif compatible avec GPE et
dependances systeme requises par le moteur.

```bash
cargo generate-lockfile
cargo run --locked
```

Commandes dans le jeu :

- Fleches gauche/droite : selectionner un emplacement de glyphe.
- Fleches haut/bas : faire defiler les glyphes.
- Entree : tenter une combinaison.
- Tab : changer de pupitre, si accessible.
- R : recommencer.
- Echap : quitter.

Le jeu **ne demande aucune connaissance externe**. Observe, essaie et
raisonne librement. Si tu bloques, note simplement ce qui t'a intrigue.

### Validation du developpement

```bash
cargo fmt --all -- --check
cargo check --locked --all-targets
cargo test --locked --all-targets
cargo clippy --locked --all-targets --all-features -- -D warnings
```

Le `Cargo.lock` n'est pas encore versionne ; une resolution provisoire est generee\navant les controles. La generation puis le commit du lockfile sont requis avant\nde considerer la beta reproductible.\n\nLe prototype utilise l'API publique du moteur GPE sur une revision Git
pointee dans `Cargo.toml`. Aucun changement du moteur n'est requis
par la conception du prototype.

## Phase suivante

Effectuer un premier test a l'aveugle, consigner les observations, puis
decider de la suite (GO / REVISE / NO-GO). **Aucun playtest n'a encore ete realise.**
