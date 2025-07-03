# Guida all'uso dell'Assistente IA SVBONY

🇺🇸 [English](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README.md) | 🇫🇷 [Français](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-FR.md) | 🇩🇪 [Deutsch](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-DE.md) | 🇮🇹 [Italiano](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-IT.md) | 🇪🇸 [Español](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-ES.md) | 🇷🇺 [Русский](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-RU.md) | 🇵🇹 [Português](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-PT.md) | 🇯🇵 [日本語](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-JP.md) | 🇨🇳 [简体中文](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-CN.md)

## Scarica le ultime versioni
[Scarica da GitHub Releases](https://github.com/dennischancs/svbony-ai-assistant/releases/latest)

## Introduzione
L'Assistente IA SVBONY è uno strumento progettato per monitorare gli eventi di pressione del pulsante IA sul dispositivo SVBONY SVHub Omni2P ed eseguire azioni configurate come l'apertura di URL, l'esecuzione di comandi o la visualizzazione di notifiche. Questo strumento supporta i sistemi Windows e macOS e può essere configurato per avviarsi automaticamente all'avvio del sistema.

## Funzionalità
- Monitorare gli eventi di pressione del pulsante IA sul dispositivo SVBONY SVHub Omni2P.
- Supportare multiple azioni, inclusa l'apertura di URL, l'esecuzione di comandi, l'invio di sequenze di tasti e la visualizzazione di notifiche.
- Supportare l'esecuzione in background o in primo piano.
- Supportare l'avvio automatico all'avvio del sistema.
- Configurazione e registrazione dettagliate.

## Installazione e utilizzo

### Installazione da binari precompilati
Se non desideri compilare il codice da solo, puoi scaricare direttamente i binari precompilati e seguire questi passaggi:
1. Scarica il file binario adatto al tuo sistema operativo.
2. Estrai il file scaricato.
3. Apri un terminale o un prompt dei comandi e naviga nella directory estratta.
4. Esegui il comando `svbony-ai-assistant` per avviare il programma.

### Compilazione dal codice sorgente
Se desideri compilare il programma dal codice sorgente, puoi seguire questi passaggi:

#### Preparazione dell'ambiente
Assicurati di aver installato l'ambiente di sviluppo Rust. Se non lo hai fatto, puoi scaricarlo e installarlo dal [sito ufficiale di Rust](https://www.rust-lang.org/tools/install).

#### Clonare il repository del codice
```bash
git clone https://github.com/dennischancs/svbony-ai-assistant.git
cd svbony-ai-assistant
```

#### Compilare il programma
```bash
cargo build --release
```
Dopo la compilazione, il file eseguibile si troverà nella directory `target/release`.

### Eseguire il programma
Dopo la compilazione, puoi eseguire il programma utilizzando il seguente comando:
```bash
./target/release/svbony-ai-assistant
```

## Argomenti della riga di comando
| Argomento | Descrizione |
| ---- | ---- |
| `-f, --foreground` | Eseguire in modalità primo piano, visualizzando tutti i messaggi di log nella console e mantenendo l'applicazione collegata al terminale. Adatto per il debugging o il monitoraggio manuale. |
| `-b, --background` | Eseguire in modalità background come processo daemon. L'applicazione si staccherà dal terminale e funzionerà silenziosamente in background. Questo è il comportamento predefinito quando lanciata da una GUI. |
| `--enable-autostart` | Configurare l'applicazione per avviarsi automaticamente quando il sistema si avvia. Questo creerà le voci di avvio automatico necessarie per il tuo sistema operativo. |
| `--disable-autostart` | Rimuovere l'applicazione dall'avvio automatico. L'applicazione non si avvierà automaticamente quando il sistema si avvia. |
| `-c, --show-config` | Visualizzare il percorso e il contenuto del file di configurazione corrente, quindi uscire senza avviare il servizio di monitoraggio. |
| `-v, --verbose` | Abilitare l'output di registrazione dettagliato. Questo mostrerà i messaggi di debug e le informazioni dettagliate sulla comunicazione del dispositivo. |
| `-q, --quiet` | Eseguire in modalità silenziosa, sopprimendo tutto l'output di log eccetto i messaggi di errore. |
| `-V, --version` | Visualizzare le informazioni sulla versione. |

### Esempi di utilizzo
```bash
# Eseguire in modalità primo piano con registrazione dettagliata
./target/release/svbony-ai-assistant --foreground --verbose

# Eseguire in modalità background e abilitare l'avvio automatico
./target/release/svbony-ai-assistant --enable-autostart
./target/release/svbony-ai-assistant

# Visualizzare la configurazione corrente
./target/release/svbony-ai-assistant --show-config
```

## File di configurazione
Il file di configurazione è utilizzato per definire il comportamento e le azioni dell'applicazione. Il file di configurazione può trovarsi nelle seguenti posizioni:
- **Windows**: `%APPDATA%\SVBONY-AI-Assistant\config.json`
- **macOS**: `~/Library/Application Support/SVBONY-AI-Assistant/config.json`
- Oppure `config.json` nella stessa directory del file eseguibile

Se il file di configurazione non esiste, l'applicazione utilizzerà la configurazione predefinita e creerà un file di configurazione nella posizione sopra indicata.

### Esempio di file di configurazione
```json
{
  "actions": [
    {
      "name": "Apri app.notta.ai",
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
      "name": "Mostra notifica Assistente IA",
      "action_type": "ShowNotification",
      "parameters": {
        "url": null,
        "command": null,
        "args": null,
        "keys": null,
        "message": "Assistente IA attivato!",
        "title": "Assistente IA SVBONY"
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

## Configurazione dell'avvio automatico
Puoi utilizzare gli argomenti `--enable-autostart` e `--disable-autostart` per abilitare o disabilitare la funzione di avvio automatico dell'applicazione. Ad esempio:
```bash
# Abilitare l'avvio automatico
./target/release/svbony-ai-assistant --enable-autostart

# Disabilitare l'avvio automatico
./target/release/svbony-ai-assistant --disable-autostart
```

## Risoluzione dei problemi
- **Registrazione**: Puoi utilizzare l'argomento `--verbose` per abilitare la registrazione dettagliata per una migliore risoluzione dei problemi.
- **Controllo istanza singola**: Se l'applicazione non riesce ad avviarsi, potrebbe essere perché un'altra istanza è già in esecuzione. Puoi utilizzare l'argomento `--foreground` per avviare più istanze per il debugging.
- **Problemi del file di configurazione**: Se ci sono problemi con il file di configurazione, puoi provare a eliminare il file di configurazione e riavviare l'applicazione. L'applicazione utilizzerà la configurazione predefinita e ricreerà il file di configurazione.

## Contributo
Se desideri contribuire al progetto Assistente IA SVBONY, segui questi passaggi:
1. Clona il repository del codice.
2. Crea un nuovo branch.
3. Apporta modifiche e test.
4. Invia una pull request.

## Licenza
Questo progetto è concesso in licenza sotto la Licenza MIT. Per i dettagli, consulta il file [LICENSE](LICENSE).