# Rust CLI Program

## Inhaltsverzeichnis
1. [Einleitung](#einleitung)
2. [Voraussetzungen](#voraussetzungen)
3. [Kompilieren des Programms](#kompilieren-des-programms)
4. [Ausführen des Programms](#ausführen-des-programms)
5. [Architektur des Programms](#architektur-des-programms)
---

## Einleitung
Dieses Projekt ist ein Kommandozeilenprogramm (CLI) geschrieben in Rust.
Es kann 4diac's DTP-Dateien zu ROS 2's MSG-Dateien konvertieren und umgekehrt.
Zusätzlich kann es für Debuggingzwecke einzelne Dateien lesen und die gelesen
Datenstruktur ausgeben.
Dieses Dokument zeigt, wie das Programm kompiliert, ausgeführt und wie seine Architektur strukturiert ist.

---

## Voraussetzungen
Bevor Sie das Programm kompilieren und ausführen, stellen Sie sicher, dass folgende Software auf Ihrem System installiert ist:
- [Rust](https://www.rust-lang.org/) (mindestens Version 1.80.1)
    - Installieren Sie Rust durch Ausführen des folgenden Befehls:
      ```bash
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      ```

---

## Kompilieren des Programms
Um das Programm zu kompilieren, folgen Sie diesen Schritten:

1. Klonen Sie das Repository:
    ```bash
    git clone https://github.com/N1CEJAN/4diac-ros2-converter.git .
    cd 4diac-ros2-converter
    ```

2. Führen Sie den folgenden Befehl aus, um das Projekt zu kompilieren:
    ```bash
    cargo build --release
    ```

    - Dieser Befehl erstellt das Programm in der Release-Version. Die Binärdateien werden im Verzeichnis `target/release` abgelegt.
    - Um im Debug-Modus zu kompilieren, verwenden Sie:
    ```bash
    cargo build
    ```

---

## Ausführen des Programms
Sobald das Programm kompiliert wurde, können Sie es ausführen. Nutzen Sie dazu den folgenden Befehl:

1. Im Release-Modus:
    ```bash
    ./target/release/ros2-4diac-converter [arguments]
    ```

2. Im Debug-Modus:
    ```bash
    ./target/debug/ros2-4diac-converter [arguments]
    ```

### Beispiel
Nutzen Sie folgende Befehle um alle Testdateien zu konvertieren, inkl. Roundtrip.
Wichtig: Es muss im Rootverzeichnis ausgeführt werden um zu funktionieren!!!
```bash
cp ./target/release/ros2-4diac-converter ros2-4diac-converter
./ros2-4diac-converter test
```

---

## Architektur des Programms
Die folgenden Abbildung erklärt die Architektur des Programms anhand der Projektordnerstruktur.
```bash
├── src/
│   ├── api/                      # API-Schicht: Hier ist die API des Werkzeugs implementiert
│   │   └── cli.rs                # Implementiert die Befehle, Argumente und -h Support für ein CLI
│   │
│   ├── business/                 # Business-Schicht: Hier ist die Problemlösung implementiert
│   │   ├── dtp_converter/        # In diesem Modul ist die Konvertierung von DTP-Dateien implementiert
│   │   │   ├── dtp_reader.rs     # Liest DTP-DTO von DTP-Datei
│   │   │   ├── converter  # Konvertiert DTP-DTOs zu MSG-DTOs
│   │   │   └── msg_writer.rs     # Schreibt MSG-DTO in MSG-Datei
│   │   ├── msg_converter/        # In diesem Modul ist die Konvertierung von MSG-Dateien implementiert       
│   │   │   ├── msg_reader.rs     # Liest MSG-DTO von MSG-Datei
│   │   │   ├── converter  # Konvertiert MSG-DTO zu DTP-DTOs
│   │   │   └── dtp_writer.rs     # Schreibt DTP-DTO in DTP-Datei
│   │   ├── handler.rs            # Implementiert die API der Problemlösung
│   │   └── error.rs              # Implementiert die Error-Klasse der Problemlösung 
│   │
│   ├── core/                     # Core-Schicht: Hier ist die Abstraktion der Entitäten implementiert
│   │   ├── dtp.rs                # Implementiert ein DTO für eine DTP-Datei
│   │   └── msg.rs                # Implementiert ein DTO für eine MSG-Datei
│   │
│   └── main.rs                   # Einstiegspunkt der Anwendung
│
├── test/                         # Enthält Testdateien, Konvertierungsergebnisse und generierten Code
│   ├── 0-dtp/                    # Selbsterstellte DTP-Dateien
│   ├── 0-msg/                    # Selbsterstellte MSG-Dateien
│   ├── 1-dtp/                    # Konvertierungsergebnisse von selbsterstellten DTP-Dateien des "test"-Befehl 
│   ├── 1-msg/                    # Konvertierungsergebnisse von selbsterstellten MSG-Dateien des "test"-Befehl
│   ├── 2-dtp/                    # Roundtripergebnisse von selbsterstellten DTP-Dateien des "test"-Befehl
│   └── 2-msg/                    # Roundtripergebnisse von selbsterstellten MSG-Dateien des "test"-Befehl
│
├── target                        # Automatisch erstelltes Verzeichnis mit den kompilierten Dateien
├── Cargo.toml                    # Konfigurationsdatei für Cargo (Projektabhängigkeiten, Metadaten)
├── Cargo.lock                    # Automatisch generierte Datei, die genaue Versionen der Abhängigkeiten festhält
├── .gitignore                    # Liste von Dateien und Verzeichnissen, die nicht in die Versionskontrolle (Git) aufgenommen werden sollen
└── README.md                     # Dokumentation des Projekts, enthält Anweisungen zu Kompilierung, Ausführung und Architektur
```
