# --- Configurazione Percorsi ---
$resultDirBase = ".\..\..\results\vtune_results"
$reportFileBase = ".\..\..\results\summary_report"
$targetApp = ".\rust_version.exe"
$pythonScript = "vtuneRust.py" 

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
$pythonArgs = @() # Array parallelo per memorizzare solo l'identificativo dell'input

if ($datasets.Count -eq 0) {
    $iterations += ,@() 
    $pythonArgs += "default"
} else {
    for ($i = 0; $i -lt $datasets.Count; $i++) {
        $currentArgs = @("-d", $datasets[$i])
        if ($targets.Count -gt 0) {
            $currentArgs += @("-t", $targets[$i])
        }
        $iterations += ,$currentArgs
        $pythonArgs += $datasets[$i] # Salviamo solo il valore del dataset (es. "100", "input1", ecc.)
    }
}

# --- Ciclo di Esecuzione VTune ---
$totalIter = $iterations.Count
for ($idx = 0; $idx -lt $totalIter; $idx++) {
    $argArray = $iterations[$idx]
    $currentInputId = $pythonArgs[$idx] # Questo è il valore pulito da passare a Python
    
    # Generazione ID per i file (manteniamo la logica precedente per coerenza file system)
    $idString = if ($argArray.Count -eq 0) { "default" } else { ($argArray -join "_") -replace '[^a-zA-Z0-9]', '_' }
    $currentResDir = "$resultDirBase`_$idString"
    $currentReport = "$reportFileBase`_$idString.csv"

    Write-Host "`n--- Avvio Profiling [$($idx + 1)/$totalIter]: Input $currentInputId ---" -ForegroundColor Cyan

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
        Write-Host "Generazione report VTune..." -ForegroundColor Yellow
        & vtune -report summary `
              -result-dir $currentResDir `
              -report-output $currentReport `
              -format csv `
              -csv-delimiter comma

        # --- Chiamata al programma Python ---
        if (Test-Path ".\$pythonScript") {
            Write-Host "Esecuzione script Python con input: $currentInputId" -ForegroundColor Magenta
            # PASSAGGIO PARAMETRO: Passiamo solo la variabile $currentInputId
            & python ".\$pythonScript" $currentInputId
        } else {
            Write-Warning "Script Python non trovato in: .\$pythonScript"
        }

    } else {
        Write-Warning "Errore durante l'esecuzione per l'input: $currentInputId"
    }

    # --- Pausa di raffreddamento ---
    if ($idx -lt $totalIter -1) {
        Write-Host "Attesa di 60 secondi per il raffreddamento del sistema..." -ForegroundColor Gray
        Start-Sleep -Seconds 60
    }
}

Write-Host "`nProcesso completato per tutte le iterazioni." -ForegroundColor Green

# --- Fusione Finale dei Risultati ---
if ($datasets.Count -gt 1) {
    $mergeScript = "merge_results.py"
    if (Test-Path ".\$mergeScript") {
        Write-Host "`n--- Avvio fusione dei file CSV ---" -ForegroundColor Cyan
        # Passiamo esattamente gli stessi dataset e target usati per VTune
        $mergeArgs = @("-d") + $datasets
        if ($targets.Count -gt 0) {
            $mergeArgs += @("-t") + $targets
        }
        & python ".\$mergeScript" $mergeArgs
    } else {
        Write-Warning "Script di fusione non trovato: $mergeScript"
    }
}