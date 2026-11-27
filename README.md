# Correcte

Correcte est un utilitaire de correction de texte en Rust. Un raccourci ouvre le texte du presse-papiers dans un éditeur. Un petit modèle de langage au format GGUF, exécuté en local par `llama-server`, corrige le texte affiché.

- **V1** · Debian 13, GNOME sous X11, x86-64. Raccourci réglé dans GNOME.
- **Interface** · une seule zone de texte. Copier laisse la fenêtre ouverte. Échap et la croix ferment la fenêtre. Chaque ouverture repart du presse-papiers.
- **Objectif d’ouverture** · 300 ms jusqu’à la saisie. Si le démarrage à froid respecte cet objectif et que la copie survit à la fermeture, l’app quitte. Sinon, elle reste en mémoire avec la fenêtre masquée.
- **Installation** · paquet `.deb` avec `llama-server` en CPU pour la V1. Le modèle GGUF est téléchargé pendant la configuration initiale, puis la correction fonctionne hors ligne.
- **Configuration** · `~/.config/correcte/config.toml` par défaut sur Debian. Le modèle GGUF et le prompt système sont personnalisables.
- **Diagnostic** · journaux locaux plafonnés à 3 Mio. Ils contiennent les états, durées et erreurs, sans le texte traité ni le prompt.
- **Avancement** · parcours clavier validé sur la machine de référence avec le prototype Slint. La mesure d’ouverture en version `release` reste à faire.
- **Après la V1** · Flatpak est la piste privilégiée pour diffuser Correcte sur plusieurs distributions Linux.

[Briques techniques et décisions](docs/superpowers/specs/2026-12-14-correcte-v1.md) · [Plan de travail](docs/superpowers/plans/2026-12-14-correcte-v1.md)
