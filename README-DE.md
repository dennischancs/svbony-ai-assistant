# SVBONY AI Assistant Benutzerhandbuch

🇺🇸 [English](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README.md) | 🇫🇷 [Français](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-FR.md) | 🇩🇪 [Deutsch](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-DE.md) | 🇮🇹 [Italiano](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-IT.md) | 🇪🇸 [Español](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-ES.md) | 🇷🇺 [Русский](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-RU.md) | 🇵🇹 [Português](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-PT.md) | 🇯🇵 [日本語](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-JP.md) | 🇨🇳 [简体中文](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-CN.md)

### Schnellstart

1. [Laden Sie das passende Binary von GitHub Releases herunter](https://github.com/dennischancs/svbony-ai-assistant/releases/latest) für Ihr System (macOS x86_64, aarch64/Apple Silicon, Windows x86_64)
2. Archiv entpacken
3. Binary ausführen

## Unterstützte Geräte
- SVBONY SVHub Omni2P (PID: 0x5053)
- SVBONY SVHub M6 (PID: 0x364d)

## Einführung
Der SVBONY AI Assistant überwacht die AI-Taste auf unterstützten SVBONY-Geräten und führt konfigurierte Aktionen aus (z.B. URLs öffnen, Befehle ausführen, Tasten senden (Platzhalter), Benachrichtigungen anzeigen). Unterstützt Windows, macOS und kann für den Autostart konfiguriert werden.

## Funktionen
- Überwachung der AI-Taste auf SVBONY SVHub Omni2P und M6.
- Unterstützt mehrere Aktionen: URLs öffnen, Befehle ausführen, Tasten senden (Platzhalter), Benachrichtigungen anzeigen.
- Betrieb im Hintergrund (Daemon) oder Vordergrund (mit Logs).
- Automatischer Start beim Systemstart (konfigurierbar, beim ersten Start automatisch eingerichtet, falls aktiviert).
- Einzelinstanz-Prüfung im Hintergrundmodus.
- Sauberes Beenden über Systemsignale (Ctrl+C, SIGTERM).
- Detaillierte Konfiguration und Protokollierung.
- Plattformübergreifende Benachrichtigungen (Windows Toast, macOS osascript).

## Installation und Nutzung

### Installation aus vorgefertigten Binaries
1. Passendes Binary für Ihr Betriebssystem herunterladen.
2. Archiv entpacken.
3. Terminal öffnen, in das Verzeichnis wechseln.
4. `svbony-ai-assistant` ausführen.

### Kompilierung aus dem Quellcode
Stellen Sie sicher, dass Rust installiert ist ([offizielle Seite](https://www.rust-lang.org/tools/install)).

```bash
git clone https://github.com/dennischancs/svbony-ai-assistant.git
cd svbony-ai-assistant
cargo build --release
```
Das Binary befindet sich danach in `target/release`.

### Ausführung
```bash
./target/release/svbony-ai-assistant
```

## Kommandozeilenargumente
| Argument | Beschreibung |
| ---- | ---- |
| `-f, --foreground` | Im Vordergrund ausführen, Logausgaben in der Konsole. |
| `-b, --background` | Im Hintergrund (Daemon) ausführen. Standard bei GUI-Start. |
| `--enable-autostart` | Autostart beim Systemstart aktivieren. |
| `--disable-autostart` | Autostart deaktivieren. |
| `-c, --show-config` | Zeigt den Pfad und Inhalt der Konfiguration an und beendet das Programm. |
| `-r, --regenerate-config` | Setzt die Konfiguration auf Werkseinstellungen zurück (alte wird gesichert). |
| `-v, --verbose` | Ausführliche Logausgabe aktivieren. |
| `-q, --quiet` | Nur Fehler ausgeben. |
| `-V, --version` | Versionsinformation anzeigen. |

### Beispiele
```bash
./target/release/svbony-ai-assistant --foreground --verbose
./target/release/svbony-ai-assistant --enable-autostart
./target/release/svbony-ai-assistant
./target/release/svbony-ai-assistant --show-config
./target/release/svbony-ai-assistant --regenerate-config
```

## Aktionstypen
- `OpenUrl`: Öffnet eine URL im Standardbrowser.
- `RunCommand`: Führt einen Systembefehl aus.
- `SendKeys`: (Platzhalter) Simuliert Tastendrücke (noch nicht implementiert).
- `ShowNotification`: Zeigt eine Systembenachrichtigung an.

## Konfigurationsdatei
Die Konfigurationsdatei definiert das Verhalten der Anwendung. Sie kann sich befinden unter:
- **Windows**: `%APPDATA%\SVBONY-AI-Assistant\config.json`
- **macOS**: `~/Library/Application Support/SVBONY-AI-Assistant/config.json`
- oder im selben Verzeichnis wie das Binary

Fehlt die Datei, wird eine Standardkonfiguration erstellt.

### Beispiel
```json
{
  "actions": [
    {
      "name": "Open app.notta.ai",
      "action_type": "OpenUrl", // Optionen: OpenUrl, RunCommand, SendKeys, ShowNotification
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
        "message": "AI Assistant aktiviert!",
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

### Versionskompatibilität
Das Feld `version` in der Konfiguration muss zur Programmversion passen. Bei einem Update:
- Wenn die Version nicht passt, wird automatisch:
  1. Die alte Konfiguration als `config.json.old` gesichert
  2. Eine neue Standardkonfiguration erstellt
- Die alte Datei bleibt als Backup erhalten.

## Autostart
- Ist `auto_start` aktiviert, wird Autostart beim ersten Start automatisch eingerichtet.
- Sie können Autostart auch manuell mit `--enable-autostart` und `--disable-autostart` steuern.

## Benachrichtigungen
- **Windows**: Toast-Benachrichtigungen (PowerShell), alternativ Ballon.
- **macOS**: `osascript`.
- **Linux**: `notify-send` oder `zenity`.

## Fehlerbehebung
- **Logs**: Mit `--verbose` erhalten Sie detaillierte Ausgaben.
- **Einzelinstanz-Prüfung**: Im Hintergrundmodus kann ein Startfehler bedeuten, dass bereits eine Instanz läuft. Nutzen Sie `--foreground` zum Debuggen.
- **Konfigurationsprobleme**: Löschen Sie die Konfigurationsdatei und starten Sie neu, um eine Standardkonfiguration zu erhalten.

## Beitrag
1. Repository klonen.
2. Branch erstellen.
3. Änderungen und Tests durchführen.
4. Pull Request einreichen.

## Lizenz
Dieses Projekt steht unter der MIT-Lizenz. Siehe [LICENSE](LICENSE).