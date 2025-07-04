# Guide d'utilisation de l'Assistant IA SVBONY

🇺🇸 [English](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README.md) | 🇫🇷 [Français](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-FR.md) | 🇩🇪 [Deutsch](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-DE.md) | 🇮🇹 [Italiano](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-IT.md) | 🇪🇸 [Español](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-ES.md) | 🇷🇺 [Русский](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-RU.md) | 🇵🇹 [Português](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-PT.md) | 🇯🇵 [日本語](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-JP.md) | 🇨🇳 [简体中文](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-CN.md)

#### Démarrage Rapide

1. Téléchargez le binaire approprié à partir des [Releases GitHub](https://github.com/dennischancs/svbony-ai-assistant/releases/latest) pour votre plateforme, par exemple : macOS (x86_64, aarch64/Apple Silicon), Windows (x86_64)
2. Extraire l'archive
3. Exécuter le binaire

## Introduction
L'Assistant IA SVBONY est un outil conçu pour surveiller les événements de pression du bouton IA sur le dispositif SVBONY SVHub Omni2P et exécuter des actions configurées telles que l'ouverture d'URL, l'exécution de commandes ou l'affichage de notifications. Cet outil prend en charge les systèmes Windows et macOS et peut être configuré pour démarrer automatiquement au démarrage du système.

## Fonctionnalités
- Surveiller les événements de pression du bouton IA sur le dispositif SVBONY SVHub Omni2P.
- Prendre en charge plusieurs actions, notamment l'ouverture d'URL, l'exécution de commandes, l'envoi de frappes de touches et l'affichage de notifications.
- Prendre en charge l'exécution en arrière-plan ou en premier plan.
- Prendre en charge le démarrage automatique au démarrage du système.
- Configuration et journalisation détaillées.

## Installation et utilisation

### Installation à partir de binaires précompilés
Si vous ne souhaitez pas compiler le code vous-même, vous pouvez télécharger directement les binaires précompilés et suivre ces étapes :
1. Téléchargez le fichier binaire adapté à votre système d'exploitation.
2. Extrayez le fichier téléchargé.
3. Ouvrez un terminal ou une invite de commande et naviguez vers le répertoire extrait.
4. Exécutez la commande `svbony-ai-assistant` pour démarrer le programme.

### Compilation à partir du code source
Si vous souhaitez compiler le programme à partir du code source, vous pouvez suivre ces étapes :

#### Préparation de l'environnement
Assurez-vous d'avoir installé l'environnement de développement Rust. Si ce n'est pas le cas, vous pouvez le télécharger et l'installer depuis le [site officiel de Rust](https://www.rust-lang.org/tools/install).

#### Cloner le dépôt de code
```bash
git clone https://github.com/dennischancs/svbony-ai-assistant.git
cd svbony-ai-assistant
```

#### Compiler le programme
```bash
cargo build --release
```
Après la compilation, le fichier exécutable sera situé dans le répertoire `target/release`.

### Exécuter le programme
Après la compilation, vous pouvez exécuter le programme en utilisant la commande suivante :
```bash
./target/release/svbony-ai-assistant
```

## Arguments de ligne de commande
| Argument | Description |
| ---- | ---- |
| `-f, --foreground` | Exécuter en mode premier plan, affichant tous les messages de journal dans la console et gardant l'application attachée au terminal. Convient pour le débogage ou la surveillance manuelle. |
| `-b, --background` | Exécuter en mode arrière-plan comme un processus démon. L'application se détachera du terminal et s'exécutera silencieusement en arrière-plan. C'est le comportement par défaut lorsqu'elle est lancée depuis une interface graphique. |
| `--enable-autostart` | Configurer l'application pour démarrer automatiquement lorsque le système démarre. Cela créera les entrées de démarrage automatique nécessaires pour votre système d'exploitation. |
| `--disable-autostart` | Supprimer l'application du démarrage automatique. L'application ne démarrera pas automatiquement lorsque le système démarre. |
| `-c, --show-config` | Afficher le chemin et le contenu du fichier de configuration actuel, puis quitter sans démarrer le service de surveillance. |
| `-v, --verbose` | Activer la sortie de journalisation détaillée. Cela affichera les messages de débogage et les informations détaillées sur la communication avec le dispositif. |
| `-q, --quiet` | Exécuter en mode silencieux, supprimant toute sortie de journal sauf les messages d'erreur. |
| `-V, --version` | Afficher les informations de version. |

### Exemples d'utilisation
```bash
# Exécuter en mode premier plan avec journalisation détaillée
./target/release/svbony-ai-assistant --foreground --verbose

# Exécuter en mode arrière-plan et activer le démarrage automatique
./target/release/svbony-ai-assistant --enable-autostart
./target/release/svbony-ai-assistant

# Afficher la configuration actuelle
./target/release/svbony-ai-assistant --show-config
```

## Fichier de configuration
Le fichier de configuration est utilisé pour définir le comportement et les actions de l'application. Le fichier de configuration peut être situé dans les emplacements suivants :
- **Windows** : `%APPDATA%\SVBONY-AI-Assistant\config.json`
- **macOS** : `~/Library/Application Support/SVBONY-AI-Assistant/config.json`
- Ou `config.json` dans le même répertoire que le fichier exécutable

Si le fichier de configuration n'existe pas, l'application utilisera la configuration par défaut et créera un fichier de configuration dans l'emplacement ci-dessus.

### Exemple de fichier de configuration
```json
{
  "actions": [
    {
      "name": "Ouvrir app.notta.ai",
      "action_type": "OpenUrl",
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
      "name": "Afficher une notification d'Assistant IA",
      "action_type": "ShowNotification",
      "parameters": {
        "url": null,
        "command": null,
        "args": null,
        "keys": null,
        "message": "Assistant IA activé !",
        "title": "Assistant IA SVBONY"
      },
      "enabled": true
    }
  ],
  "settings": {
    "auto_start": true,
    "minimize_to_tray": true,
    "log_level": "info",
    "check_updates": true
  }
}
```

## Configuration du démarrage automatique
Vous pouvez utiliser les arguments `--enable-autostart` et `--disable-autostart` pour activer ou désactiver la fonction de démarrage automatique de l'application. Par exemple :
```bash
# Activer le démarrage automatique
./target/release/svbony-ai-assistant --enable-autostart

# Désactiver le démarrage automatique
./target/release/svbony-ai-assistant --disable-autostart
```

## Dépannage
- **Journalisation** : Vous pouvez utiliser l'argument `--verbose` pour activer la journalisation détaillée pour un meilleur dépannage.
- **Vérification d'instance unique** : Si l'application ne parvient pas à démarrer, cela peut être parce qu'une autre instance est déjà en cours d'exécution. Vous pouvez utiliser l'argument `--foreground` pour démarrer plusieurs instances pour le débogage.
- **Problèmes de fichier de configuration** : S'il y a des problèmes avec le fichier de configuration, vous pouvez essayer de supprimer le fichier de configuration et de redémarrer l'application. L'application utilisera la configuration par défaut et recréera le fichier de configuration.

## Contribution
Si vous souhaitez contribuer au projet Assistant IA SVBONY, veuillez suivre ces étapes :
1. Cloner le dépôt de code.
2. Créer une nouvelle branche.
3. Apporter des modifications et des tests.
4. Soumettre une pull request.

## Licence
Ce projet est sous licence MIT. Pour plus de détails, veuillez consulter le fichier [LICENSE](LICENSE).