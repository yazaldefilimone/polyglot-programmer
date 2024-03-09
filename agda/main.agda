module main where
  
-- Declarações de módulo começam com "module" seguido pelo nome do módulo.

-- Podemos usar o pragma "{-# OPTIONS --without-K --safe --sized-types #-}" para desativar algumas
-- extensões da linguagem e tornar a compilação mais rápida e previsível. Isso pode ser útil
-- ao trabalhar em projetos maiores.

{-# OPTIONS --without-K --safe --sized-types #-}

-- Importar bibliotecas padrão:
open import Data.Nat
open import Data.String

-- Agda tem um sistema de tipos ricos e expressivos, o que permite uma programação muito precisa.

-- Vamos começar definindo alguns tipos de dados simples.

-- Tipos de dados enumerados são definidos usando a palavra-chave "data".

data Dia : Set where
  Segunda : Dia
  Terca : Dia
  Quarta : Dia
  Quinta : Dia
  Sexta : Dia
  Sabado : Dia
  Domingo : Dia

-- O tipo "Dia" tem sete valores possíveis: Segunda, Terça, Quarta, Quinta, Sexta, Sábado e Domingo.

-- Podemos definir funções sobre esses tipos de dados.

-- Por exemplo, uma função que diz se um dia é fim de semana:

eFimDeSemana : Dia → Bool
eFimDeSemana Sabado = true
eFimDeSemana Domingo = true
eFimDeSemana _ = false

-- Funções podem ser definidas usando casamento de padrões.
-- Aqui, usamos o sublinhado "_" como um padrão curinga para corresponder a qualquer outro dia.

-- Funções podem ter argumentos implícitos. Isso é útil quando o argumento pode ser inferido a partir do contexto.

-- Por exemplo, uma função que repete uma string n vezes:

repetir : {A : Set} → ℕ → A → List A
repetir zero _ = []
repetir (suc n) x = x ∷ repetir n x

-- A função "repetir" aceita um número natural "n", um valor "x" e retorna uma lista contendo "x" repetido "n" vezes.

-- Listas também são uma parte fundamental da programação em Agda.

-- Listas podem ser definidas recursivamente usando o tipo de dados "List".
-- "List" é parametrizado por um tipo "A" e tem dois construtores: "[]" para lista vazia e "x ∷ xs" para lista não vazia,
-- onde "x" é o cabeçalho da lista e "xs" é o resto da lista.

-- Por exemplo, uma função para calcular o comprimento de uma lista:

comprimento : {A : Set} → List A → ℕ
comprimento [] = zero
comprimento (_ ∷ xs) = suc (comprimento xs)

-- A função "comprimento" usa casamento de padrões para calcular o comprimento de uma lista.

-- Agda também suporta funções polimórficas, ou seja, funções que operam em tipos arbitrários.

-- Por exemplo, uma função que inverte uma lista:

inverter : {A : Set} → List A → List A
inverter [] = []
inverter (x ∷ xs) = inverter xs ++ [x]

-- A função "inverter" usa casamento de padrões para inverter uma lista.

-- Além disso, Agda permite definir funções por recursão estrutural, garantindo que todas as chamadas recursivas
-- sejam feitas em argumentos que diminuem estruturalmente.

-- Agora vamos definir uma função para calcular a soma de todos os números de 1 até n.

somaAte : ℕ → ℕ
somaAte zero = zero
somaAte (suc n) = suc n + somaAte n

-- A função "somaAte" usa casamento de padrões para calcular a soma dos números até n.

-- Para lidar com recursão estrutural, Agda oferece o conceito de "ordens bem fundadas".
-- Isso garante que a recursão em um argumento seja feita apenas em valores menores que aquele argumento.

-- Agda suporta prova assistida por computador (ou prova assistida por tipo), permitindo que você prove
-- propriedades sobre seus programas diretamente no código.

-- Por exemplo, vamos provar que a função "somaAte" calcula a soma dos números de 1 até n.

somaAteCorreta : (n : ℕ) → somaAte n ≡ n * (n + 1) div 2
somaAteCorreta zero = refl
somaAteCorreta (suc n) rewrite somaAteCorreta n = begin
  suc n + somaAte n       ≡⟨⟩
  suc n + n * (n + 1) div 2 ∎
  suc n * (suc n + 1) div 2  ∎

-- A prova é construída usando casamento de padrões e reescrita.
-- A função "refl" é usada para introduzir uma igualdade reflexiva.

-- Agda permite definir funções parciais e funções totais.
-- Funções parciais são aquelas que podem falhar para alguns argumentos,
-- enquanto funções totais são definidas para todos os argumentos.

-- Por exemplo, podemos definir uma função que retorna o primeiro elemento de uma lista, se houver:

primeiroElemento : {A : Set} → List A → Maybe A
primeiroElemento [] = nothing
primeiroElemento (x ∷ _) = just x

-- A função "primeiroElemento" retorna "nothing" se a lista estiver vazia e "just x" se a lista não estiver vazia,
-- onde "x" é o primeiro elemento da lista.

-- Agda também suporta funções anónimas (ou lambdas) e compreensão de lista, permitindo uma expressividade adicional na linguagem.

