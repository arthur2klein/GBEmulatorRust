impl register{
    fn new() -> CPU {
        CPU {
            reg_a: 0,
                reg_b: 0,
                reg_c: 0,
                reg_d: 0,
                reg_e: 0,
                reg_h: 0,
                reg_l: 0,
                reg_f: 0,
                reg_pc: 0,
                reg_sp: 0,
            }
        }
    
        // Exemple d'implémentation d'une instruction (NOP - No Operation)
        fn nop(&mut self) {
            // L'instruction NOP ne fait rien, elle passe simplement à l'instruction suivante.
            self.reg_pc += 1;
        }
    
        // Exemple d'implémentation d'une instruction de chargement dans le registre A depuis une adresse mémoire
        fn ld_a_n(&mut self, value: u8) {
            self.reg_a = value;
            self.reg_pc += 2;  // Les instructions LD ont généralement une taille de 2 octets
        }
    
        // Exemple d'implémentation d'une instruction d'addition avec le registre A
        fn add_a_n(&mut self, value: u8) {
            let (result, overflow) = self.reg_a.overflowing_add(value);
            
            // Mettre à jour les drapeaux en fonction du résultat
            self.reg_f = 0;
            if result == 0 {
                // Mettre à jour le drapeau Zéro
                self.reg_f |= 0x80;
            }
            if overflow {
                // Mettre à jour le drapeau de report (Carry)
                self.reg_f |= 0x10;
            }
    
            self.reg_a = result;
            self.reg_pc += 2;  // Les instructions ADD ont généralement une taille de 2 octets
        }
    
        // Exemple d'exécution d'une instruction
        fn execute_instruction(&mut self, opcode: u8) {
            match opcode {
                0x00 => self.nop(),
                0x3E => {
                    let value = self.read_byte();
                    self.ld_a_n(value);
                }
                0xC6 => {
                    let value = self.read_byte();
                    self.add_a_n(value);
                }
                _ => {
                    // Gérer les autres opcodes ici
                    unimplemented!("Opcode non implémenté: {:02X}", opcode);
                }
            }
        }
    
        // Méthode pour lire un octet à partir de l'adresse actuelle du PC
        fn read_byte(&self) -> u8 {
            // À implémenter : Lire l'octet depuis la mémoire à l'adresse actuelle du PC
            // Ici, nous renvoyons simplement 0 pour l'exemple.
            0
        }
    }

