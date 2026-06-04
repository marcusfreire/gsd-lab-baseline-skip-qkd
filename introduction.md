# Introducao ao QKD ETSI 014 + SKIP Baseline

Este documento explica, em portugues do Brasil, o que este projeto faz, o que
ja foi entregue, como inspecionar e executar a base local, e o que uma pessoa
deve esperar ao rodar os comandos atuais.

O objetivo e ser um guia de entrada para quem nunca viu o repositorio.

## 1. Resumo em uma frase

Este repositorio define uma base local executavel, ainda em Fase 0, para
integrar dois simuladores KME ETSI GS QKD 014, dois Key Providers independentes
expondo uma interface SKIP, e dois encryptors simulados que futuramente poderao
representar consumidores de chaves para IKEv2/RFC8784 PPK.

Em termos praticos: o projeto prepara os contratos, a arquitetura, os nomes de
campos, o desenho dos servicos, os riscos, o modelo de dados e o Docker Compose
antes de implementar a logica completa dos servicos.

## 2. Estado atual do projeto

O projeto esta na Fase 0. Isso e importante.

A Fase 0 nao entrega ainda um sistema completo com APIs funcionando de ponta a
ponta. Ela entrega a forma correta do sistema:

- documentacao de arquitetura;
- contrato da API ETSI 014 simulada;
- contrato da API SKIP;
- modelo de dados;
- plano de testes;
- premissas de seguranca;
- esqueleto Docker Compose com seis servicos;
- diretorios de servico para implementacao futura;
- uso do diretorio `kms/` como simulador KME oficial do baseline.

Portanto, ao executar o Compose hoje, voce deve esperar um scaffold local, nao
um ambiente de APIs completas pronto para trafego real.

## 3. Problema que o projeto resolve

O projeto quer provar, em laboratorio, que e possivel separar corretamente estes
papeis:

- um lado A, com `KeyProvider-A` atuando como `SAE-A`;
- um lado B, com `KeyProvider-B` atuando como `SAE-B`;
- dois KMEs logicos separados, `KME-A` e `KME-B`;
- dois consumidores simulados de SKIP, `Encryptor-A sim` e `Encryptor-B sim`.

O ponto central e evitar uma simplificacao errada: nao existe um unico KME
compartilhado nem um banco central de Key Providers. Cada lado deve operar de
forma independente.

O compartilhamento logico de chave entre `KME-A` e `KME-B` acontece por uma
fonte deterministica/falsa de chaves, controlada por seed, para que o mesmo
`key_ID` ETSI corresponda aos mesmos bytes nos dois KMEs. Isso simula a
compatibilidade logica do material de chave sem tentar simular fisica quantica.

## 4. O que este projeto nao faz

Este repositorio nao modela uma rede quantica real.

Fora de escopo nesta base:

- canal quantico;
- NetSquid;
- BB84;
- QBER;
- reconciliacao;
- privacy amplification;
- simulacao fisica de QKD;
- roteadores Cisco reais;
- IPsec real;
- negociacao IKEv2 real;
- integracao RFC8784 PPK real;
- seguranca de producao;
- KMS de producao.

O foco e entrega logica de chaves: ETSI 014 de um lado, SKIP do outro, com
contratos bem definidos para implementacao posterior.

## 5. Glossario rapido

### QKD

QKD significa Quantum Key Distribution. Neste projeto, QKD aparece somente como
origem logica de material de chave. A Fase 0 nao simula fisica quantica.

### ETSI GS QKD 014

E uma especificacao de interface para entrega de chaves entre KMEs e SAEs. O
projeto preserva o subconjunto relevante de rotas, metodos HTTP, nomes JSON e
ciclo de vida de chaves.

### KME

KME significa Key Management Entity. Aqui existem dois:

- `KME-A`;
- `KME-B`.

Ambos sao instancias logicas do simulador Rust em `kms/`.

### SAE

SAE significa Secure Application Entity. Neste projeto:

- `KeyProvider-A` atua como `SAE-A`;
- `KeyProvider-B` atua como `SAE-B`.

O SAE e a identidade que chama o KME.

### SKIP

SKIP e a interface usada pelos Key Providers para entregar material de chave a
encryptors. O contrato deste projeto segue `draft-singh-skip-00`.

### Encryptor simulado

E um consumidor local de SKIP usado para teste. Ele representa, de forma
controlada, o papel que futuramente poderia ser feito por uma integracao real
com equipamento ou software de criptografia.

### RFC8784 PPK

RFC8784 trata de Post-quantum Preshared Keys para IKEv2. Neste projeto, isso e
um consumidor futuro. A Fase 0 prepara o caminho, mas nao implementa IKEv2.

## 6. Topologia atual

A arquitetura correta tem dois lados:

```text
----------------+        ETSI 014         +---------------------+
| KME-A          | <---------------------- | KeyProvider-A       |
| ETSI simulator |                         | SAE-A + SKIP server |
+----------------+                         +----------+----------+
                                                       |
                                                       | SKIP
                                                       v
                                            +---------------------+
                                            | Encryptor-A sim     |
                                            +----------+----------+
                                                       |
                                                       | handoff do keyId
                                                       v
                                            +---------------------+
                                            | Encryptor-B sim     |
                                            +----------+----------+
                                                       ^
                                                       | SKIP
+----------------+        ETSI 014         +----------+----------+
| KME-B          | <---------------------- | KeyProvider-B       |
| ETSI simulator |                         | SAE-B + SKIP server |
+----------------+                         +---------------------+
```

Regras essenciais:

- `KeyProvider-A` chama somente `KME-A`;
- `KeyProvider-B` chama somente `KME-B`;
- `KME-A` e `KME-B` tem armazenamento separado;
- `KeyProvider-A` e `KeyProvider-B` tem bancos SQLite separados;
- nao existe banco central de Key Providers;
- o `keyId` SKIP e derivado do `key_ID` ETSI, mas nao contem material de chave.

## 7. Servicos declarados no Docker Compose

O arquivo `infra/docker-compose.yml` declara seis servicos:

| Servico | Papel | Porta local |
|---|---|---|
| `kme-a` | Instancia KME para `SAE-A` | `8443:8443` |
| `kme-b` | Instancia KME para `SAE-B` | `9443:8443` |
| `keyprovider-a` | SAE-A diante do KME-A e servidor SKIP para Alice | `8101:8080` |
| `keyprovider-b` | SAE-B diante do KME-B e servidor SKIP para Bob | `8102:8080` |
| `encryptor-a-sim` | Consumidor SKIP do lado A | sem porta publica |
| `encryptor-b-sim` | Consumidor SKIP do lado B | sem porta publica |

Na Fase 0, os comandos dos containers sao mensagens de scaffold. Eles existem
para provar a forma do runtime local, nao para iniciar APIs completas.

## 8. Fluxo ponta a ponta esperado no futuro

Este e o fluxo que a implementacao futura devera executar.

1. `Encryptor-A sim` pede uma chave para `KeyProvider-A`:

   ```http
   GET /key?remoteSystemID=Bob
   ```

2. `KeyProvider-A`, atuando como `SAE-A`, consulta o status de `KME-A` para o
   par `SAE-B`:

   ```http
   GET /api/v1/keys/SAE-B/status
   ```

3. `KeyProvider-A` solicita uma chave de criptografia ao `KME-A`:

   ```http
   POST /api/v1/keys/SAE-B/enc_keys
   content-type: application/json

   { "number": 1, "size": 256 }
   ```

4. `KME-A` retorna um container ETSI com `key_ID` e `key`:

   ```json
   {
     "keys": [
       {
         "key_ID": "QKD-000001",
         "key": "<base64>"
       }
     ],
     "key_container_extension": {}
   }
   ```

5. `KeyProvider-A` converte a chave:

   ```text
   ETSI base64 -> bytes -> SKIP hex
   ```

6. `KeyProvider-A` cria um `keyId` SKIP deterministico:

   ```text
   SKIP-SAE-A-SAE-B-QKD-000001
   ```

   O formato obrigatorio e:

   ```text
   SKIP-{master_SAE_ID}-{slave_SAE_ID}-{key_ID}
   ```

7. `KeyProvider-A` responde ao encryptor A:

   ```json
   {
     "keyId": "SKIP-SAE-A-SAE-B-QKD-000001",
     "key": "<hex>"
   }
   ```

8. `Encryptor-A sim` entrega o `keyId` para `Encryptor-B sim` por um handoff
   simulado. Esse handoff representa, no futuro, o uso de uma identidade PPK.

9. `Encryptor-B sim` pede a chave correspondente ao `KeyProvider-B`:

   ```http
   GET /key/SKIP-SAE-A-SAE-B-QKD-000001?remoteSystemID=Alice
   ```

10. `KeyProvider-B` extrai o `key_ID` ETSI do `keyId` SKIP:

   ```text
   SKIP-SAE-A-SAE-B-QKD-000001 -> QKD-000001
   ```

11. `KeyProvider-B`, atuando como `SAE-B`, chama `KME-B`:

   ```http
   POST /api/v1/keys/SAE-A/dec_keys
   content-type: application/json

   {
     "key_IDs": [
       { "key_ID": "QKD-000001" }
     ]
   }
   ```

12. `KME-B` devolve a mesma chave logica, tambem em base64 no contrato ETSI.

13. `KeyProvider-B` converte:

   ```text
   ETSI base64 -> bytes -> SKIP hex
   ```

14. O teste ponta a ponta deve provar:

   ```text
   key_hex_alice == key_hex_bob
   ```

## 9. Contrato ETSI 014 preservado

O projeto documenta este subconjunto publico de ETSI 014:

| Metodo | Rota | Funcao |
|---|---|---|
| `GET` | `/api/v1/keys/{slave_SAE_ID}/status` | Consulta status e limites do par SAE/KME |
| `POST` | `/api/v1/keys/{slave_SAE_ID}/enc_keys` | Solicita chaves novas para criptografia |
| `GET` | `/api/v1/keys/{slave_SAE_ID}/enc_keys` | Forma simples de solicitacao de chaves |
| `POST` | `/api/v1/keys/{master_SAE_ID}/dec_keys` | Recupera chaves existentes por ID |
| `GET` | `/api/v1/keys/{master_SAE_ID}/dec_keys` | Forma simples de recuperacao por `key_ID` |

Nomes JSON publicos que devem ser preservados:

- `source_KME_ID`;
- `target_KME_ID`;
- `master_SAE_ID`;
- `slave_SAE_ID`;
- `key_ID`;
- `key_IDs`;
- `additional_slave_SAE_IDs`;
- `extension_mandatory`;
- `extension_optional`.

Esses nomes sao importantes porque fazem parte da fidelidade do contrato
ETSI. A implementacao interna pode usar nomes idiomaticos, mas a API publica
deve serializar os campos exatamente assim.

## 10. Contrato SKIP preservado

O contrato SKIP esta documentado contra `draft-singh-skip-00`.

Rotas planejadas:

| Metodo | Rota | Funcao |
|---|---|---|
| `GET` | `/capabilities` | Informa capacidades do servico SKIP |
| `GET` | `/key?remoteSystemID={id}` | Retorna chave nova para sistema remoto |
| `GET` | `/key?remoteSystemID={id}&size={bits}` | Retorna chave nova com tamanho solicitado |
| `GET` | `/key/{keyId}?remoteSystemID={id}` | Retorna chave existente pelo `keyId` |
| `GET` | `/entropy` | Retorna entropia do provider |
| `GET` | `/entropy?minentropy={bits}` | Retorna entropia minima solicitada |

Campos SKIP importantes:

- `localSystemID`;
- `remoteSystemID`;
- `keyId`;
- `key`;
- `entropy`;
- `size`.

Regra de formato:

- chave ETSI (`key`) e base64;
- chave SKIP (`key`) e hexadecimal;
- tamanho padrao do baseline e 256 bits;
- `keyId` nao revela material de chave.

## 11. Modelo de dados em alto nivel

O projeto separa identificadores que poderiam parecer iguais, mas pertencem a
dominios diferentes:

| Identificador | Exemplo | Onde aparece |
|---|---|---|
| KME ID | `KME-A` | Topologia ETSI |
| SAE ID | `SAE-A` | Identidade que chama o KME |
| ETSI `key_ID` | `QKD-000001` | Ciclo de vida da chave no KME |
| SKIP `keyId` | `SKIP-SAE-A-SAE-B-QKD-000001` | Handoff para encryptors |
| `localSystemID` | `Alice` | Identidade local SKIP |
| `remoteSystemID` | `Bob` | Identidade remota SKIP |

Cada registro de chave ETSI deve ter, no minimo:

- `key_ID`;
- `master_sae_id`;
- `slave_sae_id`;
- bytes da chave;
- estado de disponibilidade/consumo.

A operacao `enc_keys` cria ou emite uma chave com metadados de dono:

- mestre: `SAE-A`;
- escravo: `SAE-B`;
- exemplo de ID: `QKD-000001`.

A operacao `dec_keys` deve validar:

- quem chamou e o SAE escravo correto;
- o SAE no caminho e o mestre correto;
- a chave existe;
- a chave ainda nao foi consumida.

Depois de um `dec_keys` bem sucedido, a chave e consumida. Uma segunda chamada
com o mesmo `key_ID` deve falhar.

## 12. Seguranca e limites de seguranca

Este baseline e local e experimental.

Ele nao deve ser tratado como:

- KMS de producao;
- VPN de producao;
- sistema de gerenciamento de chaves pronto para producao;
- implementacao completa de ETSI GS QKD 014;
- implementacao real de QKD fisico.

Regras de seguranca documentadas:

- nunca logar material completo de chave;
- usar fingerprints truncados para diagnostico;
- separar estado de `KeyProvider-A` e `KeyProvider-B`;
- aceitar `x-sae-id` somente em modo HTTP local sem mTLS;
- em modo mTLS, derivar identidade SAE do Common Name do certificado;
- nao deixar `x-sae-id` sobrescrever identidade mTLS;
- rejeitar SAE desconhecido, par desconhecido e dono master/slave incorreto;
- consumir chaves recuperadas por `dec_keys`.

## 13. Estrutura de arquivos mais importante

Arquivos de entrada:

```text
README.md                         Entrada principal em ingles
introduction.md                   Este guia em portugues do Brasil
docs/architecture.md              Arquitetura e topologia
docs/etsi014-alignment.md         Gate de alinhamento ETSI 014
docs/api-etsi014-mock.md          Contrato da API ETSI 014 simulada
docs/api-skip.md                  Contrato SKIP
docs/data-model.md                Identificadores, formatos e persistencia
docs/test-plan.md                 Plano de verificacao atual e futuro
docs/security-assumptions.md      Premissas e limites de seguranca
docs/phase-0-plan.md              Plano tecnico da Fase 0
infra/docker-compose.yml          Scaffold Docker Compose
services/README.md                Fronteira dos servicos futuros
services/keyprovider/README.md    Scaffold do Key Provider
services/encryptor-sim/README.md  Scaffold do encryptor simulado
kms/                              Simulador Rust KME oficial do baseline
```

## 14. Pre-requisitos para rodar localmente

Para inspecionar o projeto:

- Git;
- shell local;
- `rg`/ripgrep recomendado, mas nao obrigatorio.

Para validar o Docker Compose:

- Docker;
- plugin `docker compose`.

Para validar o simulador Rust `kms/`:

- Rust toolchain;
- `cargo`;
- `rustc`.

Para publicar PR pelo fluxo GSD:

- GitHub CLI `gh`;
- autenticacao com `gh auth login`;
- branch de feature;
- working tree limpo.

## 15. Primeiro contato: inspecionar o repositorio

Na raiz do repositorio, rode:

```bash
pwd
git status --short
ls
```

O esperado:

- voce esta na raiz do projeto;
- `README.md`, `docs/`, `infra/`, `services/` e `kms/` existem;
- idealmente `git status --short` nao mostra alteracoes inesperadas.

Para listar os documentos principais:

```bash
find docs -maxdepth 1 -type f -name '*.md' | sort
```

Para listar o scaffold de servicos:

```bash
find services -maxdepth 2 -type f | sort
```

## 16. Verificacao rapida de documentacao

Rode:

```bash
test -f README.md
test -f docs/architecture.md
test -f docs/etsi014-alignment.md
test -f docs/api-etsi014-mock.md
test -f docs/api-skip.md
test -f docs/data-model.md
test -f docs/test-plan.md
test -f docs/security-assumptions.md
test -f infra/docker-compose.yml
test -d kms
```

Se o comando nao imprimir nada e terminar com codigo 0, os arquivos existem.

Para verificar termos essenciais:

```bash
rg -n "KME-A|KME-B|KeyProvider-A|KeyProvider-B" README.md docs infra/docker-compose.yml
rg -n "status|enc_keys|dec_keys" docs/api-etsi014-mock.md
rg -n "source_KME_ID|target_KME_ID|master_SAE_ID|slave_SAE_ID|key_ID|key_IDs" docs
rg -n "SKIP-\\{master_SAE_ID\\}-\\{slave_SAE_ID\\}-\\{key_ID\\}|SKIP-SAE-A-SAE-B-QKD-000001" docs README.md
```

O esperado e que esses comandos encontrem ocorrencias nos documentos. Eles
servem como checagem simples contra deriva de nomenclatura.

## 17. Validar o Docker Compose

O comando mais importante da Fase 0 e:

```bash
docker compose -f infra/docker-compose.yml config
```

O que esse comando faz:

- le o Compose;
- valida a sintaxe;
- resolve a configuracao final;
- imprime a configuracao normalizada.

O que voce deve esperar:

- o comando deve terminar sem erro;
- a saida deve listar `kme-a`, `kme-b`, `keyprovider-a`, `keyprovider-b`,
  `encryptor-a-sim` e `encryptor-b-sim`;
- os volumes `kme_a_data`, `kme_b_data`, `kp_a_data` e `kp_b_data` devem
  aparecer;
- as variaveis de ambiente devem mostrar os pares A/B corretamente.

Se Docker Compose nao estiver instalado, voce pode ver erro parecido com:

```text
docker: 'compose' is not a docker command
```

Nesse caso, instale o plugin Docker Compose antes de continuar.

## 18. Rodar o scaffold com Docker Compose

Opcionalmente, rode:

```bash
docker compose -f infra/docker-compose.yml up
```

Na Fase 0, o esperado e que os containers imprimam mensagens de scaffold e
terminem. Exemplos de mensagens esperadas:

```text
Phase 0 scaffold for KME-A ETSI 014 mTLS simulator from /kms
Phase 0 scaffold for KME-B ETSI 014 mTLS simulator from /kms
Phase 0 scaffold for KeyProvider-A: SAE-A ETSI client to KME-A and SKIP server for Encryptor-A
Phase 0 scaffold for KeyProvider-B: SAE-B ETSI client to KME-B and SKIP server for Encryptor-B
Phase 0 scaffold for Encryptor-A sim: GET /key?remoteSystemID=Bob
Phase 0 scaffold for Encryptor-B sim: GET /key/{keyId}?remoteSystemID=Alice
```

Isso nao e falha. E o comportamento esperado do scaffold atual.

Nao espere ainda:

- endpoint HTTP ativo em `localhost:8101`;
- endpoint HTTP ativo em `localhost:8102`;
- chamada real a `/key`;
- chamada real a `/api/v1/keys/...`;
- banco SQLite populado por servicos reais;
- handoff real entre encryptors.

Para parar e limpar containers criados:

```bash
docker compose -f infra/docker-compose.yml down
```

Para remover tambem volumes locais do Compose:

```bash
docker compose -f infra/docker-compose.yml down -v
```

Use `down -v` com cuidado, pois remove os volumes locais declarados pelo
Compose.

## 19. Validar o simulador Rust `kms/`

Se Rust estiver instalado, voce pode rodar:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Para testar somente o crate `kms`:

```bash
cargo test -p kms
```

Se o diretorio padrao de target do Cargo nao for gravavel, use:

```bash
cargo test --target-dir /tmp/quiin-cargo-target -p kms
```

O que esperar:

- esses comandos verificam o workspace Rust;
- eles exigem `cargo` e `rustc`;
- se a maquina nao tiver Rust instalado, a verificacao nao roda;
- a Fase 0 aceita esse skip em ambientes sem Rust, porque o foco principal da
  fase e documentacao e scaffold.

## 20. Verificar que ainda nao existe logica FastAPI completa

A Fase 0 deve evitar servicos completos antes dos contratos ficarem claros.

Voce pode conferir:

```bash
test ! -f services/keyprovider/app/main.py
test ! -f services/encryptor-sim/app/main.py
```

Se esses comandos terminarem sem imprimir nada, os entrypoints FastAPI ainda
nao existem. Isso e esperado na Fase 0.

## 21. O que ja foi feito

Ate este ponto, o projeto entregou:

1. Entrada principal do projeto em `README.md`.
2. Arquitetura de dois KMEs em `docs/architecture.md`.
3. Alinhamento ETSI 014 em `docs/etsi014-alignment.md`.
4. Contrato ETSI 014 mock em `docs/api-etsi014-mock.md`.
5. Contrato SKIP em `docs/api-skip.md`.
6. Modelo de dados em `docs/data-model.md`.
7. Plano de testes em `docs/test-plan.md`.
8. Premissas de seguranca em `docs/security-assumptions.md`.
9. Plano tecnico da Fase 0 em `docs/phase-0-plan.md`.
10. Docker Compose com seis servicos em `infra/docker-compose.yml`.
11. Scaffold de servicos futuros em `services/`.
12. Oficializacao de `kms/` como simulador KME do baseline.
13. Separacao explicita de estado entre `KeyProvider-A` e `KeyProvider-B`.
14. Separacao explicita entre `KME-A` e `KME-B`.
15. Formato deterministico de `keyId` SKIP:

    ```text
    SKIP-{master_SAE_ID}-{slave_SAE_ID}-{key_ID}
    ```

16. Regra de conversao de chave:

    ```text
    ETSI base64 -> bytes -> SKIP hex
    ```

17. Regra de ciclo de vida `dec_keys`: sucesso consome a chave.
18. Limites de seguranca para evitar falsas promessas de producao.

## 22. O que esperar dos proximos passos de implementacao

As proximas fases provavelmente deverao implementar, testar e endurecer:

- servidor ETSI 014 no `kms/` com as rotas documentadas;
- fonte deterministica/falsa de chaves por seed;
- persistencia por KME;
- Key Provider em Python/FastAPI;
- repositorio SQLite por provider;
- endpoint SKIP `/capabilities`;
- endpoint SKIP `/key`;
- endpoint SKIP `/key/{keyId}`;
- endpoint SKIP `/entropy`;
- conversao base64 para bytes e bytes para hex;
- fingerprints de chaves em logs;
- testes de contrato ETSI;
- testes de contrato SKIP;
- teste ponta a ponta `key_hex_alice == key_hex_bob`;
- modo HTTP local com `x-sae-id`;
- modo mTLS com identidade por Common Name;
- rejeicoes de topologia e ownership incorretos;
- hardening futuro para integracao real.

## 23. Como interpretar falhas comuns

### `docker compose` nao existe

Provavel causa: Docker Compose plugin nao instalado.

Acao: instalar Docker Compose e rodar novamente:

```bash
docker compose -f infra/docker-compose.yml config
```

### Containers sobem e encerram rapidamente

Isso e esperado na Fase 0. Os containers executam comandos de scaffold que
imprimem mensagens e terminam.

### `curl localhost:8101/key?...` falha

Esperado na Fase 0. Ainda nao existe servidor FastAPI ativo para SKIP.

### `cargo` nao encontrado

Provavel causa: Rust nao instalado.

Acao: instalar Rust se voce quiser validar `kms/`. Para entender a Fase 0, os
documentos e o Compose ja sao suficientes.

### `gh` nao autenticado ao rodar fluxo de ship

Para criar PR via GSD, rode:

```bash
gh auth login
```

Depois, com branch de feature e arvore limpa:

```bash
gsd-ship
```

## 24. Checklist recomendado para um novo leitor

Siga esta ordem:

1. Leia este arquivo inteiro.
2. Leia `README.md`.
3. Leia `docs/architecture.md`.
4. Leia `docs/api-etsi014-mock.md`.
5. Leia `docs/api-skip.md`.
6. Leia `docs/data-model.md`.
7. Leia `docs/security-assumptions.md`.
8. Rode:

   ```bash
   docker compose -f infra/docker-compose.yml config
   ```

9. Opcionalmente rode:

   ```bash
   docker compose -f infra/docker-compose.yml up
   ```

10. Se tiver Rust, rode:

    ```bash
    cargo test -p kms
    ```

11. Confirme que voce entendeu o limite principal: a Fase 0 define o contrato e
    a forma de execucao local; a logica completa dos servicos vira depois.

## 25. Invariante principal do projeto

A implementacao futura so estara correta se, no fluxo completo:

```text
Encryptor-A -> KeyProvider-A -> KME-A
handoff do keyId
Encryptor-B -> KeyProvider-B -> KME-B
```

o material de chave entregue pelos dois lados for igual:

```text
key_hex_alice == key_hex_bob
```

Ao mesmo tempo, essa igualdade nao pode ser obtida por atalho indevido:

- nao pode usar banco central de Key Providers;
- nao pode fazer `KeyProvider-A` chamar `KME-B`;
- nao pode fazer `KeyProvider-B` chamar `KME-A`;
- nao pode esconder a topologia em um KME unico;
- nao pode revelar material de chave no `keyId`;
- nao pode permitir `dec_keys` repetido para a mesma chave ja consumida.

Essa e a essencia do baseline.
