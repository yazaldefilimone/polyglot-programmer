module advanced where

-- Em Agda, podemos definir tipos de dados avançados usando tipos dependentes, funções dependentes e provas.

-- Vamos começar definindo um tipo de dados para árvores binárias.

data Arvore (A : Set) : Set where
  Folha : Arvore A
  No : A → Arvore A → Arvore A → Arvore A

-- O tipo de dados "Arvore" representa uma árvore binária com nós contendo valores do tipo "A".

-- Podemos definir funções sobre árvores.

-- Por exemplo, uma função para calcular a altura de uma árvore.

alturaArvore : {A : Set} → Arvore A → ℕ
alturaArvore Folha = zero
alturaArvore (No _ esq dir) = suc (max (alturaArvore esq) (alturaArvore dir))

-- A função "alturaArvore" usa casamento de padrões para calcular a altura de uma árvore.

-- Agda também suporta funções dependentes, onde o tipo de retorno de uma função pode depender dos valores de seus argumentos.

-- Por exemplo, uma função que retorna uma lista de tamanho n contendo apenas o valor x.

listaRepetida : {A : Set} → ℕ → A → List A
listaRepetida zero _ = []
listaRepetida (suc n) x = x ∷ listaRepetida n x

-- A função "listaRepetida" aceita um número natural "n" e um valor "x" e retorna uma lista de tamanho "n" contendo "x" repetido.

-- Além disso, podemos definir funções dependentes de provas, onde o tipo de retorno de uma função pode depender de uma propriedade provada.

-- Por exemplo, uma função que retorna o enésimo elemento de uma lista.

enecimoElemento : {A : Set} → ℕ → List A → (p : n < comprimento xs) → A
enecimoElemento zero (x ∷ _) _ = x
enecimoElemento (suc n) (_ ∷ xs) p = enecimoElemento n xs (dec p)

-- A função "enecimoElemento" aceita um número natural "n", uma lista "xs" e uma prova "p" de que "n" é menor que o comprimento da lista "xs".

-- Agda também permite definir tipos de dados indutivos com índices, onde o tipo de um valor depende de valores anteriores.

-- Por exemplo, uma lista indexada por seu comprimento.

data ListaIx (A : Set) : ℕ → Set where
  Nil : ListaIx A zero
  Cons : {n : ℕ} → A → ListaIx A n → ListaIx A (suc n)

-- O tipo de dados "ListaIx" representa uma lista indexada por seu comprimento.

-- Podemos definir funções sobre listas indexadas.

-- Por exemplo, uma função para concatenar duas listas indexadas.

concatenar : {A : Set} {m n : ℕ} → ListaIx A m → ListaIx A n → ListaIx A (m + n)
concatenar Nil ys = ys
concatenar (Cons x xs) ys = Cons x (concatenar xs ys)

-- A função "concatenar" aceita duas listas indexadas "xs" e "ys" e retorna a concatenação de "xs" e "ys".

-- Agda também suporta tipos dependentes de caminho, onde o tipo de um valor depende de um valor anterior.

-- Por exemplo, uma lista dependente do seu último elemento.

data ListaDep (A : Set) : (ultimo : A) → Set where
  Único : (x : A) → ListaDep A x
  Adicionar : {ultimo : A} → ListaDep A ultimo → (novo : A) → ListaDep A novo

-- O tipo de dados "ListaDep" representa uma lista cujo último elemento é especificado no tipo.

-- Podemos definir funções sobre listas dependentes.

-- Por exemplo, uma função para adicionar um elemento a uma lista dependente.

adicionarElemento : {A : Set} {ultimo : A} → ListaDep A ultimo → A → ListaDep A novo
adicionarElemento (Único x) novo = Adicionar (Único x) novo
adicionarElemento (Adicionar xs _) novo = Adicionar xs novo

-- A função "adicionarElemento" aceita uma lista dependente "xs" e um novo elemento "novo" e retorna a lista dependente com o novo elemento adicionado.