# Prototype 001 - Le Signal muet

> **Statut : PROPOSÉ v0.2, non implémenté.** Revu à blanc le 19 novembre 2026. Aucun playtest réalisé.
>
> **Décision :** tester la déduction de cinq glyphes dans une seule salle. Aucune fonctionnalité GPE n'est présumée existante.

## 1. Promesse et hypothèses

*Je ne débloque pas une porte parce que j'ai deviné un code. Je la débloque parce que j'ai compris comment ces machines fonctionnent.*

- **H1 (découverte)** : le joueur identifie par observation un lien entre état des dispositifs et motifs répétés.
- **H2 (prédiction)** : il utilise ce lien pour prévoir deux actions non présentées par les exemples initiaux.
- **H3 (plaisir)** : il décrit lui-même un déclic, pas un déchiffrage mécanique sans surprise.

Les trois hypothèses seront testées ; elles ne sont pas validées par la revue à blanc.

## 2. Contrat de la salle

Une salle 2D ; cinq glyphes ; quatre observations initiales ; deux commandes à déduire ; ni chrono, ni pénalité, ni génération, ni serveur, ni LLM.

**Règle de portée indispensable : chaque pupitre est physiquement connecté à son dispositif et n'agit que sur lui.** Envoyer une commande avec un nom d'objet différent n'active **jamais** un appareil distant. Ce contrat évite de pouvoir ouvrir la porte depuis le pupitre de la pompe.

La salle expose :

1. Deux lampes **identiques**, l'une allumée et l'autre éteinte, portant chacune une bande de deux glyphes.
2. Une pompe en marche, dont le jet de vapeur **obstrue physiquement l'accès** au pupitre de la porte.
3. Une porte fermée, dont les deux glyphes restent visibles malgré la vapeur.
4. Un pupitre à deux emplacements accessible près de la pompe.
5. Un pupitre semblable relié à la porte, accessible lorsque la vapeur a disparu.

Les bandeaux ont visuellement la même conception que les pupitres. **La fiche de test ne dit pas** qu'ils contiennent les dernières commandes enregistrées. C'est une hypothèse que la mise en scène doit rendre découvrable.

### Objectif diégétique

Le joueur cherche à **quitter la salle**. La fiche ne dit pas quelle machine manipuler ni quel sens attribuer aux inscriptions. Il voit naturellement la vapeur et l'obstacle.

## 3. Grammaire (solution réservée aux concepteurs)

Les IDs latins servent exclusivement à la documentation, pas à l'affichage :

| ID | Glyphe provisoire de la fiche papier | Sens fonctionnel |
| --- | --- | --- |
| A | △ | Activer |
| B | ○ | Désactiver |
| C | ◇ | Lampe |
| D | □ | Pompe |
| E | ⬡ | Porte |

Ordre conventionnel : `[opération][objet]`.

Le second symbole **désigne le type** de l'appareil. Deux lampes du même modèle peuvent porter `A C` et `B C` parce qu'il s'agit d'appareils **locaux distincts**, commandés séparément.

Les panneaux reflètent la dernière commande locale enregistrée et l'état actuel au début de la scène. Il n'existe pas de télécommande universelle.

### Quatre observations initiales

| Dispositif | État visible | Inscription |
| --- | --- | --- |
| Lampe 1 | Allumée | `A C` |
| Lampe 2 | Éteinte | `B C` |
| Pompe | En fonctionnement, vapeur active | `A D` |
| Porte | Fermée | `B E` |

### Deux prédictions visées

- **`B D` sur le pupitre de pompe** : la pompe s'arrête, la vapeur se dissipe et l'accès physique à l'autre pupitre se libère.
- **`A E` sur le pupitre de porte** : la porte s'ouvre, le joueur peut traverser.

La bonne paire n'est **jamais** révélée par un bandeau initial.

### Sémantique des autres entrées (contrat anti-triche)

Un pupitre connecté à l'objet `X` accepte `A X` (activer) ou `B X` (désactiver). Si l'état demandé est déjà celui du dispositif, rien ne change. Toute autre combinaison est rejetée **sans effet sur le monde** et sans indication du symbole erroné.

- `A E` saisi sur la pompe ne peut pas ouvrir la porte.
- `A D` saisi sur la pompe la laisse en marche.
- `B E` saisi sur la porte la laisse fermée.
- Les commandes inappropriées et les commandes sans changement d'état ne donnent **aucun indice de traduction**, mais peuvent avoir des retours neutres d'interaction.

Les lampes sont des **exemples environnementaux**, pas des pupitres supplémentaires du MVP. On ne postule pas que le joueur puisse les modifier.

**Limite assumée :** la recherche exhaustive des paires reste possible. L'observation du cheminement du joueur et une tâche de transfert après la partie servent à distinguer compréhension et chance, sans punir les tentatives.

## 4. Parcours joueur et feedback

1. Voir les quatre machines et leurs inscriptions.
2. Comparer les lampes : même objet, états contraires, glyphe initial différent.
3. Confronter l'hypothèse à la pompe en activité et à la porte fermée.
4. Composer une paire sur la pompe et observer l'effet matériel sur la vapeur.
5. Composer une paire sur le pupitre désormais accessible.
6. Expliquer la règle (uniquement **après** le test).

Échec de commande : retour visuel discret, sans texte « erreur sur la position 1 ». Les dispositifs gardent des états distincts **sans dépendre uniquement de la couleur ou du son**.

Un joueur qui ouvre la porte par hasard mais n'explique pas ce qu'il a compris **ne valide pas H2**.

## 5. Protocole de playtest papier

Distribuer uniquement [la fiche joueur](../playtests/001-fiche-joueur.md) ; le modérateur connaît la solution et consigne les essais sans souffler la méthode.

Au début, ne pas expliquer le rôle des glyphes, le fait qu'il s'agisse de commandes, ni la solution. Le modérateur fournit uniquement les retours d'état correspondant à la paire tentée et au pupitre utilisé.

Consigner : temps de première hypothèse ; nombre de commandes soumises ; ordre ; réactions observées ; paroles spontanées ; motifs d'hésitation.

Questions **après résolution** :
- Pourquoi avoir choisi ces deux paires ?
- À quoi servent les deux positions de glyphes, selon toi ?
- Si la pompe était à nouveau arrêtée, comment la réactiverais-tu ? Pourquoi ?
- As-tu eu un moment de compréhension, ou as-tu surtout expérimenté des combinaisons ?

Six joueurs naïfs (objectif exploratoire) ; ne pas fournir la solution à ceux qui doivent encore tester.

| Mesure | Cible d'exploration |
| --- | --- |
| Première résolution sans indice direct | Au moins 4/6 |
| Capacité à justifier les deux commandes | Au moins 4/6 |
| Capacité à transférer la règle à la pompe arrêtée | Au moins 4/6 |
| Déclic déclaré spontanément ou après question neutre | Au moins 4/6 |
| Durée | Observer la médiane, ne pas fabriquer artificiellement 10 minutes |

Les seuils ne constituent pas une preuve statistique.

## 6. Résultats et décisions

- **GO après playtest** : la majorité démontre compréhension, transfert et plaisir de découverte.
- **REVISE** : interprétations alternatives fréquentes, résolution par bruteforce, feedback confus ou trivialité non satisfaisante. Réviser **les preuves ou le découpage** avant de complexifier le langage.
- **NO-GO pour cette variante** : la boucle reste sans déclic malgré une révision ciblée. Cela n'invalide pas la vision d'Archives du Silence.

## 7. Capability preflight GPE (ultérieur)

**Non effectué.** Après un GO ludique, inspecter réellement les sources et le build GPE : glyphes lisibles à la résolution cible, rendu des états, commandes clavier/manette et tactile si utile, feedback accessible, scènes, packaging natif/WASM. **Hard stop** en cas de capacité absente ; documenter le plus petit changement justifié.

## 8. Hors scope

Véritable langue complète ; carnet d'hypothèses persistant ; histoire ramifiée ; plusieurs salles ; réseau ; générateur de puzzles ; IA ; système de score ; combat ; inventaire ; systèmes de quêtes ; refactoring GPE.

## 9. Documents liés

- [Fiche sans spoilers pour les joueurs](../playtests/001-fiche-joueur.md)
- [Revue adversariale v0.2](../reviews/prototype-001-adversarial.md)
