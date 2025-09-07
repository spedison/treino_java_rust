// Aprendi a separar os meus fontes em arquivos diferentes e como chamá-los
mod arquivo;
mod integral;
mod unique;

fn main() {
    // Aprendi a ler linhas de um arquivo texto, filtrá-las e processá-las.
    // Além disso aprendi a usar expressão regular nos filtros.
    arquivo::usa_arquivo();
    // Aprendi a usar fluxo de dados vindos de um array para fazer um processamento paralelo.
    // o exemplo mais clássico é a integração usando trapésio.
    integral::calcula_integral();

    unique::unique_test();
}

