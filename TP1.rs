use std::io;

//Permet de crée une structure Compte Bancaire avec Nom qui est une chaine de caractère
//f64 est un int    
struct CompteBancaire {
    nom:String,
    solde:f64
}




impl CompteBancaire {
    // je rajoute &self uniquement sans le mut car je suis en lecture seule 
    fn afficher(&self){  // comme le this.nom = nom 
        println!("Compte bancaire de {}, solde est : {} €" , self.nom, self.solde);


    }

        fn deposer(&mut self, montant:f64){  // je dépose 30€
            self.solde += montant ; // self.solde = self.solde + montant        //  100+ 30

        println!("+{} € déposés :",montant );

    }

        fn retirer(&mut self, montant:f64){

            if self.solde >= montant {
                self.solde -= montant;
                println!("-{} € retirés",montant)
            }else{
                println!("Solde insuffisant !")
            }
        }

        fn fermer(&mut self){  // ici self est consomé, on ne peut pas utiliser l'objet ensuite 
            println!("le compte de {} est fermé, dernier solde :{} !,",self.nom, self.solde);
            self.solde =0.0

        }

}


fn lire_texte() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn lire_f64() -> f64 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<f64>().unwrap_or(0.0)
}

fn main() {
    let mut comptes: Vec<CompteBancaire> = Vec::new();

    loop {
        println!("\n1. Créer compte");
        println!("2. Afficher comptes");
        println!("3. Dépôt");
        println!("4. Retrait");
        println!("5. Fermer compte");
        println!("6. Quitter");

        let choix = lire_texte();

        match choix.as_str() {
            "1" => {
                println!("Nom :");
                let nom = lire_texte();
                println!("Solde initial :");
                let solde = lire_f64();
                comptes.push(CompteBancaire { nom, solde });
                println!("Compte créé !");
            }
            "2" => {
                for compte in &comptes {
                    compte.afficher();
                }
            }
            "3" => {
                println!("Nom du compte pour dépôt :");
                let nom_saisi = lire_texte();
                println!("Montant :");
                let montant = lire_f64();
                let mut trouve = false;
                for compte in &mut comptes {
                    if compte.nom == nom_saisi {
                        compte.deposer(montant);
                        trouve = true;
                        break;
                    }
                }
                if !trouve {
                    println!("Compte non trouvé !");
                }
            }
            "4" => {
                println!("Nom du compte pour retrait :");
                let nom_saisi = lire_texte();
                println!("Montant :");
                let montant = lire_f64();
                let mut trouve = false;
                for compte in &mut comptes {
                    if compte.nom == nom_saisi {
                        compte.retirer(montant);
                        trouve = true;
                        break;
                    }
                }
                if !trouve {
                    println!("Compte non trouvé !");
                }
            }
            "5" => {
                println!("Nom du compte à fermer :");
                let nom_saisi = lire_texte();
                let mut trouve = false;
                for compte in &mut comptes {
                    if compte.nom == nom_saisi {
                        compte.fermer();
                        trouve = true;
                        break;
                    }
                }
                if !trouve {
                    println!("Compte non trouvé !");
                }
            }
            "6" => {
                println!("Fin du programme.");
                break;
            }
            _ => println!("Choix invalide."),
        }
    }
}