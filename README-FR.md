# Guide d'utilisation de SVBONY AI Assistant

🇺🇸 [English](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README.md) | 🇫🇷 [Français](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-FR.md) | 🇩🇪 [Deutsch](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-DE.md) | 🇮🇹 [Italiano](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-IT.md) | 🇪🇸 [Español](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-ES.md) | 🇷🇺 [Русский](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-RU.md) | 🇵🇹 [Português](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-PT.md) | 🇯🇵 [日本語](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-JP.md) | 🇨🇳 [简体中文](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-CN.md)

### Démarrage rapide

1. [Téléchargez le binaire approprié depuis GitHub Releases](https://github.com/dennischancs/svbony-ai-assistant/releases/latest) pour votre plateforme (macOS x86_64, aarch64/Apple Silicon, Windows x86_64)
2. Décompressez l'archive
3. Exécutez le binaire

## Appareils pris en charge
- SVBONY SVHub Omni2P (PID : 0x5053)
- SVBONY SVHub M6 (PID : 0x364d)

## Introduction
SVBONY AI Assistant surveille les événements du bouton AI sur les appareils SVBONY pris en charge et exécute les actions configurées (ouvrir des URL, exécuter des commandes, envoyer des touches (remplaçant), afficher des notifications). Il fonctionne sous Windows, macOS, et peut être configuré pour démarrer automatiquement au démarrage du système.

## Fonctionnalités
- Surveillance des boutons AI sur SVBONY SVHub Omni2P et M6.
- Prise en charge de plusieurs actions : ouvrir des URL, exécuter des commandes, envoyer des touches (remplaçant), afficher des notifications.
- Fonctionnement en arrière-plan (daemon) ou au premier plan (avec logs).
- Démarrage automatique configurable, mis en place automatiquement au premier lancement si activé.
- Vérification d'instance unique en mode arrière-plan.
- Arrêt propre via signaux système (Ctrl+C, SIGTERM).
- Configuration détaillée et journalisation.
- Notifications multiplateformes (Windows Toast, macOS osascript).

## Installation et utilisation

### Installation à partir des binaires précompilés
1. Téléchargez le binaire adapté à votre système.
2. Décompressez-le.
3. Ouvrez un terminal, placez-vous dans le dossier extrait.
4. Lancez `svbony-ai-assistant`.

### Compilation depuis les sources
Assurez-vous d'avoir Rust installé ([site officiel](https://www.rust-lang.org/tools/install)).

```bash
git clone https://github.com/dennischancs/svbony-ai-assistant.git
cd svbony-ai-assistant
cargo build --release
```
L'exécutable sera dans `target/release`.

### Exécution
```bash
./target/release/svbony-ai-assistant
```

## Arguments en ligne de commande
| Argument | Description |
| ---- | ---- |
| `-f, --foreground` | Mode premier plan, logs dans la console. |
| `-b, --background` | Mode arrière-plan (daemon). Par défaut en lancement GUI. |
| `--enable-autostart` | Active le démarrage automatique au boot. |
| `--disable-autostart` | Désactive le démarrage automatique. |
| `-c, --show-config` | Affiche le chemin et le contenu de la configuration puis quitte. |
| `-r, --regenerate-config` | Réinitialise la configuration (sauvegarde l'ancienne). |
| `-v, --verbose` | Active les logs détaillés. |
| `-q, --quiet` | Supprime tous les logs sauf les erreurs. |
| `-V, --version` | Affiche la version. |

### Exemples
```bash
./target/release/svbony-ai-assistant --foreground --verbose
./target/release/svbony-ai-assistant --enable-autostart
./target/release/svbony-ai-assistant
./target/release/svbony-ai-assistant --show-config
./target/release/svbony-ai-assistant --regenerate-config
```

## Types d'action
- `OpenUrl` : Ouvre une URL dans le navigateur par défaut.
- `RunCommand` : Exécute une commande système.
- `SendKeys` : (Remplaçant) Simule des frappes clavier (pas encore implémenté).
- `ShowNotification` : Affiche une notification système.

## Fichier de configuration
Le fichier de configuration définit le comportement de l'application. Il peut se trouver :
- **Windows** : `%APPDATA%\SVBONY-AI-Assistant\config.json`
- **macOS** : `~/Library/Application Support/SVBONY-AI-Assistant/config.json`
- ou dans le même dossier que l'exécutable

S'il n'existe pas, un fichier par défaut sera créé.

### Exemple
```json
{
  "actions": [
    {
      "name": "Open app.notta.ai",
      "action_type": "OpenUrl", // Options : OpenUrl, RunCommand, SendKeys, ShowNotification
      "parameters": {
        "url": "https://app.notta.ai",
        "command": null,
        "args": null,
        "keys": null,
        "message": null,
        "title": null
      },
      "enabled": true
    },
    {
      "name": "Show AI Assistant Notification",
      "action_type": "ShowNotification",
      "parameters": {
        "url": null,
        "command": null,
        "args": null,
        "keys": null,
        "message": "AI Assistant activé !",
        "title": "SVBONY AI Assistant"
      },
      "enabled": true
    }
  ],
  "settings": {
    "auto_start": true,
    "minimize_to_tray": true,
    "log_level": "info",
    "check_updates": true
  },
  "version": "0.1.0"
}
```

### Compatibilité de version
Le champ `version` du fichier de configuration doit correspondre à la version de l'application. En cas de mise à jour :
- Si la version ne correspond pas, l'application :
  1. Sauvegarde l'ancienne config sous `config.json.old`
  2. Crée une nouvelle config par défaut
- L'ancien fichier reste disponible en cas de besoin.

## Démarrage automatique
- Si `auto_start` est activé, le démarrage auto sera configuré au premier lancement.
- Vous pouvez aussi activer/désactiver manuellement avec `--enable-autostart` et `--disable-autostart`.

## Notifications
- **Windows** : Toast (PowerShell), ou notification ballon en secours.
- **macOS** : `osascript`.

## Dépannage
- **Logs** : Utilisez `--verbose` pour plus de détails.
- **Instance unique** : En mode arrière-plan, si le lancement échoue, une autre instance est peut-être déjà active. Utilisez `--foreground` pour le debug.
- **Problème de config** : Supprimez le fichier de config et relancez pour repartir sur une base saine.

## Contribution
1. Clonez le dépôt.
2. Créez une branche.
3. Modifiez et testez.
4. Envoyez une pull request.

## Licence
Projet sous licence MIT. Voir [LICENSE](LICENSE).