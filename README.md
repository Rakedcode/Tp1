📘 Gestion de comptes bancaires en Rust
🚀 Fonctionnalités

✅ Créer un compte
➡️ Tu entres un nom et un solde initial. Le compte est ajouté à la liste.

✅ Afficher tous les comptes
➡️ Affiche le nom et le solde de chaque compte existant.

✅ Dépôt
➡️ Tu entres le nom du compte et un montant à déposer. Le montant s'ajoute au solde du compte.

✅ Retrait
➡️ Tu entres le nom du compte et un montant à retirer. Si le solde est suffisant, le montant est retiré.

✅ Fermer un compte
➡️ Tu entres le nom du compte. Le compte est "fermé" (le solde est remis à zéro).

✅ Quitter
➡️ Termine le programme proprement.
🖥️ Utilisation

1️⃣ Lance le programme.
2️⃣ Choisis une action en tapant le numéro correspondant (1 à 6).
3️⃣ Suis les instructions affichées (entrer le nom ou le montant selon l’action).
4️⃣ Le programme boucle jusqu’à ce que tu choisisses Quitter (6).
⚙️ Techniques utilisées dans le programme
📌 Vecteur pour stocker les comptes

let mut comptes: Vec<CompteBancaire> = Vec::new();

    Crée un vecteur de comptes bancaires dans lequel on stocke tous les comptes créés.

📌 loop

Permet de répéter le programme indéfiniment jusqu’à ce que l’utilisateur décide de quitter.
📌 match

Similaire à switch ou case en C, permet de gérer le choix de l’utilisateur de manière claire et lisible.
📌 Fonction pour lire un texte

fn lire_texte() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

➡️ Permet de lire une chaîne de caractères saisie par l’utilisateur dans le terminal.
Je l'ai bien comprise après avoir regardé des exemples sur internet.
📌 Fonction pour lire un nombre (f64)

fn lire_f64() -> f64 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<f64>().unwrap_or(0.0)
}

➡️ Permet de lire un nombre décimal saisi par l’utilisateur.
Si l’utilisateur ne rentre rien de correct, la fonction renvoie 0.0 par défaut.
📓 Résumé

Ce projet m’a permis de :
✅ Comprendre la gestion des structures en Rust.
✅ Utiliser un vecteur pour stocker des données dynamiques.
✅ Utiliser des boucles et des conditions pour gérer un menu interactif.
