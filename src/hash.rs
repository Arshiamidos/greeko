use std::collections::HashMap;
pub fn to_greek(a:&str)->String{
        let mut hs:HashMap<char,char>=HashMap::new();
        hs.insert('a','α');
        hs.insert('b','β');
        hs.insert('c','ς');
        hs.insert('d','δ');
        hs.insert('e','Σ');
        hs.insert('f','ƒ');
        hs.insert('g','9');
        hs.insert('h','н');
        hs.insert('i','ι');
        hs.insert('j','ψ');
        hs.insert('k','κ');
        hs.insert('l','λ');
        hs.insert('m','μ');
        hs.insert('n','η');
        hs.insert('o','Ω');
        hs.insert('p','Ꝓ');
        hs.insert('q','φ');
        hs.insert('r','Γ');
        hs.insert('s','5');
        hs.insert('t','τ');
        hs.insert('u','υ');
        hs.insert('v','ν');
        hs.insert('w','ω');
        hs.insert('x','χ');
        hs.insert('y','γ');
        hs.insert('z','ζ');
        a.chars().map(|c| match hs.get(&c){
                Some(e)=>*e,
                None=>c
        }).collect::<String>()
}