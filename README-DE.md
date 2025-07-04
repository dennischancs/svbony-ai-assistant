# SVBONY AI-Assistent Nutzungsanleitung

🇺🇸 [English](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README.md) | 🇫🇷 [Français](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-FR.md) | 🇩🇪 [Deutsch](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-DE.md) | 🇮🇹 [Italiano](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-IT.md) | 🇪🇸 [Español](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-ES.md) | 🇷🇺 [Русский](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-RU.md) | 🇵🇹 [Português](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-PT.md) | 🇯🇵 [日本語](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-JP.md) | 🇨🇳 [简体中文](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-CN.md)

#### Schnellstart

1. Laden Sie die passende Binärdatei von der [GitHub-Releases-Seite](https://github.com/dennischancs/svbony-ai-assistant/releases/latest) für Ihre Plattform herunter, z.B.: macOS (x86_64, aarch64/Apple Silicon), Windows (x86_64)
2. Entpacken Sie das Archiv
3. Führen Sie die Binärdatei aus

## Einführung
Der SVBONY AI-Assistent ist ein Tool, das entwickelt wurde, um die AI-Tastenereignisse auf dem SVBONY SVHub Omni2P-Gerät zu überwachen und konfigurierte Aktionen wie das Öffnen von URLs, das Ausführen von Befehlen oder das Anzeigen von Benachrichtigungen durchzuführen. Dieses Tool unterstützt Windows- und macOS-Systeme und kann so konfiguriert werden, dass es beim Systemstart automatisch startet.

## Funktionen
- Überwachung von AI-Tastenereignissen auf dem SVBONY SVHub Omni2P-Gerät.
- Unterstützung mehrerer Aktionen, einschließlich des Öffnens von URLs, Ausführens von Befehlen, Sendens von Tastenanschlägen und Anzeigen von Benachrichtigungen.
- Unterstützung für das Ausführen im Hinter- oder Vordergrund.
- Unterstützung für automatischen Start beim Systemstart.
- Detaillierte Konfiguration und Protokollierung.

## Installation und Verwendung

### Installation aus vorkompilierten Binärdateien
Wenn Sie den Code nicht selbst kompilieren möchten, können Sie die vorkompilierten Binärdateien direkt herunterladen und diese Schritte befolgen:
1. Laden Sie die für Ihr Betriebssystem geeignete Binärdatei herunter.
2. Extrahieren Sie die heruntergeladene Datei.
3. Öffnen Sie ein Terminal oder eine Eingabeaufforderung und navigieren Sie zum extrahierten Verzeichnis.
4. Führen Sie den Befehl `svbony-ai-assistant` aus, um das Programm zu starten.

### Kompilierung aus dem Quellcode
Wenn Sie das Programm aus dem Quellcode kompilieren möchten, können Sie diese Schritte befolgen:

#### Umgebungsvorbereitung
Stellen Sie sicher, dass Sie die Rust-Entwicklungsumgebung installiert haben. Wenn nicht, können Sie sie von der [offiziellen Rust-Website](https://www.rust-lang.org/tools/install) herunterladen und installieren.

#### Code-Repository klonen
```bash
git clone https://github.com/dennischancs/svbony-ai-assistant.git
cd svbony-ai-assistant
```

#### Programm kompilieren
```bash
cargo build --release
```
Nach der Kompilierung befindet sich die ausführbare Datei im Verzeichnis `target/release`.

### Programm ausführen
Nach der Kompilierung können Sie das Programm mit folgendem Befehl ausführen:
```bash
./target/release/svbony-ai-assistant
```

## Befehlszeilenargumente
| Argument | Beschreibung |
| ---- | ---- |
| `-f, --foreground` | Im Vordergrundmodus ausführen, alle Protokollmeldungen in der Konsole anzeigen und die Anwendung am Terminal anhängen. Geeignet für Debugging oder manuelle Überwachung. |
| `-b, --background` | Im Hintergrundmodus als Daemon-Prozess ausführen. Die Anwendung wird sich vom Terminal trennen und still im Hintergrund laufen. Dies ist das Standardverhalten beim Start über eine GUI. |
| `--enable-autostart` | Die Anwendung so konfigurieren, dass sie automatisch beim Systemstart startet. Dies erstellt die notwendigen Autostart-Einträge für Ihr Betriebssystem. |
| `--disable-autostart` | Die Anwendung vom automatischen Start entfernen. Die Anwendung wird nicht automatisch beim Systemstart starten. |
| `-c, --show-config` | Den aktuellen Konfigurationsdateipfad und -inhalt anzeigen, dann beenden ohne den Überwachungsdienst zu starten. |
| `-v, --verbose` | Ausführliche Protokollausgabe aktivieren. Dies zeigt Debug-Meldungen und detaillierte Informationen über die Gerätekommunikation. |
| `-q, --quiet` | Im stillen Modus ausführen, alle Protokollausgaben außer Fehlermeldungen unterdrücken. |
| `-V, --version` | Versionsinformationen anzeigen. |

### Verwendungsbeispiele
```bash
# Im Vordergrundmodus mit ausführlicher Protokollierung ausführen
./target/release/svbony-ai-assistant --foreground --verbose

# Im Hintergrundmodus ausführen und automatischen Start aktivieren
./target/release/svbony-ai-assistant --enable-autostart
./target/release/svbony-ai-assistant

# Aktuelle Konfiguration anzeigen
./target/release/svbony-ai-assistant --show-config
```

## Konfigurationsdatei
Die Konfigurationsdatei wird verwendet, um das Verhalten und die Aktionen der Anwendung zu definieren. Die Konfigurationsdatei kann sich in den folgenden Speicherorten befinden:
- **Windows**: `%APPDATA%\SVBONY-AI-Assistant\config.json`
- **macOS**: `~/Library/Application Support/SVBONY-AI-Assistant/config.json`
- Oder `config.json` im selben Verzeichnis wie die ausführbare Datei

Wenn die Konfigurationsdatei nicht existiert, verwendet die Anwendung die Standardkonfiguration und erstellt eine Konfigurationsdatei am oben genannten Speicherort.

### Beispiel einer Konfigurationsdatei
```json
{
  "actions": [
    {
      "name": "app.notta.ai öffnen",
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
      "name": "AI-Assistent-Benachrichtigung anzeigen",
      "action_type": "ShowNotification",
      "parameters": {
        "url": null,
        "command": null,
        "args": null,
        "keys": null,
        "message": "AI-Assistent aktiviert!",
        "title": "SVBONY AI-Assistent"
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

## Konfiguration des automatischen Starts
Sie können die Argumente `--enable-autostart` und `--disable-autostart` verwenden, um die automatische Startfunktion der Anwendung zu aktivieren oder zu deaktivieren. Zum Beispiel:
```bash
# Automatischen Start aktivieren
./target/release/svbony-ai-assistant --enable-autostart

# Automatischen Start deaktivieren
./target/release/svbony-ai-assistant --disable-autostart
```

## Fehlerbehebung
- **Protokollierung**: Sie können das Argument `--verbose` verwenden, um ausführliche Protokollierung für bessere Fehlerbehebung zu aktivieren.
- **Einzelinstanz-Überprüfung**: Wenn die Anwendung nicht startet, kann dies daran liegen, dass bereits eine andere Instanz läuft. Sie können das Argument `--foreground` verwenden, um mehrere Instanzen für Debugging zu starten.
- **Konfigurationsdatei-Probleme**: Bei Problemen mit der Konfigurationsdatei können Sie versuchen, die Konfigurationsdatei zu löschen und die Anwendung neu zu starten. Die Anwendung wird die Standardkonfiguration verwenden und die Konfigurationsdatei neu erstellen.

## Beitrag
Wenn Sie zum SVBONY AI-Assistent-Projekt beitragen möchten, befolgen Sie bitte diese Schritte:
1. Code-Repository klonen.
2. Einen neuen Branch erstellen.
3. Änderungen vornehmen und testen.
4. Eine Pull-Request einreichen.

## Lizenz
Dieses Projekt ist unter der MIT-Lizenz lizenziert. Für Details siehe die Datei [LICENSE](LICENSE).