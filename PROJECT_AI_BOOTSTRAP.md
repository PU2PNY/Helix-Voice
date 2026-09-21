# PROJECT_AI_BOOTSTRAP — instrução curta para agentes

Antes de analisar, planejar, programar, corrigir, publicar, fazer deploy ou modificar Helix Voice:

1. leia `PROJECT_START_HERE.md`;
2. leia `PROJECT_MASTER_SPEC.md`;
3. leia `PROJECT_RELEASE_STATUS.md`;
4. leia `PROJECT_TEST_MATRIX.md`;
5. leia `PROJECT_CHANGELOG.md`;
6. siga os documentos adicionais apontados pelo START_HERE;
7. identifique a branch real e o commit atual;
8. consulte a implementação existente antes de propor substituição;
9. preserve requisitos aprovados e testes comprovadamente PASS;
10. aplique obrigatoriamente `CLEAN_ROOM.md`;
11. nunca transforme SW em HW/PROD por inferência;
12. toda decisão permanente relevante deve atualizar documentação, teste/status e changelog.

O GitHub é a memória persistente. A conversa atual é apenas o ambiente de trabalho.

Nunca declare algo testado, integrado, compatível ou pronto sem evidência correspondente.

## Regra de execução autônoma

Quando o objetivo estiver suficientemente definido, execute a tarefa de ponta a ponta com o mínimo de intervenção humana.

- falha de build/teste/integração não encerra o trabalho: diagnostique, corrija com a menor mudança segura e teste novamente;
- dúvida técnica verificável deve ser resolvida por documentação, inspeção de código, experimento reproduzível ou teste antes de perguntar ao usuário;
- preserve funcionamento já aprovado, requisitos, rollback e níveis de evidência;
- não oculte, ignore ou transforme teste FAIL em PASS;
- não declare "pronto", "completo", "compatível", "melhor" ou "produção" sem a evidência exigida;
- peça intervenção humana somente para bloqueios realmente externos, como credenciais indisponíveis, hardware físico inacessível, autorização exclusiva ou decisão irreversível que não possa ser inferida com segurança;
- conclua o máximo possível na execução atual e nunca prometa trabalho em segundo plano que não esteja efetivamente sendo executado.
