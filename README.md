# Compar

`compar` est un outil en ligne de commande écrit en Rust qui compare deux fichiers texte et identifie les lignes du premier fichier qui ne sont pas présentes dans le second. Il offre des options pour personnaliser la comparaison, comme la possibilité de ne comparer qu'un nombre défini de caractères au début de chaque ligne.

## Fonctionnalités

- Compare deux fichiers texte ligne par ligne.
- Identifie et affiche les lignes du premier fichier absentes dans le second.
- Option pour limiter la comparaison aux `N` premiers caractères de chaque ligne.
- Barre de progression pour suivre l'avancement du traitement.
- Gestion de différents encodages de fichiers (UTF-8, UTF-16).
- Option de débogage pour un suivi détaillé du processus.

## Installation

### Prérequis

Assurez-vous d'avoir Rust et Cargo d'installés sur votre système. Vous pouvez les installer en suivant les instructions sur le site officiel de Rust : [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Compilation

1.  Clonez ce dépôt :
    ```bash
    git clone <URL_DU_DEPOT>
    cd compar
    ```

2.  Compilez le projet en mode `release` pour obtenir les meilleures performances :
    ```bash
    cargo build --release
    ```

L'exécutable se trouvera dans le répertoire `target/release/`.

## Utilisation

La syntaxe de base est la suivante :

```bash
./target/release/compar [OPTIONS] <FICHIER1> <FICHIER2>
```

### Arguments

-   `<FICHIER1>` : Le fichier contenant les lignes à chercher (les "aiguilles").
-   `<FICHIER2>` : Le fichier dans lequel chercher (la "meule de foin").

### Options

-   `-o, --output <FICHIER_SORTIE>` : Spécifie un fichier dans lequel écrire les lignes non trouvées. Si cette option n'est pas utilisée, les lignes seront affichées sur la sortie standard.
-   `--debug` : Active l'affichage des informations de débogage dans le terminal.
-   `--length <N>` : Compare uniquement les `N` premiers caractères de chaque ligne.
-   `--stat` : Affiche des statistiques détaillées sur la comparaison à la fin du traitement.
-   `--found` : Inverse la logique et affiche les lignes de FICHIER1 qui sont trouvées dans FICHIER2.
-   `-h, --help` : Affiche l'aide.
-   `-V, --version` : Affiche la version de l'outil.

### Exemples

-   **Comparaison simple** :

    ```bash
    ./target/release/compar fichier1.txt fichier2.txt
    ```

-   **Enregistrer le résultat dans un fichier** :

    ```bash
    ./target/release/compar -o resultat.txt fichier1.txt fichier2.txt
    ```

-   **Comparer uniquement les 15 premiers caractères** :

    ```bash
    ./target/release/compar --length 15 fichier1.txt fichier2.txt
    ```

-   **Utiliser le mode débogage** :

    ```bash
    ./target/release/compar --debug fichier1.txt fichier2.txt
    ```

## Dépendances

Ce projet utilise les dépendances suivantes (telles que définies dans `Cargo.toml`) :

-   `clap` (version `4.5.41`) : Pour l'analyse des arguments de la ligne de commande.
-   `indicatif` (version `0.18.0`) : Pour afficher une barre de progression.
-   `encoding_rs` (version `0.8.35`) : Pour la gestion des encodages de texte.
-   `unicode-normalization` (version `0.1.24`) : Pour la normalisation des chaînes Unicode.
-   `memchr` (version `2.7.5`) : Dépendance indirecte pour des recherches de caractères performantes.

### Compilation pour Windows (depuis Linux/macOS)

Si vous êtes sur Linux ou macOS et que vous souhaitez compiler `compar` pour Windows, vous devez d'abord ajouter la cible de compilation Windows:

```bash
rustup target add x86_64-pc-windows-gnu
# ou pour MSVC (si vous avez Visual Studio installé sur Windows)
# rustup target add x86_64-pc-windows-msvc
```

Ensuite, vous pouvez compiler le projet en spécifiant la cible:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

L'exécutable se trouvera dans `target/x86_64-pc-windows-gnu/release/compar.exe`.

## Tests

Ce projet inclut des tests unitaires pour garantir la fiabilité de la logique de remplacement. Pour exécuter ces tests, utilisez la commande suivante à la racine du projet :

```bash
cargo test
```

Cette commande compile le programme en mode test et exécute toutes les fonctions de test.