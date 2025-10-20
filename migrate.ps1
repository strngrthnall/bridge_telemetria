# Script de Migração para Git Único
# Execute este script da raiz do projeto: .\migrate.ps1

Write-Host "🔄 Iniciando migração para repositório Git único..." -ForegroundColor Cyan
Write-Host ""

# 1. Verificar se estamos na raiz do projeto
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "❌ Erro: Execute este script da raiz do projeto (bridge_telemetria)" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Verificação inicial: OK" -ForegroundColor Green
Write-Host ""

# 2. Fazer backup do histórico Git (se existir)
Write-Host "📋 Fazendo backup do histórico Git..." -ForegroundColor Yellow

if (Test-Path "telemetry_client\.git") {
    Push-Location telemetry_client
    git log --oneline > ../client_git_history.txt
    Write-Host "  ✅ Histórico do cliente salvo em client_git_history.txt" -ForegroundColor Green
    Pop-Location
}

if (Test-Path "telemetry_server\.git") {
    Push-Location telemetry_server
    git log --oneline > ../server_git_history.txt
    Write-Host "  ✅ Histórico do servidor salvo em server_git_history.txt" -ForegroundColor Green
    Pop-Location
}

Write-Host ""

# 3. Remover repositórios Git dos subdiretórios
Write-Host "🗑️  Removendo repositórios Git internos..." -ForegroundColor Yellow

if (Test-Path "telemetry_client\.git") {
    Remove-Item -Recurse -Force "telemetry_client\.git"
    Write-Host "  ✅ Removido: telemetry_client/.git" -ForegroundColor Green
} else {
    Write-Host "  ℹ️  telemetry_client/.git já não existe" -ForegroundColor Gray
}

if (Test-Path "telemetry_server\.git") {
    Remove-Item -Recurse -Force "telemetry_server\.git"
    Write-Host "  ✅ Removido: telemetry_server/.git" -ForegroundColor Green
} else {
    Write-Host "  ℹ️  telemetry_server/.git já não existe" -ForegroundColor Gray
}

Write-Host ""

# 4. Verificar se já existe repositório Git na raiz
if (Test-Path ".git") {
    Write-Host "⚠️  Já existe um repositório Git na raiz" -ForegroundColor Yellow
    $response = Read-Host "Deseja reinicializá-lo? (S/N)"
    if ($response -eq "S" -or $response -eq "s") {
        Remove-Item -Recurse -Force ".git"
        Write-Host "  ✅ Repositório anterior removido" -ForegroundColor Green
    } else {
        Write-Host "  ℹ️  Mantendo repositório existente" -ForegroundColor Gray
        Write-Host "  ℹ️  Pulando inicialização do Git" -ForegroundColor Gray
        $skipGitInit = $true
    }
}

Write-Host ""

# 5. Inicializar Git na raiz
if (-not $skipGitInit) {
    Write-Host "🔧 Inicializando repositório Git na raiz..." -ForegroundColor Yellow
    git init
    Write-Host "  ✅ Git inicializado" -ForegroundColor Green
    Write-Host ""
}

# 6. Verificar se o workspace compila
Write-Host "🔨 Verificando compilação do workspace..." -ForegroundColor Yellow
$compileResult = cargo check 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ Workspace compila corretamente" -ForegroundColor Green
} else {
    Write-Host "  ❌ Erro na compilação" -ForegroundColor Red
    Write-Host $compileResult
    exit 1
}

Write-Host ""

# 7. Adicionar arquivos ao Git (se não foi pulado)
if (-not $skipGitInit) {
    Write-Host "📦 Adicionando arquivos ao Git..." -ForegroundColor Yellow
    git add .
    Write-Host "  ✅ Arquivos adicionados" -ForegroundColor Green
    Write-Host ""

    # 8. Criar commit inicial
    Write-Host "💾 Criando commit inicial..." -ForegroundColor Yellow
    
    git commit -m "chore: migrate to unified git repository with cargo workspace" `
               -m "Create cargo workspace for telemetry_client and telemetry_server" `
               -m "Unify dependencies and build profiles" `
               -m "Add comprehensive documentation" `
               -m "Production-ready code with clean architecture"
    
    Write-Host "  ✅ Commit inicial criado" -ForegroundColor Green
    Write-Host ""
}

# 9. Mostrar status
Write-Host "📊 Status do repositório:" -ForegroundColor Cyan
git status --short
Write-Host ""

# 10. Resumo final
Write-Host "=" -NoNewline -ForegroundColor Cyan
Write-Host "=" * 60 -ForegroundColor Cyan
Write-Host "🎉 Migração concluída com sucesso!" -ForegroundColor Green
Write-Host "=" * 61 -ForegroundColor Cyan
Write-Host ""
Write-Host "📝 Próximos passos:" -ForegroundColor Yellow
Write-Host "  1. Verificar build: " -NoNewline -ForegroundColor White
Write-Host "cargo build" -ForegroundColor Cyan
Write-Host "  2. Rodar testes: " -NoNewline -ForegroundColor White
Write-Host "cargo test" -ForegroundColor Cyan
Write-Host "  3. Adicionar remote: " -NoNewline -ForegroundColor White
Write-Host "git remote add origin [url]" -ForegroundColor Cyan
Write-Host "  4. Push para remote: " -NoNewline -ForegroundColor White
Write-Host "git push -u origin main" -ForegroundColor Cyan
Write-Host ""
Write-Host "📚 Documentação:" -ForegroundColor Yellow
Write-Host "  • README.md - Guia principal do projeto" -ForegroundColor White
Write-Host "  • MIGRATION_GUIDE.md - Detalhes da migração" -ForegroundColor White
Write-Host "  • SYSTEM_ANALYSIS.md - Análise técnica completa" -ForegroundColor White
Write-Host ""
Write-Host "🚀 Comandos do workspace:" -ForegroundColor Yellow
Write-Host "  • cargo run -p telemetry_server" -ForegroundColor Cyan
Write-Host "  • cargo run -p telemetry_client" -ForegroundColor Cyan
Write-Host "  • cargo build --release" -ForegroundColor Cyan
Write-Host ""
Write-Host "✅ Sistema pronto para uso!" -ForegroundColor Green
