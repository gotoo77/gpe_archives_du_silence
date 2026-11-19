# Adversarial review à blanc - Prototype 001 « Le Signal muet »

> **Statut : simulation de revue critique, 2026-11-19.** Aucun joueur n'a encore testé l'énigme ; aucun code n'a été audité.
>
> Document associé : [spécification du prototype](../prototypes/001-le-signal-muet.md).

## Question directrice

**Peut-on distinguer la compréhension d'une règle de la simple découverte d'une combinaison de boutons ?**

Le concept est prometteur si les actions changent le monde en vertu d'une **hypothèse que le joueur formule**. Une porte qui s'ouvre après un mot de passe n'est pas une preuve de compréhension.

## Objections critiques, par priorité

### P0 - Brute force

**Attaque :** avec seulement cinq glyphes et deux positions, il est facile de tester les paires ; un joueur peut terminer sans comprendre.

**Impact :** invalide notre principal argument différenciant.

**Réponse minimale :** ne pas augmenter artificiellement le nombre de glyphes ni imposer un verrou temporel. Observer les choix dans un playtest, puis interroger le joueur sur ses prédictions et lui proposer un problème de transfert indépendant. Si le brute force domine, réviser les indices et la présentation des dispositifs. Une solution durable pourrait introduire une conséquence observable des hypothèses, mais **pas dans cette première implémentation**.

**Test falsifiable :** un testeur résout les deux actions mais ne peut pas expliquer les rôles des glyphes : score de compréhension = échec.

### P0 - Ambiguïté des inscriptions

**Attaque :** une inscription à côté d'une lampe allumée peut vouloir dire « lumière », « alimenter », « lampe », « dangereux » ou être un nom propre ; le joueur n'a aucune raison de supposer un langage de commandes.

**Réponse minimale :** utiliser un design cohérent de **bandes de commandes enregistrées** sur tous les appareils ; présenter les lampes comme identiques excepté leur état ; faire réagir visuellement les dispositifs. Les inférences doivent reposer sur des contrastes, pas sur une traduction arbitraire.

**Test falsifiable :** trois testeurs sur six ou plus interprètent les panneaux comme de simples étiquettes décoratives : revoir la mise en scène des preuves.

### P0 - Causalité artificielle

**Attaque :** pourquoi faut-il arrêter la pompe avant d'atteindre la porte ?

**Réponse minimale :** montrer le rideau de vapeur issu de la pompe qui masque effectivement le pupitre de la porte, puis sa dissipation quand la pompe s'arrête. Pas de règle « porte verrouillée tant que pompe active » invisible.

**Test falsifiable :** après l'action sur la pompe, le joueur ne peut pas verbaliser ce qui a changé dans la salle : feedback à revoir.

### P1 - Trop proche de Chants of Sennaar / Heaven's Vault

**Attaque :** les glyphes, les inscriptions et l'archéologie ont déjà été explorés avec talent.

**Réponse :** le critère de différence n'est pas le thème, mais le **transfert fonctionnel** : la règle linguistique devient un outil qui modifie les objets. Cela reste une hypothèse d'identité, pas une innovation garantie.

**Test :** demander ce que le joueur retient : « j'ai déchiffré une phrase » ou « j'ai compris comment ces machines fonctionnent ».

### P1 - Manque de richesse à moyen terme

**Attaque :** une grammaire binaire `activer/désactiver` s'épuise vite.

**Réponse :** accepter ce risque. Une première slice doit prouver un *moment de découverte*, pas la durée de vie d'un jeu complet. Différer la combinatoire grammaticale, les polysémies, les contradictions culturelles et l'émergence narrative jusqu'à validation.

### P1 - Accessibilité / ergonomie

**Attaque :** à faible résolution, les glyphes sont illisibles ; à la manette, les sélections sont lentes ; les informations uniquement sonores ou colorimétriques excluent certains joueurs.

**Réponse :** préflight GPE + glyphes à silhouettes fortement distinctes + variations de mouvement et de forme ; test sans son ; contrôle des paires au clavier/manette avant toute DA élaborée.

### P1 - Connaissance vs progression scriptée

**Attaque :** la machine « valide » seulement une paire autorisée : l'illusion de compréhension cache un puzzle linéaire très classique.

**Réponse :** pour le MVP, l'implémentation peut rester scriptée, mais le comportement doit être **cohérent avec les exemples observés**. Ne pas promettre un moteur de langue génératif ou une simulation systémique tant que non implémentés.

## Trois pistes pour sublimer le concept, mais PAS pour le MVP

1. **Hypothèses persistantes** : un carnet où le joueur note ses traductions, sans validation automatique.
2. **Sémantique contextuelle** : un même verbe se manifeste différemment selon l'objet (éveiller une lumière, éveiller une machine, éveiller une mémoire).
3. **Archéologie des erreurs** : une traduction raisonnable, mais incomplète, fonctionne dans un premier contexte et se révèle trompeuse dans un autre.

Ces pistes sont **PROPOSÉES**, non arbitrées comme fonctionnalités.

## Décisions de scope

| Décision | Contenu |
| --- | --- |
| **GELER** | Découverte par déduction, connaissances transférables, une salle et cinq glyphes |
| **PROTOTYPER** | Deux prédictions (`B D`, `A E`), preuves visuelles, mini-playtest |
| **DIFFÉRER** | Carnet, langage complet, récits à embranchements, génération, réseau, score |
| **HARD STOP** | Pas d'intégration GPE sans inspection de ses capacités réelles |

## Critères de sortie de la phase 0

- [ ] Vérifier la logique des quatre observations et deux solutions avec au moins un testeur naïf.
- [ ] Observer les tentatives au hasard et noter les explications données.
- [ ] S'assurer que l'état des dispositifs est intelligible sans son ni couleur.
- [ ] Réviser la spécification quand une ambiguïté est objectivement observée.
- [ ] Documenter le résultat des six premiers tests (ou justifier explicitement tout écart).
- [ ] Effectuer ensuite le préflight GPE avant de créer du code.

## Verdict

**GO WITH CONDITIONS** pour tester une énigme dessinée sur papier.

**NO-GO temporaire** pour promettre qu'une implémentation complète ou une « vraie langue » est déjà justifiée. Le différenciateur reste à prouver expérimentalement.
