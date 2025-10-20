use std::{
    io::{Result as IoResult, Write},
    net::TcpStream,
    thread,
    time::Duration,
    fmt,
};
use sysinfo::System;

// LocalHost IP for Tests
//const SERVER_ADDRESS: &str = "127.0.0.1:8080";
const SERVER_ADDRESS: &str = "<Server IP Here>:8080";
const TELEMETRY_INTERVAL_MS: u64 = 1000;
const JSON_BUFFER_CAPACITY: usize = 256;

type TelemetryResult<T> = IoResult<T>;

/// Tipos de hardware monitorados
#[derive(Debug, Clone, Copy)]
enum HardwareMetric {
    Cpu,
    Memory,
}

impl HardwareMetric {
    fn as_str(&self) -> &'static str {
        match self {
            HardwareMetric::Cpu => "CPU",
            HardwareMetric::Memory => "MEM",
        }
    }

    fn all() -> &'static [HardwareMetric] {
        &[HardwareMetric::Cpu, HardwareMetric::Memory]
    }
}

impl fmt::Display for HardwareMetric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

fn main() -> TelemetryResult<()> {
    let client = TelemetryClient::new(SERVER_ADDRESS)?;
    client.run()
}

struct TelemetryClient {
    connection: TcpStream,
    system: System,
    address: String,
}

impl TelemetryClient {
    fn new(address: &str) -> TelemetryResult<Self> {
        println!("🔌 Conectando ao servidor {}...", address);
        
        let connection = TcpStream::connect(address)?;
        let system = System::new_all();
        
        println!("✅ Conectado ao servidor com sucesso!");
        println!("📊 Iniciando coleta de telemetria...");
        println!("{}", "=".repeat(50));
        
        Ok(Self {
            connection,
            system,
            address: address.to_string(),
        })
    }

    fn run(mut self) -> TelemetryResult<()> {
        let mut message_count = 0u64;
        let mut json_buffer = String::with_capacity(JSON_BUFFER_CAPACITY);
        
        loop {
            match self.collect_and_send_telemetry(&mut json_buffer) {
                Ok(_) => {
                    message_count += 1;
                    if message_count % 10 == 0 {
                        println!("📤 {} mensagens enviadas", message_count);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Erro ao enviar telemetria: {}", e);
                    eprintln!("🔄 Tentando reconectar...");
                    
                    if self.try_reconnect().is_err() {
                        eprintln!("💥 Falha na reconexão. Encerrando cliente.");
                        return Err(e);
                    }
                    
                    message_count = 0;
                }
            }
            
            thread::sleep(Duration::from_millis(TELEMETRY_INTERVAL_MS));
        }
    }

    fn collect_and_send_telemetry(&mut self, json_buffer: &mut String) -> TelemetryResult<()> {
        // Limpa buffer para reutilização
        json_buffer.clear();
        
        // Coleta métricas
        self.build_telemetry_json(json_buffer);
        
        // Adiciona delimitador newline para o servidor
        json_buffer.push('\n');
        
        // Envia dados
        self.send_data(json_buffer.as_bytes())
    }

    fn build_telemetry_json(&mut self, buffer: &mut String) {
        buffer.push('{');
        
        let metrics = HardwareMetric::all();
        for (idx, metric) in metrics.iter().enumerate() {
            let value = self.collect_metric(metric);
            
            // Formato: "METRIC": value
            buffer.push('"');
            buffer.push_str(metric.as_str());
            buffer.push_str("\": ");
            buffer.push_str(&value.to_string());
            
            // Adiciona vírgula se não for o último
            if idx < metrics.len() - 1 {
                buffer.push_str(", ");
            }
        }
        
        buffer.push('}');
    }

    fn collect_metric(&mut self, metric: &HardwareMetric) -> f32 {
        match metric {
            HardwareMetric::Cpu => self.get_cpu_usage(),
            HardwareMetric::Memory => self.get_memory_usage(),
        }
    }

    fn get_cpu_usage(&mut self) -> f32 {
        self.system.refresh_cpu_usage();
        
        // Calcula média de todas as CPUs ao invés de apenas a primeira
        let cpus = self.system.cpus();
        if cpus.is_empty() {
            return 0.0;
        }
        
        let total: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum();
        total / cpus.len() as f32
    }

    fn get_memory_usage(&mut self) -> f32 {
        self.system.refresh_memory();
        self.system.used_memory() as f32
    }

    fn send_data(&mut self, data: &[u8]) -> TelemetryResult<()> {
        self.connection.write_all(data)?;
        self.connection.flush() // Garante que dados são enviados imediatamente
    }

    fn try_reconnect(&mut self) -> TelemetryResult<()> {
        // Aguarda antes de tentar reconectar
        thread::sleep(Duration::from_secs(2));
        
        match TcpStream::connect(&self.address) {
            Ok(new_connection) => {
                self.connection = new_connection;
                println!("✅ Reconexão estabelecida!");
                Ok(())
            }
            Err(e) => {
                eprintln!("❌ Falha na reconexão: {}", e);
                Err(e)
            }
        }
    }
}