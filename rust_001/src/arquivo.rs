use std::{fs, io};
use std::collections::HashMap;
use std::fs::File;
use itertools::Itertools;
use regex::Regex;
use std::io::BufRead;

pub fn usa_arquivo(){


    println!("Iniciando programa");

    // Usando os comentários
    // Usando variáveis mutáveis do tipo String.
    let mut guess = String::new();
    guess.push_str("meu teste");
    guess.push_str(" - inicio");
    // Imprimindo a variável.
    println!("Guess = {}", guess);
    // Tratando o nome do arquivo
    let filename: &str = "../nomes.txt";
    let string_formatada = format!("Processando {}", filename);
    println!("{}", string_formatada);
    // Dado o nome do arquivo ele lê arquivo inteiro.
    let _conteudo = fs::read_to_string(filename);
    // Os erros vem encapsulados do tipo de leitura.
    match _conteudo {
        Ok(txt) => println!("Lido com sucesso {}", txt),
        Err(err) => eprintln!("Erro ao ler arquivo {}", err),
    }

    println!("Criando RegExp.");
    let regex_ignorar = Regex::new(r"(?i)^(campus|instituto|[0-9])").unwrap();

    // Declaração de vetores.
    let mut nomes_iniciando_com_a = Vec::new();
    let mut nomes_iniciando_com_b = Vec::new();
    let mut todos_nomes:Vec<_> = Vec::new();

    // Lendo arquivo linha a linha
    // 1) Abre o arquivo (obvio que com sucesso)
    if let Ok(arquivo) = File::open(filename) {
        println!("Consegui abrir arquivo...Continuando");
        let leitor_arquivo = io::BufReader::new(arquivo);
        for (indice, linha) in leitor_arquivo.lines().enumerate() {
            match linha {
                Ok(text) => {
                    let ret = tratar_linha(indice + 1,
                                           &text, &regex_ignorar);
                    match ret {
                        Some(txt) => {
                            let clone_txt = txt.clone();
                            todos_nomes.push(clone_txt);
                            if txt.to_lowercase().starts_with("a") {
                                nomes_iniciando_com_a.push(txt);
                            } else if txt.to_lowercase().starts_with("b") {
                                nomes_iniciando_com_b.push(txt);
                            }
                        }
                        None => {}
                    };
                }
                Err(erro) => eprintln!("Problemas ao ler linha do arquivo :: {}", erro),
            }
        }
        println!("Nomes iniciando com A");
        for (i, nome_a) in nomes_iniciando_com_a.iter().enumerate() {
            println!("Nome A :: {} -> {}", i, nome_a);
        }
        println!("Nomes iniciando com B");
        for (i, nome_b) in nomes_iniciando_com_b.iter().enumerate() {
            println!("Nome B :: {} -> {}", i, nome_b);
        }

        println!("Contagem de elementos : ");
        let mut conta_menor_que_3 = 0;
        let mut contagem_de_letras: HashMap<char, usize> = HashMap::new();
        for palavra in &todos_nomes {
            if palavra.len() >= 3 {
                if let Some(letra) = palavra.to_lowercase().chars().nth(2) {
                    *contagem_de_letras.entry(letra).or_insert(0) += 1;
                }
            } else {
                conta_menor_que_3 += 1;
            }
        }

        // Para ordenar é preciso colocar em uma lista.
        let mut lista_para_ordenar: Vec<_> = contagem_de_letras.iter().collect();
        lista_para_ordenar.sort_by_key(|(letra, _)| *letra);
        contagem_de_letras.iter();
        println!("Menores que 3 = {}", conta_menor_que_3);
        for (letra, contagem) in lista_para_ordenar {
            println!("Na letra {} temos {:05} contagens", letra, contagem);
        }
    } else {
        eprintln!("Erro ao abrir arquivo {}", filename);
    }

    // Usando itertools
    let grupos =
        todos_nomes
            .iter()
            .filter(|nome| nome.chars().nth(2).is_some())
            .sorted_by_key(|nome| nome.chars().nth(2).unwrap())
            .chunk_by(|nome| nome.chars().nth(2).unwrap());

    for (letra, grupo) in grupos.into_iter() {
        println!("Grupo '{}': {:?}", letra.to_string(), grupo.collect::<Vec<_>>());
    }
}

fn tratar_linha(numero: usize, conteudo: &str, regexp_para_ignorar: &Regex) -> Option<String> {
    let linha_limpa = conteudo.trim();

    if linha_limpa.is_empty() {
        return None;
    }

    let linha_limpa_ignore_case = linha_limpa.to_lowercase();

    if linha_limpa_ignore_case.contains("masculino") ||
        linha_limpa_ignore_case.contains("feminino") ||
        linha_limpa_ignore_case.contains("género") ||
        linha_limpa_ignore_case.contains("nome")
    {
        return None;
    }

    if regexp_para_ignorar.is_match(linha_limpa) {
        return None;
    }

    println!("Linha {} :: {}", numero, linha_limpa);

    return Some(linha_limpa.to_string());
}