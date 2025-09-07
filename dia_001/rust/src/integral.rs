// use itertools::Itertools; // importa as funções extras no trait.
use rayon::prelude::*;


/// Função a integrar: x^2
fn f(x: f64) -> f64 {
    10. + 3. * x * x * x
}

fn integrar_intervalo(a: f64, b: f64, n: usize) -> f64 {
    let dx = (b - a) / n as f64;

    (0..n)
        .map(|i| {
            let x = a + (i as f64 + 0.5) * dx; // ponto médio
            f(x) * dx
        })
        .sum()
}

pub fn calcula_integral(){
    let inicio = -150.0;
    let fim = 150.0;
    let blocos = 140;
    let subdivisoes_por_bloco = 100000;
    let passo_bloco = (fim - inicio) / blocos as f64;

    // Cria os 40 intervalos: [(0.0, 1.25), (1.25, 2.5), ..., (48.75, 50.0)]
    let intervalos: Vec<(f64, f64)> = (0..blocos)
        .map(|i| {
            let a = inicio + i as f64 * passo_bloco;
            let b = a + passo_bloco;
            (a, b)
        })
        .collect();

    // Processa os intervalos em paralelo usando Rayon
    let resultado: f64 = intervalos
        .into_par_iter()
        .map(|(a, b)| integrar_intervalo(a, b, subdivisoes_por_bloco))
        .sum();
    println!("{:0.12}",resultado)
}
