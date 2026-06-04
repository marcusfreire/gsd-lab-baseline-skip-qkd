# Baseline: Subconjunto ETSI GS QKD 014 do KMS + SKIP

## 1. Objetivo

Este documento define a baseline local para integrar duas instancias logicas do
KMS Experimental Key-Management Simulator em `kms/`, dois Key Providers
independentes e dois encryptors simulados via SKIP.

A baseline atual nao define uma KME generica nem uma API "ETSI-like". Ela adota
o subconjunto fiel do ETSI GS QKD 014 implementado e documentado pelo `kms/`,
conforme os ADRs 0001-0005.

O objetivo e validar, em laboratorio, o seguinte fluxo:

```text
KME-A (kms/, ETSI 014 subset) -> KeyProvider-A -> SKIP -> Encryptor-A sim
       |                                                        |
       | deterministic fake key-source by key_ID                | keyId handoff
       v                                                        v
KME-B (kms/, ETSI 014 subset) -> KeyProvider-B -> SKIP -> Encryptor-B sim
```

O fluxo prova que os dois lados conseguem obter material de chave compativel
sem centralizar o estado dos Key Providers e sem simular fisica QKD.

## 2. Autoridade

Fontes normativas locais desta baseline:

- `kms/README.md`
- `kms/adrs/0001-experimental-key-management-simulator.md`
- `kms/adrs/0002-etsi-014-protocol-fidelity.md`
- `kms/adrs/0003-sae-identity-and-topology-policy.md`
- `kms/adrs/0004-key-source-and-etsi-storage-lifecycle.md`
- `kms/adrs/0005-testing-and-fidelity-guidelines.md`
- `docs/architecture.md`
- `docs/etsi014-alignment.md`
- `docs/api-etsi014-mock.md`
- `docs/api-skip.md`
- `docs/data-model.md`
- `docs/security-assumptions.md`

Autoridade externa:

- ETSI GS QKD 014 para a fronteira KME/SAE.
- `draft-singh-skip-00` para a fronteira Key Provider/encryptor.
- RFC 8784 apenas como contexto futuro para PPK/IKEv2.

## 3. Decisao de baseline

### 3.1 `kms/` e o KMS experimental

O diretorio `kms/` e o simulador oficial desta baseline. Ele e um KMS
experimental, nao um KMS de producao.

De acordo com o ADR 0001:

- o binario e configurado pelo modulo `config`;
- protocolos publicos vivem em modulos especificos;
- topologia, identidade, fonte de chaves e armazenamento sao preocupacoes
  compartilhadas do simulador;
- requisitos de producao como HSM, consistencia em cluster, hardening de
  segredos e controles de compliance estao fora do escopo.

Futuras fases devem estender `kms/` para comportamento KME/ETSI. Elas nao devem
criar um KME concorrente em `services/kme-mock`.

### 3.2 Duas KMEs, nao uma KME central

A baseline usa duas instancias separadas do simulador:

- `KME-A`, conectada a `SAE-A`.
- `KME-B`, conectada a `SAE-B`.

`KME-A` e `KME-B` possuem armazenamento KME separado. Elas podem compartilhar
somente uma politica deterministica de fake key-source/seed para que o mesmo
ETSI `key_ID` resulte nos mesmos bytes de chave nos dois lados durante testes.

Nao ha banco KME central e nao ha banco central de Key Providers.

## 4. Escopo

### 4.1 Dentro do escopo

- Entrega logica de chaves via subconjunto ETSI GS QKD 014 do `kms/`.
- Rotas publicas `status`, `enc_keys` e `dec_keys` sob `/api/v1/keys`.
- Identidade SAE resolvida por certificado mTLS ou por `x-sae-id` em modo HTTP
  local.
- Topologia SAE/KME como politica de autorizacao.
- Armazenamento ETSI por `key_ID`, com `master_sae_id`, `slave_sae_id` e bytes
  de chave.
- Consumo unico de `dec_keys`.
- KeyProvider-A e KeyProvider-B independentes, cada um com SQLite proprio.
- Exposicao SKIP para encryptors simulados.
- Mapeamento deterministico de `keyId` SKIP:
  `SKIP-{master_SAE_ID}-{slave_SAE_ID}-{key_ID}`.

### 4.2 Fora do escopo

- NetSquid.
- BB84.
- QBER.
- Canal quantico.
- Reconciliacao.
- Privacy amplification.
- Hardware QKD real.
- Trusted-node routing ou key relay multi-hop.
- Cisco real.
- IPsec/IKEv2/RFC8784 real.
- Seguranca de producao.

Esta baseline simula disponibilidade logica de material de chave. Ela nao
modela o processo fisico que geraria esse material.

## 5. Arquitetura

```text
                 ETSI GS QKD 014 subset                      SKIP subset

  +-----------------------------+                     +----------------------+
  | KME-A                       |                     | Encryptor-A sim      |
  | kms simulator instance      |                     | localSystemID=Alice  |
  | KME_ID=KME-A                |                     +----------+-----------+
  | connected SAE: SAE-A        |                                |
  | independent KME storage     |                                | GET /key?remoteSystemID=Bob
  +--------------+--------------+                                v
                 ^                                  +-------------+-------------+
                 | status, enc_keys                 | KeyProvider-A             |
                 | caller SAE: SAE-A                | ETSI client identity SAE-A|
                 | peer SAE: SAE-B                  | SKIP server for Alice     |
                 |                                  | independent SQLite DB     |
                 |                                  +-------------+-------------+
                 |                                                |
                 |                                                | keyId handoff
                 |                                                v
                 |                                  +-------------+-------------+
                 |                                  | KeyProvider-B             |
                 |                                  | ETSI client identity SAE-B|
                 |                                  | SKIP server for Bob       |
                 |                                  | independent SQLite DB     |
                 |                                  +-------------+-------------+
                 |                                                ^
                 v                                                | GET /key/{keyId}?remoteSystemID=Alice
  +--------------+--------------+                                |
  | KME-B                       |                     +----------+-----------+
  | kms simulator instance      |                     | Encryptor-B sim      |
  | KME_ID=KME-B                |                     | localSystemID=Bob    |
  | connected SAE: SAE-B        |                     +----------------------+
  | independent KME storage     |
  +-----------------------------+
```

## 6. Papel dos componentes

### 6.1 KME-A e KME-B

`KME-A` e `KME-B` sao instancias separadas do `kms/`.

Responsabilidades:

- expor o subconjunto ETSI 014 suportado;
- validar a topologia SAE/KME;
- resolver autorizacao antes de liberar chave;
- emitir containers ETSI com `key_ID` e `key`;
- armazenar chaves com ownership master/slave;
- consumir chaves apos `dec_keys` bem-sucedido.

Nao responsabilidades:

- nao sao servidores SKIP;
- nao sao encryptors;
- nao simulam canal quantico;
- nao sao KMS de producao.

### 6.2 SAE-A e SAE-B

`SAE-A` e `SAE-B` sao identidades ETSI representadas pelos Key Providers quando
eles chamam suas KMEs locais.

| Identidade ETSI | Representada por | KME local | Sistema SKIP |
|---|---|---|---|
| `SAE-A` | `KeyProvider-A` | `KME-A` | `Alice` |
| `SAE-B` | `KeyProvider-B` | `KME-B` | `Bob` |

### 6.3 KeyProvider-A e KeyProvider-B

Cada Key Provider atua em dois papeis:

- cliente ETSI 014/SAE diante da KME local;
- servidor SKIP diante do encryptor simulado local.

Regras:

- `KeyProvider-A` atua como `SAE-A` e chama somente `KME-A`.
- `KeyProvider-B` atua como `SAE-B` e chama somente `KME-B`.
- Cada provider possui seu proprio SQLite.
- Nao ha banco central entre providers.
- O provider converte `ETSI base64 -> bytes -> SKIP hex`.

### 6.4 Encryptor-A sim e Encryptor-B sim

Os encryptors simulados sao clientes SKIP. Eles nao sao SAEs ETSI.

- `Encryptor-A sim` chama `GET /key?remoteSystemID=Bob`.
- `Encryptor-B sim` recebe o `keyId` por handoff simulado e chama
  `GET /key/{keyId}?remoteSystemID=Alice`.

Cisco real e IKEv2/RFC8784 real sao fronteiras futuras.

## 7. Subconjunto ETSI GS QKD 014 adotado pelo `kms/`

O ADR 0002 exige fidelidade de rotas, metodos, nomes JSON publicos, status
codes e lifecycle para o subconjunto suportado.

### 7.1 Rotas publicas

| Metodo | Path | Uso na baseline |
|---|---|---|
| `GET` | `/api/v1/keys/{slave_SAE_ID}/status` | Consulta de status/topologia pelo master SAE para um slave SAE. |
| `POST` | `/api/v1/keys/{slave_SAE_ID}/enc_keys` | Forma completa de requisicao de chaves de cifragem. |
| `GET` | `/api/v1/keys/{slave_SAE_ID}/enc_keys` | Forma simples de requisicao de chaves de cifragem. |
| `POST` | `/api/v1/keys/{master_SAE_ID}/dec_keys` | Forma completa de recuperacao por IDs de chave. |
| `GET` | `/api/v1/keys/{master_SAE_ID}/dec_keys` | Forma simples de recuperacao por `key_ID`. |

Rotas genericas anteriores nao fazem parte da baseline atual. O contrato deve
usar somente os paths ETSI listados nesta secao.

### 7.2 Nomes JSON publicos

Os nomes no wire devem preservar o ETSI:

- `source_KME_ID`
- `target_KME_ID`
- `master_SAE_ID`
- `slave_SAE_ID`
- `key_ID`
- `key_IDs`
- `additional_slave_SAE_IDs`
- `extension_mandatory`
- `extension_optional`
- `status_extension`
- `key_container_extension`

Nomes internos Rust podem ser idiomaticos, mas a serializacao publica deve usar
os nomes ETSI.

### 7.3 `status`

`KeyProvider-A` como `SAE-A` consulta `KME-A` para o peer `SAE-B`:

```http
GET /api/v1/keys/SAE-B/status
x-sae-id: SAE-A
```

Resposta esperada:

```json
{
  "source_KME_ID": "KME-A",
  "target_KME_ID": "KME-B",
  "master_SAE_ID": "SAE-A",
  "slave_SAE_ID": "SAE-B",
  "key_size": 256,
  "stored_key_count": 1000000,
  "max_key_count": 1000000,
  "max_key_per_request": 128,
  "max_key_size": 256,
  "min_key_size": 256,
  "max_SAE_ID_count": 0,
  "status_extension": {}
}
```

### 7.4 `enc_keys`

`KeyProvider-A` como `SAE-A` solicita chave para `SAE-B`:

```http
POST /api/v1/keys/SAE-B/enc_keys
content-type: application/json
x-sae-id: SAE-A
```

```json
{ "number": 1, "size": 256 }
```

Resposta:

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

O `enc_keys` cria ou emite registro com:

- `master_sae_id = SAE-A`;
- `slave_sae_id = SAE-B`;
- `key_ID = QKD-000001`;
- bytes de chave;
- estado de disponibilidade.

### 7.5 `dec_keys`

`KeyProvider-B` como `SAE-B` recupera a chave para o master `SAE-A`:

```http
POST /api/v1/keys/SAE-A/dec_keys
content-type: application/json
x-sae-id: SAE-B
```

```json
{
  "key_IDs": [
    { "key_ID": "QKD-000001" }
  ]
}
```

Resposta:

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

`dec_keys` deve verificar:

- caller SAE e o slave armazenado;
- path `{master_SAE_ID}` e o master armazenado;
- existencia do `key_ID`;
- disponibilidade da chave.

`dec_keys` e one-time: depois de uma recuperacao bem-sucedida, o `kms/`
remove ou torna indisponivel a chave. Uma segunda recuperacao do mesmo
`key_ID` deve falhar.

## 8. Identidade e autorizacao

O ADR 0003 define topologia como politica de seguranca, nao como metadado de
status.

### 8.1 Fonte de identidade

| Modo | Fonte da identidade SAE |
|---|---|
| mTLS habilitado | Common Name do certificado de cliente autenticado |
| mTLS desabilitado | Header local `x-sae-id` |

`x-sae-id` so e valido quando mTLS esta desabilitado. Em modo mTLS, o caller
SAE vem do certificado, nao do header.

### 8.2 Rejeicoes obrigatorias

O simulador deve rejeitar antes de liberar material de chave:

- caller SAE desconhecido;
- peer SAE desconhecido;
- master incorreto;
- slave incorreto;
- ownership divergente;
- chave inexistente;
- chave ja consumida;
- tamanho de chave nao suportado;
- `extension_mandatory` nao suportado.

## 9. Fonte de chaves, storage e lifecycle

O ADR 0004 define que o material de chave vem de uma key-source explicita.

No runtime atual do `kms/`, a key-source usa aleatoriedade do sistema
operacional para chaves de 256 bits. Para esta baseline de duas KMEs, testes
podem usar uma fake key-source deterministica para que `KME-A` e `KME-B`
conhecam os mesmos bytes para o mesmo `key_ID` sem compartilhar storage.

Storage:

- SQLite e o backend duravel padrao configurado.
- Memoria e aceitavel para execucoes efemeras, testes unitarios e
  desenvolvimento local.
- O storage ETSI guarda `key_ID`, `master_sae_id`, `slave_sae_id` e bytes de
  chave.
- `dec_keys` apaga ou consome a chave apos recuperacao bem-sucedida.

## 10. SKIP e mapeamento de identificadores

O contrato SKIP segue `draft-singh-skip-00`.

### 10.1 Endpoints SKIP documentados

| Metodo | Path | Uso |
|---|---|---|
| `GET` | `/capabilities` | Capacidades do provider. |
| `GET` | `/key?remoteSystemID={id}` | Chave nova para sistema remoto. |
| `GET` | `/key?remoteSystemID={id}&size={bits}` | Chave nova com tamanho solicitado. |
| `GET` | `/key/{keyId}?remoteSystemID={id}` | Chave existente pelo `keyId`. |
| `GET` | `/entropy` | Entropia do provider. |
| `GET` | `/entropy?minentropy={bits}` | Entropia minima solicitada. |

### 10.2 Campos SKIP

- `localSystemID`
- `remoteSystemID`
- `keyId`
- `key`
- `entropy`

O `key` ETSI e base64. O `key` SKIP e hexadecimal.

### 10.3 `keyId` deterministico

O mapeamento baseline e:

```text
skip_key_id = SKIP-{master_SAE_ID}-{slave_SAE_ID}-{key_ID}
```

Exemplo:

```text
key_ID      = QKD-000001
master SAE  = SAE-A
slave SAE   = SAE-B
skip_key_id = SKIP-SAE-A-SAE-B-QKD-000001
```

O `keyId` nao contem material de chave. Ele e um identificador textual para
debug e handoff simulado.

## 11. Fluxo ponta a ponta

```text
1. KeyProvider-A as SAE-A -> KME-A
   GET /api/v1/keys/SAE-B/status

2. KeyProvider-A as SAE-A -> KME-A
   POST /api/v1/keys/SAE-B/enc_keys
   body: { "number": 1, "size": 256 }

3. KME-A returns:
   { "keys": [ { "key_ID": "QKD-000001", "key": "<base64>" } ] }

4. KeyProvider-A converts:
   ETSI base64 -> bytes -> SKIP hex

5. Encryptor-A -> KeyProvider-A
   GET /key?remoteSystemID=Bob
   response: { "keyId": "SKIP-SAE-A-SAE-B-QKD-000001", "key": "<hex>" }

6. Encryptor-A hands keyId to Encryptor-B through simulated handoff.

7. Encryptor-B -> KeyProvider-B
   GET /key/SKIP-SAE-A-SAE-B-QKD-000001?remoteSystemID=Alice

8. KeyProvider-B maps:
   SKIP-SAE-A-SAE-B-QKD-000001 -> QKD-000001

9. KeyProvider-B as SAE-B -> KME-B
   POST /api/v1/keys/SAE-A/dec_keys
   body: { "key_IDs": [ { "key_ID": "QKD-000001" } ] }

10. KME-B returns:
    { "keys": [ { "key_ID": "QKD-000001", "key": "<base64>" } ] }

11. KeyProvider-B converts:
    ETSI base64 -> bytes -> SKIP hex

12. Required invariant:
    key_hex_alice == key_hex_bob

13. KME-B consumes QKD-000001 after successful dec_keys.
```

## 12. Testes e fidelidade

O ADR 0005 define que a fidelidade deve ser testada em camadas.

Comandos exigidos para o `kms/`:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Testes unitarios devem cobrir:

- carregamento e validacao de configuracao;
- lookup de topologia;
- autorizacao SAE;
- key-source, incluindo fakes deterministicas quando adicionadas;
- storage ETSI;
- lifecycle one-time;
- validacao de requests;
- mapeamento de erros de protocolo.

Testes de router devem cobrir:

- paths e metodos ETSI exatos;
- formas GET simples e POST completas;
- nomes JSON ETSI via Serde;
- resposta de `status`;
- `enc_keys` com chave base64;
- `dec_keys` por `key_ID`;
- consumo unico apos `dec_keys`;
- rejeicoes de caller/master/slave desconhecidos;
- rejeicoes de ownership errado;
- tamanhos e extensoes obrigatorias nao suportadas;
- status codes e corpos de erro com formato ETSI quando aplicavel.

Testes de integracao futuros devem cobrir:

- servidor HTTP vivo;
- extracao de identidade mTLS por Common Name;
- persistencia SQLite apos restart;
- interoperabilidade com clientes SAE experimentais;
- modulos de protocolo futuros no mesmo binario.

## 13. Assuncoes de seguranca

- O material de chave e simulado.
- Esta baseline nao e production-ready.
- HTTP local com `x-sae-id` e apenas simplificacao de desenvolvimento/teste.
- mTLS, autenticacao de providers, hardening de segredos e redacao estruturada
  sao trabalhos futuros.
- Nunca registrar material completo de chave em logs; usar fingerprints.
- O `keyId` nao deve revelar material de chave.

## 14. Criterios de aceitacao da baseline

A baseline esta correta quando:

- `baseline.md` aponta `kms/` como simulador KME oficial.
- `KME-A` e `KME-B` sao instancias separadas, sem storage KME compartilhado.
- `KeyProvider-A` e `KeyProvider-B` tem estado SQLite separado.
- A API ETSI publica usa somente `status`, `enc_keys` e `dec_keys` nas rotas
  suportadas por ADR 0002.
- Os nomes JSON publicos preservam `source_KME_ID`, `target_KME_ID`,
  `master_SAE_ID`, `slave_SAE_ID`, `key_ID` e `key_IDs`.
- Identidade SAE respeita mTLS Common Name ou `x-sae-id` apenas sem mTLS.
- `dec_keys` e one-time.
- Material ETSI e base64; material SKIP e hexadecimal.
- O fluxo ponta a ponta preserva `key_hex_alice == key_hex_bob`.
- Cisco real e IKEv2/RFC8784 real permanecem futuros.
