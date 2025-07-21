# Guida all'uso dell'Assistente IA SVBONY

🇺🇸 [English](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README.md) | 🇫🇷 [Français](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-FR.md) | 🇩🇪 [Deutsch](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-DE.md) | 🇮🇹 [Italiano](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-IT.md) | 🇪🇸 [Español](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-ES.md) | 🇷🇺 [Русский](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-RU.md) | 🇵🇹 [Português](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-PT.md) | 🇯🇵 [日本語](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-JP.md) | 🇨🇳 [简体中文](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-CN.md)

### Avvio Rapido

1. Scarica il binario appropriato dalle [Releases di GitHub](https://github.com/dennischancs/svbony-ai-assistant/releases/latest) per la tua piattaforma, ad esempio: macOS (x86_64, aarch64/Apple Silicon), Windows (x86_64)
2. Scompatta l'archivio
3. Esegui il binario

## Dispositivi supportati
- SVBONY SVHub Omni2P (PID: 0x5053)
- SVBONY SVHub M6 (PID: 0x364d)

## Introduzione
L'Assistente IA SVBONY è uno strumento progettato per monitorare gli eventi di pressione del pulsante IA sui dispositivi SVBONY supportati ed eseguire azioni configurate come l'apertura di URL, l'esecuzione di comandi, l'invio di sequenze di tasti (in sviluppo) o la visualizzazione di notifiche. Questo strumento supporta sistemi Windows, macOS e può essere configurato per avviarsi automaticamente all'avvio del sistema.

## Funzionalità
- Monitorare gli eventi di pressione del pulsante IA sui dispositivi SVBONY SVHub Omni2P e M6.
- Supportare multiple azioni: apertura di URL, esecuzione di comandi, invio di sequenze di tasti (in sviluppo), visualizzazione di notifiche.
- Supportare l'esecuzione in background (daemon) o in primo piano (con log).
- Avvio automatico all'avvio del sistema (configurabile, e auto-configurazione al primo avvio se abilitato).
- Controllo istanza singola in modalità background.
- Arresto sicuro tramite segnali di sistema (Ctrl+C, SIGTERM).
- Configurazione e registrazione dettagliate.
- Supporto notifiche multipiattaforma (Windows Toast, macOS osascript).

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
| `-r, --regenerate-config` | Ripristinare i file di configurazione ai valori di fabbrica. Se esiste una configurazione di sistema, verrà eseguito un backup come config.json.old prima della sostituzione. Tutti i file config.json verranno ripristinati ai valori predefiniti. |
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

# Ripristinare la configurazione predefinita
./target/release/svbony-ai-assistant --regenerate-config
```

## Tipi di azione
- `OpenUrl`: Apri un URL nel browser predefinito.
- `RunCommand`: Esegui un comando di sistema con argomenti opzionali.
- `SendKeys`: (In sviluppo) Simula la pressione di tasti.
- `ShowNotification`: Mostra una notifica di sistema con titolo e messaggio.

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
  },
  "version": "0.1.0"
}
```

### Compatibilità della versione
Il file di configurazione include un campo `version` che corrisponde alla versione dell'applicazione. Quando aggiorni l'applicazione:
- Se la versione del file di configurazione non corrisponde alla versione dell'applicazione, l'applicazione automaticamente:
  1. Esegue un backup della configurazione esistente come `config.json.old`
  2. Crea un nuovo file di configurazione con le impostazioni di fabbrica
- Questo garantisce la compatibilità tra la tua configurazione e la versione dell'applicazione
- Le vecchie impostazioni possono essere trovate nel file di backup se necessario

## Configurazione dell'avvio automatico
- Se `auto_start` è abilitato nella configurazione, l'applicazione tenterà di configurare l'avvio automatico al primo avvio.
- Puoi anche abilitare/disabilitare manualmente l'avvio automatico tramite `--enable-autostart` e `--disable-autostart`.

## Notifiche
- **Windows**: Usa notifiche Toast (PowerShell), con alternativa alle notifiche balloon.
- **macOS**: Usa `osascript` per le notifiche di sistema.

## Risoluzione dei problemi
- **Registrazione**: Puoi utilizzare l'argomento `--verbose` per abilitare la registrazione dettagliata per una migliore risoluzione dei problemi.
- **Controllo istanza singola**: Se l'applicazione non riesce ad avviarsi in modalità background, potrebbe essere perché un'altra istanza è già in esecuzione. Puoi utilizzare l'argomento `--foreground` per avviare più istanze per il debugging.
- **Problemi del file di configurazione**: Se ci sono problemi con il file di configurazione, puoi provare a eliminare il file di configurazione e riavviare l'applicazione. L'applicazione utilizzerà la configurazione predefinita e ricreerà il file di configurazione.

## Contributo
Se desideri contribuire al progetto Assistente IA SVBONY, segui questi passaggi:
1. Clona il repository del codice.
2. Crea un nuovo branch.
3. Apporta modifiche e test.
4. Invia una pull request.

## Licenza
Questo progetto è concesso in licenza sotto la Licenza MIT. Per i dettagli, consulta il file [LICENSE](LICENSE).