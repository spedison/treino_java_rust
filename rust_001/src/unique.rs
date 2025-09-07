use itertools::Itertools;

pub fn unique_test() {
    // recebi um array para trabalhar.
    let lista = vec![2, 3, 2, 1, 1, 2, 2, 5, 7, 5, 2, 3, 1];

    // tornei ele um array com valores únicos.
    let unicos: Vec<_> =
        lista.
            into_iter().
            sorted().
            dedup().
            collect();

    println!("{}", unicos.clone().into_iter().join(" :: ")); //

    let mut conta_linha = 1;
    // Dado o array array agora monta grupos únicos (independente de ordem) com esses valores
    // tomadas 4 a 4.
    for par in unicos.into_iter().combinations(4) {
        let len = par.len();
        // Agora realizo permutações com esses valores.
        for par2 in par.into_iter().permutations(len) {
            println!("{:04} - [{}]", conta_linha, par2.into_iter().join(" || "));
            conta_linha += 1;
        }
        conta_linha = 1;
        //println!("[{}]", par.into_iter().join(" || "));
    }
}