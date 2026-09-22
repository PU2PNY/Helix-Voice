# Patent Review — Decimator2 / 16 kHz → 8 kHz

Status FTO técnico: **REVIEWED-LOWER-RISK**
Data: 2026-09-22
Escopo: helix-dsp Decimator2

## 1. Problema técnico

Reduzir PCM de 16 kHz para 8 kHz preservando a banda de fala e evitando aliasing, com baixo custo e estado fixo.

## 2. Estado da técnica público

A decimação inteira por:
1. filtro low-pass anti-alias;
2. downsample uniforme;

é técnica clássica de DSP. Documentação pública moderna referencia inclusive programas de DSP do IEEE de 1979 para essa classe de algoritmo.

Windowed-sinc/FIR também é documentado publicamente como método comum de projeto de filtros para sample-rate conversion.

## 3. Leads de patentes pesquisados

Foram encontrados documentos sobre:
- sample-rate conversion fracionária;
- implementações polyphase;
- alocação de MAC/hardware;
- reordenação de estruturas polyphase;
- otimizações específicas de interpolação.

Esses leads são usados como mapa do que evitar.

## 4. Design Helix atual

O Decimator2:
- tem fator fixo 2:1;
- usa FIR direto causal de 129 taps;
- gera seus próprios coeficientes por sinc ideal + janela Blackman;
- normaliza coeficientes internamente;
- calcula somente amostras de saída;
- não usa coeficientes copiados;
- não usa tabela de terceiros;
- não implementa sample-rate conversion fracionária;
- não usa arquitetura polyphase;
- não usa hardware/MAC allocation especial;
- não usa interpolação de fases/coefficient banks.

## 5. Eficiência

- estado fixo: 129 samples + 129 coefficients;
- sem heap no processamento;
- taxa de saída 8 kHz;
- um dot-product de 129 taps por output;
- implementação deliberadamente simples/auditável em vez de otimização patente-específica.

A fase futura pode otimizar somente após novo patent review.

## 6. Evidência

Testes SW:
- preservação de tom de 1 kHz dentro de tolerância;
- supressão de tom fora da banda em 6 kHz;
- streaming com chunks ímpares;
- saída finita.

## 7. Gate

- [x] estado da técnica clássico identificado
- [x] implementação própria
- [x] coeficientes gerados pelo Helix
- [x] sem polyphase/fractional SRC/hardware topology especial
- [x] testes SW
- [ ] busca jurídica completa por jurisdição
- [ ] medição de energia em hardware alvo

## 8. Conclusão

**REVIEWED-LOWER-RISK** em análise técnica preliminar.

Isso não é parecer jurídico, não garante FTO e não autoriza a expressão "patent-free". Antes de comercialização, a avaliação deve ser reconciliada com os territórios/uso efetivos.
