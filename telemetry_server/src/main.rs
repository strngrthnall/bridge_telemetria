use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, Result as IoResult, stdin},
    net::{SocketAddr, TcpListener, TcpStream},
    fmt,
    thread,
    process::Command,
};

// LocalHost IP for Tests
const DEFAULT_ADDRESS: &str = "127.0.0.1:8080";
// const DEFAULT_ADDRESS: &str = "0.0.0.0:8080";
const BUFFER_SIZE: usize = 4096;

type TelemetryResult<T> = IoResult<T>;

/// Níveis de log para diferentes tipos de mensagens
#[derive(Debug, Clone, Copy)]
enum LogLevel {
    Info,
    Warning,
    Error,
    Success,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (emoji, name) = match self {
            LogLevel::Info => ("ℹ️", "INFO"),
            LogLevel::Warning => ("⚠️", "WARN"),
            LogLevel::Error => ("❌", "ERROR"),
            LogLevel::Success => ("✅", "SUCCESS"),
        };
        write!(f, "{} {}", emoji, name)
    }
}

fn log(level: LogLevel, message: &str) {
    match level {
        LogLevel::Error => eprintln!("{}: {}", level, message),
        _ => println!("{}: {}", level, message),
    }
}

/// Comandos interativos disponíveis no servidor
#[derive(Debug, Clone, Copy)]
enum ServerCommand {
    OpenEdge,
    Help,
    Quit,
}

impl ServerCommand {
    /// Tenta parsear uma string para um comando
    fn from_input(input: &str) -> Option<Self> {
        match input.trim().to_uppercase().as_str() {
            "E" => Some(ServerCommand::OpenEdge),
            "H" | "HELP" => Some(ServerCommand::Help),
            "Q" | "QUIT" | "EXIT" => Some(ServerCommand::Quit),
            _ => None,
        }
    }

    /// Executa o comando
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
        }
    }

    /// Abre o navegador Microsoft Edge
    fn open_edge() -> IoResult<()> {
        log(LogLevel::Info, "Abrindo Microsoft Edge...");
        
        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(&["/C", "start", "msedge"])
                .spawn()?;
        }
        
        #[cfg(target_os = "linux")]
        {
            Command::new("microsoft-edge")
                .spawn()?;
        }
        
        #[cfg(target_os = "macos")]
        {
            Command::new("open")
                .args(&["-a", "Microsoft Edge"])
                .spawn()?;
        }
        
        log(LogLevel::Success, "Microsoft Edge aberto com sucesso!");
        Ok(())
    }

    /// Mostra menu de ajuda com comandos disponíveis
    fn show_help() {
        println!("\n{}", "=".repeat(50));
        println!("📋 COMANDOS DISPONÍVEIS");
        println!("{}", "=".repeat(50));
        println!("E        - Abrir Microsoft Edge");
        println!("H, HELP  - Mostrar esta ajuda");
        println!("Q, QUIT  - Encerrar servidor");
        println!("{}", "=".repeat(50));
    }
}

fn main() -> TelemetryResult<()> {
    let server = TelemetryServer::new(DEFAULT_ADDRESS)?;
    server.run()
}

struct TelemetryServer {
    listener: TcpListener,
    address: String,
}

impl TelemetryServer {
    fn new(address: &str) -> TelemetryResult<Self> {
        let listener = TcpListener::bind(address)?;
        
        Ok(Self {
            listener,
            address: address.to_string(),
        })
    }

    fn run(&self) -> TelemetryResult<()> {
        self.print_startup_message();
        
        // Inicia thread para processar comandos do usuário
        Self::start_command_handler();
        
        loop {
            match self.accept_connection() {
                Ok(_) => {
                    log(LogLevel::Info, "Aguardando nova conexão...");
                }
                Err(e) => {
                    log(LogLevel::Error, &format!("Erro ao aceitar conexão: {}", e));
                    // Continua executando mesmo com erro
                }
            }
        }
    }

    fn start_command_handler() {
        thread::spawn(|| {
            let stdin = stdin();
            let mut input = String::new();
            
            loop {
                input.clear();
                
                // Lê entrada do usuário
                if let Ok(_) = stdin.read_line(&mut input) {
                    if let Some(command) = ServerCommand::from_input(&input) {
                        if let Err(e) = command.execute() {
                            log(LogLevel::Error, &format!("Erro ao executar comando: {}", e));
                        }
                    } else if !input.trim().is_empty() {
                        log(LogLevel::Warning, "Comando não reconhecido. Digite 'H' para ajuda.");
                    }
                }
            }
        });
    }

    fn print_startup_message(&self) {
        println!("🚀 Servidor de Telemetria iniciado");
        println!("📡 Ouvindo em: {}", self.address);
        println!("{}", "=".repeat(50));
        println!("⌨️  COMANDOS INTERATIVOS:");
        println!("  E - Abrir Microsoft Edge");
        println!("  H - Mostrar ajuda");
        println!("  Q - Sair");
        println!("{}", "=".repeat(50));
        println!("⏹️  Aguardando conexões...\n");
    }

    fn accept_connection(&self) -> TelemetryResult<()> {
        let (socket, addr) = self.listener.accept()?;
        log(LogLevel::Success, &format!("Cliente conectado: {}", addr));
        
        let mut connection = ClientConnection::new(socket, addr);
        match connection.handle_client() {
            Ok(_) => {
                log(LogLevel::Info, "Conexão processada com sucesso");
                Ok(())
            }
            Err(e) => {
                log(LogLevel::Warning, &format!("Erro durante conexão com {}: {}", addr, e));
                Ok(()) // Não propaga erro para manter servidor rodando
            }
        }
    }
}

struct ClientConnection {
    reader: BufReader<TcpStream>,
    addr: SocketAddr,
    line_buffer: String,
}

impl ClientConnection {
    fn new(socket: TcpStream, addr: SocketAddr) -> Self {
        Self {
            reader: BufReader::with_capacity(BUFFER_SIZE, socket),
            addr,
            line_buffer: String::with_capacity(512),
        }
    }

    fn handle_client(&mut self) -> TelemetryResult<()> {
        loop {
            match self.read_telemetry_data() {
                Ok(Some(metrics)) => {
                    self.display_telemetry(&metrics);
                }
                Ok(None) => {
                    log(LogLevel::Info, &format!("Cliente {} desconectou", self.addr));
                    break;
                }
                Err(e) => {
                    log(LogLevel::Error, &format!("Erro ao ler dados do cliente {}: {}", self.addr, e));
                    break;
                }
            }
        }
        
        Ok(())
    }

    fn read_telemetry_data(&mut self) -> TelemetryResult<Option<HashMap<String, f32>>> {
        // Limpa o buffer para reutilização
        self.line_buffer.clear();
        
        // Lê até encontrar newline (ou EOF)
        match self.reader.read_line(&mut self.line_buffer) {
            Ok(0) => Ok(None), // Cliente desconectou
            Ok(_) => {
                // Remove whitespace das extremidades
                let trimmed = self.line_buffer.trim();
                
                if trimmed.is_empty() {
                    // Linha vazia, continua lendo
                    return self.read_telemetry_data();
                }
                
                // Parse direto da string, sem clonar
                match serde_json::from_str::<HashMap<String, f32>>(trimmed) {
                    Ok(metrics) => Ok(Some(metrics)),
                    Err(e) => {
                        log(LogLevel::Warning, &format!("Erro ao processar JSON: {}", e));
                        log(LogLevel::Info, &format!("Dados recebidos: {}", trimmed));
                        // Continua tentando ler próxima linha ao invés de desconectar
                        self.read_telemetry_data()
                    }
                }
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::InvalidData {
                    log(LogLevel::Warning, &format!("Dados UTF-8 inválidos: {}", e));
                    // Continua tentando ler ao invés de desconectar
                    self.read_telemetry_data()
                } else {
                    Err(e)
                }
            }
        }
    }

    fn display_telemetry(&self, metrics: &HashMap<String, f32>) {
        self.clear_screen();
        
        println!("📊 TELEMETRIA EM TEMPO REAL");
        println!("🔗 Cliente: {}", self.addr);
        println!("{}", "=".repeat(50));
        
        if metrics.is_empty() {
            println!("⚠️  Nenhuma métrica recebida");
        } else {
            for (metric_name, value) in metrics {
                self.format_and_print_metric(metric_name, *value);
            }
        }
        
        println!("{}", "=".repeat(50));
        println!("⏹️  Pressione Ctrl+C para sair");
    }

    fn format_and_print_metric(&self, name: &str, value: f32) {
        match name.to_uppercase().as_str() {
            "CPU" => println!("🖥️  CPU: {:.1}%", value),
            "MEM" | "MEMORY" => {
                // sysinfo retorna memória em KB (kilobytes)
                // Conversão correta: KB -> MB -> GB
                let kb = value;
                if kb >= 1_048_576.0 {
                    // >= 1024 MB (1 GB)
                    println!("💾 Memória: {:.2} GB", kb / 1_048_576.0);
                } else if kb >= 1_024.0 {
                    // >= 1 MB
                    println!("💾 Memória: {:.2} MB", kb / 1_024.0);
                } else {
                    println!("💾 Memória: {:.2} KB", kb);
                }
            }
            "DISK" | "STORAGE" => println!("💿 Disco: {:.1}%", value),
            "NETWORK" | "NET" => println!("🌐 Rede: {:.2} MB/s", value),
            "TEMPERATURE" | "TEMP" => println!("🌡️  Temperatura: {:.1}°C", value),
            _ => println!("📈 {}: {:.2}", name, value),
        }
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }
}

