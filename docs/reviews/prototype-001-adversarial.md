# Revue adversariale à blanc v0.2 - Le Signal muet

> **Nature** : simulation critique sur documents, le 19 novembre 2026. **Pas un playtest, pas un audit du moteur GPE.**
>
> **Périmètre** : [spécification v0.2](../prototypes/001-le-signal-muet.md), [fiche joueur](../playtests/001-fiche-joueur.md). Les points P0 de la v0.1 ont déclenché une correction documentaire ciblée, **sans nouveau gameplay**.

## Verdict

- **GO WITH CONDITIONS** pour une expérience papier de déduction.
- **HARD STOP** sur une implémentation GPE tant que l'énigme n'a pas été testée et le moteur inspecté.
- **Incertitude majeure** : cette micro-grammaire peut être cohérente mais trop triviale pour provoquer le déclic recherché.

## Failles détectées

### P0-1. Portée des pupitres : contournement du puzzle (corrigée sur papier)

**Attaque** : en v0.1, rien n'empêchait d'émettre `A E` depuis la pompe et d'ouvrir la porte à distance. L'ordre « pompe puis porte » était imposé sans justification système.

**Correction v0.2** : pupitre physiquement connecté à un seul dispositif ; le deuxième glyphe désigne le type d'appareil local. Commande de cible différente : aucun effet ; l'accès au pupitre de porte est bloqué par la vapeur.

**À vérifier** : en jeu, ce raccordement doit être visible sans un texte d'explication. Sinon la correction est théorique, pas perceptible.

### P0-2. Fiche joueur trop directive (corrigée sur papier)

**Attaque** : l'ancienne fiche disait « dernière commande enregistrée », puis détaillait « arrêter la pompe » et « ouvrir la porte ». Le test donnait le modèle syntaxique et la séquence d'actions à inférer.

**Correction v0.2** : la fiche décrit des bandeaux à deux glyphes et un dispositif de saisie ; le seul but explicite est de sortir de la salle. Aucun mot « commande », « activer » ou « désactiver » dans les observations.

**À vérifier** : un testeur est-il capable d'identifier la fonction des bandeaux sans un exposé verbal du modérateur ?

### P0-3. Validation de la compréhension par simple réussite (contrôle ajouté)

**Attaque** : 5 symboles, 2 emplacements, donc jusqu'à 25 paires possibles si répétitions autorisées. La chance ou le brute force peuvent ouvrir la porte sans compréhension.

**Mitigation** : noter les tentatives, demander une justification et tester après coup la prédiction `A D` sur une pompe arrêtée ; ne jamais inférer H2 de la seule porte ouverte. Aucun verrou, minuterie punitive ou pénalité d'essai.

**Risque résiduel** : un joueur peut construire une explication a posteriori. Un playtest papier ne prouve pas une cognition interne ; on cherche des indices convergents (phrases spontanées, faibles essais, transfert).

### P1-1. C'est un tableau de vérité, pas encore une langue

**Attaque** : deux verbes binaires + trois appareils ressemblent davantage à un protocole d'automate qu'à l'interprétation d'une langue ancienne. L'exploration, la culture et l'archéologie restent surtout dans l'habillage.

**Réponse** : ne pas nier le problème. **Ce prototype mesure uniquement l'émergence d'une déduction appliquée au monde.** Il ne valide pas l'ambition narrative ou linguistique. Pour les phases suivantes, tester un sens transférable à des contextes différents, sans planifier déjà un moteur grammatical.

### P1-2. Puzzle trop facile pour sa promesse de durée

**Attaque** : les deux lampes côte à côte offrent une différence directe ; la pompe et la porte rendent la permutation presque évidente. Une résolution en 30-60 secondes n'est pas forcément gratifiante.

**Réponse** : conserver le puzzle simple pour obtenir une base mesurable, noter le temps et le plaisir rapporté ; ne pas ajouter de glyphes, de fausses pistes ou de pénalités pour remplir artificiellement dix minutes.

**Signal de REVISE** : les joueurs comprennent rapidement mais disent qu'ils ont suivi une recette sans découverte.

### P1-3. Indices compatibles avec d'autres interprétations

**Attaque** : les bandeaux pourraient être des noms, des descriptions d'états ou un historique. Les indices n'imposent pas logiquement le sens « impératif de commande ».

**Réponse** : le jeu n'a pas à imposer une traduction linguistique unique, mais **la règle d'action doit être vérifiable** par le feedback. La bande des pupitres doit ressembler aux bandeaux des objets ; un test naïf dira si l'association est intelligible. Ne pas corriger l'ambiguïté en expliquant tout au joueur.

### P1-4. Incohérences d'interaction

**Attaque** : si le pupitre de la pompe accepte `A D` alors qu'elle tourne déjà, quel retour produit-il ? Si un glyphe pour la porte est tenté sur la pompe, le joueur reçoit-il un indice qui révèle la grammaire ?

**Réponse** : définir les effets par état du dispositif : commande locale cohérente = état souhaité ; commande incohérente = aucun effet. Retours neutres, sans réponse « c'était l'objet qui était faux ».

### P1-5. Accessibilité et lisibilité

**Attaque** : dans une très petite résolution, `△`, `◇`, `□` peuvent devenir difficiles à différencier ; la vapeur peut masquer le dispositif au lieu de montrer l'obstacle.

**Réponse** : les glyphes géométriques sont uniquement des placeholders papier. Tester des silhouettes distinctes, grandes et robustes ; retours visuels et audio redondants. Le préflight GPE reste à faire.

## Attaques à tenter durant le test papier

1. Émettre `A E` sur le pupitre de la pompe : **la porte ne s'ouvre pas**.
2. Émettre `B C` sur le pupitre de la pompe : **aucun effet**, aucune révélation de grammaire.
3. Émettre `A D` alors que la pompe est déjà en marche : **aucun changement d'état**.
4. Sauter directement au pupitre de porte : **inaccessible**, obstacle physique visible.
5. Résoudre uniquement par essais : **porte ouverte ne vaut pas compréhension**.
6. Proposer l'interprétation « A/B sont des états et non des verbes » : accepter si la **prédiction du fonctionnement** est correcte.
7. Comprendre très vite sans plaisir : **REVISE**, ne pas déclarer GO en se fondant sur la réussite seule.

## Idées intéressantes à différer

- **Carnet d'hypothèses** : enregistrer les significations supposées sans confirmation automatique.
- **Polysémie fonctionnelle** : un symbole comme « éveiller » s'applique à une lampe, une machine puis un souvenir.
- **Archéologie des erreurs** : une hypothèse partiellement correcte devient insuffisante plus tard.
- **Inscriptions culturelles** : symboles dans des documents, rituels et architectures, pas seulement dans des dispositifs.

Aucune n'est un engagement d'implémentation.

## Critères de sortie

- [x] Portée locale des commandes définie dans le document.
- [x] Fiche joueur rendue moins directive.
- [x] Comportement des tentatives invalides défini.
- [ ] Un testeur naïf comprend sans aide verbale.
- [ ] Les joueurs peuvent expliquer et réappliquer la règle.
- [ ] La majorité rapporte de la curiosité ou un déclic, au-delà du score.
- [ ] Les glyphes sont lisibles et les états accessibles en contexte de rendu cible.
- [ ] Préflight réel du dépôt GPE avant le moindre code.

## Conclusion

Le prototype v0.2 est un **bon test falsifiable**, pas encore un bon jeu démontré. Si la découverte ressemble à un exercice de logique scolaire, ne pas passer directement au Rust. Réviser le design d'expérience d'abord.
