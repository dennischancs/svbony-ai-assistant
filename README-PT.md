# Guia de Uso do Assistente IA SVBONY

🇺🇸 [English](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README.md) | 🇫🇷 [Français](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-FR.md) | 🇩🇪 [Deutsch](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-DE.md) | 🇮🇹 [Italiano](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-IT.md) | 🇪🇸 [Español](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-ES.md) | 🇷🇺 [Русский](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-RU.md) | 🇵🇹 [Português](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-PT.md) | 🇯🇵 [日本語](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-JP.md) | 🇨🇳 [简体中文](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-CN.md)

### Início Rápido

1. Baixe o binário apropriado a partir das [Releases do GitHub](https://github.com/dennischancs/svbony-ai-assistant/releases/latest) para sua plataforma, por exemplo: macOS (x86_64, aarch64/Apple Silicon), Windows (x86_64)
2. Extraia o arquivo compactado
3. Execute o binário

## Dispositivos Suportados
- SVBONY SVHub Omni2P (PID: 0x5053)
- SVBONY SVHub M6 (PID: 0x364d)

## Introdução
O Assistente IA SVBONY é uma ferramenta projetada para monitorar eventos de pressão do botão IA em dispositivos SVBONY suportados e executar ações configuradas como abrir URLs, executar comandos, enviar pressionamentos de tecla (em desenvolvimento) ou exibir notificações. Esta ferramenta suporta sistemas Windows, macOS, e pode ser configurada para iniciar automaticamente na inicialização do sistema.

## Recursos
- Monitorar eventos de pressão do botão IA nos dispositivos SVBONY SVHub Omni2P e M6.
- Suportar múltiplas ações: abrir URLs, executar comandos, enviar pressionamentos de tecla (em desenvolvimento), exibir notificações.
- Suportar execução em segundo plano (daemon) ou primeiro plano (com logs).
- Inicialização automática na inicialização do sistema (configurável, e auto-configuração na primeira execução se habilitado).
- Verificação de instância única no modo background.
- Encerramento seguro via sinais do sistema (Ctrl+C, SIGTERM).
- Configuração e registro detalhados.
- Suporte a notificações multiplataforma (Windows Toast, macOS osascript).

## Instalação e Uso

### Instalação a partir de Binários Pré-compilados
Se você não quiser compilar o código por conta própria, pode baixar diretamente os binários pré-compilados e seguir estes passos:
1. Baixe o arquivo binário adequado para o seu sistema operacional.
2. Extraia o arquivo baixado.
3. Abra um terminal ou prompt de comando e navegue até o diretório extraído.
4. Execute o comando `svbony-ai-assistant` para iniciar o programa.

### Compilação a partir do Código Fonte
Se você quiser compilar o programa a partir do código fonte, pode seguir estes passos:

#### Preparação do Ambiente
Certifique-se de ter instalado o ambiente de desenvolvimento Rust. Se não tiver, você pode baixar e instalar a partir do [site oficial do Rust](https://www.rust-lang.org/tools/install).

#### Clonar o Repositório de Código
```bash
git clone https://github.com/dennischancs/svbony-ai-assistant.git
cd svbony-ai-assistant
```

#### Compilar o Programa
```bash
cargo build --release
```
Após a compilação, o arquivo executável estará localizado no diretório `target/release`.

### Executar o Programa
Após a compilação, você pode executar o programa usando o seguinte comando:
```bash
./target/release/svbony-ai-assistant
```

## Argumentos da Linha de Comando
| Argumento | Descrição |
| ---- | ---- |
| `-f, --foreground` | Executar no modo primeiro plano, exibindo todas as mensagens de log no console e mantendo o aplicativo conectado ao terminal. Adequado para depuração ou monitoramento manual. |
| `-b, --background` | Executar no modo segundo plano como um processo daemon. O aplicativo se desconectará do terminal e executará silenciosamente em segundo plano. Este é o comportamento padrão quando lançado a partir de uma GUI. |
| `--enable-autostart` | Configurar o aplicativo para iniciar automaticamente quando o sistema inicializar. Isso criará as entradas de inicialização automática necessárias para o seu sistema operacional. |
| `--disable-autostart` | Remover o aplicativo da inicialização automática. O aplicativo não iniciará automaticamente quando o sistema inicializar. |
| `-c, --show-config` | Exibir o caminho e conteúdo do arquivo de configuração atual, depois sair sem iniciar o serviço de monitoramento. |
| `-r, --regenerate-config` | Redefinir os arquivos de configuração para os padrões de fábrica. Se existir uma configuração do sistema, será feito backup como config.json.old antes de ser substituída. Todos os arquivos config.json serão redefinidos para os padrões de fábrica. |
| `-v, --verbose` | Habilitar saída de registro detalhada. Isso mostrará mensagens de depuração e informações detalhadas sobre a comunicação do dispositivo. |
| `-q, --quiet` | Executar em modo silencioso, suprimindo toda saída de log exceto mensagens de erro. |
| `-V, --version` | Exibir informações da versão. |

### Exemplos de Uso
```bash
# Executar no modo primeiro plano com registro detalhado
./target/release/svbony-ai-assistant --foreground --verbose

# Executar no modo segundo plano e habilitar inicialização automática
./target/release/svbony-ai-assistant --enable-autostart
./target/release/svbony-ai-assistant

# Exibir a configuração atual
./target/release/svbony-ai-assistant --show-config

# Regenerar arquivos de configuração padrão
./target/release/svbony-ai-assistant --regenerate-config
```

## Tipos de Ação
- `OpenUrl`: Abrir uma URL no navegador padrão.
- `RunCommand`: Executar um comando do sistema com argumentos opcionais.
- `SendKeys`: (Em desenvolvimento) Simular pressionamentos de tecla.
- `ShowNotification`: Exibir uma notificação do sistema com título e mensagem.

## Arquivo de Configuração
O arquivo de configuração é usado para definir o comportamento e ações do aplicativo. O arquivo de configuração pode estar localizado nos seguintes locais:
- **Windows**: `%APPDATA%\SVBONY-AI-Assistant\config.json`
- **macOS**: `~/Library/Application Support/SVBONY-AI-Assistant/config.json`
- Ou `config.json` no mesmo diretório que o arquivo executável

Se o arquivo de configuração não existir, o aplicativo usará a configuração padrão e criará um arquivo de configuração no local acima.

### Exemplo de Arquivo de Configuração
```json
{
  "actions": [
    {
      "name": "Abrir app.notta.ai",
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
      "name": "Mostrar Notificação do Assistente IA",
      "action_type": "ShowNotification",
      "parameters": {
        "url": null,
        "command": null,
        "args": null,
        "keys": null,
        "message": "Assistente IA ativado!",
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

### Compatibilidade de Versão
O arquivo de configuração inclui um campo `version` que corresponde à versão do aplicativo. Ao atualizar o aplicativo:
- Se a versão do arquivo de configuração não corresponder à versão do aplicativo, o aplicativo irá automaticamente:
  1. Fazer backup da configuração existente como `config.json.old`
  2. Criar um novo arquivo de configuração com as configurações padrão de fábrica
- Isso garante a compatibilidade entre sua configuração e a versão do aplicativo
- As configurações antigas podem ser encontradas no arquivo de backup, se necessário

## Configuração de Inicialização Automática
- Se `auto_start` estiver habilitado na configuração, o aplicativo tentará configurar a inicialização automática na primeira execução.
- Você também pode habilitar/desabilitar manualmente a inicialização automática usando `--enable-autostart` e `--disable-autostart`.

## Notificações
- **Windows**: Usa notificações Toast (PowerShell), com alternativa para notificações balloon.
- **macOS**: Usa `osascript` para notificações do sistema.

## Solução de Problemas
- **Registro**: Você pode usar o argumento `--verbose` para habilitar registro detalhado para melhor solução de problemas.
- **Verificação de Instância Única**: Se o aplicativo falhar ao iniciar no modo background, pode ser porque outra instância já está em execução. Você pode usar o argumento `--foreground` para iniciar múltiplas instâncias para depuração.
- **Problemas do Arquivo de Configuração**: Se houver problemas com o arquivo de configuração, você pode tentar deletar o arquivo de configuração e reiniciar o aplicativo. O aplicativo usará a configuração padrão e recriará o arquivo de configuração.

## Contribuição
Se você quiser contribuir para o projeto Assistente IA SVBONY, siga estes passos:
1. Clone o repositório de código.
2. Crie uma nova branch.
3. Faça modificações e testes.
4. Envie um pull request.

## Licença
Este projeto está licenciado sob a Licença MIT. Para detalhes, consulte o arquivo [LICENSE](LICENSE).