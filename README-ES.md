# Guía de uso del Asistente IA SVBONY

🇺🇸 [English](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README.md) | 🇫🇷 [Français](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-FR.md) | 🇩🇪 [Deutsch](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-DE.md) | 🇮🇹 [Italiano](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-IT.md) | 🇪🇸 [Español](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-ES.md) | 🇷🇺 [Русский](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-RU.md) | 🇵🇹 [Português](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-PT.md) | 🇯🇵 [日本語](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-JP.md) | 🇨🇳 [简体中文](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-CN.md)

## Descargar las últimas versiones
[Descargar desde GitHub Releases](https://github.com/dennischancs/svbony-ai-assistant/releases/latest)

## Introducción
El Asistente IA SVBONY es una herramienta diseñada para monitorear los eventos de presión del botón IA en el dispositivo SVBONY SVHub Omni2P y ejecutar acciones configuradas como abrir URLs, ejecutar comandos o mostrar notificaciones. Esta herramienta es compatible con sistemas Windows y macOS y puede configurarse para iniciarse automáticamente al arrancar el sistema.

## Características
- Monitorear eventos de presión del botón IA en el dispositivo SVBONY SVHub Omni2P.
- Soportar múltiples acciones, incluyendo abrir URLs, ejecutar comandos, enviar pulsaciones de teclas y mostrar notificaciones.
- Soportar ejecución en segundo plano o primer plano.
- Soportar inicio automático al arrancar el sistema.
- Configuración y registro detallados.

## Instalación y uso

### Instalación desde binarios precompilados
Si no deseas compilar el código por ti mismo, puedes descargar directamente los binarios precompilados y seguir estos pasos:
1. Descarga el archivo binario adecuado para tu sistema operativo.
2. Extrae el archivo descargado.
3. Abre una terminal o línea de comandos y navega al directorio extraído.
4. Ejecuta el comando `svbony-ai-assistant` para iniciar el programa.

### Compilación desde código fuente
Si deseas compilar el programa desde código fuente, puedes seguir estos pasos:

#### Preparación del entorno
Asegúrate de tener instalado el entorno de desarrollo de Rust. Si no lo tienes, puedes descargarlo e instalarlo desde el [sitio web oficial de Rust](https://www.rust-lang.org/tools/install).

#### Clonar el repositorio de código
```bash
git clone https://github.com/dennischancs/svbony-ai-assistant.git
cd svbony-ai-assistant
```

#### Compilar el programa
```bash
cargo build --release
```
Después de la compilación, el archivo ejecutable estará ubicado en el directorio `target/release`.

### Ejecutar el programa
Después de la compilación, puedes ejecutar el programa usando el siguiente comando:
```bash
./target/release/svbony-ai-assistant
```

## Argumentos de línea de comandos
| Argumento | Descripción |
| ---- | ---- |
| `-f, --foreground` | Ejecutar en modo primer plano, mostrando todos los mensajes de registro en la consola y manteniendo la aplicación vinculada al terminal. Adecuado para depuración o monitoreo manual. |
| `-b, --background` | Ejecutar en modo segundo plano como proceso daemon. La aplicación se desvinculará del terminal y funcionará silenciosamente en segundo plano. Este es el comportamiento predeterminado cuando se lanza desde una GUI. |
| `--enable-autostart` | Configurar la aplicación para iniciarse automáticamente cuando el sistema arranque. Esto creará las entradas de inicio automático necesarias para tu sistema operativo. |
| `--disable-autostart` | Quitar la aplicación del inicio automático. La aplicación no se iniciará automáticamente cuando el sistema arranque. |
| `-c, --show-config` | Mostrar la ruta y el contenido del archivo de configuración actual, luego salir sin iniciar el servicio de monitoreo. |
| `-v, --verbose` | Habilitar salida de registro detallada. Esto mostrará mensajes de depuración e información detallada sobre la comunicación del dispositivo. |
| `-q, --quiet` | Ejecutar en modo silencioso, suprimiendo toda salida de registro excepto mensajes de error. |
| `-V, --version` | Mostrar información de versión. |

### Ejemplos de uso
```bash
# Ejecutar en modo primer plano con registro detallado
./target/release/svbony-ai-assistant --foreground --verbose

# Ejecutar en modo segundo plano y habilitar inicio automático
./target/release/svbony-ai-assistant --enable-autostart
./target/release/svbony-ai-assistant

# Mostrar la configuración actual
./target/release/svbony-ai-assistant --show-config
```

## Archivo de configuración
El archivo de configuración se utiliza para definir el comportamiento y las acciones de la aplicación. El archivo de configuración puede ubicarse en las siguientes ubicaciones:
- **Windows**: `%APPDATA%\SVBONY-AI-Assistant\config.json`
- **macOS**: `~/Library/Application Support/SVBONY-AI-Assistant/config.json`
- O `config.json` en el mismo directorio que el archivo ejecutable

Si el archivo de configuración no existe, la aplicación utilizará la configuración predeterminada y creará un archivo de configuración en la ubicación anterior.

### Ejemplo de archivo de configuración
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
      "name": "Mostrar notificación de Asistente IA",
      "action_type": "ShowNotification",
      "parameters": {
        "url": null,
        "command": null,
        "args": null,
        "keys": null,
        "message": "¡Asistente IA activado!",
        "title": "Asistente IA SVBONY"
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

## Configuración de inicio automático
Puedes usar los argumentos `--enable-autostart` y `--disable-autostart` para habilitar o deshabilitar la función de inicio automático de la aplicación. Por ejemplo:
```bash
# Habilitar inicio automático
./target/release/svbony-ai-assistant --enable-autostart

# Deshabilitar inicio automático
./target/release/svbony-ai-assistant --disable-autostart
```

## Solución de problemas
- **Registro**: Puedes usar el argumento `--verbose` para habilitar el registro detallado para una mejor solución de problemas.
- **Verificación de instancia única**: Si la aplicación no logra iniciarse, puede ser porque otra instancia ya está ejecutándose. Puedes usar el argumento `--foreground` para iniciar múltiples instancias para depuración.
- **Problemas del archivo de configuración**: Si hay problemas con el archivo de configuración, puedes intentar eliminar el archivo de configuración y reiniciar la aplicación. La aplicación utilizará la configuración predeterminada y recreará el archivo de configuración.

## Contribución
Si deseas contribuir al proyecto Asistente IA SVBONY, sigue estos pasos:
1. Clonar el repositorio de código.
2. Crear una nueva rama.
3. Hacer modificaciones y pruebas.
4. Enviar una pull request.

## Licencia
Este proyecto está licenciado bajo la Licencia MIT. Para más detalles, consulta el archivo [LICENSE](LICENSE).