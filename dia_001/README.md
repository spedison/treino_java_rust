# 🧠 Tarefa 001 — Treino Java + Rust

**Data**: 04/09/2025  
**Duração estimada**: 1h em Java + 1h em Rust  
**Objetivo**: Trabalhar leitura de arquivos linha a linha, aplicar filtro e exibir resultado.

---

## 📄 Descrição do Problema

Você tem um arquivo de texto chamado `nomes.txt` com centenas de nomes (um por linha), como:

```
Ana  
Pedro  
Eduardo  
Fernanda  
Carlos  
Amanda
```

Seu programa deve:

1. Ler todas as linhas do arquivo.  
2. Filtrar apenas os nomes que **começam com a letra "A"**.  
3. Imprimir na tela esses nomes, um por linha.  

---

## ✅ Requisitos (Java e Rust)

- Tratar erros (arquivo não encontrado, leitura inválida, etc.)
- Usar abordagem idiomática da linguagem.
- Código limpo, comentado e estruturado.

---

## ☕ Parte 1 — Java

### Sugestão de implementação

- Usar `Files.lines(Path)` com `Stream<String>`.
- Usar `filter()` e `startsWith("A")`.
- Imprimir com `forEach(System.out::println)`.

```java
Files.lines(Paths.get("nomes.txt"))
     .filter(nome -> nome.startsWith("A"))
     .forEach(System.out::println);
```

### 💡 Pontos de atenção

- Usar `try-with-resources`.
- Relembrar `Stream` (importante para certificação).
- Pode comparar com `BufferedReader` se quiser.

---

## 🦀 Parte 2 — Rust

### Sugestão de implementação

- Usar `std::fs::File` + `BufReader::lines()`.
- Tratar erros com `Result`.
- Usar `starts_with("A")`.

```rust
let file = File::open("nomes.txt")?;
let reader = BufReader::new(file);

for line in reader.lines() {
    let nome = line?;
    if nome.starts_with("A") {
        println!("{}", nome);
    }
}
```

### 💡 Pontos de atenção

- `BufReader` + `lines()` → cada linha é `Result<String>`
- Usar `?` para propagação de erros
- Testar com nomes acentuados (UTF-8)

---

## 🔍 Comparação Java × Rust

| Aspecto               | Java                                           | Rust                       |
| --------------------- | ---------------------------------------------- | -------------------------- |
| Leitura linha a linha | `Files.lines()` ou `BufferedReader.readLine()` | `BufReader::lines()`       |
| Filtro por prefixo    | `.filter(nome -> nome.startsWith("A"))`        | `if nome.starts_with("A")` |
| Tratamento de erros   | `try` / `catch`, `IOException`                 | `Result<T, E>` com `?`     |
| Estilo idiomático     | Streams fluentes                               | `for` com match / `?`      |

---

## 📁 Entrega sugerida

- `FiltroNomes.java`
- `filtro_nomes.rs`
- `nomes.txt` (com ao menos 10 nomes variados)

---

**Bom treino!**  
Use este exercício como aquecimento para os desafios mais complexos que virão. 🧠⚙️
