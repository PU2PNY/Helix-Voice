# Patent Review — HVC packet-loss concealment (PLC)

Status FTO técnico: **UNCERTAIN**
Data: 2026-09-22
Escopo: HvcDecoder::conceal_loss

## 1. Problema técnico

Evitar descontinuidade severa quando um frame HVC de 20 ms é perdido, mantendo:
- saída bounded;
- baixo custo;
- zero alocação;
- comportamento seguro em perdas consecutivas.

## 2. Pesquisa preliminar

A literatura patentária pesquisada descreve:
- frame repeat como uma das técnicas mais simples/comuns de concealment;
- repetição de pitch/período com classificação;
- crossfade;
- attenuation/fade-to-silence;
- adaptação conforme número de perdas;
- reconstrução usando history/future frames.

Existem documentos que tratam parte dessas técnicas como prior art e outros que reivindicam combinações/melhorias específicas.

## 3. Design Helix atual

O protótipo atual:
- mantém somente o último frame decodificado;
- em perda, reutiliza esse frame com fator de atenuação fixo;
- aplica uma transição curta no início do frame;
- após número limitado de perdas, produz silêncio;
- não estima pitch;
- não classifica voiced/noise;
- não usa future frame;
- não faz time-warp/re-phasing;
- não usa overlap-add espectral;
- não usa predictor de longo prazo.

## 4. Risco

Embora frame repeat simples seja descrito publicamente como técnica comum, a combinação com attenuation/crossfade aparece em famílias relacionadas a PLC.

Por política conservadora:
- manter o PLC atual como pesquisa;
- não habilitar em produção;
- não descrevê-lo como patent-free;
- antes de promover, escolher entre um design-around ainda mais simples ou review jurídico/claims por território.

## 5. Alternativa conservadora

Para um modo de segurança temporário, **silence insertion** de frame perdido pode ser mantida como fallback operacional separado, sem extrapolação de fala. Isso sacrifica qualidade mas reduz a complexidade do mecanismo de concealment. Mesmo assim, release comercial continua sujeito ao registro central/FTO.

## 6. Gate

- [x] pesquisa preliminar
- [x] mapa de técnicas a evitar
- [x] protótipo limitado/bounded
- [x] testes de estabilidade
- [ ] claims/status oficiais por território
- [ ] design-around final
- [ ] ABX/qualidade
- [ ] legal review quando aplicável

## 7. Conclusão

**UNCERTAIN**

PLC atual é somente experimental e está bloqueado para produção.
