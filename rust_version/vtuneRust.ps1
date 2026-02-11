# --- Configurazione Percorsi ---
$resultDirBase = ".\vtune_results"
$reportFileBase = ".\summary_report"
$targetApp = ".\rust_version.exe"

# --- Parsing dei Parametri ---
$datasets = @()
$targets = @()

for ($i = 0; $i -lt $args.Count; $i++) {
    if ($args[$i] -eq "-d") {
        $j = $i + 1
        while ($j -lt $args.Count -and $args[$j] -notlike "-*") {
            $datasets += $args[$j]
            $j++
        }
    }
    if ($args[$i] -eq "-t") {
        $k = $i + 1
        while ($k -lt $args.Count -and $args[$k] -notlike "-*") {
            $targets += $args[$k]
            $k++
        }
    }
}

# --- Controllo di Coerenza ---
if ($targets.Count -gt 0 -and $datasets.Count -ne $targets.Count) {
    Write-Error "Errore: Hai fornito dei target (-t), ma il numero ($($targets.Count)) non corrisponde a quello dei dataset ($($datasets.Count))."
    exit 1
}

# --- Determinazione dei Cicli di Esecuzione ---
$iterations = @()

if ($datasets.Count -eq 0) {
    $iterations += ,@() 
} else {
    for ($i = 0; $i -lt $datasets.Count; $i++) {
        $currentArgs = @("-d", $datasets[$i])
        if ($targets.Count -gt 0) {
            $currentArgs += @("-t", $targets[$i])
        }
        $iterations += ,$currentArgs
    }
}

# --- Ciclo di Esecuzione VTune ---
$totalIter = $iterations.Count
for ($idx = 0; $idx -lt $totalIter; $idx++) {
    $argArray = $iterations[$idx]
    
    # Generazione ID pulito per i file
    $idString = if ($argArray.Count -eq 0) { "default" } else { ($argArray -join "_") -replace '[^a-zA-Z0-9]', '_' }
    $currentResDir = "$resultDirBase`_$idString"
    $currentReport = "$reportFileBase`_$idString.csv"

    Write-Host "`n--- Avvio Profiling [$($idx + 1)/$totalIter]: $targetApp $($argArray -join ' ') ---" -ForegroundColor Cyan

    # 1. Raccolta dati 
    & vtune -collect system-overview `
          -data-limit=500 `
          -d=90 `
          -discard-raw-data `
          -finalization-mode=none `
          -r $currentResDir `
          -knob analyze-power-usage=true `
          -knob sampling-interval=500 `
          -- $targetApp @argArray

    if ($LASTEXITCODE -eq 0) {
        Write-Host "Generazione report: $currentReport" -ForegroundColor Yellow
        vtune -report summary `
              -result-dir $currentResDir `
              -report-output $currentReport `
              -format csv `
              -csv-delimiter comma
    } else {
        Write-Warning "Errore durante l'esecuzione per i parametri: $($argArray -join ' ')"
    }

    # --- Pausa tra i cicli (tranne dopo l'ultimo) ---
    if ($idx -lt $totalIter -1) {
        Write-Host "Attesa di 60 secondi per il raffreddamento del sistema..." -ForegroundColor Gray
        Start-Sleep -Seconds 60
    }
}

Write-Host "`nProcesso completato per tutte le iterazioni." -ForegroundColor Green