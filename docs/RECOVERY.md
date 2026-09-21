# Recovery and Rollback

## Baseline conhecida

Commit antes da implantação da governança:

`62ee9b83ee72ff7ba721413f9f3bdd56f620c0c5`

Branch de retorno:

`baseline/pre-governance-2026-09-21`

Essa baseline possui evidência SW de CI verde.

## Regra para mudanças de risco

Antes de integração XLX, PU2PNY, codecs externos ou mudança grande de DSP:

1. identificar commit/branch atuais;
2. registrar testes PASS que formam a baseline;
3. criar branch/tag/ponto de retorno quando necessário;
4. limitar escopo;
5. testar fora de produção;
6. comparar métricas;
7. reverter se houver regressão grave.

## XLX

Nunca substituir silenciosamente backend/transcoder operacional.

Manter:
- configuração anterior;
- unit file anterior;
- binário/pacote anterior quando aplicável;
- comando documentado de rollback;
- teste de rollback antes de produção.

## PU2PNY-OS

O serviço Helix deve ser opcional. O sistema deve continuar inicializando sem ele.

## Dados

A baseline atual não possui banco nem estado persistente de usuário dentro do Helix. Quando surgir configuração persistente, adicionar:
- formato;
- versão;
- backup;
- migração;
- rollback;
- compatibilidade reversa.

## Evidência

Rollback só é considerado testado quando executado em ENV/HW apropriado. A existência desta documentação não equivale a teste de rollback.
