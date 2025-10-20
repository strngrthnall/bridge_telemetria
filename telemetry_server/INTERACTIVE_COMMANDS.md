# 🎮 Comandos Interativos - Guia do Desenvolvedor

## 📋 Visão Geral

O servidor de telemetria possui um sistema de comandos interativos que permite executar ações enquanto o servidor está rodando, sem interromper o processamento de telemetria.

## 🏗️ Arquitetura

### Threading Model

O sistema utiliza duas threads:

1. **Main Thread**: Aceita conexões TCP e processa telemetria
2. **Command Handler Thread**: Lê e processa comandos do stdin

```rust
TelemetryServer::run()
    ├─> Main Thread (loop)
    │   ├─> accept_connection()
    │   └─> ClientConnection::handle_client()
    │
    └─> Command Handler Thread (spawn)
        ├─> stdin.read_line()
        ├─> ServerCommand::from_input()
        └─> command.execute()
```

### Enum ServerCommand

Todos os comandos são definidos no enum `ServerCommand`:

```rust
#[derive(Debug, Clone, Copy)]
enum ServerCommand {
    OpenEdge,    // Abre Microsoft Edge
    Help,        // Mostra ajuda
    Quit,        // Encerra servidor
}
```

## 🔧 Como Adicionar Novos Comandos

### Passo 1: Adicionar Variante ao Enum

```rust
enum ServerCommand {
    OpenEdge,
    Help,
    Quit,
    OpenChrome,      // ← Novo comando
    ShowStats,       // ← Outro exemplo
}
```

### Passo 2: Atualizar Parser

Adicione o mapeamento de entrada no método `from_input()`:

```rust
fn from_input(input: &str) -> Option<Self> {
    match input.trim().to_uppercase().as_str() {
        "E" => Some(ServerCommand::OpenEdge),
        "H" | "HELP" => Some(ServerCommand::Help),
        "Q" | "QUIT" | "EXIT" => Some(ServerCommand::Quit),
        "C" | "CHROME" => Some(ServerCommand::OpenChrome),  // ← Novo
        "S" | "STATS" => Some(ServerCommand::ShowStats),    // ← Novo
        _ => None,
    }
}
```

### Passo 3: Implementar Execução

Adicione o case no método `execute()`:

```rust
fn execute(&self) -> IoResult<()> {
    match self {
        ServerCommand::OpenEdge => Self::open_edge(),
        ServerCommand::Help => {
            Self::show_help();
            Ok(())
        }
        ServerCommand::Quit => {
            log(LogLevel::Info, "Encerrando servidor...");
            std::process::exit(0);
        }
        ServerCommand::OpenChrome => Self::open_chrome(),    // ← Novo
        ServerCommand::ShowStats => Self::show_stats(),      // ← Novo
    }
}
```

### Passo 4: Implementar Método do Comando

```rust
/// Abre o navegador Google Chrome
fn open_chrome() -> IoResult<()> {
    log(LogLevel::Info, "Abrindo Google Chrome...");
    
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "start", "chrome"])
            .spawn()?;
    }
    
    #[cfg(target_os = "linux")]
    {
        Command::new("google-chrome")
            .spawn()?;
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .args(&["-a", "Google Chrome"])
            .spawn()?;
    }
    
    log(LogLevel::Success, "Google Chrome aberto com sucesso!");
    Ok(())
}

/// Mostra estatísticas do servidor
fn show_stats() -> IoResult<()> {
    println!("\n{}", "=".repeat(50));
    println!("📊 ESTATÍSTICAS DO SERVIDOR");
    println!("{}", "=".repeat(50));
    println!("⏱️  Uptime: ... (implementar)");
    println!("📨 Mensagens processadas: ... (implementar)");
    println!("👥 Conexões ativas: ... (implementar)");
    println!("{}", "=".repeat(50));
    Ok(())
}
```

### Passo 5: Atualizar Menu de Ajuda

```rust
fn show_help() {
    println!("\n{}", "=".repeat(50));
    println!("📋 COMANDOS DISPONÍVEIS");
    println!("{}", "=".repeat(50));
    println!("E        - Abrir Microsoft Edge");
    println!("C        - Abrir Google Chrome");     // ← Novo
    println!("S        - Mostrar estatísticas");    // ← Novo
    println!("H, HELP  - Mostrar esta ajuda");
    println!("Q, QUIT  - Encerrar servidor");
    println!("{}", "=".repeat(50));
}
```

### Passo 6: Atualizar Startup Message

```rust
fn print_startup_message(&self) {
    println!("🚀 Servidor de Telemetria iniciado");
    println!("📡 Ouvindo em: {}", self.address);
    println!("{}", "=".repeat(50));
    println!("⌨️  COMANDOS INTERATIVOS:");
    println!("  E - Abrir Microsoft Edge");
    println!("  C - Abrir Google Chrome");     // ← Novo
    println!("  S - Mostrar estatísticas");    // ← Novo
    println!("  H - Mostrar ajuda");
    println!("  Q - Sair");
    println!("{}", "=".repeat(50));
    println!("⏹️  Aguardando conexões...\n");
}
```

## 📝 Boas Práticas

### ✅ DO's

1. **Use logging apropriado**: `log(LogLevel::Info, ...)` para feedback
2. **Trate erros**: Sempre retorne `IoResult<()>`
3. **Suporte multi-plataforma**: Use `#[cfg(target_os = "...")]`
4. **Documente**: Adicione comentários `///` para cada método
5. **Teste**: Verifique em Windows, Linux e macOS se possível
6. **Nomenclatura clara**: Use nomes descritivos (OpenChrome, não OC)

### ❌ DON'Ts

1. **Não bloqueie**: Comandos devem ser rápidos (use `spawn()` para processos)
2. **Não panic**: Use `Result` para propagação de erros
3. **Não modifique estado sem sync**: Use `Arc<Mutex<T>>` se necessário
4. **Não use unsafe**: Mantenha código seguro
5. **Não ignore erros**: Sempre logue ou propague

## 🎯 Exemplos de Comandos Úteis

### Abrir Aplicações

```rust
fn open_notepad() -> IoResult<()> {
    Command::new("notepad").spawn()?;
    Ok(())
}

fn open_calculator() -> IoResult<()> {
    #[cfg(target_os = "windows")]
    Command::new("calc").spawn()?;
    Ok(())
}
```

### Comandos do Sistema

```rust
fn clear_screen() -> IoResult<()> {
    print!("\x1B[2J\x1B[1;1H");
    Ok(())
}

fn show_time() -> IoResult<()> {
    use std::time::SystemTime;
    let now = SystemTime::now();
    println!("⏰ {}", now); // Melhorar formatação
    Ok(())
}
```

### Controle do Servidor

```rust
fn pause_telemetry() -> IoResult<()> {
    // Implementar com Arc<AtomicBool>
    log(LogLevel::Warning, "Telemetria pausada");
    Ok(())
}

fn resume_telemetry() -> IoResult<()> {
    // Implementar com Arc<AtomicBool>
    log(LogLevel::Success, "Telemetria retomada");
    Ok(())
}
```

## 🔒 Considerações de Segurança

1. **Validação de entrada**: Sempre use `trim()` e `to_uppercase()`
2. **Limite de comandos**: Não execute comandos arbitrários do shell
3. **Sanitização**: Evite injection através de input do usuário
4. **Privilégios**: Não execute comandos que requeiram elevação

## 📚 Referências

- [std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html)
- [std::thread::spawn](https://doc.rust-lang.org/std/thread/fn.spawn.html)
- [std::io::Result](https://doc.rust-lang.org/std/io/type.Result.html)

## 🤝 Contribuindo

Para adicionar novos comandos:

1. Fork o repositório
2. Crie uma branch: `git checkout -b feature/new-command`
3. Implemente seguindo este guia
4. Teste em múltiplas plataformas
5. Atualize documentação
6. Submeta Pull Request
