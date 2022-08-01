# Shopping List

## Visão geral

Este aplicativo permite adicionar a funcionalidade de **Lista de Compras** a uma loja VTEX IO.

Uma lista de compras é uma funcionalidade na qual o cliente pode
criar uma lista e adicionar produtos a ela para que posteriormente possa compra-los ou repetir a compra futuramente.

Este aplicativo é composto pelos seguintes componentes principais:

- Botão de adicionar produto a uma lista de compras.
- Tela de seleção da lista na qual o produto será adicionado.
- Tela de exibição das listas de compras do usuário autenticado.
- Tela de exibição dos produtos de uma lista de compras do usuário autenticado.

&nbsp;

***A lista de compras é um recurso que exige que o usuário esteja autenticado. Sendo assim ao tentar adicionar um produto a uma lista de compras, estando anônimo, o usuário será redirecionado para a tela de autenticação.***
&nbsp;

## Principais recursos

Este aplicativo possui os seguintes recursos:

- Criar lista de compras.
- Duplicar lista de compras.
- Editar nome da lista de compras.
- Excluir uma lista de compras.
- Adicionar produto a uma lista de compras.
- Excluir produto de uma lista de compras.
- Adicionar quantidade desejada de produtos.
- Adicionar um produto da lista no carrinho.
- Adicionar toda a lista de produtos no carrinho.
- Transformar pedido anterior em uma lista de compras.
&nbsp;

## Instalação

A instalação deste aplicativo segue os passos listados abaixo:

1. Adicione **`avantivtexio.shopping-list@1.x`** a propriedade ***peerDependencies*** do arquivo ***manifest.json*** de sua loja.

2. Utilize o Toolbelt da VTEX no terminal de comandos para executar o comando `vtex setup`.

3. Entre na página de instalação do App no painel administrativo da VTEX (***<https://{vendor}.myvtex.com/admin/shopping-list/setup>***) para criar as entidades e configurações inicias necessárias.
4. Adicione os [blocos](docs/blocks.md) nos locais desejados.
5. Estilize os componentes e páginas utilizando as classes [CSS Handles disponíveis](docs/css-handles.md).
6. Teste
7. Pronto ✅
&nbsp;

## Desenvolvimento

### Configurações

Habilite ou desabilite recursos deste aplicativo através do painel administratico da VTEX.

🔗 [Veja todas as **configurações** disponíveis](docs/blocks.md)
&nbsp;

### Blocos

Adicione componentes de Interface utilizando os blocos do VTEX IO disponibilizados por este aplicativo.

🔗 [Veja todos os **blocos** disponíveis](docs/blocks.md)
&nbsp;

### CSS Handles

Estilize os componentes de Interface utilizando as classes CSS Handles disponibilizadas por este aplicativo.

🔗 [Veja todas as **classes** disponíveis](docs/css-handles.md)
&nbsp;

### GraphQL API

Utilize as consultas e mutações disponibilizadas por este aplicativo para buscar e alterar os dados.

🔗 [Veja todas as **Queries e Mutations** disponíveis](docs/graphql.md)
&nbsp;

### REST API

Este aplicativo **não possui REST API**.
&nbsp;

## Observações gerais

*Este aplicativo tem a finalidade de atender a demanda de **lista de compras**, ou seja, atender aos casos de uso de supermecardos por exemplo, onde o cliente monta sua lista e frequentemente a revisita para adicionar aqueles mesmos produtos ao carrinho para compra-los.*

***Não confundir com lista de desejos**, que tem por finalidade criar uma lista de produtos que se deseja comprar uma única vez.*
