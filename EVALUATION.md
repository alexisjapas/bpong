# Protocole d'Evaluation Projet - bpong

Ce protocole permet d'evaluer la qualite du projet de maniere reproductible.
Chaque evaluation produit un score par axe et un score global, permettant de suivre l'evolution dans le temps.

## Mode d'emploi

1. Pour chaque axe, evaluer chaque critere selon l'echelle : **0** (absent), **1** (minimal), **2** (correct), **3** (excellent)
2. Calculer le score de l'axe : somme des criteres / somme maximale possible x 100
3. Calculer le score global : moyenne ponderee des axes
4. Reporter les resultats dans le tableau de suivi en fin de document

---

## Axe 1 — Code source (poids : 25%)

| # | Critere | Description | 0 | 1 | 2 | 3 |
|---|---------|-------------|---|---|---|---|
| 1.1 | Structure | Organisation en modules logiques, separation des responsabilites | Fichier unique monolithique | Quelques modules | Modules bien decoupes | Architecture plugin claire, chaque module a une responsabilite unique |
| 1.2 | Lisibilite | Nommage, formatage, coherence de style | Code illisible | Style inconsistant | Style coherent, noms corrects | Noms explicites, style uniforme, code auto-documente |
| 1.3 | Complexite | Fonctions courtes, peu d'imbrication, logique claire | Fonctions > 100 lignes, imbrication profonde | Quelques fonctions longues | Majorite de fonctions < 50 lignes | Fonctions courtes et focalisees, complexite cyclomatique faible |
| 1.4 | Gestion d'erreurs | Traitement des cas d'erreur, pas de unwrap en production | Panics partout | Quelques unwrap justifies | Gestion explicite dans le code critique | Result/Option utilises systematiquement, erreurs propagees proprement |
| 1.5 | Constantes et config | Pas de valeurs magiques, configuration centralisee | Valeurs en dur partout | Quelques constantes | Module de constantes | Constantes nommees, configurables, documentees |
| 1.6 | Patterns et idiomes | Utilisation idiomatique du langage (Rust) et du framework (Bevy) | Anti-patterns frequents | Usage basique | Bonne utilisation des idiomes | Patterns avances (traits, generics, ECS) utilises a bon escient |

**Score axe 1** : `___` / 18 = `___` %

---

## Axe 2 — Tests (poids : 20%)

| # | Critere | Description | 0 | 1 | 2 | 3 |
|---|---------|-------------|---|---|---|---|
| 2.1 | Tests unitaires | Presence et couverture des tests unitaires | Aucun test | Quelques tests (< 20% couverture) | Couverture correcte (20-60%) | Couverture elevee (> 60%), cas limites testes |
| 2.2 | Tests d'integration | Tests des interactions entre modules | Aucun | Quelques tests manuels documentes | Tests automatises partiels | Tests d'integration automatises couvrant les flux principaux |
| 2.3 | Tests de non-regression | Tests lies aux bugs corriges | Aucun | Bugs corriges sans test | Quelques tests de regression | Chaque bug corrige est couvert par un test |
| 2.4 | Facilite d'execution | Les tests sont simples a lancer | Pas de tests | Procedure manuelle complexe | `cargo test` fonctionne | `cargo test` + scripts dedies, execution rapide |
| 2.5 | Qualite des assertions | Messages d'erreur clairs, assertions specifiques | N/A (pas de tests) | Assertions basiques (assert!) | Assertions avec messages | Assertions specifiques (assert_eq, assert_matches) avec messages contextuels |

**Score axe 2** : `___` / 15 = `___` %

---

## Axe 3 — Documentation (poids : 10%)

| # | Critere | Description | 0 | 1 | 2 | 3 |
|---|---------|-------------|---|---|---|---|
| 3.1 | README | Presentation du projet, instructions d'installation et d'utilisation | Absent | Titre seul | Installation + utilisation | Complet : description, screenshots, installation, utilisation, contribution |
| 3.2 | Documentation du code | Commentaires et doc-comments (///, //!) | Aucun commentaire | Quelques commentaires epars | Modules et fonctions publiques documentes | Documentation complete avec exemples, doc-tests |
| 3.3 | Architecture | Documentation de l'architecture et des choix techniques | Aucune | Notes informelles | Diagramme ou document dedie | Architecture documentee, ADR (Architecture Decision Records) |
| 3.4 | Changelog | Suivi des changements entre versions | Aucun | Commits clairs | CHANGELOG.md basique | CHANGELOG suivant Keep a Changelog, lie aux releases |

**Score axe 3** : `___` / 12 = `___` %

---

## Axe 4 — CI/CD et outillage (poids : 15%)

| # | Critere | Description | 0 | 1 | 2 | 3 |
|---|---------|-------------|---|---|---|---|
| 4.1 | Pipeline CI | Integration continue automatisee | Aucune | Build seul | Build + lint | Build + lint + tests + checks de securite |
| 4.2 | Pipeline CD | Deploiement/release automatise | Manuel | Script semi-automatise | Release automatisee sur tag | Release multi-plateforme + artefacts + notes de version |
| 4.3 | Linting | Verification statique du code | Aucun | Clippy par defaut | Clippy avec regles customisees | Clippy strict + deny warnings en CI |
| 4.4 | Formatage | Coherence de formatage automatisee | Aucun | rustfmt disponible | rustfmt avec config custom | rustfmt enforce en CI, pre-commit hook |
| 4.5 | Environnement dev | Reproductibilite de l'environnement de developpement | README vague | Liste de dependances | Script/Dockerfile | Nix flake ou equivalent, one-command setup |
| 4.6 | Gestion des dependances | Suivi et mise a jour des dependances | Aucun suivi | Cargo.lock commite | Versions fixees | Audit de securite (cargo-audit), mises a jour regulieres |

**Score axe 4** : `___` / 18 = `___` %

---

## Axe 5 — Gestion de projet (poids : 10%)

| # | Critere | Description | 0 | 1 | 2 | 3 |
|---|---------|-------------|---|---|---|---|
| 5.1 | Versioning | Strategie de gestion des versions | Pas de versionning | Tags git sporadiques | Semver respecte | Semver + politique de branches + protection de main |
| 5.2 | Historique git | Qualite de l'historique de commits | Commits illisibles | Messages basiques | Messages descriptifs, commits atomiques | Conventional commits, historique lineaire, PR reviewees |
| 5.3 | Roadmap | Vision et planification du projet | Aucune | Notes informelles | Roadmap dans le README | Roadmap detaillee avec priorites, milestones, issues trackees |
| 5.4 | Issue tracking | Suivi des bugs et fonctionnalites | Aucun | Notes personnelles | Issues GitHub ouvertes | Issues categorisees, labels, milestones, templates |

**Score axe 5** : `___` / 12 = `___` %

---

## Axe 6 — Fonctionnel (poids : 15%)

| # | Critere | Description | 0 | 1 | 2 | 3 |
|---|---------|-------------|---|---|---|---|
| 6.1 | Features core | Les fonctionnalites principales sont implementees et fonctionnelles | Non fonctionnel | Prototype basique | Core fonctionnel avec bugs connus | Core complet et stable |
| 6.2 | Robustesse | Le programme gere les cas limites sans crasher | Crashes frequents | Crashes occasionnels | Stable en usage normal | Stable dans tous les cas, gestion gracieuse des limites |
| 6.3 | UX/Game feel | Experience utilisateur, feedback visuel et sonore | Aucun feedback | Feedback minimal | Feedback correct (sons, visuels) | Experience polie, animations, feedback riche |
| 6.4 | Completude vs roadmap | Avancement par rapport a la roadmap definie | 0% de la roadmap | Roadmap minimale partielle | Roadmap minimale complete | Roadmap minimale + ameliorations en cours |
| 6.5 | Bugs connus | Nombre et severite des bugs connus non resolus | Bugs bloquants | Bugs majeurs | Bugs mineurs uniquement | Aucun bug connu |

**Score axe 6** : `___` / 15 = `___` %

---

## Axe 7 — Performance et optimisation (poids : 5%)

| # | Critere | Description | 0 | 1 | 2 | 3 |
|---|---------|-------------|---|---|---|---|
| 7.1 | Temps de compilation | Temps de build raisonnable, incremental builds | > 5 min, pas de cache | Cache basique | Builds incrementaux rapides | Build optimise, cache CI, compilation rapide |
| 7.2 | Performance runtime | Framerate stable, pas de fuite memoire | < 30 FPS ou fuites | Framerate instable | 60 FPS stable en usage normal | 60+ FPS stable, profiling effectue, pas de regression |
| 7.3 | Taille du binaire | Taille de l'executable raisonnable | > 100 MB | 50-100 MB | 20-50 MB | < 20 MB ou justification des assets |
| 7.4 | Profils de build | Configurations de build adaptees (debug, release, optimized) | Defaut seul | Debug + release | Profils custom | Profils documentes avec justification des options |

**Score axe 7** : `___` / 12 = `___` %

---

## Grille de synthese

| Axe | Poids | Score brut | Score % | Score pondere |
|-----|-------|------------|---------|---------------|
| 1. Code source | 25% | __ / 18 | __% | __ |
| 2. Tests | 20% | __ / 15 | __% | __ |
| 3. Documentation | 10% | __ / 12 | __% | __ |
| 4. CI/CD et outillage | 15% | __ / 18 | __% | __ |
| 5. Gestion de projet | 10% | __ / 12 | __% | __ |
| 6. Fonctionnel | 15% | __ / 15 | __% | __ |
| 7. Performance | 5% | __ / 12 | __% | __ |
| **TOTAL** | **100%** | | | **__/100** |

### Echelle globale

| Score | Niveau | Description |
|-------|--------|-------------|
| 0-20 | Embryonnaire | Prototype non fonctionnel |
| 21-40 | Initial | Prototype fonctionnel, qualite minimale |
| 41-60 | En developpement | Projet fonctionnel, manques significatifs |
| 61-80 | Mature | Projet solide, ameliorations possibles |
| 81-100 | Excellent | Projet de qualite professionnelle |

---

## Historique des evaluations

| Date | Evaluateur | Score global | Axe 1 | Axe 2 | Axe 3 | Axe 4 | Axe 5 | Axe 6 | Axe 7 | Notes |
|------|-----------|-------------|-------|-------|-------|-------|-------|-------|-------|-------|
| 2026-04-08 | Claude | 43.2 / 100 | 72% | 0% | 17% | 61% | 33% | 53% | 58% | v0.1.0 — Baseline initiale. [Detail](.evaluations/2026-04-08.md) |
