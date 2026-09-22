# PATENT_GUARDRAILS — Helix Voice

Atualizado: 2026-09-22

## Objetivo

Reduzir risco de propriedade intelectual no Helix Voice mediante:
- desenvolvimento independente;
- pesquisa documentada de estado da técnica e patentes;
- design-around deliberado quando existir risco;
- separação entre análise de reivindicações e implementação;
- evidência mensurável de que a solução Helix possui arquitetura/representação próprias.

Este documento é uma política técnica de engenharia. **Não é parecer jurídico e não garante Freedom to Operate (FTO).** Uma implementação independente ainda pode cair no escopo de uma reivindicação válida.

## Regra universal

Esta política vale para **todo o projeto**: codec, DSP, resampler, PLC/FEC, jitter buffer, AGC, limiter, protocolos, adapters, firmware interfaces, daemon, integração XLX, PU2PNY-OS e futuras funções.

Quando uma solução tocar patente, implementação fechada, segredo comercial ou técnica de terceiros:

1. não copiar;
2. não traduzir;
3. não portar;
4. não reescrever;
5. não reproduzir tabela, constante, estrutura, parâmetro privado ou bitstream proprietário;
6. pesquisar o estado da técnica e as patentes relevantes;
7. identificar o que as reivindicações tentam proteger;
8. criar uma solução própria por princípios de engenharia, preferencialmente com arquitetura e representação diferentes;
9. medir CPU, memória, latência, tamanho e, quando viável, energia;
10. bloquear release se houver risco material não resolvido.

## Fluxo obrigatório de FTO técnico

### Etapa 1 — definir exatamente a função
Descrever o problema sem citar a implementação de terceiros.

Exemplo:
"representar voz inteligível a baixa taxa com robustez a perda"
em vez de
"fazer o que o codec X faz por dentro".

### Etapa 2 — pesquisa pública
Pesquisar, conforme o território/uso planejado:
- WIPO PATENTSCOPE e materiais WIPO;
- EPO Espacenet;
- USPTO;
- INPI Brasil;
- bases nacionais adicionais quando houver implantação/comercialização relevante;
- artigos, standards e literatura técnica pública.

Registrar:
- termo/CPC pesquisado;
- família de patente;
- prioridade;
- titulares;
- jurisdições;
- status legal aparente;
- reivindicações independentes relevantes;
- data da busca;
- links/fontes.

### Etapa 3 — mapa de restrições
Produzir um resumo de engenharia com:
- função protegida alegada;
- elementos essenciais das reivindicações relevantes;
- combinações que devem ser evitadas;
- incertezas;
- alternativas de domínio público/estado da técnica.

**Não copiar embodiments, pseudocódigo, tabelas, parâmetros ou diagramas de implementação para o código Helix.**

### Etapa 4 — design independente
A implementação deve partir de:
- matemática pública;
- literatura científica;
- standards públicos;
- técnicas de domínio público;
- dependências open source com licença compatível;
- experimentos próprios;
- requisitos observáveis de interoperabilidade.

Sempre que tecnicamente razoável, buscar:
- representação diferente;
- framing diferente;
- estados e transições diferentes;
- quantização diferente;
- organização de memória diferente;
- pipeline diferente;
- método de controle diferente;
- trade-offs próprios.

"Diferente" deve ser demonstrável em documentação, não apenas declarado.

### Etapa 5 — meta de eficiência
Sempre que comparável, tentar reduzir:
- ciclos/CPU;
- RSS/heap;
- alocações;
- latência;
- tamanho de binário/dados;
- largura de banda;
- consumo energético estimado/medido.

A prioridade é:
1. segurança jurídica/IP;
2. correção e qualidade;
3. interoperabilidade permitida;
4. eficiência.

Não alegar "mais rápido", "mais leve" ou "consome menos energia" sem benchmark reproduzível.

### Etapa 6 — gate antes de merge/release
Para feature com risco de patente, exigir:
- pesquisa documentada;
- mapa de claims/avoidance;
- justificativa de design independente;
- proveniência das fontes;
- testes;
- benchmark relevante;
- revisão clean-room;
- status FTO técnico: UNREVIEWED / UNCERTAIN / BLOCKED / REVIEWED-LOWER-RISK. LEGAL-CLEARED é reservado a revisão jurídica qualificada.

**UNREVIEWED, UNCERTAIN ou BLOCKED não podem ir para produção/comercialização. REVIEWED-LOWER-RISK continua não sendo garantia jurídica de FTO.**

## Separação de papéis

Quando possível:
- pesquisador de patentes: lê claims/status e produz apenas o mapa de restrições;
- implementador: recebe requisitos públicos + mapa do que evitar e cria a solução do zero;
- revisor: compara a arquitetura Helix com o mapa, sem importar material proprietário.

Uma mesma IA pode executar os papéis em sequência, mas deve manter essa separação documental.

## Fontes oficiais recomendadas

- WIPO — guia de identificação de invenções em domínio público / FTO:
  https://www.wipo.int/publications/en/details.jsp?id=4501
- EPO Espacenet:
  https://www.epo.org/en/searching-for-patents/technical/espacenet
- USPTO patent search guidance:
  https://www.uspto.gov/web/offices/pac/mpep/s904.html
- INPI Brasil — diretrizes de invenções implementadas em computador:
  https://www.gov.br/inpi/pt-br/servicos/patentes/consultas-publicas

## Limites importantes

- busca negativa não prova ausência de patente;
- patente pode existir em uma jurisdição e não em outra;
- família/status/anuidades/expiração precisam ser verificados;
- uma patente expirada pode ser estado da técnica útil, mas seu status deve ser confirmado;
- licença open source não implica automaticamente licença de patente suficiente para qualquer uso;
- compatibilidade de protocolo não autoriza copiar implementação proprietária;
- "clean-room" não substitui análise de reivindicações.

## Regra de bloqueio

Se houver dúvida material sobre uma reivindicação válida que possa cobrir a solução:
- não habilitar por padrão;
- não publicar como produção;
- não comercializar como livre de patente;
- registrar a incerteza;
- buscar design-around adicional ou revisão de advogado/agente de patentes qualificado.

## Registro por feature

Toda feature com risco de patente deve ter um arquivo em:
`docs/patent-reviews/<feature>.md`

Use `docs/PATENT_REVIEW_TEMPLATE.md`.


## Registro central

Todo componente relevante deve constar em PROJECT_PATENT_REGISTER.md.

Feature nova inicia como UNREVIEWED e, portanto, bloqueada para produção até que o review proporcional ao risco seja concluído.
