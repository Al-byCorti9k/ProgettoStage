# ============================================================================
# Script di profilazione con VTune per applicazione Go
# ============================================================================

# --- Configurazione Percorsi ---
$resultDirBase = "..\results\vtune_results"          # Base per le cartelle dei risultati VTune
$reportFileBase = "..\results\summary_report"        # Base per i file CSV di riepilogo
$targetApp = "..\src\goVersion.exe"                  # Eseguibile da profilare
$pythonScript = "vtuneGo.py"                          # Script Python per elaborazione aggiuntiva

# --- Parsing dei Parametri ---
$datasets = @()   # Lista dei dataset passati con -d
$targets = @()    # Lista dei target passati con -t

for ($i = 0; $i -lt $args.Count; $i++) {
    if ($args[$i] -eq "-d") {
        $j = $i + 1
        # Raccoglie tutti i valori successivi fino al prossimo flag
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
# Se vengono forniti dei target, il loro numero deve corrispondere al numero di dataset
if ($targets.Count -gt 0 -and $datasets.Count -ne $targets.Count) {
    Write-Error "Errore: Hai fornito dei target (-t), ma il numero ($($targets.Count)) non corrisponde a quello dei dataset ($($datasets.Count))."
    exit 1
}

# --- Determinazione dei Cicli di Esecuzione ---
# $iterations conterrà per ogni ciclo gli argomenti da passare all'applicativo
# $pythonArgs conterrà per ogni ciclo l'identificativo del dataset (o "default")
$iterations = @()
$pythonArgs = @()

if ($datasets.Count -eq 0) {
    # Nessun dataset: esegue una volta con argomenti vuoti
    $iterations += ,@()               # lista vuota di argomenti
    $pythonArgs += "default"
} else {
    for ($i = 0; $i -lt $datasets.Count; $i++) {
        $currentArgs = @("-d", $datasets[$i])
        if ($targets.Count -gt 0) {
            $currentArgs += @("-t", $targets[$i])
        }
        $iterations += ,$currentArgs   # aggiunge l'array come singolo elemento
        $pythonArgs += $datasets[$i]   # per il naming usiamo solo il dataset
    }
}

# --- Ciclo di Esecuzione VTune ---
$totalIter = $iterations.Count
for ($idx = 0; $idx -lt $totalIter; $idx++) {
    $argArray = $iterations[$idx]          # argomenti per l'applicativo
    $currentInputId = $pythonArgs[$idx]     # identificativo del dataset (o "default")

    # Genera un identificativo sicuro per cartelle e file basato SOLO sul dataset
    # In questo modo i target (-t) non influenzano i nomi dei risultati.
    if ($currentInputId -eq "default") {
        $idString = "default"
    } else {
        # Sostituisce eventuali caratteri non alfanumerici con underscore
        $sanitized = $currentInputId -replace '[^a-zA-Z0-9]', '_'
        $idString = "d_$sanitized"          # es. d_3
    }

    $currentResDir = "$resultDirBase`__$idString"      # cartella risultati VTune
    $currentReport = "$reportFileBase`__$idString.csv" # file di riepilogo

    Write-Host "`n--- Avvio Profiling [$($idx + 1)/$totalIter]: Input $currentInputId ---" -ForegroundColor Cyan

    # --- Esecuzione della raccolta dati con VTune ---
    #    -collect system-overview : raccolta dati di sistema
    #    -data-limit=500          : limite massimo dati (MB)
    #    -d=90                     : durata raccolta (secondi)
    #    -discard-raw-data         : non conserva i dati grezzi dopo la finalizzazione
    #    -finalization-mode=none   : nessuna finalizzazione automatica
    #    -r $currentResDir         : directory di output
    #    -knob ...                 : parametri aggiuntivi (power, intervallo campionamento)
    #    -- $targetApp @argArray   : comando da profilare con i suoi argomenti
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
        # Generazione del report di riepilogo in formato CSV
        & vtune -report summary `
              -result-dir $currentResDir `
              -report-output $currentReport `
              -format csv `
              -csv-delimiter comma

        # --- Chiamata allo script Python per eventuale post‑elaborazione ---
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

    # --- Pausa di raffreddamento tra un test e l'altro ---
    if ($idx -lt $totalIter - 1) {
        Write-Host "Attesa di 60 secondi per il raffreddamento del sistema..." -ForegroundColor Gray
        Start-Sleep -Seconds 60
    }
}

Write-Host "`nProcesso completato per tutte le iterazioni." -ForegroundColor Green

# --- Fusione Finale dei Risultati (solo se ci sono più dataset) ---
if ($datasets.Count -gt 1) {
    $mergeScript = "merge_results.py"   # script per unire i CSV
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