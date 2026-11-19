# Les Archives du Silence

A GPE-powered archaeological puzzle game where knowledge,
deduction and a forgotten language unlock the world.

## Vision

Comprendre plutôt que deviner.
Découvrir plutôt qu'accumuler.
Apprendre pour agir sur le monde.

## Etat du projet

**Prototype 001 natif Rust : beta visuelle et sonore, non validee sur le plan ludique.**

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
- H : afficher ou masquer l'aide.
- Tab : changer de pupitre, si accessible.
- Cliquer sur le pupitre numerote directement dans la salle : le selectionner.
- Cliquer sur une case du HUD : la selectionner ; un second clic fait defiler le glyphe.
- Cliquer sur ENVOYER : tenter la combinaison sur le pupitre actif.

Le pupitre actif est maintenant **surligne directement dans la salle**, ainsi que
l'appareil auquel il est relie. La saisie des deux glyphes reste distincte pour
chacun des deux pupitres. Aucun sens des glyphes n'est revele par l'interface.
- R : recommencer.
- Echap : quitter.

L'affichage utilise un framebuffer 640x360 presente dans une fenetre 1280x720.
Le bandeau de controles reste compact et l'aide est masquee par defaut.
Une ambiance grave et plusieurs effets sonores proceduraux sont generes par Rust
et diffuses via les bus audio GPE, sans assets externes.
Si le moteur ne peut pas initialiser la sortie audio, le jeu le signale.

Le jeu **ne demande aucune connaissance externe**. Observe, essaie et
raisonne librement. Si tu bloques, note simplement ce qui t'a intrigue.

### Validation du developpement

```bash
cargo fmt --all -- --check
cargo check --locked --all-targets
cargo test --locked --all-targets
cargo clippy --locked --all-targets --all-features -- -D warnings
```

Le `Cargo.lock` n'est pas encore versionne ; une resolution provisoire est generee
avant les controles. La generation puis le commit du lockfile sont requis avant
de considerer la beta reproductible.

Le prototype utilise l'API publique du moteur GPE sur une revision Git
pointee dans `Cargo.toml`. Aucun changement du moteur n'est requis
par la conception du prototype.

## Phase suivante

Effectuer un premier test a l'aveugle, consigner les observations, puis
decider de la suite (GO / REVISE / NO-GO). **Aucun playtest n'a encore ete realise.**
