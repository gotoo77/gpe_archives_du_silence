# Prototype 001 - Le Signal muet

> **Statut : PROPOSÉ, non implémenté.** Document de design v0.1, soumis à une adversarial review et à des playtests.
>
> Ce document décrit **une hypothèse de puzzle**, pas un comportement déjà disponible dans GPE.

## 1. Intention

Produire en moins de dix minutes un véritable moment de compréhension : le joueur observe une technologie inconnue, déduit une règle à partir de preuves croisées, puis **réutilise la règle dans deux nouveaux contextes**.

Promesse : *Je ne débloque pas une porte parce que j'ai trouvé un code ; je la débloque parce que j'ai compris quelque chose.*

### Ce que l'on teste

- **Hypothèse H1** : un joueur sans explication préalable peut inférer deux opérations et trois objets à partir d'indices visuels.
- **Hypothèse H2** : la règle inférée permet de prédire au moins deux effets nouveaux.
- **Hypothèse H3** : la résolution produit une satisfaction de découverte, et non seulement de tâtonnement.

Ne **pas** tester la génération procédurale, une langue complète, la narration longue, le multijoueur ou un moteur de dialogue.

## 2. Contrat du prototype

- Une seule salle en vue 2D, sans exploration complexe.
- Cinq glyphes abstraits et **une règle unique** : une inscription est une paire `[opération, objet]`.
- Quatre observations initiales, deux actions de résolution.
- Une interface minimale de combinaison de deux glyphes.
- Pas de texte traduit automatiquement, pas de tutoriel révélant la règle.
- Aucune dépendance à un LLM, à un backend ou à Internet.
- Objectif de durée : environ 5 à 10 minutes, à confirmer en playtest.

### Espace

La salle contient :
1. Une lampe allumée, avec une inscription active.
2. Une seconde lampe éteinte, avec une inscription distincte.
3. Une pompe en fonctionnement, dont la vapeur masque un pupitre près de la porte.
4. Une porte fermée, avec une inscription visible.
5. Un pupitre accessible près de la pompe, puis le pupitre de la porte une fois la vapeur dissipée.

Important : les inscriptions sont présentées comme **les dernières commandes enregistrées par les dispositifs**, et non comme des phrases historiques arbitraires. Leur emplacement et le fonctionnement visible des machines rendent leur signification inférable.

## 3. Grammaire et solution (notes concepteur, spoilers)

Les lettres ci-dessous sont **des identifiants de conception**, pas des glyphes à afficher dans l'interface finale. Les dessins définitifs des glyphes restent à concevoir.

| ID glyphe | Interprétation de travail | Rôle |
| --- | --- | --- |
| A | activer / éveiller | opération |
| B | désactiver / faire taire | opération |
| C | lumière / lampe | objet |
| D | flux / pompe | objet |
| E | passage / porte | objet |

Grammaire : `[opération][objet]`. La traduction française exacte reste secondaire ; le jeu teste les **relations fonctionnelles**, pas le vocabulaire français.

### Indices observables, dès le début

| Source | État visible ou audible | Paire affichée |
| --- | --- | --- |
| Lampe 1 | allumée | `A C` |
| Lampe 2 | éteinte | `B C` |
| Pompe | active, vapeur abondante | `A D` |
| Porte | fermée | `B E` |

Déduction attendue : le premier signe exprime l'état/opération, le second désigne le dispositif. La répétition de `C` entre les deux lampes permet de croiser les indices ; `A` se retrouve sur la pompe active.

### Deux prédictions à réaliser

1. Sur le pupitre de la pompe, composer **`B D`**. La pompe s'arrête, la vapeur se dissipe et le pupitre de la porte devient accessible.
2. Sur le pupitre de la porte, composer **`A E`**. La porte s'ouvre ; une courte réaction audiovisuelle accompagne la réussite.

Aucune des deux paires de résolution n'est présentée comme exemple préalable. Les résultats découlent des règles observées.

**Risque assumé :** avec seulement cinq glyphes, il reste possible de tenter les combinaisons au hasard. Le prototype ne prétend pas rendre le bruteforce impossible. Les playtests devront distinguer les prédictions raisonnées des essais opportunistes.

## 4. Parcours joueur attendu

1. **Observer** les quatre inscriptions, les deux lampes, la pompe et la porte.
2. **Comparer** les glyphes communs : même `C` pour deux lampes, mais `A` vs `B` selon l'état.
3. **Émettre une hypothèse** sur la position des opérations et des objets.
4. **Prédire** `B D` pour arrêter la pompe ; observer la dissipation de la vapeur.
5. **Transférer** la règle vers la porte : `A E`.
6. **Comprendre le retour du monde** : les machines réagissent conformément à une règle désormais intelligible.

### Feedback sur erreur

Une paire non valide ou sans effet identifiable doit fournir un signal discret et cohérent (son, lumière du pupitre) **sans afficher une traduction ou la bonne réponse**. Pas de vie perdue, pas de pénalité arbitraire, pas de blocage définitif.

Les dispositifs doivent avoir des états visuellement distincts ; ne jamais faire reposer l'information uniquement sur le son ou uniquement sur une couleur.

## 5. Une réussite est-elle réellement une compréhension ?

Pendant le playtest, noter la séquence d'actions et, **après la fin de l'énigme**, demander sans indice :

- « Que crois-tu que signifient les deux positions de glyphes ? »
- « Pourquoi as-tu choisi cette combinaison pour la porte ? »
- « Si une nouvelle lampe était allumée et que tu voulais l'éteindre, que ferais-tu ? »

Le troisième item permet de tester un transfert hypothétique qui n'a pas été entièrement dicté par le puzzle.

Un joueur qui ouvre la porte par essais successifs mais ne peut pas expliquer sa prédiction **n'est pas compté comme une validation de H2**.

## 6. Critères de validation de l'expérience

Test exploratoire avec **six personnes** n'ayant pas consulté la solution.

| Critère | Cible exploratoire |
| --- | --- |
| Compréhension sans indice direct | Au moins 4/6 joueurs |
| Deux paires de résolution choisies avec une justification cohérente | Au moins 4/6 |
| Capacité à expliquer la règle après jeu | Au moins 4/6 |
| Perception d'un « déclic » plutôt que de tâtonnement | Au moins 4/6, auto-déclaré |
| Durée | Médiane autour de 5-10 min ; mesurer, ne pas imposer |

Ces seuils sont des objectifs de prototypage, **pas des preuves statistiques**. Conserver les retours qualitatifs et les essais infructueux.

### Résultats possibles

- **GO** : la majorité comprend et transfère ; la boucle est jugée satisfaisante.
- **REVISE** : la règle reste ambiguë ou les joueurs réussissent surtout au hasard ; revoir les indices, pas le moteur.
- **NO-GO local** : aucun déclic récurrent malgré une révision du puzzle ; remettre en cause la mécanique précise du prototype, pas l'ambition générale.

## 7. GPE : préflight avant tout code

L'état réel du dépôt de GPE **n'a pas été inspecté pour cette note**. Vérifier avant toute intégration :

- Rendu et lisibilité de glyphes contrastés en framebuffer 320x180 (référence de projet à confirmer).
- Sélection de deux glyphes à la manette, au clavier et éventuellement au tactile.
- États visuels/sonores des dispositifs ; lisibilité sans son.
- Transition de scène, UI des deux pupitres et feedback d'erreur, avec les systèmes existants.
- Exécution et packaging natif + WASM selon les capacités actuelles du moteur.

**Hard stop** si une capacité essentielle manque. Documenter le blocage et la solution minimale, sans inventer de nouvel équipement d'UI ou modifier le moteur par anticipation.

## 8. Hors scope de la première vertical slice

Langue complète ; plusieurs salles ; carnet persistant ; confiance des hypothèses ; monde ouvert ; lore exhaustif ; IA de génération ; solutions multiples automatiques ; scoring ; compte utilisateur ; backend ; moteur de dialogue ; combats ; arbre de progression.

## 9. Prochaines décisions

1. Tester cette énigme sur papier, y compris avec un « joueur adversarial » cherchant à forcer les combinaisons.
2. Modifier uniquement les indices déficients selon les observations.
3. Procéder au capability preflight GPE.
4. Si et seulement si le préflight et la revue du puzzle sont satisfaisants, implémenter la plus petite slice jouable.

Voir aussi : [review adversariale](../reviews/prototype-001-adversarial.md).
