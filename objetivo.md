# 1. Objetivo do teste

O objetivo do ambiente é demonstrar que:

```text
Alice e Bob conseguem obter a mesma PPK
a partir de uma API simulada ETSI GS QKD 014,
por meio de Key Providers independentes,
e disponibilizá-la via SKIP para uso com IKEv2/RFC8784.
```

Nesta fase, o “QKD” não será um simulador físico. Será um **Mock KME ETSI 014**, cuja função é entregar a mesma chave para os dois lados, como se essa chave tivesse sido produzida por uma infraestrutura QKD real.

---

# 2. Arquitetura lógica recomendada

```text
                      +--------------------------------+
                      |      ETSI 014 Mock Layer        |
                      |                                |
                      |  Gera chaves sincronizadas      |
                      |  qkd_key_id + key_value         |
                      +---------------+----------------+
                                      |
                    mesma chave para Alice e Bob
                                      |
        +-----------------------------+-----------------------------+
        |                                                           |
+-------v--------+                                          +-------v--------+
|    KME-A       |                                          |    KME-B       |
| Mock ETSI 014  |                                          | Mock ETSI 014  |
| API Alice      |                                          | API Bob        |
+-------+--------+                                          +-------+--------+
        |                                                           |
        | ETSI 014-like                                             | ETSI 014-like
        | GET_STATUS                                                | GET_STATUS
        | GET_KEY                                                   | GET_KEY_WITH_KEY_IDS
        v                                                           v
+-------+--------+                                          +-------+--------+
| KeyProvider-A |                                          | KeyProvider-B |
| SKIP Server A |                                          | SKIP Server B |
| Banco local A |                                          | Banco local B |
+-------+--------+                                          +-------+--------+
        |                                                           |
        | SKIP                                                      | SKIP
        v                                                           v
+-------+--------+           IKEv2 + RFC8784 + IPsec        +-------+--------+
| Cisco Router A|<----------------------------------------->| Cisco Router B|
| Encryptor A   |                                          | Encryptor B   |
+----------------+                                          +----------------+
```

---

# 3. Decisão arquitetural importante: `skip_key_id` determinístico

Para o baseline, eu recomendo que o `skip_key_id` seja derivado de forma determinística a partir do `qkd_key_id`.

Exemplo:

```text
qkd_key_id  = QKD-000001
skip_key_id = SKIP-Alice-Bob-QKD-000001
```

Isso simplifica muito a arquitetura porque:

1. O `KeyProvider-A` recebe `qkd_key_id` da `KME-A`.
2. O `KeyProvider-A` gera `skip_key_id`.
3. O roteador Alice envia `skip_key_id` no IKEv2/RFC8784.
4. O roteador Bob consulta `KeyProvider-B` com esse `skip_key_id`.
5. O `KeyProvider-B` extrai ou resolve o `qkd_key_id`.
6. O `KeyProvider-B` consulta `KME-B` via `GET_KEY_WITH_KEY_IDS`.
7. Bob recebe a mesma PPK.

Assim, você evita, no baseline, um canal extra de sincronização entre `KeyProvider-A` e `KeyProvider-B`.

Em produção, esse identificador poderia ser opaco, por exemplo com HMAC ou UUID, mas isso exigiria sincronização adicional do mapeamento entre Key Providers.

---

# 4. Arquitetura de máquina virtual

## 4.1 Opção recomendada para desenvolvimento

Use uma única VM Ubuntu para hospedar os componentes de software:

```text
VM: qkd-skip-lab
Sistema: Ubuntu Server 22.04 ou 24.04
CPU: 4 vCPU mínimo, 8 vCPU recomendado
RAM: 8 GB mínimo, 16 GB recomendado
Disco: 80 GB mínimo
Runtime: Docker ou Podman
Linguagem: Python 3.11+
Framework: FastAPI
Banco: PostgreSQL ou SQLite inicial
Testes: pytest
```

Dentro dessa VM, execute os serviços como containers:

```text
qkd-skip-lab
├── mock-kme-a
├── mock-kme-b
├── keyprovider-a
├── keyprovider-b
├── db-kme-a
├── db-kme-b
├── db-kp-a
├── db-kp-b
└── test-runner
```

Para a primeira versão, você pode reduzir bancos separados e usar SQLite por serviço. Para uma versão mais realista, use PostgreSQL separado para cada KME e cada Key Provider.

---

## 4.2 Arquitetura com roteadores Cisco

Você terá duas opções.

### Opção A: Cisco Catalyst 8000V

Essa é a opção mais alinhada ao seu objetivo.

```text
Cisco Catalyst 8000V A  ←→  Cisco Catalyst 8000V B
```

A documentação Cisco informa que o recurso de quantum-safe encryption com PPK implementa **RFC 8784 e Cisco SKIP** para IKEv2/IPsec. Também indica suporte em plataformas como Cisco Catalyst 8000V, Catalyst 8300, Catalyst 8500, ASR 1000 e ISR 1000, com observação de que, quando se usa SKIP, a fonte de chave deve enviar a chave em formato hexadecimal. Verificado em: 2026-06-02. ([Cisco][2])

Topologia:

```text
                 Rede de gerenciamento SKIP
                 172.16.100.0/24

+-------------------+                         +-------------------+
| KeyProvider-A     |                         | KeyProvider-B     |
| 172.16.100.10     |                         | 172.16.100.20     |
+---------+---------+                         +---------+---------+
          |                                             |
          | HTTPS/SKIP                                  | HTTPS/SKIP
          |                                             |
+---------v---------+       WAN/IPsec link      +--------v----------+
| Cisco 8000V A     |-------------------------->| Cisco 8000V B     |
| 172.16.100.101    |       10.0.12.0/30        | 172.16.100.102    |
| LAN-A: 192.168.10.1                         | LAN-B: 192.168.20.1|
+-------------------+                         +-------------------+
```

Redes sugeridas:

```text
Mgmt/SKIP: 172.16.100.0/24
WAN:       10.0.12.0/30
LAN-A:     192.168.10.0/24
LAN-B:     192.168.20.0/24
```

---

### Opção B: Simulador de encryptor

Antes de integrar o Cisco real, crie dois clientes simulados:

```text
encryptor-a-sim
encryptor-b-sim
```

Eles fazem apenas:

```text
Encryptor-A:
GET /key?remoteSystemID=Bob

Encryptor-B:
GET /key/{skip_key_id}?remoteSystemID=Alice
```

Essa opção valida a arquitetura sem depender de configuração Cisco.

Minha recomendação é:

```text
Fase 1: usar encryptors simulados.
Fase 2: integrar Cisco Catalyst 8000V.
```

---

# 5. Arquitetura de serviços

## 5.1 Serviços principais

```text
mock-kme-a
mock-kme-b
keyprovider-a
keyprovider-b
encryptor-a-sim
encryptor-b-sim
```

## 5.2 Serviços auxiliares

```text
seed-service
test-runner
observability
```

O `seed-service` gera pares de chaves sincronizadas e registra a mesma chave em `KME-A` e `KME-B`.

---

# 6. Fluxo detalhado ponta a ponta

## 6.1 Geração lógica da chave

O `seed-service` gera:

```json
{
  "qkd_key_id": "QKD-000001",
  "key": "6F4A98B2C1E3...",
  "size": 256,
  "alice_kme": "KME-A",
  "bob_kme": "KME-B",
  "status": "available"
}
```

A mesma chave é inserida em `KME-A` e `KME-B`.

---

## 6.2 KeyProvider-A coleta chave

```http
POST http://mock-kme-a:8001/api/v1/keys/get_key
```

Payload:

```json
{
  "source_sae_id": "KeyProvider-A",
  "target_sae_id": "KeyProvider-B",
  "size": 256,
  "number": 1
}
```

Resposta:

```json
{
  "keys": [
    {
      "key_ID": "QKD-000001",
      "key": "6F4A98B2C1E3..."
    }
  ]
}
```

O `KeyProvider-A` cria:

```text
skip_key_id = SKIP-Alice-Bob-QKD-000001
```

E armazena:

```text
SKIP-Alice-Bob-QKD-000001 → QKD-000001 → 6F4A98B2C1E3...
```

---

## 6.3 Encryptor-A solicita PPK via SKIP

```http
GET https://keyprovider-a:9443/key?remoteSystemID=Bob
```

Resposta SKIP:

```json
{
  "keyId": "SKIP-Alice-Bob-QKD-000001",
  "key": "6F4A98B2C1E3..."
}
```

O SKIP define o modelo em que o encryptor obtém uma chave e um `keyId` de seu Key Provider local, e o peer usa esse `keyId` para recuperar a mesma chave em seu próprio Key Provider. Verificado em: 2026-06-02. ([IETF Datatracker][3])

---

## 6.4 Cisco Router A usa o `keyId`

O roteador Alice usa:

```text
PPK_IDENTITY = SKIP-Alice-Bob-QKD-000001
```

A chave não trafega pelo IKEv2. Apenas o identificador trafega.

O RFC 8784 define uma extensão para IKEv2 que permite resistência pós-quântica por meio do uso de preshared keys, misturando esse segredo adicional no processo de derivação de chaves. Verificado em: 2026-06-02. ([IETF Datatracker][4])

---

## 6.5 Encryptor-B consulta KeyProvider-B

```http
GET https://keyprovider-b:9443/key/SKIP-Alice-Bob-QKD-000001?remoteSystemID=Alice
```

O `KeyProvider-B` resolve:

```text
skip_key_id = SKIP-Alice-Bob-QKD-000001
qkd_key_id  = QKD-000001
```

Então consulta a `KME-B`:

```http
POST http://mock-kme-b:8002/api/v1/keys/get_key_with_key_ids
```

Payload:

```json
{
  "source_sae_id": "KeyProvider-B",
  "target_sae_id": "KeyProvider-A",
  "key_IDs": [
    {
      "key_ID": "QKD-000001"
    }
  ]
}
```

Resposta:

```json
{
  "keys": [
    {
      "key_ID": "QKD-000001",
      "key": "6F4A98B2C1E3..."
    }
  ]
}
```

O `KeyProvider-B` responde ao roteador Bob:

```json
{
  "keyId": "SKIP-Alice-Bob-QKD-000001",
  "key": "6F4A98B2C1E3..."
}
```

---

# 7. Modelo de redes para o lab

## 7.1 Redes Docker

```yaml
networks:
  etsi_net:
    subnet: 172.30.10.0/24

  skip_net:
    subnet: 172.30.20.0/24

  mgmt_net:
    subnet: 172.30.30.0/24
```

Uso:

```text
etsi_net:
mock-kme-a
mock-kme-b
keyprovider-a
keyprovider-b

skip_net:
keyprovider-a
keyprovider-b
encryptor-a-sim
encryptor-b-sim

mgmt_net:
test-runner
observability
```

## 7.2 Redes com Cisco 8000V

```text
Rede SKIP/MGMT:
172.16.100.0/24

Router A:
Gi1 = WAN/IPsec: 10.0.12.1/30
Gi2 = LAN-A:     192.168.10.1/24
Gi3 = MGMT:      172.16.100.101/24

Router B:
Gi1 = WAN/IPsec: 10.0.12.2/30
Gi2 = LAN-B:     192.168.20.1/24
Gi3 = MGMT:      172.16.100.102/24

KeyProvider-A:
172.16.100.10

KeyProvider-B:
172.16.100.20
```

---

# 8. Repositório recomendado

```text
qkd-skip-baseline/
├── README.md
├── docs/
│   ├── architecture.md
│   ├── api-etsi014-mock.md
│   ├── api-skip.md
│   ├── threat-model.md
│   ├── test-plan.md
│   ├── cisco-integration.md
│   └── gsd-codex-workflow.md
├── services/
│   ├── mock-kme/
│   │   ├── app/
│   │   ├── tests/
│   │   ├── Dockerfile
│   │   └── pyproject.toml
│   ├── key-provider/
│   │   ├── app/
│   │   ├── tests/
│   │   ├── Dockerfile
│   │   └── pyproject.toml
│   └── encryptor-sim/
│       ├── app/
│       ├── tests/
│       ├── Dockerfile
│       └── pyproject.toml
├── infra/
│   ├── docker-compose.yml
│   ├── docker-compose.cisco-adjacent.yml
│   ├── certs/
│   └── sql/
├── scripts/
│   ├── seed_keys.py
│   ├── run_baseline_flow.py
│   ├── verify_same_ppk.py
│   └── reset_lab.sh
├── tests/
│   ├── integration/
│   └── e2e/
└── .planning/
```

A pasta `.planning/` será criada e mantida pelo GSD-Core.

---

# 9. Como usar o GSD-Core com Codex

## 9.1 Instalação

No Ubuntu:

```bash
mkdir -p ~/projects
cd ~/projects

mkdir qkd-skip-baseline
cd qkd-skip-baseline

git init
npm --version
node --version

npx @opengsd/gsd-core@latest
```

Durante a instalação, selecione:

```text
Runtime: Codex
Installation: local project
```

O README atual do GSD-Core informa que o instalador pergunta o runtime, incluindo Codex, e se a instalação será global ou local. Também recomenda usar o instalador para compatibilidade entre runtimes, em vez de copiar arquivos manualmente. Verificado em: 2026-06-02. ([GitHub][1])

Depois:

```text
/gsd-new-project
```

---

# 10. Como pensar o projeto dentro do GSD

Use o GSD para separar o trabalho em fases.

## Fase 0: especificação

Objetivo:

```text
Gerar README, arquitetura, contratos de API e plano de testes.
```

Artefatos:

```text
docs/architecture.md
docs/api-etsi014-mock.md
docs/api-skip.md
docs/test-plan.md
docs/threat-model.md
```

## Fase 1: Mock KME ETSI 014

Objetivo:

```text
Implementar KME-A e KME-B como API simples.
```

Funcionalidades:

```text
GET /api/v1/status
POST /api/v1/keys/get_key
POST /api/v1/keys/get_key_with_key_ids
```

## Fase 2: Key Provider SKIP

Objetivo:

```text
Implementar KeyProvider-A e KeyProvider-B.
```

Funcionalidades:

```text
GET /capabilities
GET /key?remoteSystemID=Bob
GET /key/{keyId}?remoteSystemID=Alice
GET /entropy
```

## Fase 3: Encryptor simulado

Objetivo:

```text
Simular o comportamento mínimo de Alice e Bob.
```

Validação:

```text
Alice recebe PPK.
Alice envia keyId.
Bob recebe keyId.
Bob recupera a mesma PPK.
```

## Fase 4: integração Cisco

Objetivo:

```text
Substituir encryptor simulado por Cisco Catalyst 8000V.
```

Validação:

```text
Cisco Router A consulta SKIP.
Cisco Router B consulta SKIP.
IKEv2 usa dynamic PPK.
IPsec estabelece túnel.
```

---

# 11. Prompt inicial para o Codex com GSD

Use este prompt dentro do projeto:

```text
Quero desenvolver um ambiente baseline para integrar uma API simulada ETSI GS QKD 014 com Key Providers SKIP e posterior uso com IKEv2/RFC8784.

Requisitos principais:

1. Não usar NetSquid.
2. Não simular BB84 físico.
3. Não modelar canal quântico.
4. Implementar apenas uma API simples que simula o comportamento lógico do ETSI GS QKD 014.
5. Implementar duas KMEs simuladas: KME-A e KME-B.
6. Implementar dois Key Providers independentes: KeyProvider-A e KeyProvider-B.
7. Cada Key Provider deve ter banco local próprio.
8. O KeyProvider deve atuar como cliente ETSI 014 diante da KME e como servidor SKIP diante do encryptor.
9. O fluxo deve entregar a mesma PPK para Alice e Bob.
10. O skip_key_id deve ser derivado de forma determinística a partir do qkd_key_id no baseline.
11. Implementar primeiro encryptors simulados antes de integrar Cisco.
12. Usar Python 3.11+, FastAPI, pytest e Docker Compose.
13. Criar documentação em docs/.
14. Criar testes unitários, integração e e2e.
15. Criar scripts para seed de chaves, execução do fluxo e verificação de igualdade das PPKs.

Arquitetura:

ETSI 014 Mock KME → Key Provider → SKIP → Encryptor Simulado → futura integração Cisco/RFC8784.

Por favor, gere primeiro o plano de implementação, estrutura de diretórios, contratos de API, modelo de dados e plano de testes antes de escrever código.
```

---

# 12. Contratos de API para o Codex implementar

## 12.1 Mock ETSI 014

### `GET /api/v1/status`

Resposta:

```json
{
  "source_KME_ID": "KME-A",
  "target_KME_ID": "KME-B",
  "available_key_count": 100,
  "max_key_size": 256,
  "status": "ready"
}
```

### `POST /api/v1/keys/get_key`

Request:

```json
{
  "source_sae_id": "KeyProvider-A",
  "target_sae_id": "KeyProvider-B",
  "size": 256,
  "number": 1
}
```

Response:

```json
{
  "keys": [
    {
      "key_ID": "QKD-000001",
      "key": "6F4A98B2C1E3..."
    }
  ]
}
```

### `POST /api/v1/keys/get_key_with_key_ids`

Request:

```json
{
  "source_sae_id": "KeyProvider-B",
  "target_sae_id": "KeyProvider-A",
  "key_IDs": [
    {
      "key_ID": "QKD-000001"
    }
  ]
}
```

Response:

```json
{
  "keys": [
    {
      "key_ID": "QKD-000001",
      "key": "6F4A98B2C1E3..."
    }
  ]
}
```

---

## 12.2 SKIP API

### `GET /capabilities`

Response:

```json
{
  "entropy": true,
  "key": true,
  "algorithm": "ETSI014-MOCK-QKD",
  "localSystemID": "Alice",
  "remoteSystemID": [
    "Bob"
  ]
}
```

### `GET /key?remoteSystemID=Bob`

Response:

```json
{
  "keyId": "SKIP-Alice-Bob-QKD-000001",
  "key": "6F4A98B2C1E3..."
}
```

### `GET /key/{keyId}?remoteSystemID=Alice`

Response:

```json
{
  "keyId": "SKIP-Alice-Bob-QKD-000001",
  "key": "6F4A98B2C1E3..."
}
```

### `GET /entropy`

Response:

```json
{
  "entropy": "AABBCCDDEEFF001122..."
}
```

---

# 13. Modelo de dados

## 13.1 Tabela `qkd_keys`

```sql
CREATE TABLE qkd_keys (
    qkd_key_id        TEXT PRIMARY KEY,
    key_value         TEXT NOT NULL,
    source_kme_id     TEXT NOT NULL,
    target_kme_id     TEXT NOT NULL,
    source_sae_id     TEXT,
    target_sae_id     TEXT,
    key_size_bits     INTEGER NOT NULL,
    status            TEXT NOT NULL,
    created_at        TIMESTAMP NOT NULL,
    delivered_at      TIMESTAMP,
    used_at           TIMESTAMP,
    expires_at        TIMESTAMP
);
```

## 13.2 Tabela `skip_keys`

```sql
CREATE TABLE skip_keys (
    skip_key_id       TEXT PRIMARY KEY,
    qkd_key_id        TEXT NOT NULL,
    key_value         TEXT NOT NULL,
    local_system_id   TEXT NOT NULL,
    remote_system_id  TEXT NOT NULL,
    key_size_bits     INTEGER NOT NULL,
    source            TEXT NOT NULL,
    status            TEXT NOT NULL,
    created_at        TIMESTAMP NOT NULL,
    delivered_at      TIMESTAMP,
    used_at           TIMESTAMP,
    expires_at        TIMESTAMP
);
```

---

# 14. Casos de teste obrigatórios

## 14.1 Testes unitários

```text
test_generate_qkd_key_id()
test_generate_skip_key_id_from_qkd_key_id()
test_key_is_hex_encoded()
test_key_size_is_256_bits()
test_status_transition_available_to_delivered()
test_expired_key_is_rejected()
```

## 14.2 Testes de integração

```text
test_kp_a_gets_key_from_kme_a()
test_kp_b_gets_same_key_from_kme_b_by_key_id()
test_skip_key_id_maps_to_qkd_key_id()
test_kp_a_skip_key_endpoint_returns_key_and_keyid()
test_kp_b_skip_keyid_endpoint_returns_same_key()
```

## 14.3 Teste e2e principal

```text
test_e2e_alice_and_bob_receive_same_ppk()
```

Fluxo:

```text
1. seed-service cria QKD-000001.
2. KME-A e KME-B recebem a mesma chave.
3. KeyProvider-A coleta chave de KME-A.
4. Encryptor-A consulta KeyProvider-A.
5. Encryptor-A recebe keyId e key.
6. Encryptor-B consulta KeyProvider-B com keyId.
7. KeyProvider-B resolve qkd_key_id.
8. KeyProvider-B coleta chave de KME-B.
9. Encryptor-B recebe key.
10. assert key_alice == key_bob.
```

---

# 15. Integração futura com Cisco

A documentação Cisco descreve dynamic PPKs importadas via SKIP como uma alternativa à configuração manual de PPKs, com benefícios de provisionamento automático, renovação e melhor entropia. Verificado em: 2026-06-02. ([Cisco][2])

A configuração Cisco deve vir apenas depois que os seguintes testes passarem:

```text
1. GET /capabilities funciona.
2. GET /key?remoteSystemID=Bob retorna keyId e key em hexadecimal.
3. GET /key/{keyId}?remoteSystemID=Alice retorna a mesma key.
4. A chave possui tamanho compatível.
5. O keyId é estável e resolvível nos dois lados.
6. O servidor SKIP usa HTTPS.
```

Ponto crítico:

```text
Cisco espera chave em formato hexadecimal.
```

Isso já deve ser tratado desde o início no Key Provider.

---

# 16. Métricas do baseline

Colete desde o início:

```text
kme_get_key_latency_ms
kme_get_key_with_id_latency_ms
skip_get_key_latency_ms
skip_get_key_by_id_latency_ms
ppk_match_success_count
ppk_mismatch_count
expired_key_rejection_count
used_key_reuse_attempt_count
```

Essas métricas depois servem para artigo, gráfico e comparação entre:

```text
PPK estática
PPK via ETSI014 mock
PPK via ETSI014 mock + expansão HKDF/RanA
PPK via QKD real
```

---

# 17. Ordem recomendada de implementação

```text
1. Criar README.md e docs/architecture.md.
2. Criar contratos de API.
3. Implementar mock-kme.
4. Implementar seed-service.
5. Implementar key-provider.
6. Implementar encryptor-sim.
7. Implementar testes unitários.
8. Implementar testes de integração.
9. Implementar teste e2e.
10. Adicionar Docker Compose.
11. Adicionar HTTPS local.
12. Validar formato SKIP.
13. Documentar futura integração Cisco.
14. Integrar Cisco Catalyst 8000V.
```

---

# 18. Arquitetura mínima para o primeiro commit

O primeiro commit deve conter apenas:

```text
README.md
docs/architecture.md
docs/api-etsi014-mock.md
docs/api-skip.md
docs/test-plan.md
docker-compose.yml inicial
estrutura vazia de services/
```

Mensagem de commit sugerida:

```bash
git commit -m "docs: define ETSI014-SKIP baseline architecture"
```

---

# 19. Resultado esperado do ambiente

Ao final do baseline, você deve conseguir executar:

```bash
docker compose up --build
python scripts/seed_keys.py
python scripts/run_baseline_flow.py
python scripts/verify_same_ppk.py
pytest
```

E obter algo como:

```text
[OK] KME-A possui QKD-000001
[OK] KME-B possui QKD-000001
[OK] KeyProvider-A entregou SKIP-Alice-Bob-QKD-000001
[OK] KeyProvider-B resolveu SKIP-Alice-Bob-QKD-000001
[OK] PPK Alice == PPK Bob
[OK] Baseline ETSI014 → SKIP validada
```

---

# 20. Hipótese técnica do teste

A hipótese que você deve colocar no projeto é:

```text
Se uma API compatível com o comportamento lógico do ETSI GS QKD 014
disponibiliza chaves simétricas sincronizadas entre Alice e Bob,
então dois Key Providers independentes podem coletar essas chaves,
armazená-las localmente, expô-las via SKIP e entregar a mesma PPK
para uso posterior em IKEv2/RFC8784.
```

Essa hipótese separa corretamente:

```text
QKD físico: fora do escopo inicial.
ETSI 014 lógico: dentro do escopo.
SKIP: dentro do escopo.
RFC 8784: alvo de integração.
Cisco Router: validação posterior.
```