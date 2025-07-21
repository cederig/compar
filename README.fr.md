# Compar

`compar` est un outil en ligne de commande écrit en Rust qui compare deux fichiers texte et identifie les lignes du premier fichier qui ne sont pas présentes dans le second. Il offre des options pour personnaliser la comparaison, comme la possibilité de ne comparer qu'un nombre défini de caractères au début de chaque ligne.

## Fonctionnalités

- Compare deux fichiers texte ligne par ligne.
- Identifie et affiche les lignes du premier fichier absentes dans le second.
- Option pour limiter la comparaison aux `N` premiers caractères de chaque ligne.
- Barre de progression pour suivre l'avancement du traitement.
- Gestion de différents encodages de fichiers (UTF-8, UTF-16).
- Option de débogage pour un suivi détaillé du processus.

## Dépendances

Ce projet utilise les dépendances suivantes (telles que définies dans `Cargo.toml`) :

-   `clap` (version `4.5.41`) : Pour l'analyse des arguments de la ligne de commande.
-   `indicatif` (version `0.18.0`) : Pour afficher une barre de progression.
-   `encoding_rs` (version `0.8.35`) : Pour la gestion des encodages de texte.
-   `unicode-normalization` (version `0.1.24`) : Pour la normalisation des chaînes Unicode.
-   `memchr` (version `2.7.5`) : Dépendance indirecte pour des recherches de caractères performantes.

## Installation

### Prérequis

Assurez-vous d'avoir Rust et Cargo d'installés sur votre système. Vous pouvez les installer en suivant les instructions sur le site officiel de Rust : [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Compilation pour Linux (depuis Linux/macOS)
1.  Clonez ce dépôt :
    ```sh
    git clone https://github.com/cederig/compar.git
    cd compar
    ```
2.  Compilez le projet :
    ```sh
    cargo build --release
    ```
    L'exécutable se trouvera dans `target/release/compar`.

### Compilation pour macOS (depuis Linux/macOS)

Pour compiler ce projet pour Windows à partir d'un autre système d'exploitation (comme Linux ou macOS), vous pouvez utiliser la compilation croisée. Vous aurez besoin de la cible Rust pour Windows.

1.  Ajoutez la cible Windows à votre installation Rust :
    ```sh
    rustup target add x86_64-pc-windows-gnu
    ```

2.  Compilez le projet pour la cible Windows :
    ```sh
    cargo build --release --target=x86_64-pc-windows-gnu
    ```

L'exécutable pour Windows se trouvera dans `target/x86_64-pc-windows-gnu/release/compar.exe`.

### Compilation pour macOS (depuis Linux/macOS)

Pour compiler ce projet pour macOS à partir d'un autre système d'exploitation (comme Linux ou macOS), vous pouvez utiliser la compilation croisée. Vous aurez besoin de la cible Rust pour macOS.

1.  Ajoutez la cible macOS à votre installation Rust (choisissez la bonne architecture) :
    *   Pour les Mac Intel (x86_64) :
        ```sh
        rustup target add x86_64-apple-darwin
        ```
    *   Pour les Mac Apple Silicon (aarch64) :
        ```sh
        rustup target add aarch64-apple-darwin
        ```

2.  Compilez le projet pour la cible macOS (choisissez la bonne architecture) :
    *   Pour les Mac Intel :
        ```sh
        cargo build --release --target=x86_64-apple-darwin
        ```
    *   Pour les Mac Apple Silicon :
        ```sh
        cargo build --release --target=aarch64-apple-darwin
        ```

L'exécutable pour macOS se trouvera dans `target/<votre_cible_mac>/release/compar` (par exemple, `target/x86_64-apple-darwin/release/compar`).

## Utilisation

La syntaxe de base est la suivante :

```bash
./compar [OPTIONS] <file1> <file2>
```

### Arguments

-   `<file1>` : Le fichier contenant les lignes à chercher (les "aiguilles").
-   `<file2>` : Le fichier dans lequel chercher (la "meule de foin").

### Options

-   `-o, --output <output_file>` : Spécifie un fichier dans lequel écrire les lignes non trouvées. Si cette option n'est pas utilisée, les lignes seront affichées sur la sortie standard.
-   `--debug` : Active l'affichage des informations de débogage dans le terminal.
-   `--length <N>` : Compare uniquement les `N` premiers caractères de chaque ligne.
-   `--stat` : Affiche des statistiques détaillées sur la comparaison à la fin du traitement.
-   `--found` : Inverse la logique et affiche les lignes de file1 qui sont trouvées dans file2.
-   `-h, --help` : Affiche l'aide.
-   `-V, --version` : Affiche la version de l'outil.

### Exemples

-   Comparaison simple :

    ```bash
    ./compar fichier1.txt fichier2.txt
    ```

-   Enregistrer le résultat dans un fichier :

    ```bash
    ./compar -o resultat.txt fichier1.txt fichier2.txt
    ```

-   Comparer uniquement les 15 premiers caractères :

    ```bash
    ./compar --length 15 fichier1.txt fichier2.txt
    ```

-   Utiliser le mode débogage :

    ```bash
    ./compar --debug fichier1.txt fichier2.txt
    ```

## Tests

Ce projet inclut des tests unitaires; pour les exécuter, utilisez la commande suivante à la racine du projet :

```bash
cargo test
```

Cette commande compile le programme en mode test et exécute toutes les fonctions de test.