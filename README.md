Gestion de comptes bancaires en Rust
Fonctionnalités

    Créer un compte
    Tu entres un nom et un solde initial. Le compte est ajouté à la liste.

    Afficher tous les comptes
    Affiche le nom et le solde de chaque compte existant.

    Dépôt
    Tu entres le nom du compte et un montant à déposer. Le montant s'ajoute au solde du compte.

    Retrait
    Tu entres le nom du compte et un montant à retirer. Si le solde est suffisant, le montant est retiré.

    Fermer un compte
    Tu entres le nom du compte. Le compte est "fermé" (le solde est remis à zéro).

    Quitter
    Termine le programme proprement.

Utilisation

    Lancer le programme.

    Choisir une action en tapant le numéro correspondant (1 à 6).

    Suivre les instructions affichées pour entrer les noms ou montants demandés.

    Le programme boucle jusqu’à ce que tu choisisses de quitter.

Techniques utilisées dans le programme
Vecteur pour stocker les comptes
```rust
let mut comptes: Vec<CompteBancaire> = Vec::new();
```


Permet de créer un vecteur dans lequel seront stockés tous les comptes créés.
Utilisation de loop

Permet de répéter le programme indéfiniment jusqu’à ce que l’utilisateur choisisse de quitter.
Utilisation de match

Similaire à switch ou case en C, il permet de gérer le choix de l’utilisateur de manière claire.
Fonction pour lire une chaîne de caractères

```rust
fn lire_texte() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
```

Permet de lire une chaîne de caractères saisie par l’utilisateur dans le terminal.
Cette fonction a été comprise en observant des exemples sur internet.
Fonction pour lire un nombre (f64)

```rust
fn lire_f64() -> f64 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<f64>().unwrap_or(0.0)
}
```

Permet de lire un nombre décimal saisi par l’utilisateur.
Renvoie 0.0 si l’utilisateur entre une valeur invalide.


Résumé

Ce projet m’a permis de :

    Comprendre la gestion des structures en Rust.

    Utiliser un vecteur pour stocker des données dynamiques.

    Utiliser des boucles et des conditions pour gérer un menu interactif.

    Comprendre la gestion des entrées utilisateur dans un programme Rust.

    Créer un programme complet qui tourne dans le terminal.
