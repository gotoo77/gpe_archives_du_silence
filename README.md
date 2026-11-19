# Les Archives du Silence

Jeu d'exploration et de deduction dans des archives d'une civilisation disparue.
Comprendre plutot que deviner. Decouvrir plutot qu'accumuler.

## Version experimentale : Les Remanences (prototype 002)

Cette branche contient une **experience de reconstruction de souvenirs**. Elle
remplace l'enigme des deux pupitres du prototype 001, qui a echoue au premier
test de gameplay. Ce prototype 002 n'est **pas valide humainement** et ne doit
pas etre presente comme le gameplay definitif du jeu.

Le prototype conserve l'API native GPE epinglee et l'ambiance audio procedurale.

### Garder la surprise

Le proprietaire du projet souhaite jouer **a l'aveugle**.
Ne pas ouvrir `src/lib.rs`, `src/main.rs`, les tests ou la documentation
de conception avant l'essai : ils contiennent les solutions.

Aucun dictionnaire de glyphes n'est donne ici.

### Prerequis

- Rust 1.97.1 (cf. `rust-toolchain.toml`)
- Outils systeme requis par GPE pour le rendu et le son
- Sur Windows, toolchain Rust cible MSVC correctement installee

### Lancement natif

```powershell
cargo generate-lockfile
cargo run --locked
```

Le `Cargo.lock` n'est pas encore commite sur cette branche de developpement :
la compilation fonctionne apres resolution locale, mais elle n'est pas
pleinement reproductible. Ne pas annoncer la validation du lockfile.

### Controles de jeu

- **Souris** : cliquer sur une archive pour examiner sa projection, sur une
  scene pour choisir son interpretation, ou sur une rune pour ecrire.
- **Tab et fleches** : deplacer la selection selon l'ecran actif.
- **Entree** : visiter, choisir ou revenir selon le contexte.
- **S** depuis la salle : confronter les deux reconstructions. Le jeu ne donne
  jamais de score partiel ni de traduction automatique.
- **Espace** pendant l'ecriture : effacer le signe selectionne.
- **Echap** : revenir a la salle, quitter depuis la salle.
- **H** : afficher ou masquer l'aide.
- **R** : recommencer.

**Observation importante :** le fragment lu et l'inscription reconstruite sont
deux taches distinctes. Une reponse incorrecte ne dit pas quel element est faux.

### Ce que le premier test doit permettre de distinguer

- Trouver le chemin par essais successifs.
- Expliquer une regle sans que le jeu la fournisse.
- Reutiliser cette regle dans une situation nouvelle.
- Avoir ou non une decouverte narrative.
- Comprendre l'interface sans intervention du developpeur.

Une capture et un ressenti sur la lisibilite font partie de la validation :
les tests automatises ne prouvent pas qu'un dessin raconte une action.

### Verifications Rust

```bash
cargo fmt --all -- --check
cargo check --locked --all-targets
cargo test --locked --all-targets
cargo clippy --locked --all-targets --all-features -- -D warnings
git diff --check
```

Aucun resultat de CI n'est presume : verifier le statut du dernier SHA sur
GitHub Actions. La validation Win11 native n'est pas couverte par la CI Linux.

## Scope

Une salle, six fragments observables, un texte a interpreter, une scene a
inscrire, une restitution finale. Pas de monde ouvert, de langue complete,
de progression numerique, de penalite, de generation, de service IA ni de
refactoring du moteur GPE.

### Historique

- Prototype 001 : **REVISE/NO-GO ludique**, [issue #5](https://github.com/gotoo77/gpe_archives_du_silence/issues/5)
- Prototype 002 : candidature experimentale, [issue #7](https://github.com/gotoo77/gpe_archives_du_silence/issues/7)
