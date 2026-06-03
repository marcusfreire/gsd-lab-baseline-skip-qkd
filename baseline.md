# Baseline: Integração ETSI GS QKD 014 Simulado + SKIP + RFC 8784

## 1. Visão geral

Este projeto define uma arquitetura baseline para integrar chaves provenientes de uma infraestrutura QKD simulada com mecanismos clássicos de proteção de rede, em especial IPsec/IKEv2 com Post-quantum Preshared Key (PPK), conforme o RFC 8784.

Nesta baseline, não será modelado o canal quântico. O foco inicial é a arquitetura de integração, ou seja, o caminho percorrido pela chave após sua disponibilização pela camada QKD.

Para isso, será implementada uma **API simples que simula o comportamento lógico do ETSI GS QKD 014**. Essa API representa uma KME simulada capaz de disponibilizar chaves simétricas sincronizadas para dois lados da comunicação: Alice e Bob.

O objetivo é validar o seguinte fluxo:

```text
ETSI 014 Mock KME
        ↓
Key Provider
        ↓
SKIP
        ↓
Encryptor
        ↓
IKEv2/RFC 8784
        ↓
IPsec com PPK derivada de QKD
````

A baseline busca responder à seguinte pergunta arquitetural:

> Como integrar uma fonte de chaves QKD, exposta por uma interface compatível com ETSI GS QKD 014, a um sistema SKIP que entrega PPKs para uso em IKEv2/RFC 8784?

---

## 2. Escopo da baseline

### 2.1 Dentro do escopo

Esta baseline cobre:

* simulação lógica da entrega de chaves QKD;
* API compatível com o comportamento essencial do ETSI GS QKD 014;
* duas entidades KME simuladas: `KME-A` e `KME-B`;
* dois Key Providers independentes: `KeyProvider-A` e `KeyProvider-B`;
* bancos locais separados para cada Key Provider;
* mapeamento entre identificadores ETSI/QKD e identificadores SKIP;
* exposição de endpoints SKIP para os encryptors;
* entrega da mesma PPK para Alice e Bob;
* preparação para uso da PPK no RFC 8784.

### 2.2 Fora do escopo

Esta baseline não cobre:

* uso de NetSquid;
* simulação física do BB84;
* modelagem de canal quântico;
* cálculo de QBER;
* reconciliação de chaves;
* privacy amplification;
* hardware QKD real;
* trusted-node routing;
* key relay multi-hop em QKDN;
* integração completa com roteadores reais;
* avaliação criptográfica da segurança física do QKD.

Nesta etapa, a API simulada representa apenas a disponibilidade lógica de chaves simétricas sincronizadas, como se essas chaves tivessem sido produzidas por uma infraestrutura QKD real.

---

## 3. Arquitetura de alto nível

```text
                 +--------------------------------------+
                 |        ETSI 014 Mock QKD Layer        |
                 |                                      |
                 |  Gera chaves simétricas sincronizadas |
                 |  e disponibiliza via API ETSI 014     |
                 +-------------------+------------------+
                                     |
                                     |
             mesma chave e mesmo qkd_key_id para Alice e Bob
                                     |
        +----------------------------+----------------------------+
        |                                                         |
+-------v--------+                                      +---------v------+
|    KME-A       |                                      |     KME-B      |
| Mock ETSI 014  |                                      | Mock ETSI 014  |
+-------+--------+                                      +---------+------+
        |                                                         |
        | ETSI 014-like                                           | ETSI 014-like
        | GET_STATUS                                              | GET_STATUS
        | GET_KEY                                                 | GET_KEY_WITH_KEY_IDS
        |                                                         |
+-------v--------+                                      +---------v------+
| KeyProvider-A |                                      | KeyProvider-B  |
| Banco local A |                                      | Banco local B  |
+-------+--------+                                      +---------+------+
        |                                                         |
        | SKIP                                                    | SKIP
        | GET /capabilities                                      | GET /capabilities
        | GET /key?remoteSystemID=Bob                            | GET /key/{keyId}
        |                                                         |
+-------v--------+                                      +---------v------+
| Encryptor-A   |<------------ IKEv2/RFC8784 ---------->| Encryptor-B    |
| IPsec peer    |                                      | IPsec peer     |
+----------------+                                      +----------------+
```

---

## 4. Ideia central da arquitetura

A arquitetura separa três domínios.

### 4.1 Domínio ETSI 014 simulado

Esse domínio representa a entrega lógica de chaves QKD.

Ele é composto por:

```text
KME-A
KME-B
ETSI 014 Mock API
```

A função desse domínio é garantir que Alice e Bob consigam obter a mesma chave a partir de suas respectivas KMEs.

---

### 4.2 Domínio Key Provider

Esse domínio representa a camada intermediária entre o mundo QKD/ETSI e o mundo SKIP.

Ele é composto por:

```text
KeyProvider-A
KeyProvider-B
Banco local A
Banco local B
```

A função dos Key Providers é:

* consultar a KME local;
* coletar chaves QKD;
* armazenar chaves no banco local;
* criar identificadores SKIP;
* mapear `skip_key_id` para `qkd_key_id`;
* disponibilizar a chave ao encryptor via SKIP.

---

### 4.3 Domínio SKIP/RFC 8784

Esse domínio representa o consumo da chave pelos dispositivos de segurança de rede.

Ele é composto por:

```text
Encryptor-A
Encryptor-B
SKIP API
IKEv2/RFC8784
IPsec
```

A função desse domínio é:

* obter uma PPK a partir do Key Provider;
* trocar o identificador da PPK durante o IKEv2;
* recuperar a mesma PPK no lado remoto;
* misturar a PPK no processo de derivação do IKEv2;
* estabelecer uma sessão IPsec com reforço pós-quântico.

---

## 5. Componentes da baseline

## 5.1 ETSI 014 Mock KME

A `ETSI 014 Mock KME` representa uma entidade KME simplificada.

Ela não executa QKD real.

Ela não simula BB84.

Ela não modela o canal quântico.

Sua única função é simular o comportamento lógico esperado de uma KME compatível com ETSI GS QKD 014.

### Responsabilidades

A KME simulada deve:

* gerar ou receber chaves simétricas aleatórias;
* associar cada chave a um `qkd_key_id`;
* disponibilizar a mesma chave para Alice e Bob;
* responder a consultas de status;
* entregar uma chave nova para o lado iniciador;
* entregar uma chave existente por identificador para o lado respondedor;
* marcar chaves como entregues, usadas ou expiradas.

### Endpoints mínimos

A API deve expor, no mínimo:

```text
GET_STATUS
GET_KEY
GET_KEY_WITH_KEY_IDS
```

Esses métodos não precisam reproduzir integralmente a especificação ETSI GS QKD 014 nesta fase. O objetivo é reproduzir o comportamento lógico necessário para a integração.

---

## 5.2 KME-A e KME-B

`KME-A` representa a KME do lado de Alice.

`KME-B` representa a KME do lado de Bob.

As duas KMEs devem possuir acesso ao mesmo conjunto lógico de chaves sincronizadas.

Exemplo:

```text
KME-A:
qkd_key_id = QKD-000001
key        = A1B2C3...

KME-B:
qkd_key_id = QKD-000001
key        = A1B2C3...
```

A sincronização entre `KME-A` e `KME-B` é simulada. Não será implementado nesta etapa nenhum protocolo físico ou quântico de sincronização.

### Responsabilidades das KMEs

Cada KME deve:

* manter um banco de chaves QKD simuladas;
* associar chaves a pares de comunicação;
* disponibilizar chaves para Key Providers autorizados;
* controlar status de uso;
* rejeitar chaves inexistentes, expiradas ou já consumidas.

---

## 5.3 KeyProvider-A e KeyProvider-B

O Key Provider é o componente central desta baseline.

Ele atua como ponte entre:

```text
ETSI 014 Mock KME  →  Key Provider  →  SKIP  →  Encryptor
```

Cada lado possui seu próprio Key Provider.

Não há banco central.

```text
Alice → KeyProvider-A → Banco local A
Bob   → KeyProvider-B → Banco local B
```

### Responsabilidades do Key Provider

Cada Key Provider deve:

* consultar sua KME local por meio da API ETSI 014 simulada;
* obter chaves QKD;
* armazenar as chaves localmente;
* gerar ou registrar um `skip_key_id`;
* mapear o `skip_key_id` para o `qkd_key_id`;
* expor endpoints compatíveis com SKIP;
* entregar a PPK ao encryptor local;
* aplicar política de uso único, expiração e revogação.

---

## 5.4 Encryptor-A e Encryptor-B

Os encryptors representam os dispositivos que irão estabelecer uma sessão segura.

Nesta baseline, eles podem ser implementados inicialmente como clientes simulados. Não é necessário integrar imediatamente com roteadores reais.

### Responsabilidades dos encryptors

O `Encryptor-A` deve:

* consultar `KeyProvider-A` via SKIP;
* receber uma PPK e seu `skip_key_id`;
* iniciar a negociação IKEv2/RFC8784;
* enviar o identificador da PPK para Bob.

O `Encryptor-B` deve:

* receber o `skip_key_id`;
* consultar `KeyProvider-B` via SKIP;
* recuperar a mesma PPK;
* usar a PPK no processo de derivação de chaves do IKEv2/RFC8784.

---

## 6. Identificadores de chave

A arquitetura utiliza dois identificadores distintos.

---

## 6.1 `qkd_key_id`

O `qkd_key_id` pertence ao domínio ETSI/QKD.

Ele é criado ou gerenciado pela KME.

Exemplo:

```text
qkd_key_id = QKD-2026-000001
```

Esse identificador é usado para recuperar uma chave dentro da infraestrutura QKD simulada.

---

## 6.2 `skip_key_id`

O `skip_key_id` pertence ao domínio SKIP.

Ele é exposto aos encryptors.

Exemplo:

```text
skip_key_id = SKIP-A-B-000001
```

Esse identificador é usado durante a negociação IKEv2/RFC8784 para indicar qual PPK deve ser usada.

---

## 6.3 Mapeamento entre identificadores

O Key Provider deve manter o seguinte mapeamento:

```text
skip_key_id → qkd_key_id → key_value
```

Exemplo:

```text
skip_key_id     = SKIP-A-B-000001
qkd_key_id      = QKD-2026-000001
key_value       = A1B2C3...
local_system_id = Alice
remote_system_id = Bob
status          = available
```

O encryptor não precisa conhecer o `qkd_key_id`.

A KME não precisa conhecer o `skip_key_id`, exceto se uma política de integração decidir registrar esse dado como metadado.

---

## 7. Fluxo principal da baseline

## 7.1 Geração lógica da chave QKD

A camada simulada gera uma chave aleatória:

```text
key = random(256 bits)
qkd_key_id = QKD-000001
```

A mesma chave é disponibilizada para os dois lados:

```text
KME-A recebe: QKD-000001, key
KME-B recebe: QKD-000001, key
```

---

## 7.2 Coleta da chave pelo KeyProvider-A

O `KeyProvider-A` solicita uma chave à `KME-A`:

```http
POST /api/v1/keys/get_key
```

Exemplo de requisição:

```json
{
  "source_sae_id": "KeyProvider-A",
  "target_sae_id": "KeyProvider-B",
  "size": 256,
  "number": 1
}
```

Exemplo de resposta:

```json
{
  "keys": [
    {
      "key_ID": "QKD-000001",
      "key": "A1B2C3..."
    }
  ]
}
```

O `KeyProvider-A` armazena localmente:

```text
qkd_key_id = QKD-000001
key_value = A1B2C3...
peer = Bob
status = available
```

---

## 7.3 Criação do identificador SKIP

Após coletar a chave, o `KeyProvider-A` cria um identificador SKIP:

```text
skip_key_id = SKIP-A-B-000001
```

E registra o mapeamento:

```text
SKIP-A-B-000001 → QKD-000001 → A1B2C3...
```

---

## 7.4 Coleta da mesma chave pelo KeyProvider-B

O `KeyProvider-B` precisa obter a chave correspondente em sua KME local.

Ele consulta a `KME-B` usando o identificador QKD:

```http
POST /api/v1/keys/get_key_with_key_ids
```

Exemplo de requisição:

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

Exemplo de resposta:

```json
{
  "keys": [
    {
      "key_ID": "QKD-000001",
      "key": "A1B2C3..."
    }
  ]
}
```

O `KeyProvider-B` armazena localmente:

```text
skip_key_id = SKIP-A-B-000001
qkd_key_id = QKD-000001
key_value = A1B2C3...
peer = Alice
status = available
```

---

## 7.5 Entrega da chave para o Encryptor-A via SKIP

O `Encryptor-A` solicita uma chave ao `KeyProvider-A`:

```http
GET /key?remoteSystemID=Bob
```

Resposta:

```json
{
  "keyId": "SKIP-A-B-000001",
  "key": "A1B2C3..."
}
```

Nesse momento, o `Encryptor-A` passa a possuir:

```text
PPK = A1B2C3...
PPK_ID = SKIP-A-B-000001
```

---

## 7.6 Envio do identificador ao Encryptor-B

Durante a negociação IKEv2/RFC8784, o `Encryptor-A` envia o identificador da PPK:

```text
PPK_IDENTITY = SKIP-A-B-000001
```

A chave em si não é enviada pelo IKEv2.

Apenas o identificador é enviado.

---

## 7.7 Recuperação da chave pelo Encryptor-B via SKIP

O `Encryptor-B` consulta seu Key Provider local:

```http
GET /key/SKIP-A-B-000001?remoteSystemID=Alice
```

Resposta:

```json
{
  "keyId": "SKIP-A-B-000001",
  "key": "A1B2C3..."
}
```

Agora os dois lados possuem a mesma PPK:

```text
Encryptor-A: PPK = A1B2C3...
Encryptor-B: PPK = A1B2C3...
```

---

## 7.8 Uso da PPK no RFC 8784

A PPK é misturada no processo de derivação de chaves do IKEv2.

Conceitualmente:

```text
segredo IKEv2 clássico = DH/ECDH
segredo adicional      = PPK derivada de QKD
chave final IPsec      = PRF(PPK, material intermediário do IKEv2)
```

Assim, a sessão IPsec passa a depender de dois elementos:

1. o segredo negociado pelo IKEv2;
2. a PPK fornecida pela cadeia ETSI 014 simulada → Key Provider → SKIP.

---

## 8. Endpoints mínimos

## 8.1 Endpoints da KME simulada

### `GET /api/v1/status`

Consulta o estado da KME.

Resposta esperada:

```json
{
  "source_KME_ID": "KME-A",
  "target_KME_ID": "KME-B",
  "available_key_count": 100,
  "max_key_size": 256,
  "status": "ready"
}
```

---

### `POST /api/v1/keys/get_key`

Solicita uma ou mais chaves novas.

Exemplo de requisição:

```json
{
  "source_sae_id": "KeyProvider-A",
  "target_sae_id": "KeyProvider-B",
  "size": 256,
  "number": 1
}
```

Exemplo de resposta:

```json
{
  "keys": [
    {
      "key_ID": "QKD-000001",
      "key": "A1B2C3..."
    }
  ]
}
```

---

### `POST /api/v1/keys/get_key_with_key_ids`

Solicita uma chave pelo identificador.

Exemplo de requisição:

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

Exemplo de resposta:

```json
{
  "keys": [
    {
      "key_ID": "QKD-000001",
      "key": "A1B2C3..."
    }
  ]
}
```

---

## 8.2 Endpoints SKIP do Key Provider

### `GET /capabilities`

Retorna as capacidades do Key Provider.

Exemplo:

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

---

### `GET /key?remoteSystemID=Bob`

Solicita uma nova chave para comunicação com Bob.

Resposta:

```json
{
  "keyId": "SKIP-A-B-000001",
  "key": "A1B2C3..."
}
```

---

### `GET /key/{keyId}?remoteSystemID=Alice`

Solicita uma chave específica pelo identificador SKIP.

Exemplo:

```http
GET /key/SKIP-A-B-000001?remoteSystemID=Alice
```

Resposta:

```json
{
  "keyId": "SKIP-A-B-000001",
  "key": "A1B2C3..."
}
```

---

### `GET /entropy`

Opcionalmente, retorna entropia da KME ou de fonte local.

Exemplo:

```json
{
  "entropy": "F9A8B7C6D5..."
}
```

---

## 9. Modelo mínimo de banco de dados

## 9.1 Banco da KME simulada

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

Status recomendados:

```text
available
reserved
delivered
used
expired
revoked
```

---

## 9.2 Banco do Key Provider

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

Valores recomendados para `source`:

```text
ETSI014_MOCK
SIMULATED_QKD
DERIVED_HKDF
DERIVED_RANA
REAL_QKD
```

Status recomendados:

```text
available
reserved
delivered
used
expired
revoked
mismatch
```

---

## 10. Políticas mínimas

## 10.1 Uso único

Por padrão, uma PPK deve ser usada uma única vez para estabelecer uma sessão IPsec.

Após a entrega para o encryptor, a chave pode ser marcada como:

```text
delivered
```

Após confirmação de uso, pode ser marcada como:

```text
used
```

---

## 10.2 Expiração

Cada chave deve possuir um tempo de validade.

Exemplo:

```text
expires_at = created_at + 300 segundos
```

Chaves expiradas não devem ser entregues.

---

## 10.3 Vinculação ao peer

Uma chave deve estar vinculada a um par específico.

Exemplo:

```text
Alice ↔ Bob
```

O `KeyProvider-A` não deve entregar para Bob uma chave associada a outro peer.

---

## 10.4 Verificação de sincronismo

Quando Alice e Bob recuperam uma chave com o mesmo `skip_key_id`, o valor da chave deve ser idêntico.

A baseline deve permitir detectar:

* `skip_key_id` inexistente;
* `qkd_key_id` inexistente;
* chave expirada;
* chave já usada;
* chave divergente entre Alice e Bob;
* associação incorreta de peer.

---

## 11. Fluxo resumido ponta a ponta

```text
1. A KME simulada gera:
   qkd_key_id = QKD-000001
   key = A1B2C3...

2. A mesma chave é registrada em KME-A e KME-B.

3. KeyProvider-A chama KME-A via GET_KEY.

4. KME-A retorna QKD-000001 e A1B2C3...

5. KeyProvider-A cria:
   skip_key_id = SKIP-A-B-000001

6. KeyProvider-A armazena:
   SKIP-A-B-000001 → QKD-000001 → A1B2C3...

7. KeyProvider-B chama KME-B via GET_KEY_WITH_KEY_IDS.

8. KME-B retorna QKD-000001 e A1B2C3...

9. KeyProvider-B armazena:
   SKIP-A-B-000001 → QKD-000001 → A1B2C3...

10. Encryptor-A chama:
    GET /key?remoteSystemID=Bob

11. KeyProvider-A retorna:
    keyId = SKIP-A-B-000001
    key = A1B2C3...

12. Encryptor-A envia o keyId para Bob no fluxo IKEv2/RFC8784.

13. Encryptor-B chama:
    GET /key/SKIP-A-B-000001?remoteSystemID=Alice

14. KeyProvider-B retorna:
    keyId = SKIP-A-B-000001
    key = A1B2C3...

15. Encryptor-A e Encryptor-B usam a mesma PPK.

16. IKEv2 mistura a PPK na derivação de chaves conforme RFC 8784.

17. O túnel IPsec é estabelecido.

18. A chave é marcada como usada.
```

---

## 12. Validações esperadas

A baseline deve validar:

* a mesma chave é entregue aos dois lados;
* `qkd_key_id` é corretamente mapeado para `skip_key_id`;
* o Key Provider consegue atuar como cliente ETSI 014 e servidor SKIP;
* o encryptor nunca acessa diretamente a KME;
* o encryptor recebe apenas `keyId` e `key` via SKIP;
* Bob consegue recuperar a mesma chave a partir do `keyId`;
* chaves expiradas ou usadas não são reutilizadas;
* erros de sincronização são detectados;
* o fluxo está pronto para integração posterior com IKEv2/RFC8784 real.

---

## 13. Justificativa arquitetural

A baseline separa a arquitetura em camadas independentes:

```text
Camada ETSI 014 simulada:
simula a entrega de chaves QKD sincronizadas.

Camada Key Provider:
coleta chaves da KME, armazena localmente e expõe SKIP.

Camada Encryptor:
consome PPKs via SKIP e utiliza no IKEv2/RFC8784.
```

Essa separação permite validar a integração sem depender inicialmente de hardware QKD, NetSquid, BB84 ou canal quântico.

A contribuição principal desta baseline é desenhar e validar o ponto de integração entre uma fonte de chaves QKD exposta por uma interface ETSI 014 e um mecanismo de provisionamento de PPKs baseado em SKIP para uso posterior em IPsec/IKEv2 com RFC 8784.

---

## 14. Evoluções futuras

Após validar esta baseline, o projeto poderá evoluir para:

1. integração com implementação real de IKEv2/RFC8784;
2. automação de provisionamento em roteadores ou gateways IPsec;
3. integração com equipamentos QKD reais;
4. expansão de chaves com HKDF;
5. expansão de chaves com RanA;
6. comparação entre PPK estática e PPK derivada de QKD;
7. análise de latência de coleta da chave;
8. análise de taxa de consumo de PPKs;
9. política de rotação automática;
10. múltiplos peers e múltiplos Key Providers;
11. integração com HSM;
12. auditoria e trilhas de uso de chaves.

---

## 15. Hipótese da baseline

A hipótese desta baseline é:

> Se uma infraestrutura QKD disponibiliza chaves simétricas sincronizadas por uma interface compatível com ETSI GS QKD 014, então um Key Provider pode coletar essas chaves, armazená-las localmente, expô-las via SKIP e permitir seu uso como PPK em IKEv2/RFC8784, sem acoplar diretamente o encryptor à infraestrutura QKD.

---

## 16. Resultado esperado

Ao final da baseline, espera-se demonstrar que:

```text
Alice e Bob conseguem obter a mesma PPK,
a partir de KMEs simuladas compatíveis com ETSI 014,
por meio de Key Providers independentes,
usando SKIP como interface de entrega ao encryptor.
```

Esse resultado estabelece a base para uma arquitetura de integração entre QKD, ETSI 014, SKIP e IPsec/IKEv2 com RFC 8784.

```

**Confiança:** alto. A versão remove NetSquid, BB84 físico e canal quântico, e concentra a baseline na coleta lógica da chave QKD via API ETSI 014 simulada e na integração com Key Provider, SKIP e RFC 8784.
```
