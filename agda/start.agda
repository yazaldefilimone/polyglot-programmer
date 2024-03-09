module start where

-- Agda é uma linguagem de programação funcional com tipos dependentes,
-- o que significa que os tipos podem depender de valores.

-- Declarações de módulo começam com "module" seguido do nome do módulo.

-- Declarações de funções começam com "fun" e são seguidas pelo nome da função
-- e seus argumentos.

-- Por exemplo, vamos definir uma função que incrementa um número natural.

incNat : ℕ → ℕ
incNat x = x + 1

-- A função "incNat" aceita um número natural e retorna o próximo número natural.

-- Tipos são especificados após os dois pontos.

-- Para declarar tipos de dados enumerados, usamos a palavra-chave "data".

-- Por exemplo, vamos definir um tipo de dados para representar cores.

data Cor : Set where
  Vermelho : Cor
  Verde : Cor
  Azul : Cor

-- Aqui, "Cor" é um tipo de dados que possui três construtores: "Vermelho", "Verde" e "Azul".

-- A palavra-chave "Set" é o tipo de todos os tipos habitados. Podemos pensar em "Set" como
-- o tipo de conjunto que inclui todos os tipos que possuem valores.

-- O operador ":" é lido como "tem tipo".

-- Agora, vamos definir uma função que retorna uma mensagem baseada na cor fornecida.

mensagemCor : Cor → String
mensagemCor Vermelho = "Essa é a cor vermelha!"
mensagemCor Verde = "Essa é a cor verde!"
mensagemCor Azul = "Essa é a cor azul!"

-- A função "mensagemCor" aceita um valor do tipo "Cor" e retorna uma mensagem correspondente.

-- Vamos definir uma função para calcular o fatorial de um número natural.

fatorial : ℕ → ℕ
fatorial 0 = 1
fatorial (suc n) = (suc n) * fatorial n

-- A função "fatorial" usa casamento de padrões para calcular o fatorial de um número.

-- Agora, vamos definir um tipo de dados para listas de elementos de um tipo específico.

data Lista (A : Set) : Set where
  Nil : Lista A
  Cons : A → Lista A → Lista A

-- Aqui, "Lista" é um tipo de dados parametrizado por um tipo "A".
-- Tem dois construtores: "Nil", que representa uma lista vazia,
-- e "Cons", que acrescenta um elemento a uma lista existente.

-- Vamos definir uma função para calcular o comprimento de uma lista.

comprimentoLista : {A : Set} → Lista A → ℕ
comprimentoLista Nil = 0
comprimentoLista (Cons _ xs) = 1 + comprimentoLista xs

-- A função "comprimentoLista" aceita uma lista e retorna seu comprimento.

-- Agora, vamos definir uma função para adicionar dois números.

soma : ℕ → ℕ → ℕ
soma x y = x + y

-- A função "soma" aceita dois números naturais e retorna sua soma.

-- Podemos definir funções recursivas usando a palavra-chave "rec".

-- Por exemplo, vamos definir uma função para calcular a soma de todos os números
-- de 1 até n.

somaAteN : ℕ → ℕ
somaAteN 0 = 0
somaAteN (suc n) = suc n + somaAteN n

-- A função "somaAteN" usa casamento de padrões para calcular a soma dos números até n.