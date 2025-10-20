# 🚀 Quick Start - Repositório Unificado

## ✅ O que foi feito

Seu projeto agora está organizado em um **repositório Git único** com **Cargo Workspace**.

## 📁 Estrutura

```
bridge_telemetria/           ← Raiz (Git único)
├── .git/                    ✅ Repositório principal
├── .gitignore
├── Cargo.toml               ✅ Workspace
├── README.md                ✅ Documentação principal
├── LICENSE
├── MIGRATION_GUIDE.md
├── migrate.ps1              ✅ Script de migração
├── telemetry_client/
│   ├── Cargo.toml           ✅ Usa workspace
│   └── src/
└── telemetry_server/
    ├── Cargo.toml           ✅ Usa workspace
    └── src/
```

## 🔧 Execute a Migração

### Opção 1: Script Automático (Recomendado)

```powershell
# Da raiz do projeto:
.\migrate.ps1
```

O script faz:
1. ✅ Backup do histórico Git
2. ✅ Remove `.git` dos subdiretórios
3. ✅ Inicializa Git na raiz
4. ✅ Cria commit inicial
5. ✅ Verifica compilação

### Opção 2: Manual

```powershell
# 1. Backup (opcional)
cd telemetry_client; git log --oneline > ../client_history.txt; cd ..
cd telemetry_server; git log --oneline > ../server_history.txt; cd ..

# 2. Remover git interno
Remove-Item -Recurse -Force telemetry_client\.git
Remove-Item -Recurse -Force telemetry_server\.git

# 3. Inicializar Git na raiz
git init
git add .
git commit -m "chore: migrate to unified repository with cargo workspace"

# 4. Verificar
cargo build
```

## 🎯 Novos Comandos

```bash
# Compilar tudo
cargo build

# Rodar servidor
cargo run -p telemetry_server

# Rodar cliente
cargo run -p telemetry_client

# Compilar release
cargo build --release

# Verificar código
cargo check

# Testes
cargo test

# Limpar
cargo clean
```

## 🔗 Conectar ao GitHub/GitLab

```bash
# Criar repositório no GitHub primeiro
# Depois:

git remote add origin https://github.com/seu-usuario/bridge_telemetria.git
git branch -M main
git push -u origin main
```

## ✅ Verificação

```bash
# Deve compilar sem erros
cargo build

# Deve mostrar dois pacotes
cargo tree -p telemetry_client
cargo tree -p telemetry_server

# Git deve estar apenas na raiz
git status
```

## 📊 Benefícios

| Antes | Depois |
|-------|--------|
| 2 repositórios | 1 repositório |
| 2 comandos build | 1 comando build |
| Versões desincronizadas | Versão unificada |
| Dependencies duplicadas | Dependencies compartilhadas |

## 📚 Documentação

- **README.md** - Guia completo do projeto
- **MIGRATION_GUIDE.md** - Detalhes técnicos da migração
- **SYSTEM_ANALYSIS.md** - Análise do sistema

## 🆘 Ajuda

### Erro: "package not found"

```bash
# Verificar workspace
cat Cargo.toml | grep members
```

### Erro: Git ainda nos subdiretórios

```bash
# Remover manualmente
rm -rf telemetry_client/.git
rm -rf telemetry_server/.git
```

## 🎉 Pronto!

Execute `.\migrate.ps1` e seu repositório estará unificado! 🚀
