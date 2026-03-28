# Project Implementation Guide

This project provides tools to compute LOOCV, MCC, and estimated energy consumption for linear regression models across three different programming languages: **Rust**, **Go**, and **Scala**.

  - [Rust's Version](#rusts-version)
  - [Go's Version](#gos-version)
  - [Scala's Version](#scalas-version) 

> **Important Note:** For all versions, the Python scripts used to launch the programs and track energy consumption must be located in the same directory (at the same level) as the compiled executables or JAR files.

-----

# Rust's Version

The Rust (v. 1.92.0) implementation provides a  version of the model with energy tracking support via a Python wrapper.
> The official Rust installation guide: https://rust-lang.org/tools/install/
### Installation

1.  **Build the Project**: Navigate to the Rust project directory and run:

    ```bash
    cargo build --release
    ```

    **Note:** It is essential that the `Cargo.toml` file remains in its original location for the build process to function correctly.

2.  **Move the Executable**: After the build completes, move the generated binary from the target folder to the project root:

      * **On Linux**:
        ```bash
        cp target/release/rust_version .
        ```
      * **On Windows**:
        ```powershell
        copy target\release\rust_version.exe .
        ```

    These commands extract the executable from the build folder and place it in the root for easier access.

### Quickstart (with Energy Measurement)

To execute the program and capture energy consumption data, run the dedicated Python script:

```bash
python rustVersion.py <path_file>
```

-----

# Go's Version

The Go (v. 1.26.0) implementation provides a  version of the model with energy tracking support via a Python wrapper.
> The official Go installation guide: https://go.dev/doc/install

### Installation

1.  **Prepare Dependencies**: Clean up and sync your Go modules:

    ```bash
    go mod tidy
    ```
    **Note:** It is essential that the `go.mod` file remains in its original location for the build process to function correctly.

2.  **Build the Executable**: Compile the project using the specific naming convention required by the measurement scripts:

      * **On Linux**:
        ```bash
        go build -o goVersion
        ```
      * **On Windows**:
        ```bash
        go build -o goVersion.exe
        ```

    **Important:** The output name must be exactly `goVersion` (Linux) or `goVersion.exe` (Windows).

### Quickstart (with Energy Measurement)

To run the program and include energy consumption metrics, use the following command:

```bash
python goVersion.py <path_file>
```

-----

# Scala's Version

The Scala (v. 1.12.7) implementation provides a  version of the model with energy tracking support via a Python wrapper.
>I suggest following the instructions at https://docs.scala-lang.org/getting-started/install-scala.html, depending on your system. On Linux, check that coursier is on your PATH and make sure you have Java installed.

### Installation

1.  **Check Project Structure**: Before building, ensure your configuration files are in the correct locations:

      * `build.sbt`: Must be in the **project root**.
      * `plugins.sbt`: Must be inside the `project/` directory.
      * you should not modify them

2.  **Clean and Verify**:

    ```bash
    sbt clean
    sbt plugins
    ```

    Check the output of `sbt plugins` to ensure the **assembly** plugin is correctly listed.

3.  **Build the Assembly**:

    ```bash
    sbt assembly
    ```

    The command will generate a `.jar` file inside the project root directory


### Quickstart (with Energy Measurement)

To run the Scala implementation with energy consumption tracking, execute:

```bash
python scalaVersion.py <path_file>
```


