
import breeze.linalg.{DenseMatrix, DenseVector}
import breeze.stats.{mean, median}

import java.io.File
import java.nio.file.{Files, Paths}
import scala.util.Try

import java.util.logging.{Level, Logger}

object ScalaVersionBreeze {
  Logger.getLogger("").setLevel(Level.SEVERE)

  def main(args: Array[String]): Unit = {
    if (args.length < 1) {
      println("usage: scala ScalaVersionBreeze <path_file_csv>")
      println("Example: scala ScalaVersionBreeze ./data/my_dataset.csv")
      System.exit(1)
    }

    val filePath = Paths.get(args(0)).toAbsolutePath.normalize()
    if (!Files.exists(filePath)) {
      println(s"File non trovato: $filePath")
      System.exit(1)
    }

    // Caricamento e imputazione
    val rawData = loadCSV(filePath.toFile)
    val imputedData = imputeMissingValues(rawData)

    val nRows = imputedData.rows
    val nFeatures = imputedData.cols - 1
    val X_full = imputedData(::, 0 until nFeatures)
    val y_full = imputedData(::, nFeatures)

    val startTime = System.currentTimeMillis()
    val mcc = leaveOneOutCrossValidation(X_full, y_full)
    val endTime = System.currentTimeMillis()
    val timeSeconds = (endTime - startTime) / 1000.0

    println("\n----metrics----")
    println(s"Dataset: ${filePath.getFileName}")
    println(f"Tempo LOOCV: $timeSeconds%.3f secondi")
    println(f"MCC: $mcc%.8f")
  }

  def loadCSV(file: File): DenseMatrix[Double] = {
  val source = scala.io.Source.fromFile(file)
  try {
    val lines = source.getLines().toArray
    val dataLines = lines.drop(1) // salta header
    val rows = dataLines.length
    val cols = dataLines.head.split(",").length
    val arr = Array.ofDim[Double](rows, cols)
    for ((line, i) <- dataLines.zipWithIndex) {
      val tokens = line.split(",")
      for ((token, j) <- tokens.zipWithIndex) {
        arr(i)(j) = Try(token.trim.toDouble).getOrElse(Double.NaN)
      }
    }
    DenseMatrix.tabulate(rows, cols) { (i, j) => arr(i)(j) }
  } finally {
    source.close()
  }
}

  def imputeMissingValues(data: DenseMatrix[Double]): DenseMatrix[Double] = {
    val nRows = data.rows
    val nCols = data.cols
    val imputed = DenseMatrix.zeros[Double](nRows, nCols)
    for (j <- 0 until nCols) {
      val col = data(::, j).toArray
      val nonMissing = col.filter(!_.isNaN)
      if (nonMissing.isEmpty) {
        println(s"Attenzione: colonna $j tutta NaN, imputazione con 0.0")
        val replacement = 0.0
        for (i <- 0 until nRows) imputed(i, j) = replacement
      } else {
        val replacement = if (j == nCols - 1) median(nonMissing) else mean(nonMissing)
        for (i <- 0 until nRows) {
          imputed(i, j) = if (data(i, j).isNaN) replacement else data(i, j)
        }
      }
    }
    imputed
  }

  def fitOLS(X: DenseMatrix[Double], y: DenseVector[Double]): DenseVector[Double] = {
    val n = X.rows
    val X_withIntercept = DenseMatrix.horzcat(DenseMatrix.ones[Double](n, 1), X)
    (X_withIntercept.t * X_withIntercept) \ (X_withIntercept.t * y)
  }

  def predictOLS(x: DenseVector[Double], coeff: DenseVector[Double]): Double = {
    coeff(0) + coeff(1 until coeff.length).dot(x)
  }

  def leaveOneOutCrossValidation(X: DenseMatrix[Double], y: DenseVector[Double]): Double = {
    val n = X.rows
    val trueClasses = Array.ofDim[Double](n)
    val predictedClasses = Array.ofDim[Double](n)

    for (i <- 0 until n) {
      // Costruisce training set escludendo l'i-esima riga
      val trainIdx = (0 until n).filter(_ != i).toArray
      val X_rows = Array.ofDim[Array[Double]](trainIdx.length)
      val y_vals = Array.ofDim[Double](trainIdx.length)

      for (j <- trainIdx.indices) {
        val rowIdx = trainIdx(j)
        // Copia la riga in un array di double
        val rowArr = new Array[Double](X.cols)
        for (k <- 0 until X.cols) {
          rowArr(k) = X(rowIdx, k)
        }
        X_rows(j) = rowArr
        y_vals(j) = y(rowIdx)
      }

      val X_train = DenseMatrix(X_rows.toIndexedSeq: _*)   // costruisce matrice dalle righe
      val y_train = DenseVector(y_vals)

      val coeff = fitOLS(X_train, y_train)

      // Predici sull'istanza i-esima (X(i, ::).t è un vettore colonna)
      val x_test = X(i, ::).t
      val pred = predictOLS(x_test, coeff)

      predictedClasses(i) = if (pred > 0.5) 1.0 else 0.0
      trueClasses(i) = y(i)
    }

    // Calcola MCC
    var tp = 0.0; var tn = 0.0; var fp = 0.0; var fn = 0.0
    for (i <- 0 until n) {
      (predictedClasses(i), trueClasses(i)) match {
        case (1.0, 1.0) => tp += 1
        case (0.0, 0.0) => tn += 1
        case (1.0, 0.0) => fp += 1
        case (0.0, 1.0) => fn += 1
        case _ =>
      }
    }

    val numerator = tp * tn - fp * fn
    val denominator = math.sqrt((tp + fp) * (tp + fn) * (tn + fp) * (tn + fn))
    if (denominator == 0) 0.0 else numerator / denominator
  }
}